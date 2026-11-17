"""Extract the v1 real-turn regression set (Design 2.0, milestone 12).

Sources (all committed v1 run artefacts):
  - eval/trainer_snapshot.json: 8 help-scenario conversations, 68 agent turns;
  - eval/reports/*.md and eval/reports_pretrained_vault/*.md: judge chats, 228 agent turns.

Writes data/regression/turns.jsonl, one JSON object per agent turn:
  id, set ("trainer" | "judge"), conv, turn (1-based), scenario, user, v1_reply.

Run from the repo root:  python tools/extract_regression.py
"""

import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parent.parent
OUT = ROOT / "data" / "regression" / "turns.jsonl"


def trainer_turns():
    snap = json.loads((ROOT / "eval" / "trainer_snapshot.json").read_text(encoding="utf-8"))
    # Oldest conversation first, so ids are stable and follow the run order.
    for ci, conv in enumerate(reversed(snap["finished"]), start=1):
        user = None
        turn = 0
        for m in conv["transcript"]:
            if m["role"] == "teacher":
                user = m["text"]
            elif m["role"] == "agent" and user is not None:
                turn += 1
                yield {
                    "id": f"trainer-{ci}-{turn}",
                    "set": "trainer",
                    "conv": f"trainer-{ci}",
                    "turn": turn,
                    "scenario": conv["scenario"],
                    "user": user,
                    "v1_reply": m["text"],
                }
                user = None


USER = re.compile(r"^> \*\*User:\*\* (.*?)\s*$")
AGENT = re.compile(r"^> \*\*Agent:\*\* (.*?)\s*$")
JUDGE = re.compile(r"^### Judge: (.*?) \(score")


def judge_turns():
    files = sorted((ROOT / "eval" / "reports").glob("*.md")) + sorted((ROOT / "eval" / "reports_pretrained_vault").glob("*.md"))
    for f in files:
        tag = ("pre-" if f.parent.name == "reports_pretrained_vault" else "") + f.stem
        topic = None
        ji = 0
        turn = 0
        user = None
        for line in f.read_text(encoding="utf-8").splitlines():
            m = JUDGE.match(line)
            if m:
                ji += 1
                topic = m.group(1)
                turn = 0
                user = None
                continue
            m = USER.match(line)
            if m:
                user = m.group(1)
                continue
            m = AGENT.match(line)
            if m and user is not None and topic is not None:
                turn += 1
                yield {
                    "id": f"judge-{tag}-{ji}-{turn}",
                    "set": "judge",
                    "conv": f"judge-{tag}-{ji}",
                    "turn": turn,
                    "scenario": topic,
                    "user": user,
                    "v1_reply": m.group(1),
                }
                user = None


def main():
    rows = list(trainer_turns()) + list(judge_turns())
    OUT.parent.mkdir(parents=True, exist_ok=True)
    with OUT.open("w", encoding="utf-8", newline="\n") as fh:
        for r in rows:
            fh.write(json.dumps(r, ensure_ascii=False) + "\n")
    n_t = sum(r["set"] == "trainer" for r in rows)
    n_j = sum(r["set"] == "judge" for r in rows)
    print(f"wrote {len(rows)} turns ({n_t} trainer, {n_j} judge) to {OUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
