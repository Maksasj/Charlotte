# Symbolic Learning Agent – Design 3.0

**Status.** Final design. Phase A (milestones 19-21) is complete. This document governs phase B (milestones 22-29). The decisions D1-D19 in `research/DESIGN_3_decisions.md` are binding. The phase A evidence is in `research/DESIGN_3_experiments.md`. The knowledge schema is `research/DESIGN_3_schema.md`. Background documents: the drafts (`DESIGN_3_draft.md`, `DESIGN_3_draft_fable.md`), the options summary (`DESIGN_3_options.md`) and the alternatives study (`DESIGN_3_alternatives.md`). Where any of them differs from this document, this document wins.

3.0 is the third research iteration. It is not planned as the last one (D1).

## 1. Summary

Design 2.0 ended with these results:
- transfer passed 6 of 15 on the development set;
- second encounters passed 17-18 of 20;
- only the first held-out run was clean;
- about 30% of taught step lines were stored wrong;
- about 225 English strings were hard-coded in Rust;
- work drifted toward polishing replies for style judges.

3.0 changes what is measured and where the knowledge lives:

1. **Content, not polish.** An answer is a set of relation lines, one fact per line, in schema v3 (D5). There is no English realiser and no style judge. The measure of record is an exact fact-and-relation check against a re-authored ground truth (D7). It covers order, cause, condition, item-to-step, substitution and settings.
2. **Learned language, with a teacher behind it.** English turns into facts through a learned parser that is trained offline and run in Rust. When the parser is unsure, Haiku answers instead, and every such answer becomes a new training pair (D3, D16). At the end of phase A the parser keeps 19.8% of dev step lines at 95% precision, and 66% of all lines are exact. How fast that coverage grows is the main learning curve.
3. **No English in agent code.** Agent crates hold no word lists. Words live in learned data (aliases, parser weights, symbol-form rules). A test enforces this.
4. **Adaptation is learned and gated (D6, D18).** Two arms are built and compared:
   - substitution by learned function, combined with case transfer;
   - rules induced from cases.
   
   A change is proposed only with enough support. Otherwise the agent states the gap and asks the teacher, and the answer is stored as a new case.
5. **Two knowledge sources, compared (D4).** C1 is dialogue teaching only. C2 adds a generated example dataset.
6. **A measurement that can be trusted.** H3 has three authored tiers: new situation, new task and new domain, with gardening held out entirely (D15). It also has an algorithmic tier of repeated random task splits (D19). All three baselines are run (D13). Results are reported with intervals, met or not.

## 2. What phase A established

The details are in `DESIGN_3_experiments.md`.

| Question | Finding | Consequence |
|---|---|---|
| Can a learned parser read teach lines? | 66.3% of v3 dev lines exact, with no word lists and no calls. 71% on v2, about level with 2.0's Haiku (about 70%). Coverage at ≥ 95% precision is 19.8%. 35% exact on gardening | Learned parser + Haiku fallback + distillation (D16). Language learning is reported as a coverage curve |
| Does generated supervision help the parser? | 414 Haiku paraphrase pairs (20 calls): +6.9 points for a weak parser, +0 for the full one | Generated paraphrases are not the main source of parser data. Fallback answers on real lines are (distillation) |
| Can task identity work without word lists? | Overlap over the agent's own knowledge plus task aliases: 56/57 dev requests (98%). Learned associations don't transfer to new tasks | D17 |
| Can situations be noticed without cue lists? | Features tied to knowledge: 3.5% false alarms, 43% of variants noticed (dev) | Keep. A missed situation leads to a question or a teacher correction, never a wrong answer |
| Can the content check run with zero calls? | On facts, yes, exactly. Reading English replies back is lossy: the strict check passed 33% of 2.0 replies, and it disagrees with the Haiku judge on 53% of them | Answers are facts (D5). 2.0 is compared through the v2.0.0 replay, not by re-reading its replies |
| Which adaptation core? | Substitution and case transfer each find 19% of changes, with wrong changes in 9 and 20 of 48 cases. Rules find 1.4%, because situations rarely repeat | Both arms, gated (D18). Rules are measured where situations repeat (the dataset) |
| Algorithmic split? | Compound divergence is already about 0.91 for any split of about 70 tasks. The hand dev split is easier than random splits (56% against 38% parser exact) | 5 random task splits plus atom-held-out tests (D19) |
| Ground truth | 105 tasks re-authored (562 steps). 47 compound teach lines split, adding 49 steps. New facts: 419 role, 120 into/onto/from, 40 purpose, 28 setting, 18 requires_state, 16 check, 12 achieves, 4 condition, 4 avoid. 52 variants turned into situation and difference facts | `data/curriculum3/`, checked by `tools/design3/check_gt.py` |

