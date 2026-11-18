"""Experiment X2: request detection and task identity without word lists.

Task identity: pick the taught task a request is about, among all tasks in the vault
(train + dev + seed + held-out domain: everything the agent has been taught by then).
  baseline  - IDF-weighted overlap of request words with the task's own knowledge words
              (task symbol, needs, actions, objects, the teach lines' task phrase)  [M3 control]
  learned   - baseline plus word associations learned from the training requests
              (request word -> knowledge word, positive PMI), applied to unseen tasks.
Request detection: an averaged perceptron over word features separates request lines from
statements (teach and situation lines), trained on training scenarios, tested on dev.

Usage: python3 -I tools/design3/x2_requests.py [--gt v2|v3]
Calls no model.
"""
import argparse
import json
import math
import os
import random
import re
import sys
from collections import Counter, defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from gt3 import load_corpus, tokenize  # noqa: E402


def norm(t):
    # crude shared stem: first 4 letters (best of 4, 5, 6 on dev). Symmetric, applied to both sides; not a word list.
    return t[:4]


def words(text):
    return [norm(t) for t in tokenize(text) if re.match(r"[a-z]", t) and len(t) > 2]


def knowledge_words(sc):
    ws = Counter()
    for part in sc["task"].split("_"):
        ws[norm(part)] += 3
    for p, a in sc["atoms"]:
        if p in ("needs", "action", "object_of", "uses", "goal"):
            for part in a[-1].split("_"):
                ws[norm(part)] += 1
    for l in sc["teach"]:
        m = re.search(r"\bstep \d+ of ([^:]*):", l.lower()) or re.search(r"^to (.*?) you need", l.lower())
        if m:
            for w in words(m.group(1)):
                ws[w] += 1
    for p, a in sc.get("aliases", []):
        if p == "task_alias":
            for w in words(a[1].strip('"')):
                ws[w] += 2
    return ws


def build(gt):
    c = load_corpus(gt, seed=True)
    tasks = {}
    for split, scs in c.items():
        for sc in scs:
            tasks[sc["task"]] = (split, sc)
    know = {t: knowledge_words(sc) for t, (_, sc) in tasks.items()}
    df = Counter()
    for ws in know.values():
        df.update(set(ws))
    n = len(know)
    idf = {w: math.log((n + 1) / (d + 0.5)) for w, d in df.items()}
    return c, tasks, know, idf


def score_overlap(req, kw, idf):
    rw = set(words(req))
    return sum(idf.get(w, 0) * min(kw[w], 3) for w in rw if w in kw)


def learn_assoc(train_scs, know):
    pair = Counter()
    rc = Counter()
    kc = Counter()
    n = 0
    for sc in train_scs:
        kw = set(know[sc["task"]])
        for r in sc["requests"] + [v.get("request", "") for v in sc["variants"]]:
            rw = set(words(r)) - kw
            n += 1
            for a in rw:
                rc[a] += 1
                for b in kw:
                    pair[(a, b)] += 1
            for b in kw:
                kc[b] += 1
    assoc = defaultdict(dict)
    for (a, b), c in pair.items():
        if c < 2:
            continue
        pmi = math.log(c * n / (rc[a] * kc[b]))
        if pmi > 0.5:
            assoc[a][b] = pmi
    return assoc


def score_assoc(req, kw, assoc, idf):
    s = 0.0
    for a in set(words(req)):
        for b, w in assoc.get(a, {}).items():
            if b in kw:
                s += w * 0.3 * idf.get(b, 0) / 5
    return s


def identity(gt):
    c, tasks, know, idf = build(gt)
    assoc = learn_assoc(c["train"], know)
    out = {}
    for split in ("train", "dev", "heldout_domain"):
        res = {"base": Counter(), "variant": Counter()}
        misses = []
        for sc in c[split]:
            reqs = [("base", r) for r in sc["requests"]] + [("variant", v["request"]) for v in sc["variants"] if v.get("request")]
            for kind, r in reqs:
                sb = {t: score_overlap(r, know[t], idf) for t in know}
                sl = {t: sb[t] + score_assoc(r, know[t], assoc, idf) for t in know}
                top_b = max(sb, key=sb.get)
                top_l = max(sl, key=sl.get)
                res[kind]["n"] += 1
                res[kind]["baseline"] += top_b == sc["task"]
                res[kind]["learned"] += top_l == sc["task"]
                if top_l != sc["task"] and len(misses) < 12:
                    misses.append((r, sc["task"], top_l))
        out[split] = {k: dict(v) for k, v in res.items()}
        out[split]["misses"] = misses
    out["assoc_pairs"] = sum(len(v) for v in assoc.values())
    return out


# ------------------------------------------------------------------ request detection
def bow(text):
    t = tokenize(text)
    f = ["b"] + ["w=" + x for x in t] + ["f=" + t[0]] if t else ["b"]
    f += ["l=" + t[-1]] if t else []
    f += [f"bg={a}|{b}" for a, b in zip(t, t[1:])]
    return f


def detection(gt):
    c = load_corpus(gt)
    def items(scs):
        xs = []
        for sc in scs:
            xs += [(r, 1) for r in sc["requests"]] + [(v["request"], 1) for v in sc["variants"] if v.get("request")]
            xs += [(l, 0) for l in sc["teach"] + sc["situation"]]
            xs += [(v["situation"], 0) for v in sc["variants"] if v.get("situation")]
        return xs
    tr, te = items(c["train"]), items(c["dev"]) + items(c["heldout_domain"])
    w = defaultdict(float)
    tot = defaultdict(float)
    rnd = random.Random(1)
    t = 0
    for _ in range(10):
        rnd.shuffle(tr)
        for x, y in tr:
            t += 1
            fs = bow(x)
            s = sum(w[f] for f in fs)
            if (s > 0) != bool(y):
                d = 1 if y else -1
                for f in fs:
                    w[f] += d
                    tot[f] += d * t
    avg = {f: w[f] - tot[f] / t for f in w}
    cm = Counter()
    errs = []
    for x, y in te:
        p = sum(avg.get(f, 0) for f in bow(x)) > 0
        cm[(y, int(p))] += 1
        if p != bool(y) and len(errs) < 10:
            errs.append((y, x))
    req = cm[(1, 1)] + cm[(1, 0)]
    return {"test_requests": req, "request_recall": round(cm[(1, 1)] / max(req, 1), 3),
            "false_alarms_on_statements": cm[(0, 1)], "statements": cm[(0, 0)] + cm[(0, 1)], "errors": errs}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--gt", default="v2", choices=["v2", "v3"])
    a = ap.parse_args()
    print(json.dumps({"identity": identity(a.gt), "detection": detection(a.gt)}, indent=1))


if __name__ == "__main__":
    main()
