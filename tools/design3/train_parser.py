"""Train the Design 3.0 learned models and export them for the Rust runtime (crates/parse3).

Models (averaged perceptrons; feature functions are mirrored exactly in parse3):
  teach    - token tagger for teach lines (steps and needs lines)
  situation- token tagger for situation text (HAS, LACKS, WANTS, EXCL, TIME, COUNT)
  detect   - line classifier: request (1) or statement (0)
  notice   - request classifier: changed situation (1) or not (0)
  forms    - token -> symbol-word rewrites
Training data: data/curriculum3/train plus the accumulated pairs in
data/training/parser3/pairs.jsonl (fallback answers, decision D16), never dev or H3.

Usage: python3 -I tools/design3/train_parser.py [--out data/training/parser3/models/current.json]
       [--parity data/training/parser3/parity.jsonl]
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
from gt3 import ROOT, load_corpus, tokenize  # noqa: E402

PAIRS = os.path.join(ROOT, "data", "training", "parser3", "pairs.jsonl")
EPOCHS = 20
AGREE = False
NO_DEV_PAIRS = False


# ------------------------------------------------------------------ alignment
def word_match(tok, w):
    if tok == w:
        return True
    if len(w) >= 3 and tok.startswith(w) and len(tok) - len(w) <= 3:
        return True
    if len(tok) >= 3 and w.startswith(tok[:-1]) and abs(len(tok) - len(w)) <= 2 and tok[:3] == w[:3]:
        return True
    return False


def forms_of(sym, aliases):
    out = [str(sym).split("_")]
    for a in aliases.get(sym, ()):
        out.append(a)
    return out


def find_span(toks, sym, taken, aliases, lo=0):
    for words in sorted(forms_of(sym, aliases), key=len, reverse=True):
        n = len(words)
        for i in range(lo, len(toks) - n + 1):
            if any(j in taken for j in range(i, i + n)):
                continue
            if all(word_match(toks[i + k], words[k]) for k in range(n)):
                return list(range(i, i + n))
    words = str(sym).split("_")
    if len(words) > 1:
        for i in range(lo, len(toks)):
            if i not in taken and word_match(toks[i], words[-1]):
                return [i]
    return None


def label(toks, facts, aliases, lo=0):
    labels = ["O"] * len(toks)
    taken = set(range(lo))
    missed = []
    for role, sym in facts:
        span = find_span(toks, str(sym), taken if role != "OVL" else set(), aliases,
                         0 if role == "OVL" else lo)
        if span is None:
            missed.append((role, sym))
            continue
        for k, j in enumerate(span):
            labels[j] = ("B-" if k == 0 else "I-") + role
            taken.add(j)
    return labels, missed


def alias_words(sc):
    m = defaultdict(list)
    for p, a in sc["aliases"]:
        if p == "alias":
            m[a[0]].append(tokenize(a[1].strip('"')))
    return m


# ------------------------------------------------------------------ examples
def teach_examples(sc, extra_lines=()):
    al = alias_words(sc)
    needs_known = set()
    out = []
    lines = list(sc["teach"]) + list(extra_lines)
    for line in lines:
        low = line.lower()
        toks = tokenize(line)
        colon = toks.index(":") + 1 if ":" in toks else 0
        m = re.search(r"\bstep (\d+) of [^:]*:", low)
        if m:
            g = sc["steps"].get(f"{sc['task']}_s{m.group(1)}")
            if g is None:
                continue
            facts = [("ACT", g["action"])]
            if g["object"]:
                facts.append(("OBJ", g["object"]))
            for u in sorted(g["uses"]):
                facts.append(("USE", u))
            for kind, i in sorted(g.get("places", [])):
                facts.append((kind.upper(), i))
            if g["until"]:
                facts.append(("UNTIL", g["until"]))
            if g["time"]:
                lo, hi, unit = g["time"]
                facts.append(("TMIN", lo))
                if hi != lo:
                    facts.append(("TMAX", hi))
                facts.append(("UNIT", unit))
            for o in sorted(g["overlaps"]):
                facts.append(("OVL", o.rsplit("_s", 1)[1]))
            labels, missed = label(toks, facts, al, colon)
            out.append({"kind": "step", "task": sc["task"], "toks": toks, "labels": labels, "missed": missed})
        elif "you need" in low:
            labels, missed = label(toks, [("NEED", i) for i in sc["needs"]], al)
            needs_known |= {t for t, l in zip(toks, labels) if l.endswith("NEED")}
            out.append({"kind": "needs", "task": sc["task"], "toks": toks, "labels": labels, "missed": missed})
        elif ":" not in toks and sc.get("time") and any(t.isdigit() for t in toks):
            lo, hi, unit = sc["time"]
            facts = [("TMIN", lo)] + ([("TMAX", hi)] if hi != lo else []) + [("UNIT", unit)]
            labels, missed = label(toks, facts, al)
            out.append({"kind": "time", "task": sc["task"], "toks": toks, "labels": labels, "missed": missed})
    for e in out:
        e["known"] = sorted(needs_known) if e["kind"] == "step" else []
    return out


SIT_ROLE = {"has": "HAS", "lacks": "LACKS", "wants": "WANTS", "excludes": "EXCL"}


def situation_examples(sc):
    al = alias_words(sc)
    out = []
    for v in sc["variants"]:
        facts = []
        for p, a in v.get("situation_facts") or []:
            if p in SIT_ROLE:
                facts.append((SIT_ROLE[p], a[1]))
            elif p == "time_limit":
                facts.append(("TIME", a[1]))
            elif p == "count":
                facts.append(("COUNT", a[2]))
        for text in (v.get("request", ""), v.get("situation", "")):
            if not text:
                continue
            toks = tokenize(text)
            present = [f for f in facts if find_span(toks, str(f[1]), set(), al) is not None]
            labels, _ = label(toks, present, al)
            out.append({"toks": toks, "labels": labels, "known": []})
    return out


# ------------------------------------------------------------------ features (mirrored in parse3)
def tag_feats(toks, i, prev, prev2, known):
    w = toks[i]
    n = len(toks)
    f = ["b", "w=" + w, "s3=" + w[-3:], "s2=" + w[-2:], "p=" + prev, "pp=" + prev + "|" + prev2,
         "d=" + ("True" if w.isdigit() else "False")]
    for k in (-3, -2, -1, 1, 2):
        j = i + k
        t = toks[j] if 0 <= j < n else ("<s>" if j < 0 else "</s>")
        f.append("w%d=%s" % (k, t))
    f.append("w-1w=" + (toks[i - 1] if i else "<s>") + "|" + w)
    f.append("ww+1=" + w + "|" + (toks[i + 1] if i + 1 < n else "</s>"))
    colon = toks.index(":") if ":" in toks else -1
    after = colon >= 0 and i > colon
    f.append("aftercolon=" + ("True" if after else "False"))
    if after:
        f.append("cpos=" + str(min(i - colon, 6)))
    f.append("p+w=" + prev + "|" + w)
    if known:
        kn = w in known
        f.append("known=" + ("True" if kn else "False"))
        f.append("known+p=" + ("True" if kn else "False") + "|" + prev)
        f.append("known-1=" + ("True" if (i > 0 and toks[i - 1] in known) else "False"))
    return f


def bow_feats(toks):
    if not toks:
        return ["b"]
    f = ["b"] + ["w=" + x for x in toks] + ["f=" + toks[0], "l=" + toks[-1]]
    f += ["bg=%s|%s" % (a, b) for a, b in zip(toks, toks[1:])]
    return f


# ------------------------------------------------------------------ perceptrons
class Tagger:
    def __init__(self):
        self.w = defaultdict(lambda: defaultdict(float))
        self.tags = []

    def score(self, fs, t):
        return sum(self.w[f][t] for f in fs if f in self.w and t in self.w[f])

    def best(self, fs):
        best, bs = None, None
        for t in self.tags:
            s = 0.0
            for f in fs:
                d = self.w.get(f)
                if d is not None:
                    s += d.get(t, 0.0)
            if bs is None or s > bs:
                best, bs = t, s
        return best

    def tag(self, toks, known):
        prev, prev2 = "<s>", "<s>"
        out = []
        ks = set(known)
        for i in range(len(toks)):
            t = self.best(tag_feats(toks, i, prev, prev2, ks))
            out.append(t)
            prev2, prev = prev, t
        return out


def train_tagger(exs, seed=1):
    m = Tagger()
    m.tags = sorted({l for e in exs for l in e["labels"]})
    w = defaultdict(lambda: defaultdict(float))
    tot = defaultdict(lambda: defaultdict(float))
    ts = defaultdict(lambda: defaultdict(int))
    t = 0
    rnd = random.Random(seed)
    data = list(exs)
    m.w = w
    for _ in range(EPOCHS):
        rnd.shuffle(data)
        for e in data:
            prev, prev2 = "<s>", "<s>"
            ks = set(e["known"])
            for i, gold in enumerate(e["labels"]):
                fs = tag_feats(e["toks"], i, prev, prev2, ks)
                guess = m.best(fs)
                t += 1
                if guess != gold:
                    for f in fs:
                        for tag, d in ((gold, 1.0), (guess, -1.0)):
                            tot[f][tag] += (t - ts[f][tag]) * w[f][tag]
                            ts[f][tag] = t
                            w[f][tag] += d
                prev2, prev = prev, gold
    avg = {}
    for f, d in w.items():
        row = {}
        for tag, v in d.items():
            a = (tot[f][tag] + (t - ts[f][tag]) * v) / max(t, 1)
            if a != 0.0:
                row[tag] = a
        if row:
            avg[f] = row
    m.w = avg
    return m


def train_binary(items, epochs=10, seed=1):
    w = defaultdict(float)
    tot = defaultdict(float)
    rnd = random.Random(seed)
    data = list(items)
    t = 0
    for _ in range(epochs):
        rnd.shuffle(data)
        for fs, y in data:
            t += 1
            if (sum(w[f] for f in fs) > 0) != bool(y):
                d = 1.0 if y else -1.0
                for f in fs:
                    w[f] += d
                    tot[f] += d * t
    return {f: w[f] - tot[f] / t for f in w if w[f] - tot[f] / t != 0.0}


def learn_forms(exs):
    exact = defaultdict(Counter)
    by_suffix = defaultdict(Counter)
    for e in exs:
        spans = []
        cur = None
        for tok, l in zip(e["toks"], e["labels"]):
            if l.startswith("B-"):
                cur = [tok]
                spans.append(cur)
            elif l.startswith("I-") and cur is not None:
                cur.append(tok)
            else:
                cur = None
        for sp, sym in zip(spans, e.get("syms", [])):
            pass
        for (t, w) in e.get("pairs", []):
            exact[t][w] += 1
            k = 0
            while k < min(len(t), len(w)) and t[k] == w[k]:
                k += 1
            rw = [len(t) - k, w[k:]]
            for L in (1, 2, 3):
                if len(t) > L:
                    by_suffix[t[-L:]][tuple(rw)] += 1
    ex = {t: sorted(c.items(), key=lambda kv: (-kv[1], kv[0]))[0][0] for t, c in exact.items()}
    sf = {}
    for suf, c in by_suffix.items():
        if sum(c.values()) >= 3:
            (strip, add), _ = sorted(c.items(), key=lambda kv: (-kv[1], kv[0]))[0]
            sf[suf] = [strip, add]
    return {"exact": ex, "suffix": sf}


def form_pairs(sc, e):
    """(token, symbol word) pairs from an aligned teach example, for the forms model."""
    pairs = []
    syms = []
    g = None
    m = re.search(r"\bstep (\d+) of", " ".join(e["toks"]))
    if e["kind"] == "step" and m:
        g = sc["steps"].get(f"{sc['task']}_s{m.group(1)}")
    roles = defaultdict(list)
    if g:
        roles["ACT"] = [g["action"]]
        roles["OBJ"] = [g["object"]] if g["object"] else []
        roles["USE"] = sorted(g["uses"])
        roles["UNTIL"] = [g["until"]] if g["until"] else []
        roles["UNIT"] = [g["time"][2]] if g["time"] else []
        for kind, i in sorted(g.get("places", [])):
            roles[kind.upper()].append(i)
    if e["kind"] == "needs":
        roles["NEED"] = list(sc["needs"])
    spans = defaultdict(list)
    cur = None
    for tok, l in zip(e["toks"], e["labels"]):
        if l.startswith("B-"):
            cur = [tok]
            spans[l[2:]].append(cur)
        elif l.startswith("I-") and cur is not None:
            cur.append(tok)
        else:
            cur = None
    for role, syms_ in roles.items():
        for sym in syms_:
            words = str(sym).split("_")
            for sp in spans.get(role, []):
                if len(sp) == len(words) and all(word_match(a, b) for a, b in zip(sp, words)):
                    pairs += list(zip(sp, words))
                    break
    return pairs


# ------------------------------------------------------------------ main
def load_pairs():
    """Fallback answers (D16): {task, line, facts: [[pred, args...]...]} appended by teach3.

    Pairs from H3 tasks are never used (they were read during sealed H3 readings)."""
    h3 = set()
    man = os.path.join(ROOT, "data", "heldout3", "MANIFEST.sha256")
    if os.path.exists(man):
        for l in open(man):
            h3.add(l.split("  ", 1)[1].strip().split("/", 1)[1][:-3].split("__")[0])
    for f in os.listdir(os.path.join(ROOT, "data", "curriculum3", "heldout_domain")):
        h3.add(f[:-3])
    out = []
    if os.path.exists(PAIRS):
        for line in open(PAIRS, encoding="utf-8"):
            try:
                d = json.loads(line)
            except ValueError:
                continue
            if d.get("task") not in h3:
                out.append(d)
    return out


def pair_scenarios(pairs, base):
    """Turn fallback pairs into pseudo-scenarios so they align like teach lines."""
    by_task = defaultdict(list)
    for p in pairs:
        by_task[p["task"]].append(p)
    out = []
    for task, ps in by_task.items():
        facts = [(f[0], tuple(f[1:])) for p in ps for f in p["facts"]]
        steps = {}
        for pred, a in facts:
            if pred == "step":
                steps.setdefault(a[1], {"action": None, "object": None, "uses": set(), "until": None,
                                        "time": None, "overlaps": set(), "places": set()})
        for pred, a in facts:
            s = steps.get(a[0]) if a else None
            if s is None:
                continue
            if pred == "action":
                s["action"] = a[1]
            elif pred == "object_of":
                s["object"] = a[1]
            elif pred == "uses":
                s["uses"].add(a[1])
            elif pred == "until":
                s["until"] = a[1]
            elif pred == "takes_time":
                s["time"] = (a[1], a[2], a[3])
            elif pred == "overlaps":
                s["overlaps"].add(a[1])
            elif pred in ("into", "onto", "from"):
                s["places"].add((pred, a[1]))
        needs = [a[1] for pred, a in facts if pred == "needs"]
        out.append({"task": task, "teach": [p["line"] for p in ps], "steps": steps, "needs": needs,
                    "aliases": base.get(task, {}).get("aliases", []), "variants": []})
    return out


def load_dataset():
    from gt3 import scenario
    d = os.path.join(ROOT, "data", "dataset3")
    if not os.path.isdir(d):
        return []
    return [scenario(os.path.join(d, f)) for f in sorted(os.listdir(d)) if f.endswith(".md")]


def build(seed=1, exclude=(), include_dev=False, dataset=False):
    c = load_corpus("v3")
    train = c["train"] + (c["dev"] if include_dev else []) + (load_dataset() if dataset else [])
    train = [sc for sc in train if sc["task"] not in set(exclude)]
    for sc in train:
        for sid, g in sc["steps"].items():
            g["places"] = set()
        for p, a in sc["atoms"]:
            if p in ("into", "onto", "from") and a[0] in sc["steps"]:
                sc["steps"][a[0]]["places"].add((p, a[1]))
    for sc in train:
        sc["time"] = next((a[1:] for p, a in sc["atoms"] if p == "takes_time" and a[0] == sc["task"]), None)
    base = {sc["task"]: sc for sc in train}
    pairs_in = load_pairs()
    if AGREE:
        # self-training on agreement: keep a teacher reading only where a parser trained
        # without pairs gives the same action and object for that step
        plain = train_tagger([e for sc in train for e in teach_examples(sc) if e["kind"] == "step"], seed)
        kept = []
        seen = set()
        for p in pairs_in:
            key = (p["task"], p["line"])
            if key in seen:
                continue
            seen.add(key)
            acts = {f[1]: f[2] for f in p["facts"] if f[0] == "action" and len(f) == 3}
            objs = {f[1]: f[2] for f in p["facts"] if f[0] == "object_of" and len(f) == 3}
            toks = tokenize(p["line"])
            tags = plain.tag(toks, [])
            spans = defaultdict(list)
            for t, g in zip(toks, tags):
                if g.endswith("ACT") or g.endswith("OBJ"):
                    spans[g[2:]].append(t)
            la, lo = "_".join(spans.get("ACT", [])), "_".join(spans.get("OBJ", []))
            if any(a == la and (objs.get(s, "")[:4] == lo[:4]) for s, a in acts.items()):
                kept.append(p)
        pairs_in = kept
    if NO_DEV_PAIRS:
        dev_tasks = {sc["task"] for sc in c["dev"]}
        pairs_in = [p for p in pairs_in if p["task"] not in dev_tasks]
    pscs = pair_scenarios(pairs_in, base)
    teach = []
    pairs = []
    for sc in train + pscs:
        for e in teach_examples(sc):
            teach.append(e)
            pairs += form_pairs(sc, e)
    tagger = train_tagger([e for e in teach if e["kind"] == "step"], seed)
    needs_tagger = train_tagger([e for e in teach if e["kind"] == "needs"], seed)
    time_tagger = train_tagger([e for e in teach if e["kind"] == "time"], seed)
    kind_items = [(bow_feats(e["toks"]), e["kind"]) for e in teach]
    kinds = {k: train_binary([(f, int(y == k)) for f, y in kind_items]) for k in ("step", "needs", "time")}
    sit = [e for sc in train for e in situation_examples(sc)]
    # distillation: the teacher's situation readings (D16) are labelled pairs too
    sp = os.path.join(ROOT, "data", "training", "parser3", "situation_pairs.jsonl")
    if os.path.exists(sp) and not exclude:
        for line in open(sp, encoding="utf-8"):
            try:
                d = json.loads(line)
            except ValueError:
                continue
            facts = []
            for f in d["facts"]:
                if f[0] in SIT_ROLE and len(f) == 3:
                    facts.append((SIT_ROLE[f[0]], f[2]))
                elif f[0] == "time_limit" and len(f) == 4:
                    facts.append(("TIME", f[2]))
            toks = tokenize(d["text"])
            present = [x for x in facts if find_span(toks, str(x[1]), set(), {}) is not None]
            labels, _ = label(toks, present, {})
            sit.append({"toks": toks, "labels": labels, "known": []})
    sit_tagger = train_tagger(sit, seed) if sit else Tagger()
    det_items = []
    for sc in train:
        det_items += [(bow_feats(tokenize(r)), 1) for r in sc["requests"]]
        det_items += [(bow_feats(tokenize(v["request"])), 1) for v in sc["variants"] if v.get("request")]
        det_items += [(bow_feats(tokenize(l)), 0) for l in sc["teach"] + sc["situation"]]
        det_items += [(bow_feats(tokenize(v["situation"])), 0) for v in sc["variants"] if v.get("situation")]
    detect = train_binary(det_items)
    forms = learn_forms([{"toks": [], "labels": [], "pairs": pairs}])
    return {"version": 1, "epochs": EPOCHS, "train_tasks": len(train), "fallback_pairs": len(load_pairs()),
            "teach": {"tags": tagger.tags, "w": tagger.w},
            "needs": {"tags": needs_tagger.tags, "w": needs_tagger.w},
            "time": {"tags": time_tagger.tags, "w": time_tagger.w},
            "kinds": kinds,
            "situation": {"tags": sit_tagger.tags, "w": sit_tagger.w},
            "detect": detect, "forms": forms}, c, teach


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=os.path.join(ROOT, "data", "training", "parser3", "models", "current.json"))
    ap.add_argument("--parity", default=os.path.join(ROOT, "data", "training", "parser3", "parity.jsonl"))
    ap.add_argument("--agree", action="store_true", help="distil only teacher readings that agree with the parser")
    ap.add_argument("--no-pairs", action="store_true", help="train without fallback pairs")
    ap.add_argument("--no-dev-pairs", action="store_true", help="leave out pairs of dev tasks (fair dev measurement)")
    ap.add_argument("--dataset", action="store_true", help="C2: also train on data/dataset3 (out: models/c2.json)")
    ap.add_argument("--split", type=int, help="algorithmic tier: random split k of train+dev (20%% test)")
    a = ap.parse_args()
    global AGREE, PAIRS, NO_DEV_PAIRS
    AGREE = a.agree
    NO_DEV_PAIRS = a.no_dev_pairs
    if a.no_pairs:
        PAIRS = os.devnull
    exclude = ()
    if a.split is not None:
        c0 = load_corpus("v3")
        tasks = sorted(sc["task"] for sc in c0["train"] + c0["dev"])
        rnd = random.Random(1000 + a.split)
        exclude = sorted(rnd.sample(tasks, len(tasks) // 5))
        split_dir = os.path.join(ROOT, "data", "training", "parser3", "splits")
        os.makedirs(split_dir, exist_ok=True)
        with open(os.path.join(split_dir, f"split{a.split}.json"), "w") as f:
            json.dump({"split": a.split, "test": exclude}, f)
        a.out = os.path.join(split_dir, f"split{a.split}_models.json")
        a.parity = os.devnull
    if a.dataset:
        a.out = os.path.join(ROOT, "data", "training", "parser3", "models", "c2.json")
        a.parity = os.devnull
    model, c, _ = build(exclude=exclude, include_dev=a.split is not None, dataset=a.dataset)
    os.makedirs(os.path.dirname(a.out), exist_ok=True)
    with open(a.out, "w") as f:
        json.dump(model, f, sort_keys=True)
    # parity fixture: tags for every dev teach line, for the Rust runtime test
    t = Tagger()
    t.tags, t.w = model["teach"]["tags"], model["teach"]["w"]
    rows = []
    for sc in c["dev"]:
        sc["time"] = next((a[1:] for p, a in sc["atoms"] if p == "takes_time" and a[0] == sc["task"]), None)
        for e in teach_examples(sc):
            if e["kind"] != "step":
                continue
            rows.append({"toks": e["toks"], "known": e["known"], "tags": t.tag(e["toks"], e["known"])})
    with open(a.parity, "w") as f:
        for r in rows:
            f.write(json.dumps(r) + "\n")
    print(json.dumps({"out": os.path.relpath(a.out, ROOT), "features": len(model["teach"]["w"]),
                      "tags": model["teach"]["tags"], "situation_tags": model["situation"]["tags"],
                      "parity_rows": len(rows), "fallback_pairs": model["fallback_pairs"]}))


if __name__ == "__main__":
    main()
