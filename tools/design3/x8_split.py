"""Experiment X8: an algorithmic held-out split (maximum compound divergence).

Atoms are actions and items; compounds are (action, object) and (action, tool) pairs of steps.
Following Keysers et al. (2020, "Measuring compositional generalization"), a test split has
low atom divergence (its words and items are seen in training) and high compound divergence
(its combinations are new). Greedy search picks the test tasks; the hand dev split is the
comparison. Difficulty is measured with the X1 parser trained on the rest.

Usage: python3 -I tools/design3/x8_split.py [--gt v2|v3] [--size 20]
Calls no model.
"""
import argparse
import json
import os
import random
import sys
from collections import Counter

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import x1_parser as X  # noqa: E402
from gt3 import load_corpus  # noqa: E402


def units(sc):
    atoms, comps = Counter(), Counter()
    for sid in sc["order"]:
        g = sc["steps"][sid]
        a = g["action"]
        atoms[("act", a)] += 1
        if g["object"]:
            atoms[("item", g["object"])] += 1
            comps[(a, g["object"])] += 1
        for u in g["uses"]:
            atoms[("item", u)] += 1
            comps[(a, "with", u)] += 1
    return atoms, comps


def chernoff(p, q, alpha):
    sp, sq = sum(p.values()), sum(q.values())
    if not sp or not sq:
        return 1.0
    s = sum((p[k] / sp) ** alpha * (q[k] / sq) ** (1 - alpha) for k in set(p) & set(q))
    return 1 - s


def divergences(train, test, U):
    ta, tc = Counter(), Counter()
    ea, ec = Counter(), Counter()
    for t in train:
        ta.update(U[t][0]); tc.update(U[t][1])
    for t in test:
        ea.update(U[t][0]); ec.update(U[t][1])
    return chernoff(ea, ta, 0.5), chernoff(ec, tc, 0.1)


def search(tasks, U, size, iters=4000, seed=7):
    rnd = random.Random(seed)
    test = set(rnd.sample(tasks, size))

    def obj(ts):
        da, dc = divergences([t for t in tasks if t not in ts], ts, U)
        return dc - 2.0 * max(0.0, da - 0.25), da, dc
    best = obj(test)
    for _ in range(iters):
        out = rnd.choice(sorted(test))
        inn = rnd.choice([t for t in tasks if t not in test])
        cand = (test - {out}) | {inn}
        o = obj(cand)
        if o[0] >= best[0]:
            test, best = cand, o
    return sorted(test), best


def parser_exact(c_all, test):
    tr = [e for sc in c_all if sc["task"] not in test for e in X.step_examples(sc)]
    te = [e for sc in c_all if sc["task"] in test for e in X.step_examples(sc)]
    for e in tr + te:
        e["labels"], e["missed"] = X.label_line(e["toks"], e["facts"])
    X.KNOWN = False
    m = X.train([e for e in tr if e["kind"] == "step"])
    forms = X.learn_forms(tr)
    ok = n = 0
    for e in te:
        if e["kind"] == "step":
            n += 1
            ok += X.compare_step(X.decode(e["toks"], X.tag_seq(m, e["toks"]), forms), e)["exact"]
    return round(ok / max(n, 1), 3), n


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--gt", default="v2", choices=["v2", "v3"])
    ap.add_argument("--size", type=int, default=19)
    a = ap.parse_args()
    c = load_corpus(a.gt)
    scs = [sc for sc in c["train"] + c["dev"]]
    U = {sc["task"]: units(sc) for sc in scs}
    tasks = sorted(U)
    hand = [sc["task"] for sc in c["dev"]]
    hda, hdc = divergences([t for t in tasks if t not in hand], hand, U)
    mcd, (_, mda, mdc) = search(tasks, U, len(hand))
    rnd = random.Random(3)
    rand = rnd.sample(tasks, len(hand))
    rda, rdc = divergences([t for t in tasks if t not in rand], rand, U)
    out = {
        "tasks": len(tasks), "test_size": len(hand),
        "hand_dev": {"atom_div": round(hda, 3), "compound_div": round(hdc, 3), "parser_exact": parser_exact(scs, set(hand))},
        "random": {"atom_div": round(rda, 3), "compound_div": round(rdc, 3), "parser_exact": parser_exact(scs, set(rand))},
        "max_compound_div": {"atom_div": round(mda, 3), "compound_div": round(mdc, 3), "parser_exact": parser_exact(scs, set(mcd)),
                             "tasks": mcd},
    }
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