**Stated plainly:** language to meaning did not reach the pre-registered 85% in phase A. Adaptation to new situations is mostly not derivable from other tasks' knowledge. 3.0's claims are shaped by these two facts (section 9).

## 3. Principles

1. **Learned, not coded.** Agent code holds:
   - the schema (predicate names and arities);
   - generic mechanisms: tagging, alignment, overlap, mapping, Datalog evaluation;
   - the formats.

   Words, aliases, roles, symbol forms and parser weights are data, learned or authored as ground truth.
2. **The answer is the facts.** A reply is relation lines. A telegraphic gloss may follow, but it is never scored.
3. **Statistical parts propose, symbolic parts decide.** The parser, the noticing model and similarity scores propose. The validator, the vault and the Datalog rules decide what is stored and what is claimed. Every claim in an answer has a proof: a taught fact, a derived fact with a rule, or a case mapping.
4. **Honest gaps.** When the agent lacks a fact, it writes a `gap` line and asks. It never guesses an item, time or step.
5. **Every model call is reused.** Calls are cached by input in `data/training/calls3/` and never deleted (D11). Re-runs cost nothing.
6. **Sealed measurement.** H3 is read a fixed number of times, as counts only, with the user's go-ahead given once in advance by D12.

## 4. Architecture

```
 teacher / user message
        │
        ▼
 segmenter ──► learned detector: request | statement | answer-to-question
        │
        ├── statement ──► learned parser (weights from tools/design3) ──conf<θ──► Haiku fallback ──► training pair
        │                         │                                                       │
        │                         └──────────────► validator (schema v3, last gate) ◄─────┘
        │                                                   │ facts + provenance
        │                                                   ▼
        │                                     vault: procedures · cases · aliases · provenance
        │
        └── request ──► identity (overlap over vault knowledge + task aliases; confirm if close)
                          ──► situation reader (learned tagger: has / lacks / wants / excludes / time / count)
                          ──► adaptation arm:  A: substitution by role + case transfer (gated)
                                               B: induced rules (gated)
                          ──► answer = relation lines (+ gap lines + question to teacher)
                                                   │
                                   content check vs ground truth (exact, eval only)
```

### 4.1 Crates

The 2.0 crates stay unchanged, so the v2.0.0 replay baseline keeps working. 3.0 adds:

| Crate | Content |
|---|---|
| `kb3` | Schema v3 (predicates, arities, roles of arguments), fact store, provenance side table (source, session, turn, parser, confidence, uses, confirmed, retracted_by), procedure and case views, relation-line format (print and parse) |
| `parse3` | Runtime of the learned models: averaged-perceptron taggers and classifiers loaded from JSON weights, the symbol-form rules, confidence (minimum token margin). Pure Rust, no training code |
| `teach3` | Haiku fallback via the Claude CLI with the shared cache (`data/training/calls3/cache.jsonl`, same key as `tools/design3/haiku.py`). It writes fallback answers as training pairs to `data/training/parser3/pairs.jsonl` |
| `agent3` | The dialogue loop: ingestion with the validator, identity, situation reading, the two adaptation arms, answers, gaps and questions. Adaptation rules run on the existing `sla_reasoner` |
| `eval3` | The content check, curriculum runner, dataset ingestion, baselines, H3 runner with read ledger, statistics (Wilson intervals, McNemar), and export of numbers for the paper |
| `lab3` | The training lab: an append-only event log per run and a Server-Sent Events stream. The existing dashboard app subscribes to it |

