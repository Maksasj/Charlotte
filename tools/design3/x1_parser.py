"""Experiment X1: a learned teach-line parser (DESIGN_3_alternatives.md 5.2).

An averaged-perceptron role tagger is trained on the training scenarios' teach lines,
labelled by aligning the ground-truth facts to the words of each line. It is tested on the
development scenarios' teach lines. No word lists: the only knowledge is what the training
lines and their facts give. Symbol forms (plural -> singular) are learned suffix rules.

Usage: python3 -I tools/design3/x1_parser.py [--gt v2|v3] [--out FILE]
Calls no model.
"""
import argparse
import json
import os
import random
import re
import sys
from collections import Counter, defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from gt3 import load_corpus, tokenize  # noqa: E402

KNOWN = True
ROLES = ["ACT", "OBJ", "USE", "UNTIL", "TMIN", "TMAX", "UNIT", "OVL", "NEED", "SET", "INTO"]


# ---------------------------------------------------------------- alignment (authoring time)
def word_match(tok, w):
    if tok == w:
        return True
    if len(w) >= 3 and tok.startswith(w) and len(tok) - len(w) <= 3:
        return True
    if len(tok) >= 3 and w.startswith(tok[:-1]) and abs(len(tok) - len(w)) <= 2 and tok[:3] == w[:3]:
        return True
    return False


def find_span(toks, sym, taken):
    words = sym.split("_")
    n = len(words)
    for i in range(len(toks) - n + 1):
        if any(j in taken for j in range(i, i + n)):
            continue
        if all(word_match(toks[i + k], words[k]) for k in range(n)):
            return list(range(i, i + n))
    # head-word fallback: last word of a multiword symbol ("pan" for "frying_pan" is not used;
    # here: "beef" for "minced_beef" when the line says "the beef")
    if n > 1:
        for i, t in enumerate(toks):
            if i not in taken and word_match(t, words[-1]):
                return [i]
    return None


def label_line(toks, facts):
    """facts: list of (ROLE, symbol). Returns BIO labels and the facts that could not be aligned.

    Content facts are aligned after the first ':' (the task phrase before it repeats words)."""
    labels = ["O"] * len(toks)
    colon = toks.index(":") if ":" in toks else -1
    taken = set(range(colon + 1))
    missed = []
    pre = set(range(colon + 1))
    for role, sym in facts:
        span = find_span(toks, str(sym), taken - pre if role == "OVL" else taken)
        if span is None:
            missed.append((role, sym))
            continue
        for k, j in enumerate(span):
            labels[j] = ("B-" if k == 0 else "I-") + role
            taken.add(j)
    return labels, missed


# ---------------------------------------------------------------- examples
def step_examples(sc):
    """Yield (tokens, facts, gold) per step teach line."""
    out = []
    steps = sc["steps"]
    for line in sc["teach"]:
        low = line.lower()
        m = re.search(r"\bstep (\d+) of [^:]*:", low)
        if m:
            sid = f"{sc['task']}_s{m.group(1)}"
            g = steps.get(sid)
            if g is None:
                continue
            toks = tokenize(line)
            facts = [("ACT", g["action"])]
            if g["object"]:
                facts.append(("OBJ", g["object"]))
            for u in sorted(g["uses"]):
                facts.append(("USE", u))
            if g["until"]:
                facts.append(("UNTIL", g["until"]))
            if g["time"]:
                lo, hi, unit = g["time"]
                # numbers in the line: label min then max (they may be equal)
                facts.append(("TMIN", lo))
                if hi != lo:
                    facts.append(("TMAX", hi))
                facts.append(("UNIT", unit))
            for o in sorted(g["overlaps"]):
                facts.append(("OVL", o.rsplit("_s", 1)[1]))
            for attr, val, unit in sorted(g.get("settings", [])):
                facts.append(("SET", val))
            for i in sorted(g.get("into", [])):
                facts.append(("INTO", i))
            out.append({"kind": "step", "task": sc["task"], "sid": sid, "line": line, "toks": toks,
                        "facts": facts, "gold": g})
        elif re.search(r"\byou need\b", low):
            toks = tokenize(line)
            facts = [("NEED", i) for i in sc["needs"]]
            out.append({"kind": "needs", "task": sc["task"], "line": line, "toks": toks,
                        "facts": facts, "gold": set(sc["needs"])})
    return out


