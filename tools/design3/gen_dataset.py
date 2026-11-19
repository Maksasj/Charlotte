"""Generate the C2 example dataset (Design 3.0 section 5, decision D4) with Haiku.

1. Task names: Haiku proposes household tasks per domain (gardening excluded, D15).
2. Leakage filter: names that exist in data/curriculum3 or in the H3 manifest are dropped
   (names only are read from H3; contents are compared by script, counts only).
3. Each task: Haiku writes a full v3 scenario file (template: data/curriculum3/dev/bake_potato.md).
   The validator must pass; one retry with the errors. Files whose (action, object) step
   signatures overlap an H3 task by 50% or more are rejected (checked by script, counts only).
Output: data/dataset3/<task>.md. All calls cached (tools/design3/haiku.py).

Usage: python3 -I tools/design3/gen_dataset.py --per-domain 40 [--max 300]
"""
import argparse
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import haiku  # noqa: E402
from check_gt import ROOT, check  # noqa: E402
from gt3 import GARDENING, scenario  # noqa: E402

OUT = os.path.join(ROOT, "data", "dataset3")
DOMAINS = ["cooking", "cleaning", "repair", "planning", "errands", "personal_care", "travel", "crafts"]


def known_names():
    names = set()
    for d, _, fs in os.walk(os.path.join(ROOT, "data", "curriculum3")):
        names |= {f[:-3] for f in fs if f.endswith(".md")}
    for d, _, fs in os.walk(os.path.join(ROOT, "data", "curriculum")):
        names |= {f[:-3] for f in fs if f.endswith(".md")}
    man = os.path.join(ROOT, "data", "heldout3", "MANIFEST.sha256")
    for line in open(man):
        f = line.split("  ", 1)[1].strip().split("/", 1)[1]
        names.add(f[:-3].split("__")[0])
    return names | GARDENING


def h3_signatures():
    sigs = []
    base = os.path.join(ROOT, "data", "heldout3")
    for t in ("T", "G"):
        for f in sorted(os.listdir(os.path.join(base, t))):
            sc = scenario(os.path.join(base, t, f))
            sigs.append({(g["action"], g["object"]) for g in sc["steps"].values()})
    return sigs


def propose(domain, n, avoid):
    sys_ = "You list everyday household tasks. Reply with JSON only."
    prompt = (f"List {n} distinct everyday tasks in the domain '{domain}' that a person might ask a helper how to do. "
              "Each as a snake_case verb-first id (like clean_microwave, plan_picnic). No gardening, lawns, plants or "
              f"yard work. Avoid these ids: {', '.join(sorted(avoid)[:400])}.\n"
              'JSON: {"tasks": ["...", ...]}')
    ans = haiku.ask(sys_, prompt, tag="dataset_names")
    m = re.search(r"\{.*\}", ans, re.S)
    try:
        return [t for t in json.loads(m.group(0))["tasks"] if re.match(r"^[a-z][a-z0-9_]+$", t)]
    except (AttributeError, ValueError, KeyError):
        return []


def system_prompt():
    schema = open(os.path.join(ROOT, "research", "DESIGN_3_schema.md")).read()
    template = open(os.path.join(ROOT, "data", "curriculum3", "dev", "bake_potato.md")).read()
    return ("You write one training scenario file for a household-task learning agent, exactly in the format of the "
            "template, following the schema. Reply with the file content only, starting with '---'. No code fence "
            "around the whole file.\n\n# Schema\n" + schema + "\n\n# Template\n" + template)


def write_task(task, domain, sysp):
    prompt = (f"Write the scenario file for task id `{task}` (domain {domain}). Rules: 3 requests worded differently; "
              "1-2 situation lines; 3-7 steps, one action per step; every uses fact has a role fact; every item used is in "
              "needs; teach lines in the curriculum form ('To ... you need ...', '... takes N to M minutes.', 'Step N of ...: ...', "
              "'While step k runs, step N of ...: ...'); every step N appears in a teach line; aliases block; checklist; "
              "exactly 2 variants (change, request, situation, expect lines) each followed by a ```datalog situation block and a "
              "```datalog difference block. Reuse common role symbols (cut, pierce, beat, stir, mix, hold, contain, cover, coat, "
              f"season, heat, cool, clean, wipe, dry, measure, spread, fasten, loosen, lift, pour, scrub, protect). domain({task}, {domain}).")
    path = os.path.join(OUT, task + ".md")
    for attempt in range(2):
        text = haiku.ask(sysp, prompt, tag="dataset_file")
        text = text.strip()
        if text.startswith("```"):
            text = re.sub(r"^```[a-z]*\n", "", text)
            text = re.sub(r"\n```\s*$", "", text)
        os.makedirs(OUT, exist_ok=True)
        with open(path, "w") as f:
            f.write(text + "\n")
        try:
            errs = check(path)
        except Exception as e:  # unparsable file
            errs = [f"{path}:0: {e}"]
        if not errs:
            return True
        prompt += "\n\nYour previous file had these validator errors; fix them:\n" + "\n".join(e.split(": ", 1)[-1] for e in errs[:20])
    os.replace(path, path + ".rejected")
    return False


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--per-domain", type=int, default=40)
    ap.add_argument("--max", type=int, default=300)
    ap.add_argument("--shard", default="0/1", help="i/n: this worker writes every n-th task from i")
    a = ap.parse_args()
    si, sn = (int(x) for x in a.shard.split("/"))
    avoid = known_names()
    names = []
    for d in DOMAINS:
        for t in propose(d, a.per_domain, avoid | set(n for n, _ in names)):
            if t not in avoid and t not in {n for n, _ in names}:
                names.append((t, d))
    # round-robin over domains so every domain is represented
    by = {}
    for t, d in names:
        by.setdefault(d, []).append((t, d))
    rr = []
    while any(by.values()):
        for d in DOMAINS:
            if by.get(d):
                rr.append(by[d].pop(0))
    names = rr[: a.max]
    sigs = h3_signatures()
    sysp = system_prompt()
    ok = bad = leak = 0
    for idx, (t, d) in enumerate(names):
        if idx % sn != si:
            continue
        if any(os.path.exists(os.path.join(OUT, t + ext)) for ext in (".md.rejected", ".md.leak")):
            continue
        if os.path.exists(os.path.join(OUT, t + ".md")):
            ok += 1
            continue
        try:
            good = write_task(t, d, sysp)
        except haiku.BudgetExceeded as e:
            print(e)
            break
        except RuntimeError:
            good = False
        if not good:
            bad += 1
            continue
        sc = scenario(os.path.join(OUT, t + ".md"))
        sig = {(g["action"], g["object"]) for g in sc["steps"].values()}
        if any(len(sig & s) >= 0.5 * max(len(sig), 1) for s in sigs):
            os.replace(os.path.join(OUT, t + ".md"), os.path.join(OUT, t + ".md.leak"))
            leak += 1
            continue
        ok += 1
    print(json.dumps({"names": len(names), "valid": ok, "rejected": bad, "leak_rejected": leak,
                      "live_calls_total": haiku.live_calls()}))


if __name__ == "__main__":
    main()
