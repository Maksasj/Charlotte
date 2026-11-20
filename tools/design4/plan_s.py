"""Quarter-block plan for curriculum S (Design 4.0 sections 9.1, eval 1.10; milestone 36).

Units:
  group      entity-group units of topics.py (stages 1-5)
  role       role symbols of the S seed procedures with 3 or more items, about 10 per unit (stage 3)
  archive    archive3 tasks of the S domains, replayed at 0 calls (stage 6)
  procedure  dataset3 tasks of the S domains, taught live from the task name and the English
             step outline only, never the ground truth (stage 6)
  situation  the same tasks' variants: the situation in English and as situation facts; the
             teacher teaches the resolution as a case (stage 7)
Units are assigned at random (seed 4036), stratified by domain and kind, to four blocks; a
dataset3 task's procedure and situation units share a block. Each block runs stages 1-7 for its
units, so every prefix of blocks is a representative sample.

Usage: python3 -I tools/design4/plan_s.py   -> data/curriculum4/S/plan.jsonl. Zero model calls.
"""
import json
import os
import random
import re
from collections import defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
S = {"cooking", "cleaning", "repair", "baking"}
ATOM = re.compile(r"^([a-z_]+)\(([^()]*)\)\.")


def domain_of(text):
    m = re.search(r"^domain\(([a-z0-9_]+), ([a-z_]+)\)\.", text, re.M)
    return m.group(2) if m else None


def main():
    rnd = random.Random(4036)
    units = []
    for l in open(os.path.join(ROOT, "data/curriculum4/S/topics.jsonl")):
        t = json.loads(l)
        units.append({"kind": "group", "unit": t["unit"], "domain": t["domain"], "symbols": t["symbols"]})
    # role symbols with 3 or more items, from the S seed procedures (curriculum3 train and dev)
    items = defaultdict(lambda: defaultdict(set))
    for split in ("train", "dev"):
        d = os.path.join(ROOT, "data/curriculum3", split)
        for f in sorted(os.listdir(d)):
            text = open(os.path.join(d, f)).read()
            dom = domain_of(text)
            if dom not in S:
                continue
            for line in text.splitlines():
                m = ATOM.match(line.strip())
                if m and m.group(1) == "role":
                    a = [x.strip() for x in m.group(2).split(",")]
                    if len(a) == 3:
                        items[dom][a[2]].add(a[1])
    for dom in sorted(items):
        funcs = sorted(f for f, its in items[dom].items() if len(its) >= 3)
        for k in range(0, len(funcs), 10):
            chunk = funcs[k:k + 10]
            units.append({"kind": "role", "unit": f"{dom}/roles{k // 10 + 1}", "domain": dom, "symbols": chunk,
                          "text": "; ".join(f"{f}: {', '.join(sorted(items[dom][f]))}" for f in chunk)})
    # archive3 tasks of the S domains
    a3 = set()
    for l in open(os.path.join(ROOT, "data/kb4/raw/imports/archive3.jsonl")):
        a3.add(json.loads(l)["task"])
    dom3 = {}
    for split in ("train", "dev"):
        d = os.path.join(ROOT, "data/curriculum3", split)
        for f in os.listdir(d):
            dom3[f[:-3]] = domain_of(open(os.path.join(d, f)).read())
    for t in sorted(a3):
        if dom3.get(t) in S and os.path.exists(os.path.join(ROOT, "data/kb4/raw/sessions", f"a3-{t}.jsonl")):
            units.append({"kind": "archive", "unit": f"{dom3[t]}/a3-{t}", "domain": dom3[t], "task": t, "symbols": []})
    # dataset3 tasks: outline (English teach lines) and variants
    d3 = os.path.join(ROOT, "data/dataset3")
    for f in sorted(os.listdir(d3)):
        text = open(os.path.join(d3, f)).read()
        dom = domain_of(text)
        if dom not in S:
            continue
        task = f[:-3]
        teach = text.split("## teach", 1)[1].split("\n## ", 1)[0] if "## teach" in text else ""
        outline = [l[2:].strip() for l in teach.splitlines() if l.startswith("- ")]
        units.append({"kind": "procedure", "unit": f"{dom}/{task}", "domain": dom, "task": task, "symbols": [], "text": "\n".join(outline)})
        variants = []
        for part in text.split("\n## variant")[1:]:
            change = re.search(r"^- change: (.*)$", part, re.M)
            sit = part.split("```datalog situation", 1)[1].split("```", 1)[0].strip() if "```datalog situation" in part else ""
            if change:
                variants.append(f"Situation: {change.group(1).split(':')[0]}.\n{sit}")
        if variants:
            units.append({"kind": "situation", "unit": f"{dom}/{task}/situations", "domain": dom, "task": task, "symbols": [],
                          "text": "\n\n".join(variants), "variants": len(variants)})
    # blocks: stratified by domain and kind; a dataset3 task's two units share a block
    by = defaultdict(list)
    for u in units:
        if u["kind"] == "situation":
            continue
        by[(u["domain"], u["kind"])].append(u)
    block_of_task = {}
    # the rotation continues across a kind's domains, so each kind's totals per block differ by at most one
    offset = {kind: rnd.randrange(4) for kind in sorted({k[1] for k in by})}
    for k in sorted(by):
        grp = by[k]
        rnd.shuffle(grp)
        for u in grp:
            u["block"] = offset[k[1]] % 4 + 1
            offset[k[1]] += 1
            if u["kind"] == "procedure":
                block_of_task[u["task"]] = u["block"]
    for u in units:
        if u["kind"] == "situation":
            u["block"] = block_of_task[u["task"]]
    out = os.path.join(ROOT, "data/curriculum4/S/plan.jsonl")
    with open(out, "w") as f:
        for u in sorted(units, key=lambda u: (u["block"], u["kind"], u["unit"])):
            f.write(json.dumps(u) + "\n")
    summary = defaultdict(lambda: defaultdict(int))
    for u in units:
        summary[u["block"]][u["kind"]] += 1
    print(json.dumps({"units": len(units), "blocks": {b: dict(v) for b, v in sorted(summary.items())}}))


if __name__ == "__main__":
    main()
