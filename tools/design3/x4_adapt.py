"""Experiment X4: adaptation core shoot-out in fact space (decision D6: cases and rules as arms).

Each variant case is (base procedure, situation facts, difference facts). Leave one task out:
the method sees every other task's facts and every other task's cases, and must produce the
difference for the held-out case. Methods:
  null        - no change (the base procedure)
  affordance  - substitution by function: the lacked item's role in a step is filled by an item
                the user has that plays the same role in some other known task; a lacked item
                with no stand-in drops its step; a wanted extra is added with an action it takes
                elsewhere (A3-lite, hand-written mechanism over learned role facts)
  cases       - A4: transfer the difference of the most similar stored case, mapped by role
                and by order position
  rules       - A3: lifted patterns (situation pattern -> change pattern) induced from the
                stored cases, kept when supported by >= 2 cases of other tasks
Scoring compares the adapted procedures, not the difference facts, so "swap" and
"drop + add_step" that give the same procedure score the same:
  change recall  = gold changes found / gold changes   (changes = facts added or removed)
  wrong changes  = predicted changes not in gold
  case pass      = all gold changes found and no wrong change

Usage: python3 -I tools/design3/x4_adapt.py [--split dev|train|all]
Calls no model.
"""
import argparse
import json
import os
import sys
from collections import Counter, defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from gt3 import load_corpus  # noqa: E402


# ------------------------------------------------------------------ procedures as fact sets
def base_proc(sc):
    steps = {}
    for sid in sc["order"]:
        g = sc["steps"][sid]
        steps[sid] = {"action": g["action"], "object": g["object"], "uses": set(g["uses"]),
                      "roles": dict(g["roles"])}
    return {"needs": set(sc["needs"]), "steps": steps}


def apply_diff(proc, diff):
    p = {"needs": set(proc["needs"]), "steps": {k: {"action": v["action"], "object": v["object"],
                                                     "uses": set(v["uses"]), "roles": dict(v["roles"])}
                                                 for k, v in proc["steps"].items()}}
    new = {}
    for pred, a in diff:
        if pred == "step":
            new[a[1]] = {"action": None, "object": None, "uses": set(), "roles": {}}
    for pred, a in diff:
        if pred in ("action", "object_of", "uses", "role") and a[0] in new:
            s = new[a[0]]
            if pred == "action":
                s["action"] = a[1]
            elif pred == "object_of":
                s["object"] = a[1]
            elif pred == "uses":
                s["uses"].add(a[1])
            else:
                s["roles"][a[1]] = a[2]
    for pred, a in diff:
        if pred == "swap" and a[0] in p["steps"]:
            s = p["steps"][a[0]]
            if a[1] in s["uses"]:
                s["uses"].discard(a[1]); s["uses"].add(a[2])
                if a[1] in s["roles"]:
                    s["roles"][a[2]] = s["roles"].pop(a[1])
            if s["object"] == a[1]:
                s["object"] = a[2]
            p["needs"].discard(a[1]); p["needs"].add(a[2])
        elif pred == "drop":
            p["steps"].pop(a[0], None)
        elif pred == "extra_need":
            p["needs"].add(a[0])
        elif pred == "no_need":
            p["needs"].discard(a[0])
        elif pred == "change" and a[0] in p["steps"] and a[1] in ("action", "object_of"):
            p["steps"][a[0]]["action" if a[1] == "action" else "object"] = a[3]
    for sid, s in new.items():
        p["steps"][sid] = s
    return p


def facts(proc):
    f = {("need", i) for i in proc["needs"]}
    for s in proc["steps"].values():
        f.add(("step", s["action"], s["object"]))
        for u in s["uses"]:
            f.add(("tool", s["action"], s["object"], u))
    return f


def changes(base, adapted):
    b, a = facts(base), facts(adapted)
    return {("+",) + x for x in a - b} | {("-",) + x for x in b - a}


# ------------------------------------------------------------------ knowledge from other tasks
def role_index(scs):
    """item -> Counter of roles it plays; role -> Counter of items; item -> actions it is object of."""
    item_roles = defaultdict(Counter)
    role_items = defaultdict(Counter)
    obj_actions = defaultdict(Counter)
    for sc in scs:
        for sid in sc["order"]:
            g = sc["steps"][sid]
            for i, r in g["roles"].items():
                item_roles[i][r] += 1
                role_items[r][i] += 1
            if g["object"]:
                obj_actions[g["object"]][g["action"]] += 1
    return item_roles, role_items, obj_actions


