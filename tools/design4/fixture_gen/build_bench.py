import json, os, sys
sys.path.insert(0, "/home/maksasj/Programming/Charlotte/tools/design4")
import check_fixture as cf
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from qspec import QS
ROOT = "/home/maksasj/Programming/Charlotte/data/spike4"
fx = cf.Fixture(ROOT); fx.load_teach(); fx.load_answers(); fx.build_canon()
facts = fx.taught_facts()
srcs = {st["src"]: st for st in fx.statements}
P = cf.parse_atom
def C(s): return fx.canon_atom(P(s))
def S(at): return cf.astr(at)

def syms_needing_alias(atoms):
    out = []
    for at in atoms:
        for a in at[2]:
            if fx.canon(a) != a:
                out += fx.same_as_src.get(a, [])
    return out

def lines_of(proofs, extra_raw=()):
    ls = []
    raw = list(extra_raw)
    for pr in proofs:
        for lf in pr.leaves():
            if lf.src and lf.src not in ("given", "withheld"):
                ls.append(lf.src); raw.append(srcs[lf.src]["atom"])
    ls += syms_needing_alias(raw)
    seen, out = set(), []
    for l in ls:
        if l not in seen:
            seen.add(l); out.append(l)
    def key(l):
        f, n = l.rsplit(":", 1); return (f, int(n))
    return sorted(out, key=key)

deny = []
def add_deny(atoms, why):
    for a in atoms:
        deny.append((S(a), why))

recs, problems = [], []
for n, q in enumerate(QS, 1):
    qid = "q%03d" % n
    goal, given = cf.parse_query(q["q"])
    given_c = [fx.canon_atom(g) for g in given]
    rs = cf.Reasoner(facts, given_c)
    gold = {"kind": q["k"], "atoms": q["a"]}
    proofs, extra_raw = [], list(given) + ([goal] if goal[0] != "request" else [])
    hop = 0
    if q["k"] == "answer":
        for a in q["a"]:
            pr = rs.prove(C(a))
            if pr is None or pr.rule in ("taught", "given"):
                problems.append("%s %s: gold %s not derived (%s)" % (qid, q["t"], a, pr and pr.rule)); continue
            hop = max(hop, pr.cost); proofs.append(pr)
            add_deny([P(a), C(a)] + pr.inner(), qid)
        if hop != q["h"]:
            problems.append("%s %s: hops %s computed %d  %s" % (qid, q["t"], q["h"], hop, q["a"]))
        for e in q.get("ex", []):
            key = C(e)
            if rs.prove(key) is not None:
                problems.append("%s excluded %s derivable" % (qid, e))
            for dleaf, pr in rs.unfit.get(key, []):
                proofs += [dleaf, pr]
            if key not in rs.unfit:
                problems.append("%s excluded %s not unfit_known" % (qid, e))
        gold["excluded"] = q.get("ex", [])
        if not gold["excluded"]: del gold["excluded"]
    elif q["k"] == "gap":
        for a in q["a"]:
            ca = C(a)
            if ca[1] == "gap":
                pr = rs.prove(ca)
                if pr is None: problems.append("%s gap %s not derived" % (qid, a))
                add_deny([P(a), ca], qid)
            else:
                if rs.prove(ca) is not None or rs.contrary_known(ca):
                    problems.append("%s gap %s decided" % (qid, a))
                add_deny([P(a), ca], qid)
        for c in q.get("ctx", []):
            pr = rs.prove(C(c))
            if pr is None: problems.append("%s ctx %s not provable" % (qid, c))
            else: proofs.append(pr)
        for key, opens in rs.ask_fit.items():
            pass
        if q.get("miss"):
            gold["missing"] = q["miss"]; add_deny([P(m) for m in q["miss"]] + [C(m) for m in q["miss"]], qid + " withheld")
            rs2 = cf.Reasoner(facts + [(C(m), "withheld") for m in q["miss"]], given_c)
            for c in q.get("comp", []):
                pr2 = rs2.prove(C(c))
                if pr2 is None: problems.append("%s: missing does not complete %s" % (qid, c))
                else: add_deny([P(c), C(c)] + pr2.inner(), qid + " completes")
            gold["completes"] = q.get("comp", [])
        if q.get("rule"):
            gold["rule"] = q["rule"]
            st = cf.rule_stats(rs, q["rule"], fx)
            if st["accepted"]: problems.append("%s rule accepted %s" % (qid, st))
            print("   %s %s: s=%d %s c=%s kinds=%d u=%s" % (qid, q["ind"], len(st["support"]), st["support"][:6], st["counter"][:4], st["kinds"], st["unknown"][:5]))
            for x in st["support"] + st["counter"]:
                h, b = cf.parse_rule(q["rule"])
                for at in (b, h):
                    gat = (at[0], at[1], tuple(x if v == "X" else fx.canon(v) for v in at[2]))
                    pr = rs.prove(gat) or rs.prove((not gat[0], gat[1], gat[2]))
                    if pr: proofs.append(pr)
    elif q["k"] == "hyp":
        gold["rule"] = q["rule"]
        st = cf.rule_stats(rs, q["rule"], fx)
        print("   %s hyp: s=%d %s c=%s kinds=%d u=%s acc=%s" % (qid, len(st["support"]), st["support"][:8], st["counter"], st["kinds"], st["unknown"], st["accepted"]))
        if not st["accepted"]: problems.append("%s rule not accepted" % qid)
        h, b = cf.parse_rule(q["rule"])
        for a in q["a"]:
            ca = C(a); x = ca[2][0]
            if rs.prove(ca) is not None or rs.contrary_known(ca): problems.append("%s hyp decided" % qid)
            bat = (b[0], b[1], tuple(x if v == "X" else fx.canon(v) for v in b[2]))
            bp = rs.prove(bat); proofs.append(bp)
            add_deny([P(a), ca] + bp.inner(), qid)
        for x in st["support"]:
            for at in (b, h):
                gat = (at[0], at[1], tuple(x if v == "X" else fx.canon(v) for v in at[2]))
                proofs.append(rs.prove(gat))
        gold["min_support"] = 3
    gold["proof_lines"] = lines_of(proofs, extra_raw)
    rec = {"id": qid, "type": q["t"], "hops": q["h"], "near_miss": bool(q.get("nm", False)), "query": q["q"], "gold": gold}
    if q.get("ind"): rec["ind_case"] = q["ind"]
    if q.get("note"): rec["note"] = q["note"]
    recs.append(rec)
    print("%s %-4s h=%s %-6s %s" % (qid, q["t"], q["h"], q["k"], " | ".join(q["a"])))

with open(os.path.join(ROOT, "bench/questions.jsonl"), "w") as fh:
    for r in recs: fh.write(json.dumps(r) + "\n")
seen = {}
for a, why in deny:
    seen.setdefault(a, []).append(why)
with open(os.path.join(ROOT, "bench/deny.txt"), "w") as fh:
    fh.write("% Design 4.0 spike fixture deny list (D4.8): every gold atom, every intermediate proof node, and every\n")
    fh.write("% withheld piece of bench/questions.jsonl. None may be taught. Compare after same_as canonicalisation;\n")
    fh.write("% the swap form of each stand_in is listed too. Comment: the questions that need the atom.\n")
    for a in sorted(seen):
        fh.write("%-70s %% %s\n" % (a, ", ".join(sorted(set(seen[a])))))
print("\n%d problems" % len(problems)); print("\n".join(problems))
