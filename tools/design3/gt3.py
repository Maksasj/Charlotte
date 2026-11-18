"""Corpus loader for the Design 3.0 experiments: v2 (data/curriculum) or v3 (data/curriculum3).

Splits: train, dev, heldout_domain (gardening, decision D15), seed (v3 only).
Calls no model.
"""
import os
import re
from collections import defaultdict

from check_gt import ROOT, parse

GARDENING = {"water_plant", "mow_lawn", "rake_leaves", "repot_plant", "water_garden"}


def tokenize(text):
    return re.findall(r"[a-z0-9]+(?:'[a-z]+)?|[^\sa-z0-9]", text.lower())


def scenario(path):
    head, sections, blocks = parse(path)
    task = head.get("task") or head.get("id")
    atoms = [(p, a) for b in blocks if b[0] == "ground truth" for p, a, _ in b[2]]
    aliases = [(p, a) for b in blocks if b[0] == "aliases" for p, a, _ in b[2]]
    variants = []
    vlines = sections.get("variant", [])
    sit = [b[2] for b in blocks if b[0] == "variant" and b[1] == "situation"]
    dif = [b[2] for b in blocks if b[0] == "variant" and b[1] == "difference"]
    cur = None
    for l in vlines:
        if l.startswith("- ") and ":" in l:
            k, v = l[2:].split(":", 1)
            k = k.strip()
            if k == "change":
                cur = {"change": v.strip()}
                variants.append(cur)
            elif cur is not None:
                cur[k] = v.strip()
    for i, v in enumerate(variants):
        v["situation_facts"] = [(p, a) for p, a, _ in sit[i]] if i < len(sit) else None
        v["difference_facts"] = [(p, a) for p, a, _ in dif[i]] if i < len(dif) else None
    steps = {}
    order = []
    for p, a in atoms:
        if p == "step":
            steps[a[1]] = {"action": None, "object": None, "uses": set(), "until": None, "time": None,
                           "overlaps": set(), "settings": [], "into": set(), "roles": {}}
            order.append(a[1])
    for p, a in atoms:
        s = steps.get(a[0]) if a else None
        if s is None:
            continue
        if p == "action":
            s["action"] = a[1]
        elif p == "object_of":
            s["object"] = a[1]
        elif p == "uses":
            s["uses"].add(a[1])
        elif p == "until":
            s["until"] = a[1]
        elif p == "takes_time":
            s["time"] = (a[1], a[2], a[3])
        elif p == "overlaps":
            s["overlaps"].add(a[1])
        elif p == "setting":
            s["settings"].append((a[1], a[2], a[3]))
        elif p in ("into", "onto"):
            s["into"].add(a[1])
        elif p == "role":
            s["roles"][a[1]] = a[2]
    return {
        "path": path, "task": task, "head": head,
        "requests": [l[2:] for l in sections.get("requests", []) if l.startswith("- ")],
        "situation": [l[2:] for l in sections.get("situation", []) if l.startswith("- ")],
        "teach": [l[2:] for l in sections.get("teach", []) if l.startswith("- ")],
        "checklist": [l[2:] for l in sections.get("checklist", []) if l.startswith("- ")],
        "atoms": atoms, "aliases": aliases, "variants": variants,
        "needs": [a[1] for p, a in atoms if p == "needs"],
        "steps": steps, "order": order,
        "domain": next((a[1] for p, a in atoms if p == "domain"), None),
    }


def _dir(path):
    return [scenario(os.path.join(path, f)) for f in sorted(os.listdir(path)) if f.endswith(".md")] \
        if os.path.isdir(path) else []


def load_corpus(gt="v3", seed=False):
    if gt == "v2":
        base = os.path.join(ROOT, "data", "curriculum")
        train = _dir(os.path.join(base, "training"))
        dev = _dir(os.path.join(base, "scenarios"))
        allsc = train + dev
        out = {
            "train": [s for s in train if s["task"] not in GARDENING],
            "dev": [s for s in dev if s["task"] not in GARDENING],
            "heldout_domain": [s for s in allsc if s["task"] in GARDENING],
        }
        if seed:
            out["seed"] = _dir(os.path.join(base, "seed"))
        return out
    base = os.path.join(ROOT, "data", "curriculum3")
    out = {"train": _dir(os.path.join(base, "train")), "dev": _dir(os.path.join(base, "dev")),
           "heldout_domain": [s for s in _dir(os.path.join(base, "heldout_domain")) if s["teach"]]}
    if seed:
        out["seed"] = _dir(os.path.join(base, "seed"))
    return out
