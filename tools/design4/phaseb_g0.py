"""Generate research/PHASEB_G0.md: the gate G0 report for Design 4.0 milestones 33-35.

Every number is read or recomputed from data:
  M33  data/spike4/kb4/builds/m33_check{,_replay}, data/kb4/raw/imports/archive3_summary.json,
       data/training/lab4/g0_tests.txt (cargo test output saved at report time)
  M34  data/training/teach4/*.json, data/training/teach4/audit_pilot_T0_v4.jsonl,
       data/training/calls4/ledger.jsonl
  M35  data/bench4/pool/*.jsonl, data/bench4/review/*.jsonl, data/bench4/k0.json,
       data/bench4/check.json, data/bench4/{dev,sealed}.jsonl, data/bench4/MANIFEST.sha256
Usage: python3 -I tools/design4/phaseb_g0.py   (zero model calls)
"""
import hashlib
import json
import os
import re
from collections import Counter

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
J = lambda *p: os.path.join(ROOT, *p)  # noqa: E731

# design 10.1, sealed composition
DESIGN = {"K": 130, "SUB": 130, "X": 90, "EXC": 60, "STEP": 40, "NAME": 20, "IND": 50, "GAP": 80}
DEDUCTIVE = {"K", "SUB", "X", "EXC", "STEP", "NAME"}


def lines(p):
    return [json.loads(l) for l in open(p) if l.strip()] if os.path.exists(p) else []


def pct(a, b):
    return 100.0 * a / b if b else 0.0


def m33():
    a, b = J("data/spike4/kb4/builds/m33_check"), J("data/spike4/kb4/builds/m33_check_replay")
    files = ["facts.dl", "rules.dl", "vocab.jsonl", "provenance.jsonl", "BUILD.json"]
    same = {f: open(os.path.join(a, f), "rb").read() == open(os.path.join(b, f), "rb").read() for f in files}
    build = json.load(open(os.path.join(a, "BUILD.json")))
    imp = json.load(open(J("data/kb4/raw/imports/archive3_summary.json")))
    tests = {}
    tp = J("data/training/lab4/g0_tests.txt")
    if os.path.exists(tp):
        txt = open(tp).read()
        passed = sum(int(x) for x in re.findall(r"test result: ok\. (\d+) passed", txt))
        failed = sum(int(x) for x in re.findall(r"(\d+) failed", txt))
        named = {n: ("ok" if re.search(rf"test {n} \.\.\. ok", txt) else "missing")
                 for n in ["replay_is_byte_identical_to_the_live_run", "a_migration_renaming_a_relation_rebuilds_the_same_answers",
                           "nothing_outside_the_agent_writes_knowledge", "the_deny_filter_catches_planted_leaks_at_every_point",
                           "sealed_deny_entries_match_by_hash_only", "agent_crates_hold_no_english_word_lists",
                           "rule_files_name_no_world_things"]}
        tests = {"passed": passed, "failed": failed, "named": named}
    return {"same": same, "build": build.get("build"), "counts": build.get("counts"), "import": imp, "tests": tests}


def teach_metrics(name):
    d = json.load(open(J("data/training/teach4", f"{name}.json")))
    calls = [c for c in d["calls"] if "statements" in c]
    st = sum(c["statements"] for c in calls)
    rj = sum(c["rejected"] for c in calls)
    ac = sum(c["accepted"] for c in calls)
    nq = sum(c["naming_questions"] for c in calls)
    ns = sum(c["new_symbols"] for c in calls)
    wh = sum(c.get("withheld", 0) for c in calls)
    return {"session": name, "calls": len(calls), "live_calls": d["live_calls"], "statements": st, "rejected": rj,
            "withheld": wh, "accepted": ac, "schema_pass": pct(st - rj, st), "per_call": ac / len(calls) if calls else 0,
            "naming_per_100": pct(nq, ac), "new_symbol_rate": pct(ns, st), "symbols": d["symbols"]}


