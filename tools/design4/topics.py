"""Topic planner (Design 4.0 section 9.1): units for each stage, from 3.0's symbols.

Seeds come from the train and dev ground truth in data/curriculum3 (items, actions, role
symbols and states); gardening and H3 tasks are never used. Units are domain x entity group,
each with at most 40 symbols (section 9.3). Size S covers the domains cooking (including
baking), cleaning and repair.

Usage: python3 -I tools/design4/topics.py [--size S]
Writes data/curriculum4/<size>/topics.jsonl and registry_seed.jsonl. Zero model calls.
"""
import json
import os
import re
import sys
from collections import defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SIZE_DOMAINS = {"S": ["cooking", "cleaning", "repair"],
                # M adds the other non-gardening 3.0 domains (design 9.2); units here are M's own
                "M": ["planning", "errands", "travel", "crafts", "personal_care"]}
GROUP = 40
ATOM = re.compile(r"^([a-z_]+)\(([^()]*)\)\.")


def tasks():
    out = []
    for split in ("train", "dev"):
        d = os.path.join(ROOT, "data", "curriculum3", split)
        for f in sorted(os.listdir(d)):
            text = open(os.path.join(d, f)).read()
            gt = text.split("## ground truth", 1)[-1].split("```", 2)[1] if "## ground truth" in text else ""
            facts = [ATOM.match(l.strip()) for l in gt.splitlines()]
            facts = [(m.group(1), [a.strip() for a in m.group(2).split(",")]) for m in facts if m]
            dom = next((a[1] for p, a in facts if p == "domain"), None)
            out.append((f[:-3], dom, facts))
    return out


def main():
    size = sys.argv[sys.argv.index("--size") + 1] if "--size" in sys.argv else "S"
    domains = SIZE_DOMAINS[size]
    syms = defaultdict(dict)
    for task, dom, facts in tasks():
        if dom not in domains:
            continue
        for p, a in facts:
            if p in ("needs", "uses", "object_of", "into", "onto") and len(a) == 2:
                syms[dom].setdefault(a[1], "item")
            elif p == "action" and len(a) == 2:
                syms[dom].setdefault(a[1], "action")
            elif p == "role" and len(a) == 3:
                syms[dom].setdefault(a[1], "item")
                syms[dom].setdefault(a[2], "function")
            elif p in ("until", "achieves", "requires_state") and len(a) == 2:
                syms[dom].setdefault(a[1], "state")
    units = []
    registry = []
    for dom in domains:
        items = sorted(syms[dom].items())
        for k in range(0, len(items), GROUP):
            chunk = items[k:k + GROUP]
            unit = f"{dom}/g{k // GROUP + 1}"
            units.append({"stage": 0, "domain": dom, "unit": unit, "symbols": [s for s, _ in chunk], "sorts": dict(chunk)})
            registry.extend({"sym": s, "sort": t, "unit": unit} for s, t in chunk)
    out = os.path.join(ROOT, "data", "curriculum4", size)
    os.makedirs(out, exist_ok=True)
    with open(os.path.join(out, "topics.jsonl"), "w") as f:
        for u in units:
            f.write(json.dumps(u) + "\n")
    with open(os.path.join(out, "registry_seed.jsonl"), "w") as f:
        for r in registry:
            f.write(json.dumps(r) + "\n")
    print(json.dumps({"size": size, "units": len(units), "symbols": len(registry), "by_domain": {d: len(syms[d]) for d in domains}}))


if __name__ == "__main__":
    main()