def situation(case):
    s = defaultdict(set)
    for p, a in case["situation_facts"] or []:
        s[p].add(a[1] if len(a) > 1 else a[0])
    return s


# ------------------------------------------------------------------ methods
def m_null(sc, case, kb):
    return []


def m_affordance(sc, case, kb):
    item_roles, role_items, obj_actions = kb
    sit = situation(case)
    proc = base_proc(sc)
    diff = []
    for lack in sorted(sit["lacks"] | sit["excludes"]):
        hit = False
        for sid, s in proc["steps"].items():
            if lack in s["uses"] or s["object"] == lack:
                role = s["roles"].get(lack)
                roles = {role} if role else set(item_roles.get(lack, {}))
                cands = [j for j in sorted(sit["has"]) if j not in proc["needs"] and roles & set(item_roles.get(j, {}))]
                if cands:
                    best = max(cands, key=lambda j: sum(item_roles[j][r] for r in roles))
                    diff.append(("swap", (sid, lack, best)))
                elif lack in s["uses"] and s["object"] != lack:
                    diff.append(("drop", (sid,)))
                hit = True
        if hit:
            diff.append(("no_need", (lack,)))
    for want in sorted(sit["wants"]):
        if want in proc["needs"]:
            continue
        acts = obj_actions.get(want)
        if not acts:
            continue
        act = acts.most_common(1)[0][0]
        last = sc["order"][-1]
        nid = f"{sc['task']}_s9"
        diff += [("extra_need", (want,)), ("add_step", (nid, last)), ("step", (sc["task"], nid)),
                 ("action", (nid, act)), ("object_of", (nid, want))]
    return diff


def signature(sc, case):
    """Lifted situation pattern: (predicate, role of the item in the base procedure)."""
    sit = situation(case)
    proc = base_proc(sc)
    sig = set()
    for p in ("lacks", "excludes", "wants"):
        for i in sit[p]:
            roles = {r for s in proc["steps"].values() for j, r in s["roles"].items() if j == i}
            if any(s["object"] == i for s in proc["steps"].values()):
                roles.add("object")
            sig |= {(p, r) for r in (roles or {"none"})}
    if sit["time_limit"] or any(pp == "time_limit" for pp, _ in case["situation_facts"] or []):
        sig.add(("time_limit", "-"))
    if any(pp == "count" for pp, _ in case["situation_facts"] or []):
        sig.add(("count", "-"))
    return sig


def lift(sc, case):
    """Change pattern of a case, in role terms: ops on the steps that use the situation's items."""
    sit = situation(case)
    proc = base_proc(sc)
    pats = []
    for p, a in case["difference_facts"] or []:
        if p == "swap":
            s = proc["steps"].get(a[0])
            r = s["roles"].get(a[1]) if s else None
            pats.append(("swap", r or ("object" if s and s["object"] == a[1] else None), a[2] in sit["has"]))
        elif p == "drop":
            s = proc["steps"].get(a[0])
            rs = {r for i, r in (s["roles"].items() if s else []) if i in sit["lacks"] | sit["excludes"]}
            pats.append(("drop", tuple(sorted(rs)) or None))
        elif p == "add_step":
            pats.append(("add", a[1] == sc["order"][-1] or a[1] == "start"))
        elif p == "action" and a[0].endswith("_s9"):
            pats.append(("add_action", a[1]))
    return pats


