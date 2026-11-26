#!/usr/bin/env python3
"""Karin language benchmark: replay labelled chats against a running karin-server.

Every chat runs in its own *incognito* session (a fresh engine loaded from the vault,
never saved or logged), so a bench run never touches Karin's live memory or chat logs.

    python bench/run_bench.py                         # server at http://127.0.0.1:8080
    python bench/run_bench.py --label after-likes     # name the run
    python bench/run_bench.py --start-server          # start a private server for the run (CI)
    python bench/run_bench.py --fail-under 0.6 --max-wrong 0.1   # exit 1 if worse

Writes bench/results/<time>-<label>/{report.md, summary.json, results.jsonl}
and compares with the previous run (regressions / fixes). Standard library only.

Scoring per turn (see bench/README.md):
  correct        right intent, right kind of response, expected values present
  partial        right intent, wrong kind of response (e.g. answered instead of asking)
  value_error    right intent and kind, an expected value missing
  wrong          answered with a different meaning            (the worst outcome)
  missed         in-scope, but Karin got stuck
  correct_reject noise, and Karin asked to repeat / got stuck
  false_accept   noise, but Karin answered as if she understood
"""

from __future__ import annotations

import argparse
import fnmatch
import json
import os
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request
from collections import Counter, defaultdict
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUTCOMES = ["correct", "partial", "value_error", "wrong", "missed", "correct_reject", "false_accept"]
GOOD = {"correct", "correct_reject"}
BAD = {"wrong", "false_accept"}


# ---------------------------------------------------------------------------
# HTTP
# ---------------------------------------------------------------------------

class Api:
    def __init__(self, base: str):
        self.base = base.rstrip("/")

    def call(self, method: str, path: str, body: dict | None = None):
        data = json.dumps(body).encode() if body is not None else None
        req = urllib.request.Request(
            self.base + path, data=data, method=method,
            headers={"Content-Type": "application/json"} if data else {},
        )
        with urllib.request.urlopen(req, timeout=60) as r:
            raw = r.read()
            return json.loads(raw) if raw else None

    def healthy(self) -> bool:
        try:
            return bool(self.call("GET", "/api/health").get("ok"))
        except Exception:
            return False


# ---------------------------------------------------------------------------
# Intent mapping
# ---------------------------------------------------------------------------

class IntentMap:
    def __init__(self, path: Path):
        spec = json.loads(path.read_text(encoding="utf-8"))
        self.neutral = set(spec["neutral"])
        self.clarify = set(spec["clarify"])
        self.patterns = [(pat, intent) for intent, pats in spec["intents"].items() for pat in pats]

    def intent_of(self, rule: str) -> str | None:
        hits = [(pat, intent) for pat, intent in self.patterns if fnmatch.fnmatchcase(rule, pat)]
        if not hits:
            return None
        # The most specific pattern wins (fewest wildcards, then longest).
        hits.sort(key=lambda h: (h[0].count("*"), -len(h[0])))
        return hits[0][1]


def family(intent: str) -> str:
    """All unsupported.* intents count as the same thing: an honest 'I can't do that yet'."""
    return "unsupported" if intent.startswith("unsupported.") else intent


def read_trace(trace: dict, imap: IntentMap) -> dict:
    """What Karin did in one turn: intents of the replying rules, kind of response, unmapped rules."""
    steps = trace.get("steps", [])
    intents, kinds, unmapped = [], [], []
    for i, step in enumerate(steps):
        if not any(e.startswith("=>") for e in step.get("effects", [])):
            continue
        rule = step["winnerName"]
        kind = "clarify" if rule in imap.clarify else "answer"
        decider = rule
        if rule in imap.neutral:
            clause = step.get("text", "")
            for prev in reversed(steps[:i]):
                if prev.get("text", "") == clause and prev["winnerName"] not in imap.neutral:
                    decider = prev["winnerName"]
                    break
        intent = imap.intent_of(decider)
        if intent is None:
            unmapped.append(decider)
            intent = f"unmapped:{decider}"
        if intent.startswith("unsupported."):
            kind = "decline"
        intents.append(intent)
        kinds.append(kind)
    if trace.get("impasse") or not intents:
        kind = "stuck"
    elif all(k == "clarify" for k in kinds):
        kind = "clarify"
    elif all(k == "decline" for k in kinds):
        kind = "decline"
    else:
        kind = "answer"
    return {"intents": intents, "kind": kind, "unmapped": unmapped}


def haystack(trace: dict) -> str:
    parts = [trace.get("reply", "")]
    for s in trace.get("steps", []):
        parts += list(s.get("bindings", {}).values())
        parts += s.get("effects", [])
    return " ".join(parts).lower()


