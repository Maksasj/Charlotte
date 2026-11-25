#!/usr/bin/env python3
"""Offline phrasing expander for Karin using a small local Ollama model (e.g. qwen3.5:4b).

Reads existing text-triggered rules from `vault/core/`, asks a local Ollama model
for natural paraphrases one rule-group at a time (tiny prompt, fits easily in 4k
context), strictly validates placeholders and Karin unification rules, optionally
checks against a running `karin-server` to skip already-handled phrasings and
verify `/api/selftest`, and writes auto-loaded rules to `vault/generated-<model>/`.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
import urllib.error
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Optional, Sequence, Set, Tuple

REPO_ROOT = Path(__file__).resolve().parent.parent
VAULT_DIR = REPO_ROOT / "vault"
CORE_DIR = VAULT_DIR / "core"

VAR_RE = re.compile(r"\$[A-Za-z0-9_]+")
THINK_RE = re.compile(r"<think>.*?</think>", re.DOTALL | re.IGNORECASE)
FORBIDDEN_CHARS = ('"', "`", "\n", "\r", "=>", "++", "@", "when:", "then:")

SAMPLE_VALUES: Dict[str, str] = {
    "$city": "Tokyo",
    "$topic": "drink",
    "$thing": "matcha",
    "$expr": "2 + 2",
    "$name": "Alice",
    "$org": "Acme",
    "$text": "buy milk",
    "$value": "5",
    "$from": "km",
    "$to": "m",
    "$entity": "Mars",
    "$subj": "Mars",
    "$obj": "Sun",
    "$pred": "orbits",
    "$item": "bread",
    "$date": "Friday",
    "$day": "Friday",
}


@dataclass(frozen=True)
class SeedRule:
    rule_id: str
    source_rel: str
    template: str
    extra_when: str
    then_lines: Tuple[str, ...]
    variables: Tuple[str, ...]

    @property
    def group_key(self) -> Tuple[str, Tuple[str, ...]]:
        return (self.extra_when, self.then_lines)


def normalize(text: str) -> str:
    collapsed = " ".join(text.strip().lower().split())
    return collapsed.rstrip("?!.,;").strip()


def slugify(text: str) -> str:
    slug = re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")
    return slug or "model"


def extract_vars(template: str) -> List[str]:
    return VAR_RE.findall(template)


def has_adjacent_vars(template: str) -> bool:
    tokens = normalize(template).split()
    return any(
        tokens[i].startswith("$") and tokens[i + 1].startswith("$")
        for i in range(len(tokens) - 1)
    )


def parse_when_line(when_rest: str) -> Optional[Tuple[str, str]]:
    s = when_rest.strip()
    if not s.startswith('"'):
        return None
    end = 1
    while end < len(s):
        if s[end] == "\\" and end + 1 < len(s):
            end += 2
            continue
        if s[end] == '"':
            break
        end += 1
    if end >= len(s):
        return None
    template = s[1:end]
    remainder = s[end + 1 :].lstrip()
    if remainder.startswith(","):
        remainder = remainder[1:].strip()
    return template, remainder


def parse_vault_rules(root: Path) -> List[SeedRule]:
    rules: List[SeedRule] = []
    if not root.exists():
        return rules
    for md_path in sorted(root.rglob("*.md")):
        rel = md_path.relative_to(VAULT_DIR).as_posix()
        in_fence = False
        parse_fence = False
        cur_id: Optional[str] = None
        cur_when: Optional[Tuple[str, str]] = None
        cur_then: List[str] = []
        in_then = False

        def flush() -> None:
            nonlocal cur_id, cur_when, cur_then, in_then
            if cur_id and cur_when and cur_then:
                tpl, extra = cur_when
                rules.append(
                    SeedRule(
                        rule_id=cur_id,
                        source_rel=rel,
                        template=tpl,
                        extra_when=extra,
                        then_lines=tuple(cur_then),
                        variables=tuple(extract_vars(tpl)),
                    )
                )
            cur_id = None
            cur_when = None
            cur_then = []
            in_then = False

        for raw_line in md_path.read_text(encoding="utf-8").splitlines():
            trimmed = raw_line.strip()
            if trimmed.startswith("```"):
                lang = trimmed[3:].strip()
                if not in_fence:
                    in_fence = True
                    parse_fence = lang == "" or lang.lower() == "karin"
                else:
                    flush()
                    in_fence = False
                    parse_fence = False
                continue
            if not in_fence or not parse_fence or not trimmed or trimmed.startswith("#"):
                continue

            if trimmed.startswith("rule "):
                flush()
                cur_id = trimmed[5:].strip()
                continue

            if cur_id is None:
                continue

            if trimmed.startswith("when:"):
                in_then = False
                parsed = parse_when_line(trimmed[5:])
                cur_when = parsed
            elif trimmed.startswith("then:"):
                in_then = True
                rest = trimmed[5:].strip()
                if rest:
                    cur_then.append(rest)
            elif in_then and (raw_line.startswith(" ") or raw_line.startswith("\t")):
                cur_then.append(trimmed)
            else:
                flush()
        flush()
    return rules


def collect_all_existing_templates() -> Set[str]:
    existing: Set[str] = set()
    for sub in sorted(VAULT_DIR.iterdir()):
        if sub.is_dir() and (sub.name == "core" or sub.name.startswith("generated-")):
            for r in parse_vault_rules(sub):
                existing.add(normalize(r.template))
    return existing


@dataclass(frozen=True)
class SeedFamily:
    family_id: str
    source_rel: str
    templates: Tuple[str, ...]
    branches: Tuple[SeedRule, ...]
    variables: Tuple[str, ...]


def group_seeds(rules: Sequence[SeedRule]) -> List[SeedFamily]:
    """Group rules that share either a `(extra_when, then)` target or a normalized template.

    This ensures multi-branch rules (e.g. known/unknown branches of `"what is my name"`)
    are prompted once and every accepted paraphrase is emitted for all branches.
    """
    n = len(rules)
    parent = list(range(n))

    def find(x: int) -> int:
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def union(a: int, b: int) -> None:
        ra, rb = find(a), find(b)
        if ra != rb:
            parent[rb] = ra

    by_action: Dict[Tuple[str, Tuple[str, ...]], int] = {}
    by_template: Dict[str, int] = {}
    for i, r in enumerate(rules):
        ak = r.group_key
        if ak in by_action:
            union(by_action[ak], i)
        else:
            by_action[ak] = i
        tk = normalize(r.template)
        if tk in by_template:
            union(by_template[tk], i)
        else:
            by_template[tk] = i

    clusters: Dict[int, List[SeedRule]] = {}
    order: List[int] = []
    for i, r in enumerate(rules):
        root = find(i)
        if root not in clusters:
            clusters[root] = []
            order.append(root)
        clusters[root].append(r)

    families: List[SeedFamily] = []
    for root in order:
        members = clusters[root]
        seen_tpl: Set[str] = set()
        templates: List[str] = []
        seen_branch: Set[Tuple[str, Tuple[str, ...]]] = set()
        branches: List[SeedRule] = []
        for r in members:
            tn = normalize(r.template)
            if tn not in seen_tpl:
                seen_tpl.add(tn)
                templates.append(r.template)
            if r.group_key not in seen_branch:
                seen_branch.add(r.group_key)
                branches.append(r)
        families.append(
            SeedFamily(
                family_id=members[0].rule_id,
                source_rel=members[0].source_rel,
                templates=tuple(templates),
                branches=tuple(branches),
                variables=members[0].variables,
            )
        )
    return families


def build_prompt(family: SeedFamily, count: int) -> str:
    examples = "\n".join(f'- "{t}"' for t in family.templates[:6])
    vars_list = list(family.variables)
    var_rule = (
        f"Every paraphrase MUST include these exact placeholders once each, separated by spaces: {', '.join(vars_list)}."
        if vars_list
        else "Do NOT include any $variables."
    )
    return (
        f"Generate {count} natural, diverse English ways a user might say the same thing as:\n"
        f"{examples}\n\n"
        f"Rules:\n"
        f"1. {var_rule}\n"
        f"2. Keep each paraphrase short (2 to 10 words), lowercase or normal English.\n"
        f"3. Vary phrasing naturally (polite forms, casual forms, short forms, plural/singular).\n"
        f"4. Do NOT attach punctuation to a $variable (write '$city' not '$city?' or '$city\\'s').\n"
        f"5. Output ONLY a JSON array of strings, nothing else.\n"
    )


def call_ollama(
    ollama_url: str,
    model: str,
    prompt: str,
    num_ctx: int,
    temperature: float,
    timeout: int,
) -> str:
    url = ollama_url.rstrip("/") + "/api/generate"
    payload = {
        "model": model,
        "prompt": prompt,
        "stream": False,
        "think": False,
        "options": {
            "num_ctx": num_ctx,
            "temperature": temperature,
            "num_predict": 512,
        },
    }
    req = urllib.request.Request(
        url,
        data=json.dumps(payload).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            return str(data.get("response", ""))
    except urllib.error.URLError as exc:
        raise RuntimeError(f"Failed to reach Ollama at {url}: {exc}") from exc


def parse_candidates(raw: str) -> List[str]:
    cleaned = THINK_RE.sub("", raw).strip()
    start = cleaned.find("[")
    end = cleaned.rfind("]")
    if start != -1 and end > start:
        try:
            parsed = json.loads(cleaned[start : end + 1])
            if isinstance(parsed, list):
                return [str(x).strip() for x in parsed if isinstance(x, str)]
        except json.JSONDecodeError:
            pass

    lines: List[str] = []
    for line in cleaned.splitlines():
        item = line.strip()
        item = re.sub(r"^[-*•\d.)]+\s*", "", item).strip()
        if len(item) >= 2 and item[0] == item[-1] and item[0] in ('"', "'"):
            item = item[1:-1].strip()
        if item:
            lines.append(item)
    return lines


def validate_candidate(
    candidate: str,
    seed: SeedRule,
    existing_norm: Set[str],
    allow_adjacent_vars: bool,
) -> Optional[str]:
    text = " ".join(candidate.strip().split()).rstrip("?!.,;").strip()
    if not text:
        return None
    if any(bad in text for bad in FORBIDDEN_CHARS):
        return None

    norm = normalize(text)
    if not norm or norm in existing_norm:
        return None

    tokens = norm.split()
    if len(tokens) > 12:
        return None

    expected_vars = list(seed.variables)
    found_vars = extract_vars(text)
    if sorted(found_vars) != sorted(expected_vars):
        return None

    # Every $var must be an exact standalone token (unify.rs splits on whitespace).
    for tok in text.split():
        if "$" in tok and not re.fullmatch(r"\$[A-Za-z0-9_]+", tok):
            return None

    # Require at least one literal word so a rule never matches every message.
    literal_tokens = [t for t in tokens if not t.startswith("$")]
    if not literal_tokens:
        return None
    if not expected_vars and len(literal_tokens) < 2:
        return None
    seed_literals = [t for t in normalize(seed.template).split() if not t.startswith("$")]
    if len(literal_tokens) == 1 and len(seed_literals) >= 2:
        return None

    if not allow_adjacent_vars and has_adjacent_vars(text):
        return None

    return text


def fill_sample(template: str) -> str:
    out = template
    for var in sorted(set(extract_vars(template)), key=len, reverse=True):
        out = out.replace(var, SAMPLE_VALUES.get(var, "alpha"))
    return out


def http_json(url: str, body: Optional[dict] = None, timeout: int = 10) -> dict:
    data = json.dumps(body).encode("utf-8") if body is not None else None
    req = urllib.request.Request(
        url,
        data=data,
        headers={"Content-Type": "application/json"} if data is not None else {},
        method="POST" if data is not None else "GET",
    )
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        return json.loads(resp.read().decode("utf-8"))


def karin_is_reachable(karin_url: str) -> bool:
    try:
        http_json(karin_url.rstrip("/") + "/api/state", timeout=3)
        return True
    except Exception:
        return False


def karin_already_handles(karin_url: str, utterance: str) -> bool:
    base = karin_url.rstrip("/")
    http_json(base + "/api/chat/new", {})
    res = http_json(base + "/api/chat", {"message": utterance})
    reply = str(res.get("reply", "")).lower()
    return "impasse" not in reply and "don't know how to handle" not in reply


def karin_reload_and_selftest(karin_url: str) -> Tuple[bool, str]:
    base = karin_url.rstrip("/")
    http_json(base + "/api/reset", {})
    res = http_json(base + "/api/selftest", {})
    results = res.get("results", [])
    passed = sum(1 for r in results if r.get("passed"))
    total = len(results)
    failed = [r.get("name", "?") for r in results if not r.get("passed")]
    if passed == total and total > 0:
        return True, f"{passed}/{total} selftests passed"
    return False, f"{passed}/{total} passed; failed: {', '.join(failed)}"


def render_rule(rule_id: str, template: str, seed: SeedRule) -> str:
    when = f'when: "{template}"'
    if seed.extra_when:
        when += f", {seed.extra_when}"
    if len(seed.then_lines) == 1:
        then = f"then: {seed.then_lines[0]}"
    else:
        body = "\n".join(f"  {line}" for line in seed.then_lines)
        then = f"then:\n{body}"
    return f"rule {rule_id}\n{when}\n{then}"


def render_module(model: str, sections: List[Tuple[SeedFamily, List[str]]]) -> str:
    lines = [
        f"# Expanded Phrasings ({model})",
        "",
        "Auto-generated paraphrase rules produced by `scripts/expand_phrasings.py`.",
        "",
    ]
    model_prefix = slugify(model)
    for family, phrasings in sections:
        lines.append(f"## From `{family.family_id}` ({family.source_rel})")
        lines.append("")
        lines.append("```karin")
        for idx, phrase in enumerate(phrasings, start=1):
            for branch in family.branches:
                rid = f"{model_prefix}-{branch.rule_id}-p{idx}"
                lines.append(render_rule(rid, phrase, branch))
                lines.append("")
        if lines[-1] == "":
            lines.pop()
        lines.append("```")
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def main(argv: Optional[Sequence[str]] = None) -> int:
    parser = argparse.ArgumentParser(
        description="Expand Karin vault phrasings using a local Ollama model (e.g. qwen3.5:4b)."
    )
    parser.add_argument("--model", default="qwen3.5:4b", help="Ollama model tag (default: qwen3.5:4b)")
    parser.add_argument("--ollama-url", default="http://127.0.0.1:11434", help="Ollama base URL")
    parser.add_argument("--karin-url", default="http://127.0.0.1:8080", help="Karin server base URL")
    parser.add_argument(
        "--source",
        default="vault/core",
        help="Vault file or directory to read seed rules from (default: vault/core)",
    )
    parser.add_argument(
        "--only",
        default="",
        help="Optional regex to filter seed rule IDs or templates (e.g. 'intent-weather|ask-user-likes')",
    )
    parser.add_argument("--count", type=int, default=10, help="Paraphrases to request per rule group")
    parser.add_argument("--limit", type=int, default=0, help="Max rule groups to process (0 = all)")
    parser.add_argument("--ctx", type=int, default=4096, help="Ollama num_ctx (default: 4096)")
    parser.add_argument("--temperature", type=float, default=0.7, help="Sampling temperature")
    parser.add_argument("--timeout", type=int, default=120, help="Ollama request timeout in seconds")
    parser.add_argument(
        "--out",
        default="",
        help="Output markdown path (default: vault/generated-<model>/phrasings.md)",
    )
    parser.add_argument("--no-verify", action="store_true", help="Skip live karin-server checks")
    parser.add_argument("--dry-run", action="store_true", help="Print generated markdown without writing")
    parser.add_argument(
        "--list-seeds",
        action="store_true",
        help="List matching seed rule groups and exit without calling Ollama",
    )
    args = parser.parse_args(argv)

    source_path = (REPO_ROOT / args.source).resolve() if not Path(args.source).is_absolute() else Path(args.source)
    if source_path.is_file():
        seed_rules = [r for r in parse_vault_rules(source_path.parent) if (VAULT_DIR / r.source_rel).resolve() == source_path]
    else:
        seed_rules = parse_vault_rules(source_path)

    if args.only:
        pat = re.compile(args.only, re.IGNORECASE)
        seed_rules = [r for r in seed_rules if pat.search(r.rule_id) or pat.search(r.template)]

    groups = group_seeds(seed_rules)
    if args.limit > 0:
        groups = groups[: args.limit]

    if args.list_seeds:
        print(f"Found {len(groups)} seed phrasing family(s) ({len(seed_rules)} rules):")
        for fam in groups:
            samples = ", ".join(f'"{t}"' for t in fam.templates[:3])
            extra = f" (+{len(fam.templates) - 3} more)" if len(fam.templates) > 3 else ""
            binfo = f" [{len(fam.branches)} branches]" if len(fam.branches) > 1 else ""
            print(f"  - {fam.family_id}{binfo} ({fam.source_rel}): {samples}{extra}")
        return 0

    if not groups:
        print("No matching seed rules found.", file=sys.stderr)
        return 1

    verify_live = not args.no_verify and karin_is_reachable(args.karin_url)
    if not args.no_verify and not verify_live:
        print(f"Note: karin-server not reachable at {args.karin_url}; skipping live checks.", file=sys.stderr)

    existing_norm = collect_all_existing_templates()
    sections: List[Tuple[SeedFamily, List[str]]] = []
    total_added = 0

    for idx, family in enumerate(groups, start=1):
        seed = family.branches[0]
        allow_adj = any(has_adjacent_vars(t) for t in family.templates)
        prompt = build_prompt(family, args.count)
        print(f"[{idx}/{len(groups)}] Expanding {family.family_id} (\"{family.templates[0]}\")...", flush=True)
        try:
            raw = call_ollama(
                args.ollama_url,
                args.model,
                prompt,
                num_ctx=args.ctx,
                temperature=args.temperature,
                timeout=args.timeout,
            )
        except RuntimeError as exc:
            print(f"  ERROR: {exc}", file=sys.stderr)
            return 2

        raw_candidates = parse_candidates(raw)
        accepted: List[str] = []
        for cand in raw_candidates:
            valid = validate_candidate(cand, seed, existing_norm, allow_adj)
            if not valid:
                continue
            if verify_live:
                sample_utterance = fill_sample(valid)
                if karin_already_handles(args.karin_url, sample_utterance):
                    continue
            existing_norm.add(normalize(valid))
            accepted.append(valid)

        print(
            f"  -> {len(accepted)}/{len(raw_candidates)} valid new phrasings"
            f" ({len(accepted) * len(family.branches)} rules)",
            flush=True,
        )
        if accepted:
            sections.append((family, accepted))
            total_added += len(accepted) * len(family.branches)

    if not sections:
        print("No new valid phrasings were produced.")
        return 0

    md = render_module(args.model, sections)
    if args.dry_run:
        print("\n" + md)
        return 0

    model_dir = VAULT_DIR / f"generated-{slugify(args.model)}"
    out_path = Path(args.out) if args.out else model_dir / "phrasings.md"
    if not out_path.is_absolute():
        out_path = REPO_ROOT / out_path
    out_path.parent.mkdir(parents=True, exist_ok=True)
    previous = out_path.read_text(encoding="utf-8") if out_path.exists() else None
    out_path.write_text(md, encoding="utf-8")
    print(f"Wrote {total_added} rules to {out_path.relative_to(REPO_ROOT)}")

    if verify_live:
        ok, summary = karin_reload_and_selftest(args.karin_url)
        print(f"Selftest after reload: {summary}")
        if not ok:
            if previous is None:
                out_path.unlink(missing_ok=True)
            else:
                out_path.write_text(previous, encoding="utf-8")
            karin_reload_and_selftest(args.karin_url)
            print("Rolled back output because selftest regressed!", file=sys.stderr)
            return 3

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
