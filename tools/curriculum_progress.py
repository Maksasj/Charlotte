"""Progress of a running `sla_eval curriculum` run, read from its vault log.

    python tools/curriculum_progress.py            # newest run, once
    python tools/curriculum_progress.py --watch    # refresh every 5 s until done
    python tools/curriculum_progress.py --vault DIR

Each user request in the log is matched to its scenario and encounter by the
scenario files' wordings (encounter 1 and its retry use the first wording,
the repeat and the spaced queue the second, retention the third, transfer the
variant's request), so the bar counts encounters started.
"""
import glob
import os
import re
import sys
import tempfile
import time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PHASES = ["teach", "repeat", "transfer", "retention"]


def scenarios():
    out = []
    for f in sorted(glob.glob(os.path.join(ROOT, "data", "curriculum", "scenarios", "*.md"))):
        reqs, variant, sec = [], None, ""
        for line in open(f, encoding="utf-8"):
            line = line.strip()
            if line.startswith("## "):
                sec = line[3:].strip().lower()
            elif line.startswith("- ") and sec == "requests":
                reqs.append(line[2:].strip())
            elif line.startswith("- request:") and sec == "variant":
                variant = line[len("- request:"):].strip().strip('"')
        out.append((os.path.basename(f)[:-3], reqs, variant))
    return out


def newest_vault():
    runs = sorted(glob.glob(os.path.join(tempfile.gettempdir(), "sla_curriculum_*")), key=os.path.getmtime)
    return runs[-1] if runs else None


def bar(done, total, width=30):
    total = max(total, 1)
    n = int(width * min(done, total) / total)
    return "[" + "#" * n + "-" * (width - n) + f"] {done}/{total}"


def status(vault):
    """Encounters end with the user's "Thanks!" (always logged); teach blocks
    are the scenario files' teach sentences. Replaying that sequence in the
    runner's order gives the phase and scenario of each encounter."""
    scen = scenarios()
    teach_lines = set()
    for f in sorted(glob.glob(os.path.join(ROOT, "data", "curriculum", "scenarios", "*.md"))):
        sec = ""
        for line in open(f, encoding="utf-8"):
            line = line.strip()
            if line.startswith("## "):
                sec = line[3:].strip().lower()
            elif line.startswith("- ") and sec == "teach":
                teach_lines.add(line[2:].strip())
    n = len(scen)
    n_var = sum(1 for s in scen if s[2])
    events, turns, last = [], 0, ""
    for f in sorted(glob.glob(os.path.join(vault, "log", "*.md"))):
        for line in open(f, encoding="utf-8", errors="replace"):
            m = re.match(r"- (\S+) \| turn \| (.*?) \| ", line)
            if not m:
                continue
            turns += 1
            last = m.group(1)
            text = m.group(2).strip()
            if text == "Thanks!":
                events.append("E")
            elif text in teach_lines and (not events or events[-1] != "L"):
                events.append("L")
    # Replay: phase 1 = per scenario E, or E L E (taught, then the retry).
    i, done = 0, {"teach": 0, "repeat": 0, "transfer": 0, "spaced": 0, "retention": 0}
    retries = 0
    while i < len(events) and done["teach"] < n:
        if events[i] == "E":
            if i + 1 < len(events) and events[i + 1] == "L":
                if i + 2 < len(events):
                    i += 3
                    retries += 1
                    done["teach"] += 1
                else:
                    break
            else:
                i += 1
                done["teach"] += 1
        else:
            i += 1
    if done["teach"] < n:
        i = len(events)  # still teaching: later phases have not started
    for phase, total in [("repeat", n), ("transfer", n_var)]:
        while i < len(events) and done[phase] < total:
            if events[i] == "E":
                done[phase] += 1
            i += 1
    # The spaced queue: a teach group, then as many repeats as taught scenarios.
    queued = 0
    while i < len(events) and events[i] == "L":
        queued += 1
        i += 1
        while i < len(events) and events[i] == "L":
            i += 1
    while i < len(events) and done["spaced"] < queued:
        done["spaced"] += events[i] == "E"
        i += 1
    while i < len(events):
        done["retention"] += events[i] == "E"
        i += 1
    totals = {"teach": n, "repeat": n, "transfer": n_var, "retention": n}
    lines = [f"run: {os.path.basename(vault)}   turns logged: {turns}   last activity: {last}"]
    for p in PHASES:
        extra = f"   (retried after teaching: {retries})" if p == "teach" else ""
        lines.append(f"  {p:<10} {bar(done[p], totals[p])}{extra}")
        if p == "transfer" and queued:
            lines.append(f"  {'spaced':<10} {bar(done['spaced'], queued)}")
    all_done = sum(done[p] for p in PHASES) + done["spaced"]
    all_total = sum(totals.values()) + queued
    lines.append(f"  {'overall':<10} {bar(all_done, all_total)}  {100 * all_done // max(all_total, 1)}%")
    finished = done["retention"] >= n
    if finished:
        lines.append("  all encounters done; labelling and the report come last")
    return "\n".join(lines), finished


def main():
    args = sys.argv[1:]
    vault = args[args.index("--vault") + 1] if "--vault" in args else newest_vault()
    if not vault:
        print("no curriculum run found")
        return
    while True:
        text, done = status(vault)
        if "--watch" in args:
            os.system("cls" if os.name == "nt" else "clear")
        print(text)
        if "--watch" not in args or done:
            break
        time.sleep(5)


if __name__ == "__main__":
    main()
