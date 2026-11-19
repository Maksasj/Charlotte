# H3 reading ledger

H3 is read 3 times (Design 3.0 section 6.1): a baseline after milestone 24, a checkpoint after milestone 26, and the final reading in milestone 28. Each reading is recorded here with its date, commit, build and what was run. Only counts are reported until after the final reading.

| # | Date | Commit | What |
|---|---|---|---|
| 1 | 2026-11-19 | c5fc5be | Baseline after milestone 24: C1-trained vault (c1_both_r8), arm function, fallback on, no variant teaching (budget); tiers S, T, G |
| 2 | 2026-11-19 | dcf1d8e | Checkpoint after milestone 26: C2 (dialogue + 98-task dataset), vault c2_both_r1, arm function, fallback on, no variant teaching |
| 3 | 2026-11-19 | 12fbc05 | Final (milestone 28), one frozen build: C1 function arm, C1 rules arm, B1 (adaptation off), ablation without fallback; C2 from reading 2; B2 and B3 baselines. Vault c1_both_r8, no variant teaching |