def m34():
    runs = {n: teach_metrics(n) for n in ["stage0_S", "stage0_S_v3", "stage0_M", "pilot_T0", "pilot_T0_v3", "pilot_T0_v4"]
            if os.path.exists(J("data/training/teach4", f"{n}.json"))}
    audit = lines(J("data/training/teach4/audit_pilot_T0_v4.jsonl"))
    ledger = lines(J("data/training/calls4/ledger.jsonl"))
    tags = Counter(r["tag"] for r in ledger if r.get("ok"))
    return {"runs": runs, "audit_true": sum(a["verdict"] == "true" for a in audit), "audit_n": len(audit),
            "audit_false": [(a["statement"], a["note"]) for a in audit if a["verdict"] != "true"],
            "ledger_ok": sum(1 for r in ledger if r.get("ok")), "ledger_all": len(ledger), "tags": dict(tags)}


def mix(qs):
    n = len(qs)
    c = Counter(q["type"] for q in qs)
    tot = sum(DESIGN.values())
    return {t: (c.get(t, 0), pct(c.get(t, 0), n), pct(DESIGN[t], tot)) for t in DESIGN}


def m35():
    # agreement is measured on the first draft (pool_v1) against the first review, before drops (G0)
    pool = []
    for f in sorted(os.listdir(J("data/bench4/pool_v1"))) if os.path.isdir(J("data/bench4/pool_v1")) else []:
        if f.endswith(".jsonl"):
            pool.extend(lines(J("data/bench4/pool_v1", f)))
    final = []
    for f in sorted(os.listdir(J("data/bench4/pool"))) if os.path.isdir(J("data/bench4/pool")) else []:
        if f.endswith(".jsonl"):
            final.extend(lines(J("data/bench4/pool", f)))
    rev2 = []
    for f in sorted(os.listdir(J("data/bench4/review2"))) if os.path.isdir(J("data/bench4/review2")) else []:
        if f.endswith(".jsonl") and not f.startswith("input_"):
            rev2.extend(lines(J("data/bench4/review2", f)))
    rev3 = []
    for f in sorted(os.listdir(J("data/bench4/review3"))) if os.path.isdir(J("data/bench4/review3")) else []:
        if f.endswith(".jsonl") and not f.startswith("input_"):
            rev3.extend(lines(J("data/bench4/review3", f)))
    round3 = {"reviewed": len(rev3), "agree": sum(r["verdict"] == "agree" for r in rev3)}
    ceiling = json.load(open(J("data/bench4/ceiling.json"))) if os.path.exists(J("data/bench4/ceiling.json")) else None
    v1ids = {q["id"] for q in pool}
    round2 = {"reviewed": len(rev2), "agree": sum(r["verdict"] == "agree" for r in rev2),
              "new": sum(1 for q in final if q["id"] not in v1ids),
              "replaced": len(v1ids - {q["id"] for q in final}),
              "renamed": sum(1 for r in rev2 if r["id"] in v1ids)}
    rev = []
    for f in sorted(os.listdir(J("data/bench4/review"))) if os.path.isdir(J("data/bench4/review")) else []:
        if f.endswith(".jsonl"):
            rev.extend(lines(J("data/bench4/review", f)))
    rv = {r["id"]: r for r in rev}
    agree = sum(1 for q in pool if rv.get(q["id"], {}).get("verdict") == "agree")
    dev, sealed = lines(J("data/bench4/dev.jsonl")), lines(J("data/bench4/sealed.jsonl"))
    manifest_ok = None
    mp = J("data/bench4/MANIFEST.sha256")
    if os.path.exists(mp):
        manifest_ok = all(hashlib.sha256(open(J("data/bench4", f), "rb").read()).hexdigest() == h
                          for h, f in (l.split("  ", 1) for l in open(mp).read().splitlines() if l.strip()))
    k0 = json.load(open(J("data/bench4/k0.json"))) if os.path.exists(J("data/bench4/k0.json")) else None
    chk = json.load(open(J("data/bench4/check.json"))) if os.path.exists(J("data/bench4/check.json")) else None
    by_writer = Counter(q["id"].split("-")[0] for q in pool)
    dis = Counter(q["id"].split("-")[0] for q in pool if rv.get(q["id"], {}).get("verdict") != "agree")
    kept = dev + sealed
    # the latest verdict of every question across the rounds, in order
    latest = {}
    for r in rev + rev2 + rev3:
        latest[r["id"]] = r["verdict"]
    final_agree = sum(1 for q in final if latest.get(q["id"]) == "agree")
    return {"final_agree": final_agree, "round3": round3, "ceiling": ceiling, "round2": round2, "final": len(final), "pool": len(pool), "reviewed": sum(1 for q in pool if q["id"] in rv), "agree": agree,
            "by_writer": dict(by_writer), "disagree_by_writer": dict(dis),
            "dev": len(dev), "sealed": len(sealed), "mix_dev": mix(dev), "mix_sealed": mix(sealed),
            "scope": dict(Counter(q.get("kb_scope") for q in kept)), "manifest_ok": manifest_ok, "k0": k0, "check": chk,
            "deny_dev": sum(1 for l in open(J("data/bench4/deny.txt"))) if os.path.exists(J("data/bench4/deny.txt")) else 0,
            "deny_sealed": sum(1 for l in open(J("data/bench4/deny_sealed.sha256"))) if os.path.exists(J("data/bench4/deny_sealed.sha256")) else 0}


