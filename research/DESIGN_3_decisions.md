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
