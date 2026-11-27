#!/usr/bin/env python3
"""Karin bench: schema-free checks of what Karin understood.

Each test is a TOML file anywhere under bench/data/ (all subfolders, e.g. data/chats/): optional `before` lines and
`text` (fed to Karin, not scored), then `[[turn]]`s that are scored by looking
only at Karin's reply and whether she was stuck (an impasse) - never at rule or
intent names, so the tests survive any change to the vault's schema.

    chat   = "chat-1795695278"         # id (any string)
    split  = "test"                    # dev: rules were written from it; test: unseen
    before = ["my name is Maksim"]     # said first, not scored
    text   = '''Oscar moved to Paris. He loves tea.'''   # read sentence by sentence, not scored

    [[turn]]
    say      = "where does Oscar live"
    mentions = ["Paris"]               # the reply must contain each (case-insensitive)
    avoid    = ["Learned"]             # the reply must contain none of these
    tags     = ["people.where"]        # free labels for the per-topic breakdown
    note     = "..."                   # free text

    expect = "understand"  (default)   she must not be stuck
           = "ask"                     she must ask a question back (not stuck, reply has "?")
           = "ignore"                  noise: she must not act on it (stuck)
           = "any"                     not scored (still said, for context)

Every file runs in its own incognito session (never saved, never logged).

    python bench/run.py --start-server --cargo "cargo +stable-x86_64-pc-windows-msvc" --label my-change
    python bench/run.py --label my-change --baseline earlier-label

Results go to bench/results/<time>-<label>/ (git-ignored). Standard library only.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
import time
import tomllib
import urllib.error
import urllib.request
from collections import Counter, defaultdict
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FORMAT = 2  # results format; runs of the older intent-based bench are never used as a baseline


# ---------------------------------------------------------------------------
# Server
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


def git_version() -> str:
    try:
        sha = subprocess.check_output(["git", "rev-parse", "--short", "HEAD"], cwd=ROOT, text=True).strip()
        dirty = subprocess.check_output(["git", "status", "--porcelain"], cwd=ROOT, text=True).strip()
        return sha + (" + uncommitted changes" if dirty else "")
    except Exception:
        return "unknown"



EXPECTS = {"understand", "ask", "ignore", "any"}
TURN_KEYS = {"say", "expect", "mentions", "avoid", "tags", "note"}
FILE_KEYS = {"chat", "split", "source", "labels", "before", "text", "turn"}


# ---------------------------------------------------------------------------
# Loading (strict: unknown keys and values are errors)
# ---------------------------------------------------------------------------

def load(folder: Path, only: list[str], split: str | None) -> list[dict]:
    tests = []
    for path in sorted(folder.rglob("*.toml")):
        data = tomllib.loads(path.read_text(encoding="utf-8"))
        where = path.relative_to(folder).as_posix()
        bad = set(data) - FILE_KEYS
        if bad:
            sys.exit(f"{where}: unknown keys {sorted(bad)}")
        data.setdefault("chat", path.stem)
        data.setdefault("split", "dev")
        if data["split"] not in ("dev", "test"):
            sys.exit(f"{where}: split must be dev or test")
        for i, t in enumerate(data.get("turn", []), 1):
            bad = set(t) - TURN_KEYS
            if bad:
                sys.exit(f"{where} turn {i}: unknown keys {sorted(bad)}")
            if not isinstance(t.get("say"), str) or not t["say"].strip():
                sys.exit(f"{where} turn {i}: needs say")
            t.setdefault("expect", "understand")
            if t["expect"] not in EXPECTS:
                sys.exit(f"{where} turn {i}: expect must be one of {sorted(EXPECTS)}")
            if (t.get("mentions") or t.get("avoid")) and t["expect"] != "understand":
                sys.exit(f"{where} turn {i}: mentions and avoid only go with expect = understand")
            t["n"] = i
        data["file"] = where
        if only and not any(o in data["chat"] for o in only):
            continue
        if split and data["split"] != split:
            continue
        tests.append(data)
    return tests


def sentences(text: str) -> list[str]:
    """Split prose into sentences, one per line or ending in . ! ?"""
    out = []
    for block in re.split(r"\n\s*\n|\n(?=\s*[-*] )", text.strip()):
        block = re.sub(r"^\s*[-*]\s+", "", block.strip())
        out += [s.strip() for s in re.split(r"(?<=[.!?])\s+", block.replace("\n", " ")) if s.strip()]
    return out


# ---------------------------------------------------------------------------
# Scoring (only the reply text and the impasse flag)
# ---------------------------------------------------------------------------

def check(turn: dict, reply: str, stuck: bool) -> tuple[str, str]:
    """(outcome, why): outcome is pass, fail or skip."""
    expect = turn["expect"]
    if expect == "any":
        return "skip", ""
    if expect == "ignore":
        return ("pass", "") if stuck else ("fail", "acted on noise")
    if stuck:
        return "fail", "stuck"
    if expect == "ask":
        return ("pass", "") if "?" in reply else ("fail", "did not ask")
    low = reply.lower()
    missing = [m for m in turn.get("mentions", []) if str(m).lower() not in low]
    if missing:
        return "fail", "missing " + ", ".join(map(str, missing))
    present = [a for a in turn.get("avoid", []) if str(a).lower() in low]
    if present:
        return "fail", "said " + ", ".join(map(str, present))
    return "pass", ""


def kind_of(turn: dict) -> str:
    if turn["expect"] != "understand":
        return turn["expect"]
    return "mentions" if turn.get("mentions") or turn.get("avoid") else "understand"


def run_test(api: Api, test: dict) -> list[dict]:
    sid = api.call("POST", "/api/incognito")["id"]
    rows = []
    try:
        for line in test.get("before", []) + sentences(test.get("text", "")):
            api.call("POST", f"/api/incognito/{sid}/chat", {"input": line})
        for t in test.get("turn", []):
            trace = api.call("POST", f"/api/incognito/{sid}/chat", {"input": t["say"]})["trace"]
            reply = trace.get("reply", "")
            stuck = bool(trace.get("impasse"))
            outcome, why = check(t, reply, stuck)
            rows.append({
                "chat": test["chat"], "file": test["file"], "split": test["split"], "n": t["n"], "say": t["say"],
                "kind": kind_of(t), "mentions": t.get("mentions", []), "avoid": t.get("avoid", []),
                "tags": t.get("tags", []),
                "outcome": outcome, "why": why, "stuck": stuck, "reply": reply,
            })
    finally:
        try:
            api.call("DELETE", f"/api/incognito/{sid}")
        except Exception:
            pass
    return rows


# ---------------------------------------------------------------------------
# Summary and report
# ---------------------------------------------------------------------------

def summarize(rows: list[dict]) -> dict:
    scored = [r for r in rows if r["outcome"] != "skip"]
    pct = lambda rs: round(sum(r["outcome"] == "pass" for r in rs) / len(rs), 4) if rs else None
    of = lambda *kinds: [r for r in scored if r["kind"] in kinds]
    handle = of("understand", "mentions", "ask")
    by_tag = defaultdict(Counter)
    for r in scored:
        for tag in r["tags"] or ["(untagged)"]:
            by_tag[tag][r["outcome"]] += 1
    return {
        "turns": len(rows),
        "scored": len(scored),
        "pass": pct(scored),
        # Not stuck on things she should handle (coverage).
        "understood": round(sum(not r["stuck"] for r in handle) / len(handle), 4) if handle else None,
        "answers": pct(of("mentions")),
        "asks": pct(of("ask")),
        "ignores": pct(of("ignore")),
        "counts": {k: len(of(k)) for k in ("understand", "mentions", "ask", "ignore")},
        "by_tag": {k: dict(v) for k, v in sorted(by_tag.items())},
    }


def previous(base: Path, current: Path, baseline: str | None) -> Path | None:
    def ours(run: Path) -> bool:
        try:
            return json.loads((run / "summary.json").read_text(encoding="utf-8")).get("format") == FORMAT
        except Exception:
            return False
    runs = sorted(p.parent for p in base.glob("*/results.jsonl") if p.parent != current and ours(p.parent))
    if baseline:
        named = [r for r in runs if r.name == baseline or r.name.endswith(f"-{baseline}")]
        if not named:
            sys.exit(f"no earlier run (of this bench format) matches --baseline {baseline!r} in {base}")
        return named[-1]
    return runs[-1] if runs else None


def compare(rows: list[dict], prev_dir: Path | None) -> tuple[list, list]:
    if not prev_dir:
        return [], []
    prev = {}
    for line in (prev_dir / "results.jsonl").read_text(encoding="utf-8").splitlines():
        r = json.loads(line)
        prev[(r["chat"], r["n"])] = r
    broken, fixed = [], []
    for r in rows:
        p = prev.get((r["chat"], r["n"]))
        if not p or p["outcome"] == r["outcome"] or "skip" in (p["outcome"], r["outcome"]):
            continue
        (fixed if r["outcome"] == "pass" else broken).append((p, r))
    return broken, fixed


def fmt(v) -> str:
    return "-" if v is None else f"{v:.1%}"


def headline(s: dict) -> str:
    return (f"pass {fmt(s['pass'])} · understood {fmt(s['understood'])} · answers {fmt(s['answers'])} · "
            f"asks {fmt(s['asks'])} · ignores noise {fmt(s['ignores'])}")


def report(meta: dict, s: dict, splits: dict, rows: list[dict], broken: list, fixed: list) -> str:
    esc = lambda x: str(x).replace("|", "\\|")
    out = [f"# Bench: {meta['label']}", "",
           f"{meta['time']} · {meta['version']} · {meta['files']} files · {s['scored']} scored turns", "",
           "| | pass | understood | answers | asks | ignores noise |", "|---|---|---|---|---|---|"]
    for name, v in [("all", s), *splits.items()]:
        out.append(f"| {name} | {fmt(v['pass'])} | {fmt(v['understood'])} | {fmt(v['answers'])} | {fmt(v['asks'])} | {fmt(v['ignores'])} |")
    out += ["", "- **understood**: not stuck on a turn she should handle",
            "- **answers**: the reply mentions every expected value (and none to avoid)",
            "- **asks**: she asked a question back when she should",
            "- **ignores noise**: stuck (did not act) on noise", ""]
    if meta["baseline"]:
        out += [f"## Compared with {meta['baseline']}: {len(fixed)} fixed, {len(broken)} broken", ""]
        for title, pairs in (("Broken", broken), ("Fixed", fixed)):
            if pairs:
                out += [f"### {title}", "", "| file · turn | said | before → now |", "|---|---|---|"]
                for p, r in pairs:
                    out.append(f"| {r['chat'][-6:]}·{r['n']} | {esc(r['say'])} | {p['outcome']} → **{r['outcome']}** {esc(r['why'])} |")
                out.append("")
    fails = [r for r in rows if r["outcome"] == "fail"]
    out += [f"## Failing turns ({len(fails)})", "", "| file · turn | said | why | reply |", "|---|---|---|---|"]
    for r in fails:
        out.append(f"| {r['chat'][-6:]}·{r['n']} | {esc(r['say'])} | {esc(r['why'])} | {esc(r['reply'][:90])} |")
    out += ["", "## By tag", "", "| tag | pass | fail |", "|---|---|---|"]
    for tag, c in s["by_tag"].items():
        out.append(f"| {tag} | {c.get('pass', 0)} | {c.get('fail', 0)} |")
    return "\n".join(out) + "\n"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--server", default="http://127.0.0.1:8080")
    ap.add_argument("--start-server", action="store_true", help="start a private karin-server for this run")
    ap.add_argument("--port", type=int, default=8099)
    ap.add_argument("--cargo", default="cargo")
    ap.add_argument("--tests", type=Path, default=ROOT / "bench" / "data")
    ap.add_argument("--out", type=Path, default=ROOT / "bench" / "results")
    ap.add_argument("--label", default="run")
    ap.add_argument("--baseline", help="compare with this earlier run (label or folder name) instead of the latest")
    ap.add_argument("--only", nargs="*", default=[])
    ap.add_argument("--split", choices=["dev", "test"])
    ap.add_argument("--fail-under", type=float, help="exit 1 if the pass rate is below this (0..1)")
    args = ap.parse_args()
    for stream in (sys.stdout, sys.stderr):
        try:
            stream.reconfigure(encoding="utf-8")
        except Exception:
            pass

    tests = load(args.tests, args.only, args.split)
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
        rows = [r for t in tests for r in run_test(api, t)]
    except urllib.error.HTTPError as e:
        print(f"server error: {e} — does this server have /api/incognito?", file=sys.stderr)
        return 2
    finally:
        if proc:
            proc.terminate()

    stamp = datetime.now().strftime("%Y-%m-%d_%H%M%S")
    out_dir = args.out / f"{stamp}-{args.label}"
    out_dir.mkdir(parents=True, exist_ok=True)
    prev = previous(args.out, out_dir, args.baseline)
    broken, fixed = compare(rows, prev)
    s = summarize(rows)
    splits = {sp: summarize([r for r in rows if r["split"] == sp]) for sp in sorted({r["split"] for r in rows})}
    meta = {"format": FORMAT, "label": args.label, "time": stamp, "version": git_version(), "server": server,
            "files": len(tests), "baseline": prev.name if prev else None}
    (out_dir / "results.jsonl").write_text("".join(json.dumps(r, ensure_ascii=False) + "\n" for r in rows), encoding="utf-8")
    (out_dir / "summary.json").write_text(json.dumps({**meta, **s, "fixed": len(fixed), "broken": len(broken),
                                                      "splits": {k: {kk: v[kk] for kk in ("scored", "pass", "understood", "answers", "asks", "ignores")} for k, v in splits.items()}},
                                                     indent=2, ensure_ascii=False), encoding="utf-8")
    (out_dir / "report.md").write_text(report(meta, s, splits, rows, broken, fixed), encoding="utf-8")

    print(headline(s))
    for sp, v in splits.items():
        print(f"  {sp:<4} {v['scored']:>3} scored: {headline(v)}")
    if prev:
        print(f"  vs {prev.name}: {len(fixed)} fixed, {len(broken)} broken")
    print(f"  report: {out_dir / 'report.md'}")
    if args.fail_under is not None and (s["pass"] or 0) < args.fail_under:
        print(f"FAIL: pass {fmt(s['pass'])} < {args.fail_under:.1%}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
