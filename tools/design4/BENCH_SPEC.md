# Spec: the Design 4.0 derivation benchmark, version 2 (milestone 35, decisions D4.9 and D4.17)

Version 2 replaces the first benchmark, which was withdrawn because almost every question needed entities beyond the registry (D4.17). The changes are the **scope rule** below, binding registry sorts, and a scope check in `check_bench.py`.

You write derivation questions for a symbolic agent that learns household knowledge from formal teaching and answers by deriving new facts with proofs. **Read only** these sources:
- this spec;
- `research/Symbolic Learning Agent – Design 4.0.md`, sections 4.1-4.4 (relations, sorts, base rules B1-B9) and 10.1-10.3 (the benchmark);
- `data/bench4/registry.jsonl` (all symbol names and sorts: the full registry), `data/bench4/registry_S.jsonl` (the S registry: cooking, cleaning, repair), and `data/bench4/procedure_ids.txt` (task and step ids that the curriculum teaches);
- for procedures, the ground-truth blocks of `data/curriculum3/train/*.md`, `data/curriculum3/dev/*.md` and `data/dataset3/*.md` (step ids, actions, roles). Read only the `## ground truth` block of a dataset3 file, never its variants.

**Do not read** `crates/`, `data/spike4/`, `data/heldout3/`, `data/kb4/`, `data/training/` or `data/bench4_v1/`. The benchmark must be written without sight of the agent, its teaching or its outputs.

## What a question is

A question states a goal the agent should be able to derive from **pieces**: facts a teacher could plausibly teach, true of ordinary household things. The answer itself is **never** a piece. Every gold answer must follow from its pieces (plus `given` facts) by the base rules B1-B9 at the stated hop count, and be true in the world.

## Record format (one JSON object per line, schema `bench4/1`)

```json
{"id": "w1-001", "schema": "bench4/1", "type": "K", "hops": 2, "kb_scope": "S", "domains": ["cooking"],
 "query": "query: can_go_in(loaf_tin, oven).",
 "gold": {"kind": "answer", "atoms": ["can_go_in(loaf_tin, oven)"]},
 "pieces": ["is_a(loaf_tin, bakeware).", "is_a(bakeware, cookware).", "can_go_in(cookware, oven)."],
 "new": ["bakeware:item", "cookware:item"],
 "gold_proof": "can_go_in(loaf_tin, oven) <- B2 inherits can_go_in(cookware, oven) via is_a(loaf_tin, bakeware), is_a(bakeware, cookware)",
 "rules_needed": ["B1", "B2"],
 "deny": ["can_go_in(loaf_tin, oven)", "can_go_in(bakeware, oven)", "is_a(loaf_tin, cookware)"]}
```

- `query`: either `query: <atom>.` (a ground atom, or `not <atom>` for a negative), with optional `given: <facts>.`, or `request(<task>). <situation facts>.` for substitution and step questions. Situation facts are `lacks(user, I)`, `has(user, J)` and `state_of(I, S)`.
- `gold.kind`:
  - `answer` with `atoms` (what must be derived);
  - `gap` with `atoms` (for a request, `gap(Step, Item)` atoms; for a query, the underivable goal), `missing` (the withheld pieces) and `completes` (what becomes derivable once they are added);
  - `hyp` with `atoms` and `min_support` (IND: no base-rule proof, but a pattern over at least 3 taught examples from at least 2 kinds supports the rule).
