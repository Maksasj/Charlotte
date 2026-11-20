# Milestone 36, first S run: findings before gate G2

Numbers below come from `data/training/teach4/S_main.json`, `data/training/curve4/*.summary.json`, `tools/design4/leak_check.py` and the counts in this note's commit message. They are interim; the G2 report will be generated from data.

## The run

- Curriculum S ran all four quarter blocks: 150 units, of which 146 were live Haiku calls.
  - 6,180 facts were accepted and 2,051 symbols declared.
  - Schema pass per stage was 95.5-99.8%.
  - Accepted facts per call were 16.8-44.4 for stages 1-6, and 5.2 for stage 7 (case facts are not counted as accepted).
- Delivered leaks: 0 in 8,809 statements, by the independent post-hoc check.
- Prompt fixes during the run (stopped runs are kept as data):
  - v6: steps are named `<task>_s<n>`;
  - v7: literal `topic` lines for cases;
  - `user` is now a protocol constant;
  - P1 filters the agent's own derived lines out of the history.
- The calls4 ledger stands at 251.

## Dev curve

| Point | Correct | Coverage | Inventions |
|---|---|---|---|
| S25 | 2 / 300 | 1% | 0 |
| S50 | 2 / 300 | 1% | 0 |
| S75 | 1 / 300 | 1% | 0 |

- Learned rules on and off give the same picture.
- Every claim's proof checks.

## Why coverage is near zero

Of the 1,286 pieces the 235 answerable dev questions need:

| Status of the piece | Count | Share |
|---|---|---|
| Taught | 128 | 10% |
| Its symbols are known, but the fact was not taught | 312 | 24% |
| It uses a symbol outside both the registry and the knowledge base (an entity the writers invented) | 830 | 65% |

No question has all its pieces taught.

## The cause: a milestone 35 error

Design 10.1 defines `kb_scope` L as a question that needs entities beyond the registry, and sets targets of about 40% S, 30% M and 30% L. The benchmark spec I wrote let writers add new entities freely and label scope by domain, and no check tested scope against the registry.

Recomputed by the definition, from registry membership of every piece symbol:

| Split | S | M | L |
|---|---|---|---|
| Dev | 3 | 2 | 295 |
| Sealed | 13 | 1 | 586 |

The G0 report's scope line (51/25/24) was wrong, and the benchmark mostly measures entities the curriculum is never asked to teach. The sealed part has not been read, so no sealed reading has been spent.

## Also found

With learned rules on, replaying the full S run for scoring did not finish in 27 minutes of CPU, while S75 had a p95 of 303 ms. The agent holds 817 learned-rule records, and retry or induction at full size needs a look against the p95 ≤ 2 s target.
