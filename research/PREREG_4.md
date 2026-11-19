# Design 4.0 pre-registration (DRAFT)

**Status.** Drafted on 2026-11-19, after the go decision D4.13, from `research/SPIKE_4.md`. It is **frozen at milestone 36**, after the first sealed reading R1's protocol is fixed and before R1 is read (design section 11). Until then the user may change any value. Every value marked "set after the spike" in the design is proposed here, with the spike number it comes from.

## Values set after the spike

| Value (design milestone) | Proposed | From the spike |
|---|---|---|
| G2: 2-hop correct at S100 on dev (36) | ≥ 60% | Fixture 30/36 = 83% (CI 68-92%). A Haiku-taught knowledge base is noisier than the hand-built fixture, so the bar stays at the spike gate's 60% |
| G2: S50 → S100 paired gain (36) | ≥ +5 points | Fixture prefixes: 47 → 76 correct (50% → 100%) |
| G2: inventions / proof validity (36) | 0 / 100% | Fixture: 0 / 62 of 62 |
| Hypothesis precision (37) | ≥ 0.70 | Fixture IND: 8 of 10 after the fixes |
| K2 over K1 (learned rules on against off) (37) | ≥ +1 correct, and wrong answers not higher | Fixture: +1 correct, +1 wrong (q088) |
| G3: coverage rises S → M (38) | paired gain ≥ +3 points on dev, CI not including 0 | Curve on the fixture: monotone, 31 → 47 → 56 → 76 |
| p95 answer time at M (38) | ≤ 2 s per question | x50 (34,598 facts): about 1.5 s per question with full recomputation |
| Learned-rule body length | ≤ 2 literals | Kept rules all have bodies of 1-2 literals |
| Hop limit | 4 | Deepest fixture gold: 3 hops |
| Support threshold k | 3 (D4.7) | k = 2: +2 correct, +1 wrong; k = 4: nothing learned that matters |
| Prediction budget | 5 times support before confirmation (design 7.1) | Held back the 2 spurious material rules |

## Scoring decision left open by D4.13 (to be confirmed by the user before the freeze)

**Proposed:** a claim marked `hypothesis`, with its rule and support, on a question whose gold is a gap is scored in its own class, **hypothesis-on-gap**. It counts against hypothesis precision, not as a wrong answer. A wrong answer stays a claim contrary to the gold, or a deduced claim where the gold is a gap. The spike's q088 is this case.

## Primary contrasts (R3, Holm-corrected)

1. S100 against S25: does the agent get better as it is taught more?
2. Learned rules on against off (K2 against K1) at the largest size.
3. The agent against Haiku open-book (K3) on 2-hop questions, with inventions reported for both.

## Readings

At most 3 sealed readings: R1 after milestone 36, R2 after 38, R3 final. Each is logged with its commit; only counts are shown until R3.
