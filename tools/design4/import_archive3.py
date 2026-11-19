"""Archive3 import (Design 4.0 section 8.4, decision D4.2).

3.0's cached formal readings (Haiku's `parse_fallback` answers) of train and dev procedures
become raw teacher messages on channel `archive3`, one session per task, to be replayed
through the agent's conversation path. They are never injected. Readings tied to H3 or to
gardening are excluded. A reading's facts are mapped where they fit schema v4 (predicate and
arity); facts that do not fit are counted as dropped, and a reading with nothing mappable is
listed for re-teaching. 3.0's `situation_fallback` readings carry no difference facts, so they
are listed as re-teach (stage 7 pairs situations with their changes live).

Usage: python3 -I tools/design4/import_archive3.py
Writes data/kb4/raw/imports/archive3.jsonl (manifest) and data/kb4/raw/sessions/a3-<task>.jsonl.
Zero model calls; the 3.0 cache is only read.
"""
import json
import os
import re
from collections import Counter, defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
CACHE = os.path.join(ROOT, "data", "training", "calls3", "cache.jsonl")
OUT = os.path.join(ROOT, "data", "kb4", "raw")

# schema v4 predicates a procedure reading may carry, with arity (crates/agent4/src/sorts.rs)
V4 = {"task": 1, "step": 2, "action": 2, "object_of": 2, "uses": 2, "needs": 2, "role": 3, "before": 2,
      "overlaps": 2, "requires_state": 2, "achieves": 2, "into": 2, "onto": 2, "from": 2, "takes_time": 4, "until": 2,
      "purpose": 2, "condition": 2, "check": 2, "warning": 2, "setting": 4, "amount": 4, "optional": 1}
ATOM = re.compile(r"^\s*(not\s+)?([a-z_][a-z0-9_]*)\(([^()]*)\)\s*\.?\s*$")


def tasks_allowed():
    """Train and dev tasks of data/curriculum3 with their domain; never H3 or gardening."""
    out = {}
    for split in ("train", "dev"):
        d = os.path.join(ROOT, "data", "curriculum3", split)
        for f in sorted(os.listdir(d)):
            text = open(os.path.join(d, f)).read()
            m = re.search(r"^domain\(([a-z0-9_]+), ([a-z_]+)\)\.", text, re.M)
            dom = m.group(2) if m else "household"
            if dom != "gardening":
                out[f[:-3]] = dom
    return out


def main():
    allowed = tasks_allowed()
    seen = set()
    sessions = defaultdict(list)
    manifest = []
    dropped = Counter()
    counts = Counter()
    for line in open(CACHE, encoding="utf-8"):
        try:
            r = json.loads(line)
        except ValueError:
            continue
        if not r.get("ok"):
            continue
        tag = r.get("tag")
        if tag == "situation_fallback":
            counts["situation_reteach"] += 1
            continue
        if tag != "parse_fallback":
            continue
        m = re.search(r"Task id: ([a-z0-9_]+)", r["prompt"])
        lm = re.search(r"Line: (.*)", r["prompt"])
        if not m or not lm:
            continue
        task, source_line = m.group(1), lm.group(1).strip()
        if task not in allowed:
            counts["excluded_h3_or_gardening"] += 1
            continue
        if (task, source_line) in seen:
            counts["duplicate"] += 1
            continue
        seen.add((task, source_line))
        mapped, drop = [], []
        for fl in r["answer"].splitlines():
            a = ATOM.match(fl.strip().strip("`").lstrip("- "))
            if not a:
                continue
            neg, pred, args = a.group(1), a.group(2), [x.strip() for x in a.group(3).split(",")]
            if neg or pred not in V4 or V4[pred] != len(args):
                drop.append(pred)
                dropped[pred] += 1
                continue
            mapped.append(f"{pred}({', '.join(args)}).")
        status = "mapped" if mapped else "reteach"
        counts[status] += 1
        counts["facts_mapped"] += len(mapped)
        counts["facts_dropped"] += len(drop)
        manifest.append({"rec": "import", "schema": "import4/1", "source": {"file": "data/training/calls3/cache.jsonl", "key": r["key"], "tag": tag},
                         "task": task, "line": source_line, "status": status, "mapped": mapped, "dropped": drop})
        if mapped:
            sessions[task].append((r["key"], mapped))
    os.makedirs(os.path.join(OUT, "imports"), exist_ok=True)
    os.makedirs(os.path.join(OUT, "sessions"), exist_ok=True)
    with open(os.path.join(OUT, "imports", "archive3.jsonl"), "w") as f:
        for m in manifest:
            f.write(json.dumps(m) + "\n")
    for task, items in sorted(sessions.items()):
        sid = f"a3-{task}"
        rows = [{"rec": "raw", "schema": "raw4/1", "lang": "formal4/1", "msg_id": f"{sid}-t00000", "session": sid, "turn": 0,
                 "speaker": "teacher", "channel": "archive3", "text": f"topic {allowed[task]}/{task}."},
                {"rec": "raw", "schema": "raw4/1", "lang": "formal4/1", "msg_id": f"{sid}-t00001", "session": sid, "turn": 1,
                 "speaker": "teacher", "channel": "archive3", "text": f"task({task})."}]
        for key, mapped in items:
            for st in mapped:
                t = len(rows)
                rows.append({"rec": "raw", "schema": "raw4/1", "lang": "v3facts", "msg_id": f"{sid}-t{t:05d}", "session": sid, "turn": t,
                             "speaker": "teacher", "channel": "archive3", "text": st,
                             "source": {"file": "data/training/calls3/cache.jsonl", "key": key, "tag": "parse_fallback"}})
        with open(os.path.join(OUT, "sessions", f"{sid}.jsonl"), "w") as f:
            for row in rows:
                f.write(json.dumps(row) + "\n")
    summary = {"tasks_allowed": len(allowed), "sessions": len(sessions), "readings": dict(counts), "dropped_by_predicate": dict(dropped.most_common())}
    with open(os.path.join(OUT, "imports", "archive3_summary.json"), "w") as f:
        json.dump(summary, f, indent=1)
    print(json.dumps(summary, indent=1))


if __name__ == "__main__":
    main()
