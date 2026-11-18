"""Experiment X6: can the fact-and-relation check score replies with zero model calls?

Reads the 2.0 curriculum records (data/training/curriculum/*.jsonl), takes every help reply
that lists numbered steps, reads it into facts with the X1 learned parser (each numbered line
is framed as "step N of <task>:"), and scores it against the ground truth:
  - steps aligned by (action, object) in order
  - graded recall of needs, step actions, objects, tools, order
  - strict pass: every needed item and every step (action+object) present and in order
It then compares the strict pass with 2.0's own symbolic check and the Haiku judge.

Usage: python3 -I tools/design3/x6_content.py [--gt v2|v3]
Calls no model.
"""
import argparse
import glob
import json
import os
import re
import sys
from collections import Counter

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import x1_parser as X  # noqa: E402
from gt3 import ROOT, load_corpus, tokenize  # noqa: E402


def build_parser(gt):
    c = load_corpus(gt)
    ex = [e for sc in c["train"] for e in X.step_examples(sc)]
    for e in ex:
        e["labels"], e["missed"] = X.label_line(e["toks"], e["facts"])
    steps = [e for e in ex if e["kind"] == "step"]
    needs = [e for e in ex if e["kind"] == "needs"]
    X.KNOWN = False  # replies do not come with a needs line of their own
    return X.train(steps), X.train(needs), X.learn_forms(ex), c


def alias_map(sc):
    m = {}
    for p, a in sc.get("aliases", []):
        if p == "alias":
            m.setdefault(a[0], set()).add("_".join(tokenize(a[1].strip('"'))))
    return m


def same(x, y, aliases):
    if x is None or y is None:
        return False
    if x == y:
        return True
    return x in aliases.get(y, ()) or y in aliases.get(x, ())


def read_reply(text, task, smodel, nmodel, forms):
    lines = [l.strip() for l in text.split("\n")]
    steps = []
    needs = set()
    for l in lines:
        m = re.match(r"^(\d+)\.\s*(.*)$", l)
        if m:
            body = m.group(2)
            body = re.sub(r"^while you [^,]*,\s*", "", body, flags=re.I)
            framed = f"Step {m.group(1)} of {task.replace('_', ' ')}: {body}"
            toks = tokenize(framed)
            steps.append(X.decode(toks, X.tag_seq(smodel, toks), forms))
        elif re.search(r"\byou need\b", l, re.I):
            seg = re.split(r"(?i)you need", l, maxsplit=1)[1].split(".")[0]
            framed = f"To {task.replace('_', ' ')} you need{seg}."
            toks = tokenize(framed)
            needs |= X.decode(toks, X.tag_seq(nmodel, toks), forms)["needs"]
    return needs, steps


def score(needs, steps, sc):
    al = alias_map(sc)
    gneeds = set(sc["needs"])
    n_hit = sum(1 for g in gneeds if any(same(p, g, al) for p in needs))
    gsteps = [sc["steps"][s] for s in sc["order"]]
    # align in order: greedy, each answer step to the first unmatched gold step with same action
    j = 0
    matched = []
    used = set()
    for st in steps:
        for k, g in enumerate(gsteps):
            if k in used:
                continue
            if same(st["action"], g["action"], al) and (g["object"] is None or same(st["object"], g["object"], al)):
                used.add(k)
                matched.append(k)
                break
    in_order = matched == sorted(matched)
    tool_hit = tool_n = 0
    for k, g in enumerate(gsteps):
        for u in g["uses"]:
            tool_n += 1
    return {
        "needs_recall": n_hit / max(len(gneeds), 1),
        "steps_recall": len(used) / max(len(gsteps), 1),
        "in_order": in_order,
        "strict": n_hit == len(gneeds) and len(used) == len(gsteps) and in_order,
        "loose": n_hit >= 0.8 * len(gneeds) and len(used) >= 0.8 * len(gsteps) and in_order,
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--gt", default="v2", choices=["v2", "v3"])
    a = ap.parse_args()
    smodel, nmodel, forms, c = build_parser(a.gt)
    scs = {sc["task"]: sc for split in c.values() for sc in split}
    agree = Counter()
    n = 0
    recalls = Counter()
    for f in sorted(glob.glob(os.path.join(ROOT, "data", "training", "curriculum", "*.jsonl"))):
        for line in open(f):
            run = json.loads(line)
            for enc in run.get("encounters", []):
                sc = scs.get(enc.get("scenario"))
                if sc is None or enc.get("kind") in ("transfer", "transfer_retry"):
                    continue
                reply = next((t["agent"] for t in enc["turns"] if re.search(r"^\s*1\.", t["agent"], re.M)), None)
                if reply is None:
                    continue
                needs, steps = read_reply(reply, sc["task"], smodel, nmodel, forms)
                r = score(needs, steps, sc)
                n += 1
                recalls["needs"] += r["needs_recall"]
                recalls["steps"] += r["steps_recall"]
                recalls["strict"] += r["strict"]
                recalls["loose"] += r["loose"]
                sp = bool(enc.get("symbolic_pass"))
                jp = bool(enc.get("passed"))
                agree[("strict", r["strict"], "sym", sp)] += 1
                agree[("strict", r["strict"], "judge", jp)] += 1
    out = {
        "replies_scored": n,
        "mean_needs_recall": round(recalls["needs"] / max(n, 1), 3),
        "mean_steps_recall": round(recalls["steps"] / max(n, 1), 3),
        "strict_pass": recalls["strict"], "loose_pass": recalls["loose"],
        "vs_2.0_symbolic": {f"strict={k[1]} sym={k[3]}": v for k, v in agree.items() if k[2] == "sym"},
        "vs_haiku_judge": {f"strict={k[1]} judge={k[3]}": v for k, v in agree.items() if k[2] == "judge"},
    }
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
