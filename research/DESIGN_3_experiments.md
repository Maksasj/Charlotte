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


## Results on the v3 ground truth (all 105 tasks re-authored)

### X1 (v3), learned parser

v3 is stricter than v2:
- implied tools now count (the oven in "bake the potato");
- `into` and `uses` are separate;
- 45 more compound steps were split.

| Split | Exact step lines | Object | Tools | Coverage at ≥ 95% precision |
|---|---|---|---|---|
| dev (101 lines) | **66.3%** | 84.2% | 84.2% | **19.8%** |
| dev, without the needs-line feature | 57.4% | 79.2% | 75.2% | |
| gardening (20 lines) | 35.0% | 70.0% | 60.0% | 0% |

- **Ceiling.** 95 of 101 dev lines have every fact in their own words. The other 6 hold an implied fact, mostly `into`.
- **Coverage at 95%.** This is the share of lines the parser keeps when it trusts only its most confident lines (minimum token margin), at the largest share whose accuracy is still at least 95%. Confidence is weakly calibrated. The most confident 25% are 88% exact, and the most confident 50% are 82%.
- **One feature change was tried and reverted:** a "governor word" feature gave 62.4%. No further tuning was done on dev, to avoid fitting it.

### X7, supervision pilot (21 live Haiku calls, all cached in `data/training/calls3/`)

Haiku rewrote the step facts of 20 training tasks as 3 differently worded instructions each, giving 414 pairs labelled by construction.

| Parser configuration | Before | After +414 pairs |
|---|---|---|
| Weak (no needs-line feature, unaligned lines dropped) | 50.5% | 57.4% (+6.9, 1.7 points per 100 pairs) |
| Full X1 configuration | 66.3% | 66.3% (+0) |

Generated paraphrases help a weak parser but not the full one. Gains per 100 pairs are small and shrink, so more generated pairs of this kind will not reach 85%.

### X2 (v3), requests

| Split | Baseline top-1 | Learned top-1 |
|---|---|---|
| dev, base requests | **56/57 (98.2%)** | 54/57 |
| dev, variant requests | 13/14 | 12/14 |
| gardening, base | 10/12 | 10/12 |
| gardening, variant | 4/4 | 4/4 |

- With the v3 task aliases in the agent's knowledge, word overlap reaches 98%, and learned associations make it worse.
- **Detection:** request recall is 86/87 (98.9%), with 0 false alarms on 231 statements.

### X5, situation noticing tied to knowledge

`tools/design3/x5_notice.py`. The perceptron's features are only the places where the request links to a known item (2-word context, whether the item belongs to the identified task) or a number. It is trained on the training requests: 156 base and 34 variant.

| Split | False alarms on base requests | Variants noticed |
|---|---|---|
| dev | 2/57 (3.5%) | 6/14 (43%) |
| gardening | 1/12 (8.3%) | 3/4 |

[M4], which flagged any unusual word, raised false alarms on 169-204 of 225 base requests. Tying signals to knowledge cuts false alarms to 3.5%, but it notices only 43% of variants.

### X4, adaptation in fact space (leave one task out, 48 train and dev cases; 4 gardening)

| Method | Change recall | Cases passed | Cases with a wrong change |
|---|---|---|---|
| null | 0.0% | 3/48 | 0 |
| affordance (substitution by learned role) | 19.2% | 5/48 | 9 |
| cases (A4, most similar case mapped by role) | 19.2% | 4/48 | 20 |
| rules (A3-lite, patterns with support ≥ 2) | 1.4% | 3/48 | 0 |
| gardening: affordance / cases / rules | 25.9 / 25.9 / 14.8% | 1/4 each | 1 / 2 / 0 |

- **Most variants need knowledge that is in no other task.** Examples: microwaving a potato first, or what a newspaper can do to a window. Situations rarely repeat (DESIGN_3_alternatives [M2]), so induced rules almost never fire.
- Case transfer finds as much as substitution, but makes twice as many wrong changes.

## Decision outcomes (rules of DESIGN_3_alternatives section 5.3)

| Rule | Outcome | Decision |
|---|---|---|
| **5.3.1 Teaching parser:** ≥ 95% → learned only; 85-95% → learned with fallback; < 85% after X7 → frame grammar | **66.3% after X7: below 85%** | **Deviation, recorded:** user decision D3 (a learned parser, with Haiku as its teacher and fallback) and the no-word-list rule rule out a hand frame grammar. Phase B builds the learned parser with Haiku fallback below a confidence threshold. Every fallback answer is stored as a training pair, so the parser is retrained from it (distillation). The measure of language learning becomes **coverage at ≥ 95% precision as teaching accumulates**, now 19.8%. The final PDF states plainly that language-to-meaning was not learned to 85% in phase A |
| **5.3.2 User channel:** recall ≥ 95% and learned top-1 ≥ baseline + 10 | Detection 98.9%. Learned identity −3.5 points | Request detection is learned. Task identity is overlap over the agent's own knowledge and learned task aliases (98% on dev). If the top two scores are close, the agent confirms with the user. Haiku is the fallback only for unparsed requests |
| **5.3.3 Adaptation:** A4 if it is ≥ 15 points over null **and** wrong-claim rate ≤ null's; rules if they add ≥ 5 | Cases +19.2 points but 20 cases with wrong changes, against 0 for null. Rules +1.4 | **Neither arm passes the wrong-claim condition.** Both stay as arms (D6), but **gated**: a proposed change is offered only when its support is enough (a role seen in at least one other task, or a case with similarity ≥ 0.5), otherwise the agent states the gap and asks the teacher. The transfer claim becomes "substitution by learned function, plus honest gaps, plus learning the variant from one teaching". Rules are kept as the arm that grows with repeated situations, and are measured on the generated dataset, where situations repeat |
| **5.3.4 Noticing:** ≤ 10% false alarms → keep the blindness target | 3.5% on dev | Keep. Noticing is tied to knowledge; a missed situation costs a clarifying question, not a wrong answer |
| **5.3.5 Output** | No experiment needed | Relation lines (D5) |
| **X8 split** | Every split already has compound divergence of about 0.91, and the hand dev split is easier than random | Algorithmic tier = repeated random task splits (5 seeds) plus atom-held-out tests |
