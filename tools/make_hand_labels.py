"""Make the hand-label sheet for calibrating the curriculum checklist judges.

    python tools/make_hand_labels.py [RUN_INDEX]

Takes the encounters of one recorded full curriculum run (default: the last)
from data/training/curriculum/<date>.jsonl: up to 14 where the model judge and
the symbolic check disagree, and 6 where they agree (a control), in a fixed
shuffled order. Writes eval/curriculum/hand_labels.md with the checklist and the
conversation of each, verdicts hidden. Fill in each `label:` line with `pass`
or `fail`; `sla_eval calibrate` then scores both judges against your labels.
"""
import glob
import hashlib
import json
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def checklist(scenario):
    path = os.path.join(ROOT, "data", "curriculum", "scenarios", scenario + ".md")
    out, sec = [], ""
    for line in open(path, encoding="utf-8"):
        line = line.strip()
        if line.startswith("## "):
            sec = line[3:].strip().lower()
        elif line.startswith("- ") and sec == "checklist":
            out.append(line[2:])
    return out


def main():
    files = sorted(glob.glob(os.path.join(ROOT, "data", "training", "curriculum", "*.jsonl")))
    runs = []
    for f in files:
        for line in open(f, encoding="utf-8"):
            try:
                r = json.loads(line)
            except ValueError:
                continue
            if r.get("scenarios") == 20 and any("symbolic_pass" in e for e in r["encounters"]):
                runs.append(r)
    if not runs:
        print("no recorded run with symbolic verdicts")
        return
    run = runs[int(sys.argv[1]) if len(sys.argv) > 1 else -1]
    pool = [e for e in run["encounters"] if e["kind"] in ("teach_retry", "repeat", "retention", "spaced_repeat")]
    key = lambda e: hashlib.sha1((e["scenario"] + e["kind"]).encode()).hexdigest()
    disagree = sorted([e for e in pool if e["passed"] != e.get("symbolic_pass")], key=key)[:14]
    agree = sorted([e for e in pool if e["passed"] == e.get("symbolic_pass")], key=key)[:6]
    items = sorted(disagree + agree, key=key)
    md = ["# Hand labels for the checklist judges", "",
          "For each encounter, read the checklist and the conversation, and write `pass`",
          "if the assistant's replies do every check correctly (wording and small omissions",
          "of detail do not matter, wrong facts do), otherwise `fail`. A note after the",
          "label is welcome. Then run `sla_eval calibrate`.", ""]
    for i, e in enumerate(items, 1):
        md.append(f"## {i}. {e['scenario']} ({e['kind']})")
        md.append("")
        md.append("Checklist:")
        for c in checklist(e["scenario"]):
            md.append(f"- {c}")
        md.append("")
        md.append("Conversation:")
        for t in e["turns"]:
            md.append(f"> **User:** {t['user']}")
            md.append(">")
            md.append(f"> **Assistant:** {t['agent']}".replace("\n", " / "))
            md.append("")
        md.append("label: ")
        md.append("")
    out = os.path.join(ROOT, "eval", "curriculum", "hand_labels.md")
    if os.path.exists(out) and re.search(r"^label: \S", open(out, encoding="utf-8").read(), re.M):
        print("hand_labels.md already has labels; not overwriting")
        return
    open(out, "w", encoding="utf-8").write("\n".join(md))
    print(f"wrote {out}: {len(items)} encounters ({len(disagree)} where the judges disagree, {len(agree)} where they agree)")


if __name__ == "__main__":
    main()