# ---------------------------------------------------------------- tagger
def feats(toks, i, prev, prev2, known=frozenset()):
    w = toks[i]
    f = ["b", "w=" + w, "s3=" + w[-3:], "s2=" + w[-2:], "p=" + prev, "pp=" + prev + "|" + prev2,
         "d=" + str(w.isdigit())]
    for k in (-3, -2, -1, 1, 2):
        j = i + k
        t = toks[j] if 0 <= j < len(toks) else ("<s>" if j < 0 else "</s>")
        f.append(f"w{k}={t}")
    f.append("w-1w=" + (toks[i - 1] if i else "<s>") + "|" + w)
    f.append("ww+1=" + w + "|" + (toks[i + 1] if i + 1 < len(toks) else "</s>"))
    # position relative to the first ':' and to the clause
    colon = toks.index(":") if ":" in toks else -1
    f.append("aftercolon=" + str(colon >= 0 and i > colon))
    if colon >= 0 and i > colon:
        f.append("cpos=" + str(min(i - colon, 6)))
    f.append("p+w=" + prev + "|" + w)
    if known:
        f.append("known=" + str(w in known))
        f.append("known+p=" + str(w in known) + "|" + prev)
        f.append("known-1=" + str(i > 0 and toks[i - 1] in known))
    return f


class Perceptron:
    def __init__(self):
        self.w = defaultdict(float)
        self.tot = defaultdict(float)
        self.ts = defaultdict(int)
        self.t = 0
        self.tags = set()

    def score(self, fs, tag):
        return sum(self.w.get((f, tag), 0.0) for f in fs)

    def predict(self, fs):
        return max(sorted(self.tags), key=lambda t: self.score(fs, t))

    def update(self, truth, guess, fs):
        self.t += 1
        if truth == guess:
            return
        for f in fs:
            for tag, d in ((truth, 1.0), (guess, -1.0)):
                k = (f, tag)
                self.tot[k] += (self.t - self.ts[k]) * self.w[k]
                self.ts[k] = self.t
                self.w[k] += d

    def average(self):
        for k in list(self.w):
            self.tot[k] += (self.t - self.ts[k]) * self.w[k]
            self.w[k] = self.tot[k] / max(self.t, 1)


def tag_seq(model, toks, known=frozenset(), margins=None):
    prev, prev2 = "<s>", "<s>"
    out = []
    for i in range(len(toks)):
        fs = feats(toks, i, prev, prev2, known)
        if margins is None:
            t = model.predict(fs)
        else:
            sc = sorted(((model.score(fs, tg), tg) for tg in sorted(model.tags)), reverse=True)
            t = sc[0][1]
            margins.append(sc[0][0] - sc[1][0] if len(sc) > 1 else 99.0)
        out.append(t)
        prev2, prev = prev, t
    return out


def selective(model, forms, examples, levels=(0.25, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0)):
    """Accuracy when the parser keeps only its most confident lines (min token margin)."""
    rows = []
    for e in examples:
        if e["kind"] != "step":
            continue
        m = []
        tags = tag_seq(model, e["toks"], e.get("known", frozenset()), margins=m)
        ok = compare_step(decode(e["toks"], tags, forms), e)["exact"]
        rows.append((min(m) if m else 0.0, ok))
    rows.sort(key=lambda r: -r[0])
    out = {}
    for lv in levels:
        k = max(1, int(round(lv * len(rows))))
        out[str(lv)] = round(sum(o for _, o in rows[:k]) / k, 3)
    best = 0
    for k in range(1, len(rows) + 1):
        if sum(o for _, o in rows[:k]) / k >= 0.95:
            best = k
    out["coverage_at_95"] = round(best / max(len(rows), 1), 3)
    return out


