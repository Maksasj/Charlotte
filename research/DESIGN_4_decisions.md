# Design 4.0 decision log

These decisions come from the 4.0 pre-planning (P1-P16, in `research/DESIGN_4_preplanning.md`) and from the design session. The drafts behind them are `DESIGN_4_draft.md`, `DESIGN_4_logic.md` and `DESIGN_4_eval_data.md`.

| # | Date | Decision | Choice | Notes |
|---|---|---|---|---|
| D4.1 | 2026-11-19 | Haiku budget | A fresh 5,000 calls for 4.0, in its own ledger `data/training/calls4/`. It is a hard limit, raised only when the user says so | 3.0's 4,486 calls do not count |
| D4.2 | 2026-11-19 | Reusing 3.0 readings | Cached formal readings of 3.0 procedures and situations are replayed to the agent as teacher messages through the conversation path, at 0 calls, where they fit the 4.0 schema. They are never injected. Readings tied to H3 are excluded | "If it fits": readings that do not map onto the 4.0 schema are re-taught or dropped |
| D4.3 | 2026-11-19 | Spike facts | Written by a sub-agent as a test fixture that the user reviews; used only in the spike, never in the phase B knowledge base | An exception to P2, for the spike only |
| D4.4 | 2026-11-19 | Relations | 11 typed relations: `is_a`, `same_as`, `has_part`, `made_of`, `has_property`, `can_do`, `can_do_on`, `used_for`, `can_go_in`, `effect`, `precondition`. All can be taught negated. Plus `demands(Step, Function, Rel, Value)`, linking a step's role to conditions | DESIGN_4_logic.md section 1 |
| D4.5 | 2026-11-19 | Exceptions | Most specific wins: taught about the item itself, then the nearer kind, then the farther kind, then a learned rule. Incomparable conflicts become a question to the teacher. Learned rules never override teaching | Stratified Datalog on the existing reasoner; DESIGN_4_logic.md section 2 |
| D4.6 | 2026-11-19 | Base rules | B1-B9: logic (is_a transitivity, inheritance with specificity, contradiction becomes a question, same_as by canonicalisation, part transitivity) and the procedure bridges (role and used_for imply can_do; stand-in with a demands check; step replacement by effect; gaps). Each is kept only if switching it off costs answers and it harms none. Material properties are left to induction | DESIGN_4_logic.md section 3 |
| D4.7 | 2026-11-19 | Rule learning | Support of at least 3 examples from 2 or more tasks or kinds, and no unhandled counterexample. One confirming question to the teacher per new rule. A counterexample specialises the rule (an added condition) or retracts it. Rules are recorded in layers, with `built_from` | DESIGN_4_logic.md section 4 |
| D4.8 | 2026-11-19 | Leakage | A denied teacher message is withheld before the agent sees it. Any delivered leak invalidates the run | Deny list covers answers, proof steps and withheld pieces |
| D4.9 | 2026-11-19 | Benchmark size | 900 questions: 300 dev, 600 sealed; at most 3 sealed readings. The spike has its own 100 questions | DESIGN_4_eval_data.md power analysis |
| D4.10 | 2026-11-19 | Knowledge-base sizes | S about 1,500 facts, M about 4,000; L about 10,000 only if coverage rises from S to M (the scaling gate). Gardening is never taught | |
| D4.11 | 2026-11-19 | Spike gate | Phase B goes ahead if the spike reaches at least 60% correct on 2-hop questions, 0 inventions, 100% valid proofs, and learned rules add correct answers without adding wrong ones. The user decides go or no-go on the spike report | |
| D4.12 | 2026-11-19 | Spike fixture review (D4.3) | Approved by the user, unchanged | The sealed `data/spike4/MANIFEST.sha256` is the reviewed fixture |
| D4.13 | 2026-11-19 | Spike go/no-go (G1) | **Go to phase B.** The rule-learning fix loop ran first. The remaining wrong answer (q088) is treated as a fixture IND/GAP tension; how a marked hypothesis on a gap gold is scored is decided in phase B (PREREG_4) | SPIKE_4.md sections 2 and 10 |
| D4.14 | 2026-11-19 | PREREG_4 values | All proposed values confirmed, including hypothesis-on-gap scoring | Frozen at milestone 36 |
