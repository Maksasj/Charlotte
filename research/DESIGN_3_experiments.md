# Design 3.0 phase A experiments

Phase A, milestone 20. The plan for each experiment is in `DESIGN_3_alternatives.md` section 5.2, and its decision rule in section 5.3.
- **Scripts:** `tools/design3/`. Run them with `python3 -I`. All of them call no model except X7.
- **Splits:** train is the 55 training scenarios and dev is the 20 development scenarios. Gardening is held out of both (D15) and reported as its own split.
- **Results:** the "v2" columns use the old ground truth. "v3" columns are added when the re-authored ground truth is complete.

## X1. Learned teach-line parser

`tools/design3/x1_parser.py`

**Method.**
- The tagger is an averaged perceptron with roles ACT, OBJ, USE, UNTIL, TMIN, TMAX, UNIT, OVL and NEED.
- Training labels come from aligning the ground-truth facts to the words of each teach line, after the colon of "Step N of …:".
- Symbol forms such as plural to singular are rewrite rules learned from the aligned training pairs.
- The parser has one feature that comes from knowledge rather than code: whether a word belongs to an item the same task's needs line was parsed to. The needs line is parsed by the same learned model first.

There are no word lists.

**Results, v2 ground truth (dev, 97 step lines; 19 needs lines).**

| Variant | Exact step lines | Action | Object | Tools | Until | Time | Overlaps | Needs lines exact |
|---|---|---|---|---|---|---|---|---|
| Learned parser | **71.1%** | 100% | 83.5% | 83.5% | 95.9% | 100% | 100% | 11/19 |
| Without the needs-line feature | 53.6% | 96.9% | 79.4% | 70.1% | 94.8% | 100% | 100% | 10/19 |
| Gardening (held-out domain, 18 lines) | 55.6% | 94.4% | 77.8% | 72.2% | 88.9% | 100% | 100% | 1/4 |

- **Fit to the training lines is only 81%.** Much of the error is ground-truth noise, not parser error:
  - the ground truth names `pan` where the line says "frying pan";
  - `boil_egg_s4` uses `bowl` but not the "cold water" in the same line;
  - 36 training lines hold a fact that matches no word in the line.
- **Reference point.** Haiku as 2.0's translator stored about 70% of step lines correctly (REPORT_DESIGN_2 §11.10, "30% stored wrong"). With zero calls, the learned parser is already level with it on dev.
- **Decision rule (5.3.1).** The rule needs at least 95% to adopt with no fallback, 85-95% to adopt with a Haiku fallback, and below 85% means returning to a frame grammar after X7. **v2 result: 71%, below 85%.** The rule is applied on v3 ground truth after X7, which is the order the rule itself sets.

## X2. Request detection and task identity

`tools/design3/x2_requests.py`

**Method.**
- Identity picks one task among all 105 taught tasks.
- The baseline is IDF-weighted overlap between the request's words and the task's own knowledge words: task symbol, needs, actions, objects, and the task phrase from the teach lines. Words are cut to a shared 4-letter stem, the best of 4, 5 and 6 letters on dev.
- The learned version adds request-word-to-knowledge-word associations (positive PMI) learned from the training requests.
- Detection is a perceptron over words and bigrams that separates requests from teach and situation lines.

**Results (v2).**

| Split | Requests | Baseline top-1 | Learned top-1 | Variant requests: baseline / learned |
|---|---|---|---|---|
| dev | 57 | 52 (91.2%) | 53 (93.0%) | 14/14, 14/14 |
| gardening | 12 | 7 | 8 | 3/4, 3/4 |
| train (fit) | 156 | 135 | 149 | 30/34, 32/34 |

Detection: request recall is 87/87 (100%). There are 2 false alarms in 225 statements, and both are situation lines that mention a wish ("I want to have it at home…").

**Misses are mostly task names never seen:**
- "hard-boiled egg" matched against `boil_egg`, where the stem "boile" against "boil" was fixed by the 4-letter cut;
- "framed print" against `hang_picture`;
- "hot cup of tea" against `make_hot_chocolate`.

