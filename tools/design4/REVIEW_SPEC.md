# Spec: reviewing the Design 4.0 benchmark (milestone 35, design 10.1)

You are a reviewer, independent of the writers. For each question in your input file, judge **world truth**: would an ordinary careful adult agree with it about ordinary household things?

**Read only** this spec, the input file named in your task, and `research/Symbolic Learning Agent – Design 4.0.md` sections 4.1-4.4 (to read the formal language). Do not read `crates/`, `data/spike4/`, `data/kb4/`, `data/training/` or the writers' scratch scripts. No Haiku or Claude CLI calls. Do not commit.

For each question, check:
1. **Pieces**: is each piece true in the world? A piece about a typical member (`can_go_in(baking_tin, oven)`) is true when it holds for typical members.
2. **Gold**:
   - `answer`: is each gold atom true in the world?
   - `gap`: is the answer truly not settled by the pieces, and would the `missing` pieces (if true) settle it?
   - `hyp`: is the hypothesis a reasonable generalisation from the examples, and true in the world?
3. **Type**: does the question test what its `type` says (spec table in `tools/design4/BENCH_SPEC.md`, which you may also read)?

Write one JSON line per question to your output file:

```json
{"id": "w1-001", "verdict": "agree", "pieces_ok": true, "gold_ok": true, "type_ok": true, "note": ""}
```

`verdict` is `agree` only when all three hold; otherwise `disagree`, with a short note naming the false piece or gold. Be strict: when unsure, disagree. Judge every question; skip none.
