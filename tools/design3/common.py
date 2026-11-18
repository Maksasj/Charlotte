"""Shared loaders for the Design 3.0 phase A experiments.

Reads the curriculum scenario files (data/curriculum/scenarios = development,
data/curriculum/training = training). Calls no model. Run with python3 -I.
"""
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DEV_DIR = os.path.join(ROOT, "data", "curriculum", "scenarios")
TRAIN_DIR = os.path.join(ROOT, "data", "curriculum", "training")

# Decision D15: gardening is the held-out domain. These tasks never enter training.
GARDENING = {"water_plant", "mow_lawn", "rake_leaves", "repot_plant"}

ATOM = re.compile(r"^([a-z_]+)\((.*)\)\.\s*$")


def parse_scenario(path):
    text = open(path, encoding="utf-8").read()
    sc = {"path": path, "sections": {}, "atoms": []}
    m = re.search(r"^id:\s*(\S+)", text, re.M)
    sc["id"] = m.group(1) if m else os.path.basename(path)[:-3]
    m = re.search(r"^task:\s*(\S+)", text, re.M)
    sc["task"] = m.group(1) if m else sc["id"]
    cur = None
    in_code = False
    for line in text.splitlines():
        if line.startswith("```"):
            in_code = not in_code
            continue
        if in_code:
            a = ATOM.match(line.strip())
            if a:
                args = [x.strip() for x in a.group(2).split(",")]
                sc["atoms"].append((a.group(1), tuple(args)))
            continue
        if line.startswith("## "):
            cur = line[3:].strip()
            sc["sections"].setdefault(cur, [])
        elif cur and line.startswith("- "):
            sc["sections"][cur].append(line[2:].strip())
    return sc


def load(split):
    d = DEV_DIR if split == "dev" else TRAIN_DIR
    return [parse_scenario(os.path.join(d, f)) for f in sorted(os.listdir(d)) if f.endswith(".md")]


def load_all():
    return {"train": load("train"), "dev": load("dev")}


def step_atoms(sc):
    """Map step id -> dict of roles from the ground truth."""
    steps = {}
    order = []
    for p, args in sc["atoms"]:
        if p == "step":
            steps[args[1]] = {"action": None, "object": None, "uses": set(), "until": None,
                              "time": None, "overlaps": set()}
            order.append(args[1])
    for p, args in sc["atoms"]:
        s = steps.get(args[0])
        if s is None:
            continue
        if p == "action":
            s["action"] = args[1]
        elif p == "object_of":
            s["object"] = args[1]
        elif p == "uses":
            s["uses"].add(args[1])
        elif p == "until":
            s["until"] = args[1]
        elif p == "takes_time":
            s["time"] = (args[1], args[2], args[3])
        elif p == "overlaps":
            s["overlaps"].add(args[1])
    return steps, order


def tokenize(text):
    return re.findall(r"[a-z0-9]+(?:'[a-z]+)?|[^\sa-z0-9]", text.lower())