def train(examples, epochs=20, seed=1):
    m = Perceptron()
    for ex in examples:
        m.tags.update(ex["labels"])
    rnd = random.Random(seed)
    data = list(examples)
    for _ in range(epochs):
        rnd.shuffle(data)
        for ex in data:
            prev, prev2 = "<s>", "<s>"
            for i, gold in enumerate(ex["labels"]):
                fs = feats(ex["toks"], i, prev, prev2, ex.get("known", frozenset()))
                guess = m.predict(fs)
                m.update(gold, guess, fs)
                prev2, prev = prev, gold
    m.average()
    return m


# ---------------------------------------------------------------- learned symbol forms
def learn_forms(examples):
    """Learn token -> symbol-word rewrites from aligned training data.

    For every aligned (token, symbol word) pair, record the rewrite (strip a token suffix, add a
    symbol suffix; identity included) under the token's last 1-3 letters. At use, the longest
    suffix with enough evidence decides by majority. Nothing here is an English list.
    """
    by_suffix = defaultdict(Counter)
    exact = {}
    for ex in examples:
        for role, sym in ex["facts"]:
            if role in ("TMIN", "TMAX", "OVL", "SET"):
                continue
            words = str(sym).split("_")
            span = [i for i, l in enumerate(ex["labels"]) if l.endswith("-" + role)]
            toks = [ex["toks"][i] for i in span]
            if len(toks) != len(words):
                continue
            for t, w in zip(toks, words):
                exact.setdefault(t, Counter())[w] += 1
                k = 0
                while k < min(len(t), len(w)) and t[k] == w[k]:
                    k += 1
                rw = (len(t) - k, w[k:])
                for L in (1, 2, 3):
                    if len(t) > L:
                        by_suffix[t[-L:]][rw] += 1
    return {t: c.most_common(1)[0][0] for t, c in exact.items()}, by_suffix


def to_symbol(toks, forms):
    exact, by_suffix = forms
    out = []
    for t in toks:
        if t in exact:
            out.append(exact[t])
            continue
        rw = (0, "")
        for L in (3, 2, 1):
            c = by_suffix.get(t[-L:])
            if c and sum(c.values()) >= 3:
                rw = c.most_common(1)[0][0]
                break
        strip, add = rw
        out.append((t[: len(t) - strip] if strip else t) + add if len(t) - strip >= 2 else t)
    return "_".join(out)


def decode(toks, tags, forms):
    spans = defaultdict(list)
    cur = None
    for t, tag in zip(toks, tags):
        if tag.startswith("B-"):
            cur = [tag[2:], [t]]
            spans[cur[0]].append(cur[1])
        elif tag.startswith("I-") and cur and cur[0] == tag[2:]:
            cur[1].append(t)
        else:
            cur = None
    sym = lambda r: [to_symbol(s, forms) for s in spans.get(r, [])]
    return {
        "action": (sym("ACT") or [None])[0],
        "object": (sym("OBJ") or [None])[0],
        "uses": set(sym("USE")),
        "until": (sym("UNTIL") or [None])[0],
        "tmin": ("".join(spans["TMIN"][0]) if spans.get("TMIN") else None),
        "tmax": ("".join(spans["TMAX"][0]) if spans.get("TMAX") else None),
        "unit": (sym("UNIT") or [None])[0],
        "overlaps": {"".join(s) for s in spans.get("OVL", [])},
        "needs": set(sym("NEED")),
    }


def compare_step(pred, ex):
    g = ex["gold"]
    task = ex["task"]
    res = {}
    res["action"] = pred["action"] == g["action"]
    res["object"] = pred["object"] == g["object"]
    res["uses"] = pred["uses"] == set(g["uses"])
    res["until"] = pred["until"] == g["until"]
    if g["time"]:
        lo, hi, unit = g["time"]
        ptime = (pred["tmin"], pred["tmax"] or pred["tmin"], pred["unit"])
        res["time"] = ptime == (lo, hi, unit)
    else:
        res["time"] = pred["tmin"] is None
    res["overlaps"] = pred["overlaps"] == {o.rsplit("_s", 1)[1] for o in g["overlaps"]}
    res["exact"] = all(res.values())
    return res