def score(turn: dict, trace: dict, got: dict) -> str:
    expected = turn["intent"] if isinstance(turn["intent"], list) else [turn["intent"]]
    also = set(turn.get("also", []))
    respond = turn.get("respond", "answer")
    allowed_kinds = set(respond if isinstance(respond, list) else [respond])
    detected = {family(i) for i in got["intents"]}
    stuck = got["kind"] == "stuck"

    if expected == ["noise"]:
        if detected & {family(a) for a in also}:
            return "correct"
        return "correct_reject" if stuck else "false_accept"
    if stuck:
        return "missed"
    meaning_ok = all(family(e) in detected for e in expected) or bool(detected & {family(a) for a in also})
    if not meaning_ok:
        return "wrong"
    kind_ok = got["kind"] in allowed_kinds or ("answer" in allowed_kinds and got["kind"] == "decline"
                                               and all(e.startswith("unsupported.") for e in expected))
    if not kind_ok:
        return "partial"
    hay = haystack(trace)
    if any(str(v).lower() not in hay for v in turn.get("slots", [])):
        return "value_error"
    return "correct"


# ---------------------------------------------------------------------------
# Running
# ---------------------------------------------------------------------------

def load_cases(folder: Path, only: list[str]) -> list[dict]:
    cases = []
    for f in sorted(folder.glob("*.json")):
        case = json.loads(f.read_text(encoding="utf-8"))
        if case.get("skip"):
            continue
        if only and not any(o in case["chat"] for o in only):
            continue
        cases.append(case)
    return cases


def run_case(api: Api, case: dict, imap: IntentMap) -> list[dict]:
    sid = api.call("POST", "/api/incognito")["id"]
    rows = []
    try:
        for line in case.get("setup", []):
            api.call("POST", f"/api/incognito/{sid}/chat", {"input": line})
        for turn in case["turns"]:
            trace = api.call("POST", f"/api/incognito/{sid}/chat", {"input": turn["say"]})["trace"]
            if turn.get("skip"):
                continue
            got = read_trace(trace, imap)
            rows.append({
                "chat": case["chat"],
                "n": turn["n"],
                "say": turn["say"],
                "expected": turn["intent"],
                "also": turn.get("also", []),
                "respond": turn.get("respond", "answer"),
                "slots": turn.get("slots", []),
                "got": got["intents"],
                "kind": got["kind"],
                "outcome": score(turn, trace, got),
                "reply": trace.get("reply", ""),
                "understood": trace.get("understood", []),
                "missed": trace.get("missed", []),
                "rules": [s["winnerName"] for s in trace.get("steps", [])],
                "unmapped": got["unmapped"],
                "note": turn.get("note", ""),
            })
    finally:
        try:
            api.call("DELETE", f"/api/incognito/{sid}")
        except Exception:
            pass
    return rows