Offline Python in `tools/design3/` (D8) trains the models, exports weights, generates the dataset and builds the paper's numbers. Only Rust runs at agent runtime.

### 4.2 The learned models

All are trained on `data/curriculum3/train` plus the accumulated pairs. Development uses `dev`. H3 is never used for training.

| Model | Input → output | Training data |
|---|---|---|
| Statement detector | line → request / statement | Requests against teach and situation lines |
| Teach-line tagger | tokens → roles (ACT, OBJ, USE, INTO, UNTIL, TMIN, TMAX, UNIT, OVL, NEED, SET, PURPOSE, COND, WARN) | Teach lines aligned to ground-truth facts, plus fallback pairs |
| Situation tagger | tokens → HAS / LACKS / WANTS / EXCLUDES / TIME / COUNT spans | Situation lines and variant situation blocks |
| Noticing model | linked contexts → changed situation or not | Base against variant requests |
| Symbol forms | token → symbol word | Aligned pairs (suffix rewrites) |
| Alias store | surface phrase → symbol | Aligned spans, from teaching only |

**Confidence and fallback.** A line whose minimum margin is below θ goes to Haiku. θ is set on dev so that kept lines reach ≥ 95% precision. Haiku's answer is validated like any other parse, stored with `parser = fallback`, and appended as a training pair. The offline trainer retrains between curriculum rounds (`tools/design3/train_parser.py`), and the agent reloads the weights.

### 4.3 Answers and the content check

The format is in the schema, section 5. The check (schema, section 6) aligns steps by (action, object) with ground-truth aliases. It then scores:
- **strict pass:** all core facts (needs, step action and object, order, adaptation changes), with no wrong claim;
- graded recall over all facts;
- precision;
- wrong claims.

Negative controls are part of the test suite: a missing step, an extra step, a wrong order, a fact dump, and a swap to a lacked item must each fail.

### 4.4 Adaptation arms (D6, D18)

- **Arm A: function and cases.**
  - For a lacked or excluded item with role R in step S, the agent proposes `swap(S, I, J)` for an item J the user has, if J plays R in some taught task. Support is the number of such tasks.
  - Otherwise it maps the most similar stored case (situation signature by role, Jaccard similarity of at least 0.5) and transfers its change.
  - A lacked tool with no stand-in drops its step only if the step is `optional`. Otherwise the answer carries a gap.