def realise(sc, case, pats, kb):
    """Turn lifted change patterns into concrete difference facts for this task."""
    item_roles, role_items, obj_actions = kb
    sit = situation(case)
    proc = base_proc(sc)
    diff = []
    lacked = sit["lacks"] | sit["excludes"]
    for pat in pats:
        if pat[0] == "swap":
            role = pat[1]
            for sid, s in proc["steps"].items():
                for i in sorted(lacked):
                    if (role and s["roles"].get(i) == role) or (role == "object" and s["object"] == i):
                        cands = [j for j in sorted(sit["has"]) if j not in proc["needs"]]
                        if role and role != "object":
                            cands = [j for j in cands if role in item_roles.get(j, {})] or cands
                        if cands:
                            diff.append(("swap", (sid, i, cands[0])))
        elif pat[0] == "drop":
            for sid, s in proc["steps"].items():
                if any(i in s["uses"] for i in lacked):
                    diff.append(("drop", (sid,)))
        elif pat[0] == "add_action":
            want = next(iter(sorted(sit["wants"] - proc["needs"])), None) or next(iter(sorted(sit["has"] - proc["needs"])), None)
            if want:
                nid = f"{sc['task']}_s9"
                diff += [("extra_need", (want,)), ("step", (sc["task"], nid)), ("action", (nid, pat[1])),
                         ("object_of", (nid, want))]
    for i in sorted(lacked):
        if any(d[0] == "swap" and d[1][1] == i for d in diff) or any(d[0] == "drop" for d in diff):
            diff.append(("no_need", (i,)))
    return diff


def m_cases(sc, case, kb, library):
    sig = signature(sc, case)
    best, bs = None, 0.0
    for osc, ocase in library:
        osig = signature(osc, ocase)
        if not sig or not osig:
            continue
        j = len(sig & osig) / len(sig | osig)
        if j > bs:
            best, bs = (osc, ocase), j
    if not best or bs < 0.34:
        return []
    return realise(sc, case, lift(*best), kb)


def m_rules(sc, case, kb, library):
    rules = defaultdict(Counter)
    for osc, ocase in library:
        key = frozenset(signature(osc, ocase))
        for pat in lift(osc, ocase):
            rules[key][pat] += 1
    sig = frozenset(signature(sc, case))
    pats = [pat for key, c in rules.items() if key and key <= sig for pat, n in c.items() if n >= 2]
    return realise(sc, case, pats, kb)


# ------------------------------------------------------------------ evaluation
def evaluate(split):
    c = load_corpus("v3", seed=True)
    pools = {"train": c["train"], "dev": c["dev"], "heldout_domain": c["heldout_domain"]}
    every = c["train"] + c["dev"] + c["seed"] + c["heldout_domain"]
    cases = [(sc, v) for name in (["train", "dev"] if split == "all" else [split]) for sc in pools[name]
             for v in sc["variants"] if v.get("difference_facts") is not None]
    if split == "heldout_domain":
        cases = [(sc, v) for sc in c["heldout_domain"] for v in sc["variants"] if v.get("difference_facts") is not None]
    library_all = [(sc, v) for sc in c["train"] + c["dev"] for v in sc["variants"] if v.get("difference_facts") is not None]
    res = {m: Counter() for m in ("null", "affordance", "cases", "rules")}
    examples = defaultdict(list)
    for sc, case in cases:
        kb = role_index([o for o in every if o["task"] != sc["task"]])
        lib = [(o, v) for o, v in library_all if o["task"] != sc["task"]]
        base = base_proc(sc)
        gold = changes(base, apply_diff(base, case["difference_facts"]))
        preds = {"null": m_null(sc, case, kb), "affordance": m_affordance(sc, case, kb),
                 "cases": m_cases(sc, case, kb, lib), "rules": m_rules(sc, case, kb, lib)}
        for m, d in preds.items():
            got = changes(base, apply_diff(base, d))
            hit = len(gold & got)
            wrong = len(got - gold)
            r = res[m]
            r["cases"] += 1
            r["gold_changes"] += len(gold)
            r["found"] += hit
            r["wrong"] += wrong
            r["pass"] += (hit == len(gold) and wrong == 0)
            r["any_wrong"] += wrong > 0
            if m in ("affordance", "cases") and len(examples[m]) < 6 and (hit < len(gold) or wrong):
                examples[m].append({"task": sc["task"], "change": case["change"],
                                    "missed": sorted(map(str, gold - got))[:4], "wrong": sorted(map(str, got - gold))[:4]})
    out = {}
    for m, r in res.items():
        out[m] = {"cases": r["cases"], "change_recall": round(r["found"] / max(r["gold_changes"], 1), 3),
                  "case_pass": r["pass"], "cases_with_wrong_change": r["any_wrong"], "wrong_changes": r["wrong"]}
    out["examples"] = examples
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--split", default="all", choices=["dev", "train", "all", "heldout_domain"])
    a = ap.parse_args()
    print(json.dumps(evaluate(a.split), indent=1))


if __name__ == "__main__":
    main()