def run(gt, quiet=False, extra=None):
    corpus = load_corpus(gt)
    ex = {split: [e for sc in corpus[split] for e in step_examples(sc)] for split in corpus}
    if extra:
        ex["train"] = ex["train"] + list(extra)
    report = {"gt": gt, "counts": {}, "alignment": {}}
    for split, es in ex.items():
        missed_lines = 0
        miss_roles = Counter()
        for e in es:
            e["labels"], e["missed"] = label_line(e["toks"], e["facts"])
            if e["missed"]:
                missed_lines += 1
                for r, _ in e["missed"]:
                    miss_roles[r] += 1
        report["counts"][split] = len(es)
        report["alignment"][split] = {"lines_with_unaligned_fact": missed_lines,
                                      "unaligned_by_role": dict(miss_roles)}
    train_ex = ex["train"]
    if KNOWN:
        for e in train_ex:
            pass
        by_task = defaultdict(set)
        for e in train_ex:
            if e["kind"] == "needs":
                by_task[e["task"]] |= {t for t, l in zip(e["toks"], e["labels"]) if l.endswith("NEED")}
        for e in train_ex:
            if e["kind"] == "step":
                e["known"] = frozenset(by_task[e["task"]])
        needs_model = train([e for e in train_ex if e["kind"] == "needs"])
        for split in [s for s in ex if s != "train"]:
            by_task = defaultdict(set)
            for e in ex[split]:
                if e["kind"] == "needs":
                    tags = tag_seq(needs_model, e["toks"])
                    by_task[e["task"]] |= {t for t, l in zip(e["toks"], tags) if l.endswith("NEED")}
            for e in ex[split]:
                if e["kind"] == "step":
                    e["known"] = frozenset(by_task[e["task"]])
    model = train(train_ex)
    forms = learn_forms(train_ex)
    for split in [s for s in ex if s != "train"]:
        roles = Counter()
        n = 0
        needs_ok = needs_n = 0
        fails = []
        for e in ex[split]:
            tags = tag_seq(model, e["toks"], e.get("known", frozenset()))
            pred = decode(e["toks"], tags, forms)
            if e["kind"] == "step":
                r = compare_step(pred, e)
                n += 1
                for k, v in r.items():
                    roles[k] += v
                if not r["exact"] and len(fails) < 25:
                    fails.append({"line": e["line"], "wrong": [k for k, v in r.items() if not v and k != "exact"],
                                  "pred": {k: (sorted(v) if isinstance(v, set) else v) for k, v in pred.items() if k != "needs"}})
            else:
                needs_n += 1
                needs_ok += pred["needs"] == e["gold"]
        report[split] = {
            "step_lines": n,
            "exact": roles["exact"],
            "exact_rate": round(roles["exact"] / max(n, 1), 3),
            "per_role": {k: round(roles[k] / max(n, 1), 3) for k in ("action", "object", "uses", "until", "time", "overlaps")},
            "needs_lines": needs_n,
            "needs_exact": needs_ok,
            "fail_examples": fails,
        }
        report[split]["selective"] = selective(model, forms, ex[split])
    # training fit
    roles = Counter()
    for e in [x for x in train_ex if x["kind"] == "step"]:
        r = compare_step(decode(e["toks"], tag_seq(model, e["toks"], e.get("known", frozenset())), forms), e)
        roles["exact"] += r["exact"]
    report["train_fit_exact_rate"] = round(roles["exact"] / max(1, sum(1 for x in train_ex if x["kind"] == "step")), 3)
    return report


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--gt", default="v2", choices=["v2", "v3"])
    ap.add_argument("--out")
    ap.add_argument("--no-known", action="store_true", help="ablation: no needs-line feature")
    a = ap.parse_args()
    global KNOWN
    KNOWN = not a.no_known
    rep = run(a.gt)
    s = json.dumps(rep, indent=1)
    if a.out:
        open(a.out, "w").write(s)
    summary = {k: v for k, v in rep.items() if k not in ("dev", "heldout_domain")}
    for split in ("dev", "heldout_domain"):
        if split in rep:
            summary[split] = {k: v for k, v in rep[split].items() if k != "fail_examples"}
    print(json.dumps(summary, indent=1))


if __name__ == "__main__":
    main()
