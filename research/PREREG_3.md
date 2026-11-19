# Design 3.0 pre-registration (frozen after H3 reading 1)

Frozen on 2026-11-19, after the H3 baseline reading (`data/training/runs3/h3_r1/summary.json`, counts only), before any change made in response to it.

## Targets

The design allowed the targets of Design 3.0 section 6.4 to be lowered once, after the baseline. **They are not lowered.** The baseline is far below most of them, and lowering them now would fit the targets to the result. Every target is reported in the final PDF as met or not met.

| Metric | Target | H3 baseline (reading 1) |
|---|---|---|
| M1 tier S (new situation, first encounter) | ≥ 40% and ≥ B1 + 20 points | 5/60 (8.3%) |
| M1 tier T (new task, second encounter after one teaching) | ≥ 60% | 15/60 (25.0%) |
| M1 tier G (new domain, second encounter) | ≥ 40% | 7/34 (20.6%) |
| M3 stored-fact strict pass, tier T | ≥ 90% | 17/60 (28.3%); core recall 0.848 |
| M4 parser coverage at ≥ 95% precision (dev, fair calibration) | ≥ 50% | 10.9% |
| Wrong-claim rate (answers with a false claim) | ≤ 5% | 77 of 432 encounters (17.8%) |
| M8 honest gaps | ≥ 80% | measured at the final reading |
| Word lists in agent crates | 0 | 0 (test passes) |
| C2 against C1 | paired test, no target | — |

## Protocol for the remaining readings

- **Reading 2 (checkpoint):** C2 (dialogue plus dataset) on the same build as reading 3.
- **Reading 3 (final):**
  - C1 and C2, each with the function arm and the rules arm;
  - B1 (adaptation off), B2 (Haiku with the teaching) and B3 (v2.0.0 replay, tier S, because of the call budget);
  - ablations: no fallback, rules arm.
- **Statistics:** Wilson 95% intervals. Paired arms are compared with an exact McNemar test on the same encounters.
- **Variant teaching:** H3 readings run without variant teaching (call budget). Repeats after teaching are measured on train and dev.
- **Freeze rule:** no H3 transcript is read before reading 3. Changes between readings may use train and dev only.
