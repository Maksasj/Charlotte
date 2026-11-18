"""Experiment X5: noticing a changed situation, gated by knowledge (no cue lists).

A request "has a situation" when it changes the base procedure (the variant requests) and
"has none" when the plain procedure answers it (the base requests). Candidate signals are only
the places where the request links to the agent's knowledge:
  - an item the agent knows (any task's needs, uses or aliases), with its 2-word context and
    whether it belongs to the identified task's needs;
  - a number (time, count) and the word after it.
An averaged perceptron learns which linked contexts signal a change, from training requests
(base = 0, variant = 1). Tested on dev and gardening. Compared with [M4], which flagged any
unusual word (169-204 of 225 base requests).

Usage: python3 -I tools/design3/x5_notice.py
Calls no model.
"""
import json
import os
import random
import sys
from collections import Counter, defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from gt3 import load_corpus, tokenize  # noqa: E402


def item_lexicon(scs):
    lex = {}
    for sc in scs:
        for p, a in sc["atoms"]:
            if p in ("needs", "uses", "object_of"):
                for w in a[1].split("_"):
                    if len(w) > 2:
                        lex[w[:4]] = True
        for p, a in sc.get("aliases", []):
            if p == "alias":
                for w in tokenize(a[1].strip('"')):
                    if len(w) > 2:
                        lex[w[:4]] = True
    return lex


def own_words(sc):
    ws = set()
    for i in sc["needs"]:
        ws |= {w[:4] for w in i.split("_")}
    ws |= {w[:4] for w in sc["task"].split("_")}
    for p, a in sc.get("aliases", []):
        if p == "task_alias" or (p == "alias" and a[0] in sc["needs"]):
            ws |= {w[:4] for w in tokenize(a[1].strip('"'))}
    return ws


def feats(text, sc, lex):
    t = tokenize(text)
    own = own_words(sc)
    f = ["b"]
    for i, w in enumerate(t):
        k = w[:4]
        if w.isdigit():
            nxt = t[i + 1] if i + 1 < len(t) else "</s>"
            f += ["num", "num+" + nxt]
        elif len(w) > 2 and k in lex:
            inown = k in own
            f.append(f"item own={inown}")
            for d in (-2, -1, 1):
                j = i + d
                if 0 <= j < len(t):
                    f.append(f"ctx{d}={t[j]} own={inown}")
    return f


def main():
    c = load_corpus("v3", seed=True)
    lex = item_lexicon(c["train"] + c["dev"] + c["seed"] + c["heldout_domain"])

    def items(scs):
        xs = []
        for sc in scs:
            xs += [(r, sc, 0) for r in sc["requests"]]
            xs += [(v["request"], sc, 1) for v in sc["variants"] if v.get("request")]
        return xs
    tr = items(c["train"])
    w = defaultdict(float)
    tot = defaultdict(float)
    rnd = random.Random(1)
    t = 0
    pos = sum(y for _, _, y in tr)
    for _ in range(15):
        rnd.shuffle(tr)
        for x, sc, y in tr:
            t += 1
            fs = feats(x, sc, lex)
            if (sum(w[f] for f in fs) > 0) != bool(y):
                d = (1 if y else -1)
                for f in fs:
                    w[f] += d
                    tot[f] += d * t
    avg = {f: w[f] - tot[f] / t for f in w}
    out = {"train_variants": pos, "train_base": len(tr) - pos}
    for split in ("dev", "heldout_domain", "train"):
        cm = Counter()
        errs = []
        for x, sc, y in items(c[split]):
            p = sum(avg.get(f, 0) for f in feats(x, sc, lex)) > 0
            cm[(y, int(p))] += 1
            if p != bool(y) and len(errs) < 6:
                errs.append({"variant": bool(y), "request": x})
        base = cm[(0, 0)] + cm[(0, 1)]
        var = cm[(1, 0)] + cm[(1, 1)]
        out[split] = {"base": base, "false_alarms": cm[(0, 1)], "false_alarm_rate": round(cm[(0, 1)] / max(base, 1), 3),
                      "variants": var, "noticed": cm[(1, 1)], "notice_rate": round(cm[(1, 1)] / max(var, 1), 3),
                      "errors": errs}
    top = sorted(avg.items(), key=lambda kv: -kv[1])[:12]
    out["top_learned_signals"] = [(f, round(v, 2)) for f, v in top]
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
