# Design 3.0 decision log

Phase A, milestone 19. Each row is a decision the user made. The inputs are `DESIGN_3_options.md`, `DESIGN_3_draft.md`, `DESIGN_3_draft_fable.md` and `DESIGN_3_alternatives.md`.

| # | Date | Decision | Choice | Notes |
|---|---|---|---|---|
| D1 | 2026-11-19 | Iteration status | 3.0 is one iteration, not planned as the last | If it goes well it could be the final one; otherwise a 4.0 follows |
| D2 | 2026-11-19 | PDFs | One PDF only: the research PDF at the end of phase B | Phase A ends with the Design 3.0 document in Markdown; no plan PDF |
| D3 | 2026-11-19 | English to facts | Learned parser | Trained on teacher-generated pairs; Haiku is its teacher and a logged fallback when it is unsure (alternatives A1). The hand frame grammar is not the main path |
| D4 | 2026-11-19 | Knowledge source | Dialogue teaching plus a generated dataset, compared | Main experiment: dialogue only against the mix |
| D5 | 2026-11-19 | Answer form | Relation lines | Fixed lines of facts and relations, checked exactly; no English text generator (alternatives A6) |
| D6 | 2026-11-19 | Adaptation core | Both as arms | Case-based analogy (alternatives A4) and induced rules (inductive logic programming, Popper offline first), compared as separate arms |
| D7 | 2026-11-19 | Ground truth | Re-authored to the new schema by a Claude Code sub-agent | Split compound steps, aliases, causes, conditions, warnings; a script checks it and the user spot-checks a sample |
| D8 | 2026-11-19 | Language of tools | Python allowed offline only | Parser training, Popper and analysis in Python; weights and rules exported as data; the runtime agent stays pure Rust |
| D9 | 2026-11-19 | Held-out set | Hand-written tiers plus algorithmic splits, about 150-300 scenarios | Affordable because the learned parser makes evaluation nearly free |
| D10 | 2026-11-19 | Branch | New branch `design-3` from `design-2` | The user pushes; no remote work by Claude |
| D11 | 2026-11-19 | Call budget | Start with 5,000 Haiku calls; expand later if needed | Keep calls practical. Every Haiku call is archived and reusable many times for later learning or testing (cache by input, never discard), so no call is spent once |
| D12 | 2026-11-19 | Autonomy | Work through 3.0 without stopping | Within the call budget of D11 |
| D13 | 2026-11-19 | Baselines | All three: word-overlap retrieval (0 calls), Haiku with the teaching in its prompt, v2.0.0 replay on the new test set | About 600-1,400 calls in total, all cached and reused |
| D14 | 2026-11-19 | Held-out domain | One whole domain never seen in training (words, tools, task shapes); gardening preferred | Tests whether general learning carries over (e.g. "a missing tool can be replaced by one with the same function"). Fallback: bike and home repair if gardening is too small |
| D15 | 2026-11-19 | Held-out domain, concrete | Gardening | Today only 4 tasks are gardening (water_plant, mow_lawn, rake_leaves, repot_plant). All 4 leave training, and the new-domain test tier gets new gardening tasks written for it. No gardening word, tool or task shape may appear in training data (checked by a script) |
| D16 | 2026-11-19 | Parser path after X1/X7 (rule 5.3.1 not met: 66% < 85%) | Learned parser plus Haiku fallback below a confidence threshold; every fallback answer becomes a training pair (distillation); no hand frame grammar | Follows D3 and the no-word-list rule. Measure of language learning: coverage at ≥ 95% precision over teaching (19.8% at the end of phase A) |
| D17 | 2026-11-19 | Request identity (X2) | Overlap over the agent's own knowledge plus learned task aliases; confirm with the user when the top two scores are close; learned request detection | Learned associations did not transfer to new tasks |
| D18 | 2026-11-19 | Adaptation (X4) | Both arms (D6), gated by support; otherwise state the gap and ask the teacher | Neither arm met the wrong-claim condition ungated |
| D19 | 2026-11-19 | Algorithmic held-out tier (X8) | 5 repeated random task splits plus atom-held-out tests | MCD has nothing to push at about 70 tasks |
| D20 | 2026-11-19 | Fallback policy (phase B finding 5) | Keep the learned action and object, and add the teacher's other predicates | Learned 196/217 against fallback 176/217 exact on unsure lines |
| D21 | 2026-11-19 | Distillation (finding 8) | Agreement-filtered distillation adopted; all-pairs distillation rejected | All pairs: 49/91. Agreement: 66/101 (fair). Coverage at 95% fell |
| D22 | 2026-11-19 | Targets after the H3 baseline | Not lowered (`research/PREREG_3.md`) | Lowering after seeing the baseline would fit targets to results |
| D23 | 2026-11-19 | Call budget in the readings | H3 readings without variant teaching; B3 on 40 tier-S cases; split tier on cached calls | Stays within 5,000 (4,486 used) |
| D24 | 2026-11-19 | Outcome | 3.0 done; no frozen target met except "no word lists" | See `paper/main.pdf` |
