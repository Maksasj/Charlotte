"""Score the agent on bench4 questions (Design 4.0 sections 10.2-10.3, milestone 36).

Input: the files of `sla4 bench --from ... --file Q.jsonl --score OUT` (OUT.jsonl answers,
OUT_proofs.jsonl proof records, OUT_proofs.jsonl.facts delivered facts, OUT_canon.json names).
Every proof is checked by the independent tools/design4/ref_reason.py; coverage (the KB-relative
gold) is computed by the independent reasoner of check_fixture.py over the delivered facts.

Classes (exactly one per question):
  correct      the world gold is found, every claim's proof checks, no excluded binding claimed;
               for IND (gold hyp) the gold is claimed (marked hypothesis or deduced)
  invented     a claim whose proof does not check (including a correct answer with a bad proof)
  hyp          a hypothesis (learned rule, rank L) claimed where the gold is not a hypothesis (D4.14)
  wrong        claims with valid proofs, but not the world gold (or an excluded binding)
  gap_correct  no claim where the world gold is gap
  gap_just     no claim where the KB-relative gold is not derivable (a justified gap; no credit)
  gap_missed   no claim where the world gold was derivable from what was taught
Usage: python3 -I tools/design4/score_bench.py --run OUT --file Q.jsonl [--json SUMMARY.json]
"""
import json
import math
import os
import subprocess
import sys
from collections import Counter, defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from check_fixture import Reasoner, parse_atom, parse_query  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))


def norm(s):
    t = "".join(s.split()).rstrip(".")
    return t.replace("swap(", "stand_in(", 1)


def wilson(k, n, z=1.96):
    if n == 0:
        return (0.0, 0.0)
    p = k / n
    d = 1 + z * z / n
    c = p + z * z / (2 * n)
    r = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n))
    return ((c - r) / d, (c + r) / d)


def surface_fact(line):
    a = parse_atom(line.strip())
    if a is None:
        return None
    neg, p, args = a
    if p in ("pos", "neg") and len(args) == 3:
        return (p == "neg", args[1], (args[0], args[2]))
    if p in ("pos3", "neg3") and len(args) == 4:
        return (p == "neg3", args[1], (args[0], args[2], args[3]))
    return (neg, p, args)


def main(argv):
    run = argv[argv.index("--run") + 1]
    qfile = argv[argv.index("--file") + 1]
    qs = {json.loads(l)["id"]: json.loads(l) for l in open(qfile) if l.strip()}
    ans = {json.loads(l)["id"]: json.loads(l) for l in open(run + ".jsonl") if l.strip()}
    canon_map = json.load(open(run + "_canon.json")) if os.path.exists(run + "_canon.json") else {}

    def canon(x):
        seen = set()
        while x in canon_map and canon_map[x] != x and x not in seen:
            seen.add(x)
            x = canon_map[x]
        return x
    # proof checks
    ref_out = run + "_ref.jsonl"
    subprocess.run([sys.executable, "-I", os.path.join(HERE, "ref_reason.py"), "--proofs", run + "_proofs.jsonl",
                    "--facts", run + "_proofs.jsonl.facts", "--out", ref_out], capture_output=True)
    ok = defaultdict(dict)
    for l in open(ref_out):
        r = json.loads(l)
        ok[r["q"]][norm(r["claim"])] = r["ok"]
    # delivered facts, surface form, for coverage
    facts = [f for f in (surface_fact(l) for l in open(run + "_proofs.jsonl.facts") if l.strip()) if f]
    taught = [(f, "t") for f in facts]
    rows = []
    for qid, q in qs.items():
        a = ans.get(qid, {"claims": [], "gaps": [], "gold_canon": [], "excluded_canon": []})
        claims = {norm(c["claim"]): c for c in a["claims"]}
        valid = {c: ok[qid].get(c, False) for c in claims}
        kind = q["gold"]["kind"]
        golds = [norm(g) for g in a.get("gold_canon") or q["gold"].get("atoms", [])]
        excl = [norm(g) for g in a.get("excluded_canon") or q["gold"].get("excluded", [])]
        # coverage: does the world gold follow from what was taught (plus the question's given)?
        covered = None
        if kind == "answer":
            goal, given = parse_query(q["query"])
            cg = [(n_, p_, tuple(canon(x) for x in args)) for n_, p_, args in (given or [])]
            rs = Reasoner(taught, cg)
            covered = all(rs.prove((g_[0], g_[1], tuple(canon(x) for x in g_[2]))) is not None
                          for g_ in (parse_atom(g) for g in q["gold"]["atoms"]) if g_)
        bad = [c for c, v in valid.items() if not v]
        hyp_only = claims and all(c.get("kind") == "hypothesis" for c in claims.values())
        if kind == "answer":
            found = all(g in claims for g in golds) and golds
            if bad:
                cls = "invented"
            elif found and not any(e in claims for e in excl):
                cls = "hyp" if all(claims[g].get("kind") == "hypothesis" for g in golds) else "correct"
            elif claims:
                cls = "hyp" if hyp_only else "wrong"
            else:
                cls = "gap_missed" if covered else "gap_just"
        elif kind == "gap":
            cls = "invented" if bad else ("wrong" if claims else "gap_correct")
        else:  # hyp
            found = golds and all(g in claims for g in golds)
            cls = "invented" if bad else ("correct" if found else ("wrong" if claims else "gap_just"))
        rows.append({"id": qid, "type": q["type"], "hops": q.get("hops"), "kind": kind, "scope": q.get("kb_scope"),
                     "class": cls, "covered": covered, "claims": len(claims), "bad_claims": len(bad), "ms": a.get("ms", 0)})
    n = len(rows)
    cc = Counter(r["class"] for r in rows)
    claims_total = sum(r["claims"] for r in rows)
    bad_total = sum(r["bad_claims"] for r in rows)
    ans_rows = [r for r in rows if r["kind"] == "answer"]
    cov = [r for r in ans_rows if r["covered"]]
    two = [r for r in ans_rows if r["hops"] == 2]
    ms = sorted(r["ms"] for r in rows)
    summary = {
        "questions": n, "classes": dict(cc),
        "correct": cc["correct"], "correct_ci": wilson(cc["correct"], n),
        "claims": claims_total, "invalid_claims": bad_total, "proof_validity": 1 - bad_total / claims_total if claims_total else 1.0,
        "invented_questions": cc["invented"],
        "answerable": len(ans_rows), "covered": len(cov), "coverage": len(cov) / len(ans_rows) if ans_rows else 0,
        "conditional_accuracy": sum(r["class"] == "correct" for r in cov) / len(cov) if cov else 0,
        "two_hop": [sum(r["class"] == "correct" for r in two), len(two)],
        "s_scope_coverage": [sum(1 for r in ans_rows if r["scope"] == "S" and r["covered"]), sum(1 for r in ans_rows if r["scope"] == "S")],
        "by_type": {t: dict(Counter(r["class"] for r in rows if r["type"] == t)) for t in sorted({r["type"] for r in rows})},
        "p95_ms": ms[int(0.95 * (len(ms) - 1))] if ms else 0,
    }
    with open(run + "_scored.jsonl", "w") as f:
        for r in rows:
            f.write(json.dumps(r) + "\n")
    if "--json" in argv:
        json.dump(summary, open(argv[argv.index("--json") + 1], "w"), indent=1)
    print(json.dumps(summary))


if __name__ == "__main__":
    main(sys.argv[1:])