- **Arm B: induced rules.** Lifted patterns (situation pattern to change pattern) are induced offline from the case library and kept when supported by at least 2 cases from different tasks. They are exported as Datalog rules and run on `sla_reasoner`. Popper is tried offline if it can be installed. If not, the built-in pattern inducer is the arm.
- **Gap and teach.** If no proposal has support, the answer lists the base procedure plus `gap lacks <item> in step <n>` and asks the teacher. The teacher (Haiku, given the scenario's facts) answers in English. The answer is parsed and stored as a case (situation plus difference) with provenance, and it is used from then on.

## 5. Knowledge sources and the curriculum

- **C1, dialogue.** For each training task, in order:
  1. Request.
  2. The agent asks.
  3. The teacher teaches, using the authored teach lines.
  4. Second encounter, with a new request wording.
  5. Variant encounter: the agent adapts or asks, and the teacher answers.
  6. Retention, after other tasks.

  Haiku plays the teacher only where the agent asks something the authored lines do not cover: clarification, gap questions and corrections. The parser fallback also calls Haiku.
- **C2, dialogue plus dataset.** C1, plus a generated dataset of about 300 tasks. Haiku writes each one as a v3 scenario file:
  - teach lines and facts, so every pair is labelled by construction;
  - requests, situations and 2 variants.

  Every file passes the validator. A leakage check rejects any task whose name, or whose (action, object) signature set, overlaps H3, and any gardening content. The dataset trains the parser and the noticing model, and fills the case library and the rule inducer.
- **Comparison.** C1 against C2 on H3, on the same build, as a paired test.

## 6. Evaluation

### 6.1 Sets

| Set | Content | Use |
|---|---|---|
| train | 52 tasks (`data/curriculum3/train`) | Teaching, model training |
| dev | 19 tasks + bake_potato (`data/curriculum3/dev`) | Development, θ, debugging |
| H3-S | 60 new situations on known tasks | Tier S |
| H3-T | 60 new tasks in known domains | Tier T |
| H3-G | 30 new gardening tasks + the 5 held-out gardening tasks | Tier G (new domain) |
| H3-R | 5 random splits of train ∪ dev tasks (train on the rest) | Algorithmic tier |

H3 is sealed by `data/heldout3/MANIFEST.sha256`, and a test checks the hash. H3 has **3 readings**:
1. a baseline reading after milestone 24;
2. a checkpoint after milestone 26;
3. the final reading in milestone 28.

The lab serves counts only for H3 runs.

### 6.2 Metrics (all per tier, with Wilson 95% intervals)

- **M1, content pass:** strict pass of the answer on the encounter.
- **M2, graded recall and precision of facts;** wrong-claim rate.
- **M3, stored-fact accuracy:** taught facts against ground truth, after teaching.
- **M4, parser coverage at ≥ 95% precision,** and its curve over teaching rounds.
- **M5, identity top-1;** noticing false alarms and hits.
- **M6, adaptation:** change recall, wrong changes and gaps asked, per arm.
- **M7, cost:** Haiku calls per task taught and per answer (live and cached), and seconds.
- **M8, honest-gap rate:** share of unanswerable cases where the agent asked instead of guessing.

### 6.3 Baselines (D13)

- **B1, word-overlap retrieval.** It recites the most similar taught procedure with no adaptation. Zero calls.
- **B2, Haiku with the teaching in its prompt.** Haiku gets the authored teach lines and the request, and answers in relation lines. One call per H3 case, cached.
- **B3, v2.0.0 replay.** The 2.0 agent runs on H3 through its own curriculum runner, and its replies are read back by the X1 parser. Read-back is lossy (X6), so the score is labelled as a lower bound.

### 6.4 Targets (frozen after the H3 baseline reading; reported met or not)

| Metric | Target |
|---|---|
| M1, tier S (new situation) | ≥ 40% and ≥ B1 + 20 points |
| M1, tier T (new task, after one teaching) | ≥ 60% |
| M1, tier G (new domain, after one teaching) | ≥ 40% |
| M3, stored-fact accuracy, tier T | ≥ 90% (with fallback) |
| M4, parser coverage at 95%, end of training | ≥ 50% (from 19.8%) |
| Wrong-claim rate | ≤ 5% of answers |
| M8, honest gaps | ≥ 80% |
| Word lists in agent crates | 0 |
| C2 against C1 | Reported with a paired test; no target |

The targets may be lowered once, after the baseline reading, with the reason written into `research/PREREG_3.md`. They are frozen after that.

## 7. Milestones (phase B)

Each milestone commits on `design-3` with its acceptance tests passing (D12: no stops). Haiku calls stay inside the budget ledger.

**22. Schema, vault and check in Rust.**
- Build: `kb3`, relation-line format, content check, negative controls, word-list test.
- Accept:
  - gold answers score 100%;
  - each negative control fails;
  - relation lines round-trip;
  - the word-list test passes on `kb3`, `parse3`, `agent3`, `teach3`.

**23. Learned models at runtime.**
- Build: `tools/design3/train_parser.py` (all four models, JSON export), `parse3` loader and inference, `teach3` fallback with the shared cache.
- Accept:
  - Rust and Python predictions are identical on all dev lines;
  - θ is set on dev;
  - fallback pairs are written and retraining reads them.

**24. Teaching and answering (C1, base tasks).**
- Build: `agent3` ingestion, identity, base answers, the `eval3` curriculum runner (C1) and `lab3` events.
- Accept on dev:
  - M3 ≥ 90%;
  - second-encounter M1 ≥ 80%;
  - identity ≥ 95%.
- Then the **H3 baseline reading**, and the targets are frozen.

**25. Situations and adaptation arms.**
- Build: situation tagger, noticing, arms A and B, gaps, teacher answers stored as cases.
- Accept on dev variants:
  - wrong-claim rate ≤ 5%;
  - every gap case asks;
  - a taught variant is reused on its repeat.

**26. Dataset (C2).**
- Build: generator (about 300 tasks, about 400 calls with retries), leakage check, C2 training run.
- Accept:
  - 0 leakage hits;
  - every file valid;
  - C2 run complete.
- Then the **H3 checkpoint reading**.

**27. Baselines.**
- Build: B1, B2, B3.
- Accept: each runs on dev and H3, with cached calls.

**28. Final measurement.**
- Build: the **final H3 reading** for C1, C2, both arms and the baselines. Ablations:
  - no fallback (parser only);
  - no aliases;
  - no gating;
  - arm A only;
  - arm B only.
  
  Run 5 random splits.
- Accept: every target reported with an interval, met or not; the ledger shows 3 readings.

**29. Research PDF.**
- Build: `paper/` (LaTeX), with every number generated from `data/` by `tools/paper/numbers.py`. It covers v1, 2.0 and 3.0: method, results, failures and limits.
- Accept:
  - `make -C paper` builds from a clean checkout;
  - `numbers.py --check` shows no difference;
  - every number in the text is a generated macro.

## 8. Haiku budget (D11: 5,000 calls; 21 used in phase A)

| Use | Estimate |
|---|---|
| Parser fallback during C1 and C2 training (decreasing with distillation) | 800-1,200 |
| Teacher answers to gaps and questions | 300-500 |
| Dataset generation (C2) | 350-450 |
| H3 readings: fallback on new tasks' teach lines and requests (cached across readings) | 600-900 |
| B2 Haiku with teaching | 200-300 |
| B3 v2.0.0 replay (2.0 translator calls) | 600-900 |
| Reserve | about 700 |
| **Total** | **about 3,500-4,900** |

If the ledger reaches 4,500 before milestone 28, the ablations run on cached calls only, and B3 is cut to tier S and tier T.

## 9. What 3.0 can claim, and what it cannot

- **Can claim, if the targets are met:**
  - an agent with no English in its code learns procedures from teaching;
  - it reuses them under new wording and new situations, with exact, checkable content;
  - its language layer moves from teacher-dependent to self-sufficient at a measured rate.
- **Cannot claim:**
  - fluency;
  - knowledge it was never taught: most new situations need a teaching;
  - that synthetic text stands for human text, since every text in the project is written by a model.

  The PDF states each of these limits.

## 10. Every 2.0 failure, its fix and its test (condensed)

| 2.0 failure (REPORT_DESIGN_2) | 3.0 fix | Test |
|---|---|---|
| About 30% of step lines stored wrong (§11.10) | Learned parser + validator + fallback; compound steps split in the ground truth | M3 on dev and H3-T |
| Contaminated held-out set; one clean run | Sealed H3, 3 readings, counts only | Hash test; ledger |
| Style judges steered the work | No style metric; relation lines | No metric depends on wording |
| Lenient symbolic check | Strict pass + precision + wrong claims | Negative controls |
| About 225 hard-coded English strings | Words as data; word-list test | Test fails on any English list in agent crates |
| Task identity failures ("jacket potato") | Learned task aliases + confirmation | M5 |
| Translator inconsistency (71% of repeats translated differently) | Deterministic learned parser; cached fallback | Same line, same parse (test) |
| Transfer only by item swap | Function roles + cases + rules, gated, + gaps | M6, M8 |
| Polling dashboard | `lab3` SSE stream | Lab integration test |

## 11. Later (beyond 3.0)

- Fluent English output from the relation lines.
- Human-written test text (a wikiHow slice).
- Active learning: the agent choosing what to ask.
- Larger domains.
- Importing outside knowledge.
