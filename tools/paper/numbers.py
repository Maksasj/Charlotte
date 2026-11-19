"""Generate every number in the research PDF from data/ (Design 3.0 milestone 29).

Writes paper/generated/numbers.tex (LaTeX macros) and paper/generated/SOURCES.md (macro ->
file and how it was computed, with the SHA-256 of each input file). `--check` regenerates
into memory and fails if anything differs from the committed files.

Usage: python3 -I tools/paper/numbers.py [--check]
Calls no model.
"""
import glob
import hashlib
import json
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
OUT = os.path.join(ROOT, "paper", "generated")
macros = {}
sources = {}
inputs = {}


def rel(p):
    return os.path.relpath(p, ROOT)


def read_json(p):
    inputs[rel(p)] = hashlib.sha256(open(p, "rb").read()).hexdigest()
    return json.load(open(p))


def put(name, value, src, how):
    assert re.match(r"^[A-Za-z]+$", name), name
    if isinstance(value, float):
        value = f"{value:.1f}"
    macros[name] = str(value)
    sources[name] = f"{src}: {how}"


def pct(k, n):
    return 100.0 * k / n if n else 0.0


def v1():
    d = read_json(os.path.join(ROOT, "data/paper/v1.json"))
    put("vOneKnowledgeHeldout", d["knowledge_tests_heldout_pct"], "data/paper/v1.json", "knowledge_tests_heldout_pct")
    put("vOneRealAnswers", d["real_answers"], "data/paper/v1.json", "real_answers")
    put("vOneRealAnswersOf", d["real_answers_of"], "data/paper/v1.json", "real_answers_of")
    put("vOneHelpUnderstood", d["understanding_help_pct"], "data/paper/v1.json", "understanding_help_pct")
    put("vOnePredicates", d["distinct_predicates_invented"], "data/paper/v1.json", "distinct_predicates_invented")


def v2():
    f = os.path.join(ROOT, "data/training/curriculum/2026-11-17.jsonl")
    inputs[rel(f)] = hashlib.sha256(open(f, "rb").read()).hexdigest()
    run1 = json.loads(open(f).read().splitlines()[1])
    for k, name in (("repeat", "Second"), ("transfer", "Transfer"), ("retention", "Retention")):
        put(f"vTwoClean{name}", run1[f"{k}_passed"], rel(f), f"line 2 (clean run 1) {k}_passed")
        put(f"vTwoClean{name}Of", run1[f"{k}_total"], rel(f), f"line 2 {k}_total")
    f = os.path.join(ROOT, "data/training/curriculum/2026-11-18.jsonl")
    inputs[rel(f)] = hashlib.sha256(open(f, "rb").read()).hexdigest()
    last = json.loads(open(f).read().splitlines()[-2])
    for k, name in (("repeat", "Second"), ("transfer", "Transfer"), ("retention", "Retention")):
        put(f"vTwoDev{name}", last[f"{k}_passed"], rel(f), f"second-to-last line (final development run) {k}_passed")


def runs():
    for p in sorted(glob.glob(os.path.join(ROOT, "data/training/runs3/*/summary.json"))):
        s = read_json(p)
        tag = re.sub(r"[^A-Za-z]", "", s["name"].title())
        for kind, v in s.get("by_kind", {}).items():
            k = re.sub(r"[^A-Za-z]", "", kind.title())
            put(f"run{tag}{k}Pass", v["pass"], rel(p), f"by_kind.{kind}.pass")
            put(f"run{tag}{k}N", v["n"], rel(p), f"by_kind.{kind}.n")
            put(f"run{tag}{k}Rate", 100 * v["rate"], rel(p), f"100 * by_kind.{kind}.rate")
            put(f"run{tag}{k}Lo", 100 * v["ci95"][0], rel(p), f"100 * by_kind.{kind}.ci95[0]")
            put(f"run{tag}{k}Hi", 100 * v["ci95"][1], rel(p), f"100 * by_kind.{kind}.ci95[1]")
            put(f"run{tag}{k}Wrong", v["with_wrong_claim"], rel(p), f"by_kind.{kind}.with_wrong_claim")
            put(f"run{tag}{k}Recall", 100 * v["mean_recall"], rel(p), f"100 * by_kind.{kind}.mean_recall")
        if "live_calls" in s:
            put(f"run{tag}Calls", s["live_calls"], rel(p), "live_calls")
        ing = s.get("ingest")
        if ing:
            put(f"run{tag}Lines", ing["lines"], rel(p), "ingest.lines")
            put(f"run{tag}Fallback", ing["fallback"], rel(p), "ingest.fallback")
            put(f"run{tag}Learned", ing["learned"], rel(p), "ingest.learned")
        for st in ("stored_T", "stored_G"):
            if st in s:
                k = st.split("_")[1]
                put(f"run{tag}Stored{k}Pass", s[st]["pass"], rel(p), f"{st}.pass")
                put(f"run{tag}Stored{k}N", s[st]["n"], rel(p), f"{st}.n")


def models():
    p = os.path.join(ROOT, "data/training/parser3/models/theta.json")
    if os.path.exists(p):
        t = read_json(p)
        put("parserCoverage", 100 * t["coverage_at_95"], rel(p), "100 * coverage_at_95")
        put("parserDevLines", t["dev_step_lines"], rel(p), "dev_step_lines")
    led = os.path.join(ROOT, "data/training/calls3/ledger.jsonl")
    inputs[rel(led)] = hashlib.sha256(open(led, "rb").read()).hexdigest()
    put("haikuCalls", sum(1 for _ in open(led)), rel(led), "line count (live calls)")
    ds = glob.glob(os.path.join(ROOT, "data/dataset3/*.md"))
    put("datasetTasks", len(ds), "data/dataset3/*.md", "file count")


def corpus():
    sys.path.insert(0, os.path.join(ROOT, "tools", "design3"))
    from gt3 import load_corpus
    c = load_corpus("v3", seed=True)
    steps = sum(len(sc["steps"]) for v in c.values() for sc in v)
    tasks = sum(len(v) for v in c.values())
    put("gtTasks", tasks + 1, "data/curriculum3", "scenario files (incl. water_garden seed)")
    put("gtSteps", steps, "data/curriculum3", "step facts")
    for t in ("S", "T", "G"):
        put(f"hThree{t}", len(glob.glob(os.path.join(ROOT, f"data/heldout3/{t}/*.md"))), f"data/heldout3/{t}", "file count")


def render():
    tex = "% generated by tools/paper/numbers.py -- do not edit\n" + "".join(
        f"\\newcommand{{\\{k}}}{{{v}}}\n" for k, v in sorted(macros.items()))
    src = "# Sources of every number\n\n| Macro | Value | Source |\n|---|---|---|\n" + "".join(
        f"| `\\{k}` | {macros[k]} | {sources[k]} |\n" for k in sorted(macros)) + "\n## Inputs (SHA-256)\n\n" + "".join(
        f"- `{k}` {v}\n" for k, v in sorted(inputs.items()))
    return tex, src


def main():
    v1(); v2(); runs(); models(); corpus()
    tex, src = render()
    if "--check" in sys.argv:
        old = open(os.path.join(OUT, "numbers.tex")).read()
        if old != tex:
            print("numbers.tex differs from data; run tools/paper/numbers.py")
            sys.exit(1)
        print(f"ok: {len(macros)} numbers match")
        return
    os.makedirs(OUT, exist_ok=True)
    open(os.path.join(OUT, "numbers.tex"), "w").write(tex)
    open(os.path.join(OUT, "SOURCES.md"), "w").write(src)
    print(f"{len(macros)} numbers written")


if __name__ == "__main__":
    main()
