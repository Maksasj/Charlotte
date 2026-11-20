"""Draw a per-stage teacher-audit sample from a teaching session (design 10.8, gate G2).

The live calls in data/training/teach4/<session>.json are in order, and each begins with the
agent hearing `topic <unit>.` on the live channel, so the raw rows between two live topic
lines are one call's delivered statements. N statements per stage are drawn (seed 4037),
excluding declarations, topic lines and case-local situation facts.

Usage: python3 -I tools/design4/audit_sample.py SESSION N
Writes data/training/teach4/audit_<SESSION>_sample.jsonl (verdicts are added by hand).
"""
import json
import os
import random
import sys
from collections import defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))


def main(session, n):
    run = json.load(open(os.path.join(ROOT, "data/training/teach4", f"{session}.json")))
    live = [c for c in run["calls"] if "statements" in c]
    rows = [json.loads(l) for l in open(os.path.join(ROOT, "data/kb4/raw/sessions", f"{session}.jsonl"))]
    starts = [i for i, r in enumerate(rows) if r.get("speaker") == "teacher" and r.get("channel") == "live" and r["text"].startswith("topic ")
              and any(r["text"] == f"topic {c['unit']}." for c in live)]
    # pair the live topic rows with the call records in order
    by_stage = defaultdict(list)
    ci = 0
    for k, i in enumerate(starts):
        while ci < len(live) and rows[i]["text"] != f"topic {live[ci]['unit']}.":
            ci += 1
        if ci >= len(live):
            break
        stage = live[ci]["stage"]
        end = starts[k + 1] if k + 1 < len(starts) else len(rows)
        for r in rows[i + 1:end]:
            t = r.get("text", "")
            if r.get("speaker") == "teacher" and r.get("channel") == "live" and not t.startswith(("topic ", "new(", "answer ")) \
                    and not t.split("(")[0].replace("not ", "") in ("has", "lacks", "state_of", "wants", "excludes"):
                by_stage[stage].append({"stage": stage, "unit": live[ci]["unit"], "msg_id": r.get("msg_id"), "statement": t})
        ci += 1
    rnd = random.Random(4037)
    out = []
    for st in sorted(by_stage):
        pool = by_stage[st]
        out.extend(rnd.sample(pool, min(n, len(pool))))
    p = os.path.join(ROOT, "data/training/teach4", f"audit_{session}_sample.jsonl")
    with open(p, "w") as f:
        for o in out:
            f.write(json.dumps(o) + "\n")
    print(json.dumps({"session": session, "per_stage": {s: len(v) for s, v in sorted(by_stage.items())}, "sampled": len(out), "file": p}))


if __name__ == "__main__":
    main(sys.argv[1], int(sys.argv[2]))