In the gardening domain, the tasks share words (plant, pot, water), so identity is ambiguous inside it.

**Decision rule (5.3.2).** The rule needs request recall of at least 95% and learned top-1 at least 10 points above the baseline. **Detection meets it, but identity does not:** the learned version is +1.8 points, because associations learned on training tasks do not transfer to new tasks. **Outcome:** request detection is learned. Task identity uses the overlap score over the agent's own knowledge, plus the task names it learns (`task_alias`). When the top two scores are close, the agent confirms with the user. Haiku is the fallback for requests whose identity score is low.

## X6. Content-check feasibility on 2.0 replies

`tools/design3/x6_content.py`

**Method.** It takes the 2,503 help replies with numbered steps in the 2.0 curriculum records. Each numbered line is framed as "Step N of task:" and read into facts with the X1 parser. Steps are aligned to the ground truth by (action, object) in order.

**Results (v2).**

| Measure | Value |
|---|---|
| Mean needs recall | 0.91 |
| Mean step recall | 0.78 |
| Strict pass (all needs, all steps, in order) | 837 (33%) |
| Loose pass (80% of needs and steps, in order) | 1,431 (57%) |

Agreement with 2.0's own symbolic check:

| | 2.0 symbolic pass | 2.0 symbolic fail |
|---|---|---|
| Strict pass | 807 | 30 |
| Strict fail | 1,229 | 437 |

Agreement with the Haiku judge:

| | Judge pass | Judge fail |
|---|---|---|
| Strict pass | 470 | 367 |
| Strict fail | 949 | 717 |

**Reading.**
- **Reading English back into facts is lossy.** Parser errors and name mismatches make the strict check fail many replies that 2.0's looser check passed.
- The strict check and the Haiku judge disagree on 1,316 of 2,503 replies (53%). They measure different things.
- **This supports D5.** 3.0's answers are relation lines, the facts themselves, so the check compares facts with facts and no reading step can add error.
- The 2.0 results cannot be re-scored exactly. Any 2.0 number in the final PDF is labelled "read back by the X1 parser", or the v2.0.0 replay baseline (D13) is used instead.
- **Outcome for the check rules:** align steps by (action, object) with ground-truth aliases; strict pass on core facts; graded recall and precision reported beside it (schema section 6).

## X8. Algorithmic split

`tools/design3/x8_split.py`

**Method.**
- Atoms are actions and items. Compounds are (action, object) and (action, tool) pairs.
- Divergence is the Chernoff coefficient, with α = 0.5 for atoms and 0.1 for compounds (Keysers et al. 2020).
- A greedy search over 4,000 swaps looks for a 19-task test split with the highest compound divergence, while keeping atom divergence low.
- Three splits are compared: the hand dev split, a random split and the maximum-compound-divergence split.
- Difficulty is the X1 parser's exact rate, trained on the remaining tasks without the needs-line feature.

**Results (v2, 71 curriculum tasks, gardening excluded).**

| Split | Atom divergence | Compound divergence | Parser exact |
|---|---|---|---|
| Hand dev (19 tasks) | 0.576 | 0.913 | 55.7% (97 lines) |
| Random (19) | 0.588 | 0.909 | 37.8% (90) |
| Max compound divergence (19) | 0.447 | 0.906 | 39.8% (93) |

**Reading.**
- **Every split already has compound divergence of about 0.91.** With about 70 tasks, almost every (action, object) pair occurs in one task only, so maximum-compound-divergence search has nothing to push. The data is too sparse for MCD to separate "new combinations" from "new everything".
- **The hand dev split is easier than random splits** (56% against 38%). The 20 development scenarios were the first ones written, and they are more typical than the 55 training scenarios added later. Numbers on the hand dev split alone overstate generality.

**Outcome for D9 (both kinds of held-out).**
- The algorithmic tier is **repeated random task splits**: 5 seeds, reported as a mean and range. This replaces a single MCD split.
- Atom-held-out tests add the cases MCD cannot make here: one item or one action never seen in training.
- MCD is re-run on the larger generated dataset, where compounds repeat.