- `pieces`: every fact the proof rests on, as teaching statements. For requests, include the procedure facts needed (`task`, `step`, `uses`, `role`, `demands`, `before`, `requires_state`, `achieves`, `action`, `object_of`) using real step ids from the 3.0 ground truth, or invent a short procedure for gardening (X) questions and put it in `given`.
- `deny`: the gold atoms and every intermediate atom of the proof (not the pieces). The curriculum will never teach these.
- `kb_scope` follows the **scope rule** (checked by `check_bench.py`):
  - The scope symbols are:
    - the symbols of the query goal;
    - for a request, the items in `lacks`, `has` and `state_of`;
    - the symbols of the gold atoms.
  - These do not count as scope symbols:
    - task and step ids listed in `procedure_ids.txt`;
    - symbols introduced only by the question's own `given` facts (an X question's gardening procedure);
    - the common materials and properties listed below.
  - `S`: every scope symbol is in `registry_S.jsonl`. `M`: every scope symbol is in `registry.jsonl`. `L`: anything else.
  - **Pieces may use other symbols**, such as kinds the registry lacks (`cookware`, `hand_tool`). The pieces are one valid proof, and the agent may derive the gold another way from what it was taught (design 10.1: any other valid proof is accepted). Prefer registry symbols wherever they fit.
- Symbols: a registry symbol keeps its registry sort; a second sense needs a distinct symbol (`glass` is an item, the material is `glass_material`). Common materials and properties (`metal`, `plastic`, `glass_material`, `paper_material`, `wood`, `ceramic`, `sharp`, `heat_resistant`, `absorbent`, `fragile`) may be used freely. Any other symbol not in the registry must be listed in a top-level field `"new": ["sym:sort", ...]`, and must have one sort across the whole benchmark.

## Types (design 10.1)

| Code | What it tests | Gold |
|---|---|---|
| K | Kind reasoning: `is_a` chains, inheritance, `has_part` chains | answer |
| SUB | B7 stand-in with `demands`: `request(task). lacks(user, I). has(user, J).`; include a look-alike that fails a demand | answer: `stand_in(Step, I, J)` |
| X | Cross-domain: a gardening procedure in `given`, or pieces from other domains (gardening is never taught) | as K or SUB |
| EXC | Exception: naive inheritance gives the wrong answer; a more specific fact overrides it | answer (often a `not ...` atom) |
| STEP | B8: a step whose tool is lacked is replaced by an action with the same effect | answer: `replace_step(Step, Action)` |
| NAME | Needs `same_as` (a piece says two names are one thing) plus at least one real derivation | answer |
| IND | No base-rule proof; at least 3 taught examples from at least 2 kinds share a pattern | hyp |
| GAP | A needed piece is deliberately missing; a quarter are near-misses sharing entities with answerable questions | gap |

**Hops** count rule applications: each `is_a` step of an inheritance counts 1; B5, the role bridge, B7, B8 and each learned rule count 1; `same_as` renaming counts 0.

## Your share

Each writer writes **225 questions** in its domain group:

| Writer | Domains | Output |
|---|---|---|
| w1 | cooking (including baking) | `data/bench4/pool/w1.jsonl` |
| w2 | cleaning | `data/bench4/pool/w2.jsonl` |
| w3 | repair | `data/bench4/pool/w3.jsonl` |
| w4 | planning, errands, travel, crafts | `data/bench4/pool/w4.jsonl` |

Composition per writer:

| Type | Count | 1-hop | 2-hop | 3-hop |
|---|---|---|---|---|
| K | 49 | 15 | 19 | 15 |
| SUB | 49 | 9 | 21 | 19 |
| X | 34 | 4 | 17 | 13 |
| EXC | 22 | 7 | 10 | 5 |
| STEP | 15 | 4 | 5 | 6 |
| NAME | 7 | 2 | 4 | 1 |
| IND | 19 | | | |
| GAP | 30 (8 near-miss, `"near_miss": true`) | | | |

`kb_scope` per writer: w1, w2 and w3 each write **120 S, 15 M (a registry symbol from the M domains or from another S domain's group), 90 L**; w4 writes **225 M**. Overall that is 360 S, 270 M and 270 L (40/30/30, design 10.1). Spread each type over the scopes in proportion.

## Rules

- Every gold must be true of ordinary household things. Write nothing you are unsure of.
- No question may repeat another up to renaming, and no piece should appear in more than 5% of your questions.
- No denied atom of one question may be a piece of another.
- Validate with `python3 -I tools/design4/check_bench.py data/bench4/pool/<you>.jsonl` until it reports 0 errors.
- No Haiku or Claude CLI calls. Do not commit.
