# Design 4.0 decision log

These decisions come from the 4.0 pre-planning (P1-P16, in `research/DESIGN_4_preplanning.md`) and from the design session. The drafts behind them are `DESIGN_4_draft.md`, `DESIGN_4_logic.md` and `DESIGN_4_eval_data.md`.

| # | Date | Decision | Choice | Notes |
|---|---|---|---|---|
| D4.1 | 2026-11-19 | Haiku budget | A fresh 5,000 calls for 4.0, in its own ledger `data/training/calls4/`. It is a hard limit, raised only when the user says so | 3.0's 4,486 calls do not count |
| D4.2 | 2026-11-19 | Reusing 3.0 readings | Cached formal readings of 3.0 procedures and situations are replayed to the agent as teacher messages through the conversation path, at 0 calls, where they fit the 4.0 schema. They are never injected. Readings tied to H3 are excluded | "If it fits": readings that do not map onto the 4.0 schema are re-taught or dropped |
| D4.3 | 2026-11-19 | Spike facts | Written by a sub-agent as a test fixture that the user reviews; used only in the spike, never in the phase B knowledge base | An exception to P2, for the spike only |
