"""Live view of a curriculum run: progress and every message as it happens.

    python tools/curriculum_live.py              # follow the newest run
    python tools/curriculum_live.py FILE         # a given live file
    python tools/curriculum_live.py --once       # print the current state and exit

Reads data/training/curriculum/live/<time>.jsonl, which `sla_eval curriculum`
appends to as it runs. Colours: user cyan, agent green, teacher yellow,
judge PASS green / FAIL red.
"""
import glob
import json
import os
import shutil
import sys
import time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LIVE_DIR = os.path.join(ROOT, "data", "training", "curriculum", "live")

if os.name == "nt":
    os.system("")  # turn on ANSI colours in the Windows console

C = {"user": "\033[36m", "agent": "\033[32m", "teacher": "\033[33m", "phase": "\033[1;35m", "pass": "\033[1;32m", "fail": "\033[1;31m", "dim": "\033[2m", "end": "\033[0m"}

# Encounters per kind in a full run of 20 scenarios (15 with variants).
TOTALS = {"teach": 20, "repeat": 20, "transfer": 15, "retention": 20}


def newest():
    files = sorted(glob.glob(os.path.join(LIVE_DIR, "*.jsonl")), key=os.path.getmtime)
    return files[-1] if files else None


def read(path):
    out = []
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            try:
                out.append(json.loads(line))
            except ValueError:
                pass
    return out


def bar(done, total, width=24):
    total = max(total, 1)
    n = int(width * min(done, total) / total)
    return "#" * n + "-" * (width - n)


def render(path, events):
    cols, rows = shutil.get_terminal_size((110, 40))
    judged = [e for e in events if e["who"] == "judge"]
    by_kind = {}
    passed = {}
    for e in judged:
        k = e["kind"]
        by_kind[k] = by_kind.get(k, 0) + 1
        passed[k] = passed.get(k, 0) + e["text"].startswith("PASS")
    head = [f"{C['phase']}curriculum run {os.path.basename(path)[:-6]}{C['end']}   messages: {len(events)}"]
    for k in ["teach", "teach_retry", "repeat", "transfer", "spaced_repeat", "retention"]:
        n = by_kind.get(k, 0)
        total = TOTALS.get(k)
        if not n and not total:
            continue
        frac = f"{n}/{total}" if total else f"{n}"
        head.append(f"  {k:<14} [{bar(n, total or max(n, 1))}] {frac:<6} passed {passed.get(k, 0)}")
    last_t = events[-1]["t"] if events else 0
    idle = time.time() - last_t if last_t else 0
    head.append(f"{C['dim']}  last message {idle:.0f} s ago{C['end']}")
    head.append("-" * min(cols, 110))
    # The most recent messages that fit on screen.
    room = max(rows - len(head) - 2, 5)
    lines = []
    for e in events:
        who = e["who"]
        text = " ".join(str(e["text"]).split())
        if who == "phase":
            lines.append(f"{C['phase']}== {text}{C['end']}")
        elif who == "judge":
            col = C["pass"] if text.startswith("PASS") else C["fail"]
            lines.append(f"{col}   judge: {text[:cols - 12]}{C['end']}")
        else:
            col = C.get(who, "")
            label = f"{who:>7}: "
            width = max(cols - len(label) - 1, 20)
            first = True
            while text:
                chunk, text = text[:width], text[width:]
                lines.append(f"{col}{label if first else ' ' * len(label)}{chunk}{C['end']}")
                first = False
    return "\n".join(head + lines[-room:])


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    path = args[0] if args else newest()
    if not path:
        print("no live run found in", LIVE_DIR)
        return
    once = "--once" in sys.argv
    seen = -1
    while True:
        events = read(path)
        if len(events) != seen:
            seen = len(events)
            if not once:
                sys.stdout.write("\033[2J\033[H")
            print(render(path, events))
            sys.stdout.flush()
        if once:
            return
        time.sleep(1)
        # A newer run started: follow it.
        n = newest() if not args else path
        if n and n != path:
            path, seen = n, -1


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass
