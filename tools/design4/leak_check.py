"""Post-hoc leak check of a teaching session (Design 4.0 section 10.5, D4.8; gate G2).

Every teacher statement delivered to the agent (raw rows spoken by the teacher) is checked
against the dev deny list (data/bench4/deny.txt, plain atoms) and the sealed deny list
(data/bench4/deny_sealed.sha256, sha256 of the canonical atom), independently of the agent's
filter code. A delivered leak invalidates the run.

Usage: python3 -I tools/design4/leak_check.py SESSION [SESSION ...]
"""
import hashlib
import json
import os
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))


def key(s):
    t = "".join(s.strip().rstrip(".").split())
    return t[3:] if t.startswith("not") else t


def main(sessions):
    plain = {l.strip() for l in open(os.path.join(ROOT, "data/bench4/deny.txt")) if l.strip()}
    hashed = {l.strip()[7:] for l in open(os.path.join(ROOT, "data/bench4/deny_sealed.sha256")) if l.startswith("sha256:")}
    out = {}
    for s in sessions:
        n = 0
        leaks = []
        for l in open(os.path.join(ROOT, "data/kb4/raw/sessions", f"{s}.jsonl")):
            r = json.loads(l)
            if r.get("speaker") != "teacher":
                continue
            text = r.get("text", "")
            if text.startswith(("topic ", "new(")):
                continue
            body = text.split(":", 1)[1] if text.startswith("answer ") else text
            for part in body.split(". "):
                k = key(part)
                if not k or "(" not in k:
                    continue
                n += 1
                if k in plain or hashlib.sha256(k.encode()).hexdigest() in hashed:
                    leaks.append({"msg": r.get("msg_id"), "channel": r.get("channel"), "atom": k if k in plain else "sealed:" + hashlib.sha256(k.encode()).hexdigest()[:12]})
        out[s] = {"statements": n, "delivered_leaks": len(leaks), "examples": leaks[:5]}
    print(json.dumps(out, indent=1))
    return 1 if any(v["delivered_leaks"] for v in out.values()) else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