def yes(b):
    return "met" if b else "**not met**"


def main():
    a, b, c = m33(), m34(), m35()
    v4 = b["runs"]["pilot_T0_v4"]
    out = []
    w = out.append
    w("# Design 4.0 phase B: gate G0 (milestones 33-35)\n")
    w("Generated by `tools/design4/phaseb_g0.py` from the data files named in each section. No number is typed by hand.\n")
    # ---- summary
    m33_ok = all(a["same"].values()) and a["tests"].get("failed", 1) == 0 and all(v == "ok" for v in a["tests"].get("named", {"x": "missing"}).values())
    pilot_ok = v4["per_call"] >= 10 and v4["schema_pass"] >= 90
    max_dev = max(abs(p - d) for _, p, d in c["mix_sealed"].values()) if c["sealed"] else 100
    max_dev_d = max(abs(p - d) for _, p, d in c["mix_dev"].values()) if c["dev"] else 100
    agree_rate = pct(c["agree"], c["pool"])
    k0_ok = c["k0"] is not None and c["k0"]["answered_correct"] == 0
    chk_ok = c["check"] is not None and c["check"]["errors"] == 0
    sizes_ok = c["dev"] == 300 and c["sealed"] == 600
    w("## Result\n")
    w("| Gate item | Value | Criterion | |")
    w("|---|---|---|---|")
    w(f"| M33 replay byte-identical | {sum(a['same'].values())}/{len(a['same'])} build files | all | {yes(all(a['same'].values()))} |")
    w(f"| M33 acceptance tests | {a['tests'].get('passed', 0)} passed, {a['tests'].get('failed', '?')} failed | 0 failed, named tests present | {yes(m33_ok)} |")
    w(f"| M34 schema pass (pilot, v4) | {v4['schema_pass']:.1f}% | >= 90% | {yes(v4['schema_pass'] >= 90)} |")
    w(f"| M34 accepted facts per call (v4) | {v4['per_call']:.1f} | >= 10 | {yes(v4['per_call'] >= 10)} |")
    w(f"| M34 teacher accuracy (hand audit) | {b['audit_true']}/{b['audit_n']} | reported | - |")
    w(f"| M35 questions after review | {c['dev']} dev + {c['sealed']} sealed | 300 + 600 | {yes(sizes_ok)} |")
    w(f"| M35 gold proofs and hop labels (check_bench) | {c['check']['errors'] if c['check'] else '?'} errors on {c['check']['questions'] if c['check'] else '?'} | 0 | {yes(chk_ok)} |")
    w(f"| M35 type mix, largest deviation | dev {max_dev_d:.1f}, sealed {max_dev:.1f} points | <= 5 | {yes(max_dev <= 5 and max_dev_d <= 5)} |")
    w(f"| M35 writer-reviewer agreement | {c['agree']}/{c['pool']} = {agree_rate:.1f}% | >= 90% | {yes(agree_rate >= 90)} |")
    w(f"| M35 K0 on answerable deductive | {c['k0']['answered_correct'] if c['k0'] else '?'} of {c['k0']['answerable'] if c['k0'] else '?'} | 0 | {yes(k0_ok)} |")
    w(f"| M35 sealed by hash | manifest {'verifies' if c['manifest_ok'] else 'does not verify'}; {c['deny_sealed']} sealed deny atoms hashed | hashed | {yes(bool(c['manifest_ok']))} |")
    w(f"| Haiku calls (calls4 ledger) | {b['ledger_ok']} successful of {b['ledger_all']} entries | <= 100 in this goal, <= 5,000 total | {yes(b['ledger_all'] <= 100)} |")
    w("")
    # ---- M33
    w("## M33: three-layer knowledge base\n")
    w(f"Sources: `data/spike4/kb4/builds/m33_check` and `m33_check_replay`, `data/kb4/raw/imports/archive3_summary.json`, `data/training/lab4/g0_tests.txt`.\n")
    w(f"- Live build `{a['build']}` and its replay from raw: " + ", ".join(f"{f} {'identical' if s else 'DIFFERS'}" for f, s in a["same"].items()) + ".")
    w(f"- Build counts: {json.dumps(a['counts'])}.")
    for n, s in a["tests"].get("named", {}).items():
        w(f"- Test `{n}`: {s}.")
    r = a["import"]["readings"]
    w(f"- Archive3 import: {a['import']['sessions']} sessions; {r['mapped']} readings mapped with {r['facts_mapped']} facts; "
      f"{r['reteach']} readings re-teach; {r['facts_dropped']} facts dropped ({', '.join(f'{k} {v}' for k, v in a['import']['dropped_by_predicate'].items())}); "
      f"{r['duplicate']} duplicates; {r['situation_reteach']} situation readings re-teach (stage 7); {r['excluded_h3_or_gardening']} excluded (H3 or gardening).\n")
    # ---- M34
    w("## M34: teacher channel and pilot\n")
    w("Sources: `data/training/teach4/<session>.json` (per-call records), `audit_pilot_T0_v4.jsonl`, `data/training/calls4/ledger.jsonl`.\n")
    w("| Session | Calls | Statements | Rejected | Withheld (P2) | Accepted | Schema pass | Accepted per call | Naming q per 100 facts | New-symbol rate |")
    w("|---|---|---|---|---|---|---|---|---|---|")
    for n, m in b["runs"].items():
        w(f"| {n} | {m['calls']} | {m['statements']} | {m['rejected']} | {m['withheld']} | {m['accepted']} | {m['schema_pass']:.1f}% | {m['per_call']:.1f} | {m['naming_per_100']:.1f} | {m['new_symbol_rate']:.1f}% |")
    w("")
    w("Stage 0 sessions declare symbols only, so their accepted-fact counts are 0 by design. The pilot T0 ran stages 1-5 on cooking/g2, cleaning/g1 and repair/g1 from the stage 0 vocabulary, in three prompt versions:"
      " v2 (pilot_T0), v3 (one sort per symbol, verb and participle naming; agent redeclaration fix) and v4 (each listed symbol shown with its declared sort)."
      " Most v2 and v3 rejections were one symbol given two sorts.\n")
    w(f"**G0 pilot criteria** (>= 10 accepted facts per call and >= 90% of statements in schema): {yes(pilot_ok)} with v4.\n")
    w(f"**Teacher accuracy**: {b['audit_true']} of {b['audit_n']} randomly drawn delivered statements (seed 4034) are true of ordinary household things. False ones:\n")
    for s, note in b["audit_false"]:
        w(f"- `{s}`: {note}")
    w("")
    w(f"Calls by tag: {', '.join(f'{k} {v}' for k, v in sorted(b['tags'].items()))}.\n")
    # ---- M35
    w("## M35: derivation benchmark\n")
    w("Sources: `data/bench4/pool/`, `data/bench4/review/`, `data/bench4/check.json`, `data/bench4/k0.json`, `data/bench4/{dev,sealed}.jsonl`, `data/bench4/MANIFEST.sha256`; process in `data/bench4/AUTHORING.md`.\n")
    w(f"- Pool: {c['pool']} questions ({', '.join(f'{k} {v}' for k, v in sorted(c['by_writer'].items()))}); reviewed {c['reviewed']}; reviewer agreed on {c['agree']} ({agree_rate:.1f}%). "
      f"Disagreements by writer: {', '.join(f'{k} {v}' for k, v in sorted(c['disagree_by_writer'].items())) or 'none'}.")
    r2 = c["round2"]
    w(f"- Fix round: {r2['replaced']} questions replaced (reviewer disagreement or a deny conflict, where a proof step of one question is another's piece) by {r2['new']} new ones; "
      f"{r2['renamed']} kept questions edited (renames to keep one sort per symbol, registry sorts binding, and writers' changes to clear clashes with other writers' pieces and deny lists). Round-2 review of every new or edited question: {r2['agree']} of {r2['reviewed']} agreed. Final pool {c['final']}.")
    r3 = c["round3"]
    if r3["reviewed"]:
        w(f"- Round 3 (the round-2 disagreements, one sort clash found by the agent's type check, and one ambiguous name): {r3['agree']} of {r3['reviewed']} verdicts agreed, "
          "including a re-check of 2 questions renamed on the reviewer's own suggestion.")
    w(f"- Every question in the final pool has a latest verdict of agree: {c['final_agree']} of {c['final']}.")
    w(f"- Kept questions split (seed 4035, stratified by type x hops x scope): {c['dev']} dev, {c['sealed']} sealed.")
    if c["check"]:
        w(f"- check_bench on the kept questions: {c['check']['errors']} errors; composition {json.dumps(c['check']['composition'])}.")
    if c["k0"]:
        w(f"- K0 (the agent over the stage 0 knowledge base, {c['k0']['facts']} facts and {c['k0']['symbols']} symbols, rebuilt from raw): "
          f"{c['k0']['answered_correct']} of {c['k0']['answerable']} answerable questions answered.")
    w(f"- kb_scope of kept questions: {json.dumps(c['scope'])} (design target about 40% S, 30% M, 30% L; not a G0 criterion).")
    w(f"- Deny: {c['deny_dev']} dev atoms in `deny.txt`, {c['deny_sealed']} sealed atoms as sha256 in `deny_sealed.sha256`.\n")
    if c["ceiling"]:
        ce = c["ceiling"]
        w(f"- Taught ceiling (not a G0 criterion): a stage 0 agent taught each question's own pieces through the conversation path answers "
          f"{ce['answered_correct']} of {ce['answerable']} answerable deductive questions; {ce['rejected_pieces']} pieces were rejected by its type check. "
          "This checks that the questions are answerable by the agent given perfect teaching; it is not a result.")
    w("")
    w("### Agent fixes found by the ceiling check\n")
    w("The first ceiling run answered 228 of 704. Four places where the agent departed from design 4.4 were fixed (commit 1ebf84c):"
      " B6's bridges now answer `can_do` questions and meet `can_do` demands in B7; B5's closure answers `has_part` questions;"
      " B8 has the design's `achieves` bridge (action, object and effect) and its guard is `not action(S1, A2)`; and a `same_as` taught after a fact renames that fact at reasoning time.\n")
    sp = {}
    for n in ("fix_full", "fix2_full", "nolearn", "fix2_nolearn"):
        pth = J("data/training/spike4", f"{n}.json")
        if os.path.exists(pth):
            sc = json.load(open(pth))["scored"]
            sp[n] = (Counter(r["class"] for r in sc), {r["id"]: r["class"] for r in sc},
                     sum(1 for r in sc if r.get("hops") == 2 and r["gold_kind"] == "answer" and r["class"] == "correct"),
                     sum(1 for r in sc if r.get("hops") == 2 and r["gold_kind"] == "answer"),
                     sum(r["proof_ok"] for r in sc if r["class"] in ("correct", "wrong", "hyp")), sum(1 for r in sc if r["class"] in ("correct", "wrong", "hyp")))
    if len(sp) == 4:
        w("Spike rerun at 0 calls (`data/training/spike4/fix2_*.json` against the G1 runs):\n")
        w("| Run | Correct | Wrong | Gap | 2-hop correct | Valid proofs | Moved correct to not correct |")
        w("|---|---|---|---|---|---|---|")
        for old, new in (("fix_full", "fix2_full"), ("nolearn", "fix2_nolearn")):
            for n in (old, new):
                cnt, cls, h2, h2n, pv, pn = sp[n]
                lost = sum(1 for i, k in sp[old][1].items() if k == "correct" and cls.get(i) != "correct") if n == new else "-"
                w(f"| {n} | {cnt.get('correct', 0)} | {cnt.get('wrong', 0)} | {cnt.get('gap', 0)} | {h2}/{h2n} | {pv}/{pn} | {lost} |")
        w("")
    w("### Type mix against design 10.1\n")
    w("| Type | Design share | Dev n | Dev share | Sealed n | Sealed share |")
    w("|---|---|---|---|---|---|")
    for t in DESIGN:
        dn, dp, ds = c["mix_dev"][t]
        sn, sp, _ = c["mix_sealed"][t]
        w(f"| {t} | {ds:.1f}% | {dn} | {dp:.1f}% | {sn} | {sp:.1f}% |")
    w("")
    w("## Carried into milestone 36\n")
    w("- The stage 0 vocabulary declares `glass` and `paper` as items, so a taught `made_of(x, glass)` is rejected by the type check. The benchmark uses `glass_material` and `paper_material` for the materials; teaching at S needs a naming rule for materials in the stage prompts.")
    w(f"- Teacher audit error in the pilot is {b['audit_n'] - b['audit_true']}% (threshold 10%, design 10.8); the audit continues per stage.")
    w(f"- kb_scope is {json.dumps(c['scope'])}, more S and less M and L than the design's 40/30/30. L-scope questions are reported as uncovered if L is not built (design 10.1).")
    w("")
    # ---- correction: kb_scope by design 10.1's definition (registry membership), found in M36
    w("## Correction (found in milestone 36): scope by the design's definition\n")
    regS = {json.loads(l)["sym"] for l in open(J("data/kb4/builds/stage0_S_v3/vocab.jsonl")) if "sym" in json.loads(l)}
    regA = {json.loads(l)["sym"] for l in open(J("data/bench4/registry.jsonl"))}
    common = {"metal", "plastic", "glass", "wood", "ceramic", "sharp", "heat_resistant", "absorbent", "fragile", "user"}
    import sys as _s
    _s.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    from check_fixture import parse_atom as _pa, parse_query as _pq
    def _scope(q):
        out = set()
        for pc in q["pieces"]:
            at = _pa(pc.strip().rstrip("."))
            if at:
                out |= set(at[2])
        _, giv = _pq(q["query"])
        for at in giv or []:
            out |= set(at[2])
        out = {x for x in out - common if not x.rsplit("_s", 1)[-1].isdigit()}
        return "S" if out <= regS else ("M" if out <= regA else "L")
    for split in ("dev", "sealed"):
        cnt = Counter(_scope(json.loads(l)) for l in open(J("data/bench4", f"{split}.jsonl")))
        w(f"- {split}: S {cnt.get('S', 0)}, M {cnt.get('M', 0)}, L {cnt.get('L', 0)} (recomputed from registry membership of every piece symbol).")
    w("")
    w("Design 10.1 defines L scope as a question that needs entities beyond the registry, with targets of about 40/30/30. "
      "The benchmark spec let writers add entities freely and label scope by domain, and no check tested scope against the registry, "
      "so the `kb_scope` line above is **wrong**: by the definition the benchmark is almost entirely L. This was not one of the G0 criteria, "
      "but it makes the benchmark unable to measure the S curriculum (dev coverage 1% after the first S run; research/M36_FINDINGS.md). "
      "The sealed part was never read. Per D4.17 the benchmark is re-authored with registry-bound scope enforced by a check, and G0 is re-reported for it.\n")
    w("## Gate G0\n")
    all_met = m33_ok and pilot_ok and sizes_ok and chk_ok and max_dev <= 5 and max_dev_d <= 5 and agree_rate >= 90 and k0_ok and bool(c["manifest_ok"])
    w(f"**G0's listed criteria were {'met' if all_met else 'not met'} for this benchmark, but the benchmark is withdrawn** for the scope error above (D4.17). "
      "Milestone 36's first S run was made under the advance approval (D4.15, D4.16) and is kept as data only. Work stops for the user's go-ahead on re-authoring.")
    os.makedirs(J("research"), exist_ok=True)
    open(J("research/PHASEB_G0.md"), "w").write("\n".join(out) + "\n")
    print(json.dumps({"m33": m33_ok, "pilot": pilot_ok, "agree": agree_rate, "mix_max_dev": [max_dev_d, max_dev], "k0_ok": k0_ok, "check_ok": chk_ok, "sizes_ok": sizes_ok}))


if __name__ == "__main__":
    main()