def start_server(port: int, cargo: str) -> subprocess.Popen:
    """Start a private karin-server: its own port, temp state file and build folder
    (so it never clashes with a karin-server you already have running)."""
    env = dict(os.environ, PORT=str(port), STATE_PATH=str(Path(tempfile.mkdtemp()) / "state.json"))
    target = ROOT / "software" / "target" / "bench"
    cmd = cargo.split() + ["run", "--quiet", "--target-dir", str(target), "-p", "karin-server"]
    proc = subprocess.Popen(cmd, cwd=ROOT / "software", env=env,
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    api = Api(f"http://127.0.0.1:{port}")
    for _ in range(1200):
        if api.healthy():
            return proc
        if proc.poll() is not None:
            sys.exit("karin-server exited before becoming healthy")
        time.sleep(0.5)
    proc.kill()
    sys.exit("karin-server did not become healthy in time")


# ---------------------------------------------------------------------------
# Reporting
# ---------------------------------------------------------------------------

def git_version() -> str:
    try:
        sha = subprocess.check_output(["git", "rev-parse", "--short", "HEAD"], cwd=ROOT, text=True).strip()
        dirty = subprocess.check_output(["git", "status", "--porcelain"], cwd=ROOT, text=True).strip()
        return sha + (" + uncommitted changes" if dirty else "")
    except Exception:
        return "unknown"


def summarize(rows: list[dict]) -> dict:
    c = Counter(r["outcome"] for r in rows)
    total = len(rows)
    in_scope = [r for r in rows if r["expected"] != "noise"]
    noise = [r for r in rows if r["expected"] == "noise"]
    pct = lambda n, d: round(n / d, 4) if d else 0.0
    by_intent = defaultdict(Counter)
    for r in rows:
        key = r["expected"] if isinstance(r["expected"], str) else "+".join(r["expected"])
        by_intent[key][r["outcome"]] += 1
    return {
        "turns": total,
        "outcomes": {o: c.get(o, 0) for o in OUTCOMES},
        "accuracy": pct(c["correct"] + c["correct_reject"], total),
        "understanding": pct(sum(1 for r in in_scope if r["outcome"] == "correct"), len(in_scope)),
        "wrong_rate": pct(c["wrong"] + c["false_accept"], total),
        "false_accept_rate": pct(c["false_accept"], len(noise)),
        "missed_rate": pct(c["missed"], len(in_scope)),
        "in_scope_turns": len(in_scope),
        "noise_turns": len(noise),
        "by_intent": {k: dict(v) for k, v in sorted(by_intent.items())},
        "unmapped_rules": sorted({u for r in rows for u in r["unmapped"]}),
    }


def previous_run(base: Path, current: Path) -> Path | None:
    runs = sorted(p for p in base.glob("*/results.jsonl") if p.parent != current)
    return runs[-1].parent if runs else None


def compare(rows: list[dict], prev_dir: Path | None) -> tuple[list, list]:
    if not prev_dir:
        return [], []
    prev = {}
    for line in (prev_dir / "results.jsonl").read_text(encoding="utf-8").splitlines():
        r = json.loads(line)
        prev[(r["chat"], r["n"])] = r
    worse, better = [], []
    for r in rows:
        p = prev.get((r["chat"], r["n"]))
        if not p or p["outcome"] == r["outcome"]:
            continue
        was_good, is_good = p["outcome"] in GOOD, r["outcome"] in GOOD
        if was_good and not is_good:
            worse.append((p, r))
        elif is_good and not was_good:
            better.append((p, r))
    return worse, better


def fmt_expected(r: dict) -> str:
    e = r["expected"] if isinstance(r["expected"], str) else " + ".join(r["expected"])
    return e + (f" (or {', '.join(r['also'])})" if r["also"] else "")


def report_md(meta: dict, s: dict, rows: list[dict], worse: list, better: list) -> str:
    o = s["outcomes"]
    lines = [
        f"# Karin language bench — {meta['label']}",
        "",
        f"- Run: {meta['time']} · code: `{meta['version']}` · server: {meta['server']}",
        f"- Cases: {meta['chats']} chats, {s['turns']} scored turns "
        f"({s['in_scope_turns']} in scope, {s['noise_turns']} noise)",
        f"- Compared with: {meta['baseline'] or '— (first run)'}",
        "",
        "## Headline",
        "",
        "| Metric | Value | Meaning |",
        "|---|---|---|",
        f"| **Accuracy** | **{s['accuracy']:.1%}** | correct + correctly rejected noise, over all turns |",
        f"| Understanding | {s['understanding']:.1%} | fully correct, over in-scope turns |",
        f"| **Wrong-answer rate** | **{s['wrong_rate']:.1%}** | wrong meaning or noise taken as meaning (lower is better) |",
        f"| False-accept rate | {s['false_accept_rate']:.1%} | noise answered as if understood |",
        f"| Missed rate | {s['missed_rate']:.1%} | in-scope turns where Karin got stuck |",
        "",
        "## Outcomes",
        "",
        "| Outcome | Turns |",
        "|---|---|",
    ]
    lines += [f"| {k} | {o[k]} |" for k in OUTCOMES]

    if worse or better:
        lines += ["", f"## Changes since previous run: {len(better)} fixed, {len(worse)} broken", ""]
        for title, pairs in (("Broken (was good, now not)", worse), ("Fixed (now good)", better)):
            if not pairs:
                continue
            lines += [f"### {title}", "", "| Chat · turn | Said | Before → now |", "|---|---|---|"]
            for p, r in pairs:
                lines.append(f"| {r['chat'][-6:]}·{r['n']} | {r['say']} | {p['outcome']} → **{r['outcome']}** |")
            lines.append("")

    lines += ["", "## By intent", "", "| Intent | Turns | ✓ | partial | value err | wrong | missed | noise ok | false acc |",
              "|---|---|---|---|---|---|---|---|---|"]
    for intent, c in sorted(s["by_intent"].items(), key=lambda kv: -sum(kv[1].values())):
        n = sum(c.values())
        lines.append(f"| {intent} | {n} | {c.get('correct', 0)} | {c.get('partial', 0)} | "
                     f"{c.get('value_error', 0)} | {c.get('wrong', 0)} | {c.get('missed', 0)} | "
                     f"{c.get('correct_reject', 0)} | {c.get('false_accept', 0)} |")

    sections = [
        ("wrong", "Wrong answers"), ("false_accept", "Noise taken as meaning"),
        ("value_error", "Right meaning, missing value"), ("partial", "Right meaning, wrong kind of response"),
        ("missed", "Missed (stuck on something in scope)"),
    ]
    for key, title in sections:
        items = [r for r in rows if r["outcome"] == key]
        if not items:
            continue
        lines += ["", f"## {title} ({len(items)})", "", "| Chat · turn | Said | Expected | Got | Reply |",
                  "|---|---|---|---|---|"]
        for r in items:
            got = ", ".join(r["got"]) or r["kind"]
            if key == "value_error":
                got += f" — needs {', '.join(r['slots'])}"
            if key == "partial":
                got += f" ({r['kind']}, wanted {r['respond']})"
            reply = r["reply"].replace("|", "\\|")
            reply = reply if len(reply) < 110 else reply[:107] + "…"
            lines.append(f"| {r['chat'][-6:]}·{r['n']} | {r['say']} | {fmt_expected(r)} | {got} | {reply} |")

    if s["unmapped_rules"]:
        lines += ["", "## Unmapped rules", "",
                  "These rules replied but have no intent in bench/intents.json (scored as a different meaning):", ""]
        lines += [f"- `{u}`" for u in s["unmapped_rules"]]
    return "\n".join(lines) + "\n"


# ---------------------------------------------------------------------------

def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--server", default="http://127.0.0.1:8080")
    ap.add_argument("--start-server", action="store_true", help="start a private karin-server for this run")
    ap.add_argument("--port", type=int, default=8099, help="port for --start-server")
    ap.add_argument("--cargo", default="cargo", help='cargo command for --start-server, e.g. "cargo +stable-x86_64-pc-windows-msvc"')
    ap.add_argument("--cases", type=Path, default=ROOT / "bench" / "cases")
    ap.add_argument("--intents", type=Path, default=ROOT / "bench" / "intents.json")
    ap.add_argument("--out", type=Path, default=ROOT / "bench" / "results")
    ap.add_argument("--label", default="run")
    ap.add_argument("--only", nargs="*", default=[], help="only chats whose id contains one of these")
    ap.add_argument("--fail-under", type=float, help="exit 1 if accuracy is below this (0..1)")
    ap.add_argument("--max-wrong", type=float, help="exit 1 if the wrong-answer rate is above this (0..1)")
    args = ap.parse_args()
    for stream in (sys.stdout, sys.stderr):
        try:
            stream.reconfigure(encoding="utf-8")  # Windows consoles default to a legacy code page
        except Exception:
            pass

    proc = None
    server = args.server
    if args.start_server:
        server = f"http://127.0.0.1:{args.port}"
        print(f"starting karin-server on {server} …", flush=True)
        proc = start_server(args.port, args.cargo)
    api = Api(server)
    if not api.healthy():
        print(f"karin-server is not reachable at {server} (start it, or use --start-server)", file=sys.stderr)
        return 2

    try:
        imap = IntentMap(args.intents)
        cases = load_cases(args.cases, args.only)
        rows = []
        for case in cases:
            rows += run_case(api, case, imap)
    except urllib.error.HTTPError as e:
        print(f"server error: {e} — does this server have /api/incognito?", file=sys.stderr)
        return 2
    finally:
        if proc:
            proc.terminate()

    stamp = datetime.now().strftime("%Y-%m-%d_%H%M%S")
    out_dir = args.out / f"{stamp}-{args.label}"
    out_dir.mkdir(parents=True, exist_ok=True)
    prev = previous_run(args.out, out_dir)
    worse, better = compare(rows, prev)
    s = summarize(rows)
    meta = {"label": args.label, "time": stamp, "version": git_version(), "server": server,
            "chats": len(cases), "baseline": prev.name if prev else None}
    (out_dir / "results.jsonl").write_text("".join(json.dumps(r, ensure_ascii=False) + "\n" for r in rows), encoding="utf-8")
    (out_dir / "summary.json").write_text(json.dumps({**meta, **s, "fixed": len(better), "broken": len(worse)},
                                                     indent=2, ensure_ascii=False), encoding="utf-8")
    (out_dir / "report.md").write_text(report_md(meta, s, rows, worse, better), encoding="utf-8")

    o = s["outcomes"]
    print(f"accuracy {s['accuracy']:.1%} · understanding {s['understanding']:.1%} · "
          f"wrong {s['wrong_rate']:.1%} · false-accept {s['false_accept_rate']:.1%} · missed {s['missed_rate']:.1%}")
    print("  " + " · ".join(f"{k} {o[k]}" for k in OUTCOMES))
    if prev:
        print(f"  vs {prev.name}: {len(better)} fixed, {len(worse)} broken")
    if s["unmapped_rules"]:
        print(f"  unmapped rules: {', '.join(s['unmapped_rules'])}")
    print(f"  report: {out_dir / 'report.md'}")

    failed = False
    if args.fail_under is not None and s["accuracy"] < args.fail_under:
        print(f"FAIL: accuracy {s['accuracy']:.1%} < {args.fail_under:.1%}", file=sys.stderr)
        failed = True
    if args.max_wrong is not None and s["wrong_rate"] > args.max_wrong:
        print(f"FAIL: wrong-answer rate {s['wrong_rate']:.1%} > {args.max_wrong:.1%}", file=sys.stderr)
        failed = True
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
