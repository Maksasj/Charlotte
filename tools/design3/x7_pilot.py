"""Experiment X7: does teacher-generated supervision help the learned parser?

Haiku rewrites the facts of each training step as several differently worded instructions
(the facts are known, so each sentence is a labelled pair by construction). The X1 parser is
retrained with these pairs and re-tested on the development teach lines.

Usage: python3 -I tools/design3/x7_pilot.py --tasks 20 [--gt v3]
Live calls: one per task (cached; re-runs cost nothing).
"""
import argparse
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import haiku  # noqa: E402
import x1_parser as X  # noqa: E402
from gt3 import load_corpus, tokenize  # noqa: E402

SYSTEM = ("You write household instructions for a dataset. Reply with JSON only, no prose, "
          "no code fences.")


def describe(sc):
    rows = []
    for sid in sc["order"]:
        g = sc["steps"][sid]
        parts = [f"action={g['action']}"]
        if g["object"]:
            parts.append(f"object={g['object']}")
        for u in sorted(g["uses"]):
            parts.append(f"uses={u}")
        for i in sorted(g["into"]):
            parts.append(f"into/onto={i}")
        if g["until"]:
            parts.append(f"until={g['until']}")
        if g["time"]:
            lo, hi, u = g["time"]
            parts.append(f"time={lo}-{hi} {u}" if lo != hi else f"time={lo} {u}")
        for a, v, u in g["settings"]:
            parts.append(f"setting {a}={v} {u}")
        rows.append(f"{sid.rsplit('_s', 1)[1]}: " + ", ".join(parts))
    return "\n".join(rows)


def prompt_for(sc):
    return (
        f"Task: {sc['task'].replace('_', ' ')}\n"
        f"Items needed: {', '.join(i.replace('_', ' ') for i in sc['needs'])}\n"
        f"Steps (facts):\n{describe(sc)}\n\n"
        "For each step, write 3 different one-sentence instructions that state exactly these facts and "
        "nothing else. Vary the sentence structure (word order, 'using'/'with', clause order, "
        "imperative or 'you'), but write every action, object and item with the words given "
        "(underscores become spaces; plurals are fine). Write numbers as digits.\n"
        "Also write 3 different sentences that list the items needed.\n"
        'JSON: {"steps": {"1": ["...", "...", "..."], ...}, "needs": ["...", "...", "..."]}'
    )


def parse_answer(text):
    m = re.search(r"\{.*\}", text, re.S)
    return json.loads(m.group(0)) if m else None


def generated_examples(sc, ans):
    out = []
    phrase = sc["task"].replace("_", " ")
    for num, sents in (ans.get("steps") or {}).items():
        sid = f"{sc['task']}_s{num}"
        if sid not in sc["steps"]:
            continue
        for s in sents:
            line = f"Step {num} of {phrase}: {s}"
            tmp = dict(sc)
            tmp["teach"] = [line]
            out += [e for e in X.step_examples(tmp) if e["kind"] == "step"]
    for s in ans.get("needs") or []:
        line = f"To {phrase} you need {re.sub(r'(?i)^.*?(you need|needed:|need)\s*', '', s)}"
        tmp = dict(sc)
        tmp["teach"] = [line]
        out += [e for e in X.step_examples(tmp) if e["kind"] == "needs"]
    return out


def evaluate(train_ex, dev_ex):
    for e in train_ex + dev_ex:
        e["labels"], e["missed"] = X.label_line(e["toks"], e["facts"])
    usable = [e for e in train_ex if not e["missed"]]
    X.KNOWN = False
    model = X.train([e for e in usable if e["kind"] == "step"])
    forms = X.learn_forms(usable)
    ok = n = 0
    roles = {}
    for e in dev_ex:
        if e["kind"] != "step":
            continue
        r = X.compare_step(X.decode(e["toks"], X.tag_seq(model, e["toks"]), forms), e)
        n += 1
        ok += r["exact"]
        for k, v in r.items():
            roles[k] = roles.get(k, 0) + v
    return {"train_lines": len(usable), "dev_lines": n, "exact": ok, "exact_rate": round(ok / max(n, 1), 3),
            "per_role": {k: round(v / max(n, 1), 3) for k, v in roles.items() if k != "exact"}}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--tasks", type=int, default=20)
    ap.add_argument("--gt", default="v3", choices=["v2", "v3"])
    a = ap.parse_args()
    c = load_corpus(a.gt)
    base = [e for sc in c["train"] for e in X.step_examples(sc)]
    dev = [e for sc in c["dev"] for e in X.step_examples(sc)]
    gen = []
    calls = 0
    for sc in c["train"][: a.tasks]:
        ans = parse_answer(haiku.ask(SYSTEM, prompt_for(sc), tag="x7_paraphrase"))
        calls += 1
        if ans:
            gen += generated_examples(sc, ans)
    before = evaluate([dict(e) for e in base], [dict(e) for e in dev])
    after = evaluate([dict(e) for e in base] + gen, [dict(e) for e in dev])
    # the full X1 configuration (needs-line feature, all training lines) with and without the pairs
    X.KNOWN = True
    full_before = X.run(a.gt)["dev"]
    X.KNOWN = True
    full_after = X.run(a.gt, extra=[dict(e) for e in gen])["dev"]
    full = {k: {x: v[x] for x in ("exact_rate", "per_role")} for k, v in (("before", full_before), ("after", full_after))}
    out = {"tasks": a.tasks, "calls": calls, "generated_pairs": len(gen), "before": before, "after": after,
           "gain_per_100_pairs": round((after["exact_rate"] - before["exact_rate"]) * 100 / max(len(gen) / 100, 1e-9), 2),
           "live_calls_total": haiku.live_calls(), "x1_config": full}
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
