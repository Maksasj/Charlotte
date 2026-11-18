# REPORT_DESIGN_2: The symbolic learning agent, second build

*Report date: 2026-11-18. It covers the Design 2.0 build from the design commit (`fe7b901`, 2026-11-17 10:04) to the v2.0.0 tag (`fe21084`, 2026-11-18 14:00). The design is `research/Symbolic Learning Agent – Design 2.0.md`. The first build's report is `research/REPORT_DESIGN_1.md`. The build log is `PROGRESS.md`. Every number in this report comes from files in this repository: `eval/`, `data/`, `PROGRESS.md` and the git history.*

This report has two jobs. It records what Design 2.0 built, measured and learned. It also collects what Design 3.0 should start from. It follows the outline of the v1 report, so the two can be read side by side.

## Contents

1. Executive summary
2. Synthesis: what we know now
3. How the work went: timeline, decisions and incidents
4. Architecture: what was built
5. Results and data analysis
6. What still fails, and why
7. Code-level weaknesses and technical debt
8. Knowledge gained: lessons for the research question
9. Inputs for Design 3.0
10. Appendix: glossary, artefacts, how to reproduce

---

## 1. Executive summary

**What exists.** All seven milestones of Design 2.0 (12 to 18) are built and measured. They are on branch `design-2`, tagged `v2.0.0`. Every acceptance target the user kept in scope is met.

The size of the build, against v1:

| Measure | v1 (`99c6fa4`) | v2.0.0 |
|---|---|---|
| Rust code | 11,297 lines | 25,340 lines in 9 crates (`crates/translate` and `crates/lexicon` are new) |
| Tests | 68 | 208, all passing; no test calls a model |
| Commits | 29 (2026-11-16) | 121 more (58 on 2026-11-17, 35 on 2026-11-18, plus the design commit) |

Around the agent there is now:

- **A curriculum training system** (`sla_eval curriculum`):
  - scenarios with a ground truth, a teach script, a checklist and a variant;
  - 20 held-out scenarios (15 of them with a variant), 55 training scenarios and 30 seed procedures;
  - up to 16 parallel Haiku teachers, eight encounter kinds, and a teacher fix pass;
  - a Haiku checklist judge, which is the judge of record.
- **A training dashboard** (`sla_trainer`): a Rust backend and a React (Vite) UI with token auth, a live curriculum panel and knowledge health. It stops automatically at 95% of the session limit or 70% of the week.

Over the two days, 12,626 model calls were logged in `data/training/calls/`.

**What the numbers say.** The held-out stage runs after training. Each scenario is taught once and then met again in new words.

| Measure (held-out, 20 scenarios) | v1 | Start of 2.0 (run 1, 11-17) | v2.0.0 (last six runs, 11-18) | Target |
|---|---|---|---|---|
| Real answers to help requests | 0-1 of 68 | | | |
| Second encounter solved without teaching | | 6 of 20 (30%) | **15-18 of 20 (75-90%)** | 70% |
| Transfer to a changed situation (15 variants) | | 0 of 15 | **6 of 15 (40%)** in 5 of 6 runs | 40% |
| Retention (later, after other scenarios) | | 5 of 20 | **14-17 of 20** in the last four runs | none set |
| Not understood | 44% (help turns) | 1.5% | 0.4-1.3% | 10% or less |
| Helpful + fitting clarification | 0% (help turns) | 61.7% | 74-80% | 60% |
| Stored facts contradicting ground truth | | 1.0% | 0.8-1.0% | 1% or less |
| Judge score, 1-10 | 2.4-4.3 | 3.65 (run 6) | **5.10-5.42** | 6.0 (out of scope) |
| Preferred over the milestone 12 build | | | 20 of 20 pairs | 70% |
| Agent's own time per turn, 95th percentile | 639 ms | | 63-71 ms | 100 ms |

**The diagnosis in one paragraph.** v1's diagnosis was right. The missing piece was representation, not intelligence. With procedures, dialogue state, a validated translator and a curriculum that measures repeated encounters, the same symbolic core went from helping no one to answering 75-90% of taught tasks correctly in new words.

The second half of the work showed that the remaining gap is mostly **what was taught, not what the agent can do**:

- **Transfer:** once item swaps were handled, the variants that still fail are exactly the ones that need knowledge no teacher gave, such as a soft-boiled egg's time or brown rice's simmer time.
- **Judge score:** it rose from 3.6 to 5.4 by changing the wording alone. The reasons the judge gives for the rest are tips, sample wording and safety warnings that nothing taught.

**The big caveat.** In the afternoon of 2026-11-18, the held-out transcripts were read to find failures, and the fixes were tested on the same held-out set. The fixes are general mechanisms, not scenario patches: lack cues, proposals, extra ingredients, wording. Still, **the held-out set is no longer truly held out.** The 40% transfer and 85-90% second encounter should be confirmed on a fresh, independently written set (section 9, R1).

---

## 2. Synthesis: what we know now

1. **Procedures plus dialogue state was the right bet.** The v1 report's top recommendation (R1 to R3) was built in milestones 13 and 15. It turned "0 helpful replies" into a working help dialogue. The dinner dialogue from the v1 report passes end to end, with the oracle parser and live with Haiku.
2. **Measure the right thing, then the numbers move.** v1's fixed tests stayed at 98-100% while conversation stayed at 3-4 out of 10. In 2.0 the second-encounter measure exposed real defects at every run, for example a task taught under one name and asked for under another, needs not listed, or steps without their tools. Fixing them moved the number from 30% to 90%.
3. **Most failures are understanding failures, not reasoning failures.** Across all curriculum runs, the reasoner was almost never the cause. Failures came from:
   - translation: wrong task name, a lack not detected, a clause glued into one symbol;
   - matching a request to a known task;
   - wording: "until it is drain", "remove the tube from the tire lever".
4. **The translator is safe now, but not reliable.** Grounding, typing and the closed registry keep invented knowledge out: contradicting facts stay under 1%. But the same message can translate differently on two calls; one pancake turn adapted in one run and said "Okay." in the next. Act accuracy is 92%.
5. **LLM judges measure taste as much as correctness.** The Haiku checklist judge and the symbolic ground-truth check disagreed widely on the same replies: 10 of 20 against 18 of 20 in run 4. The 1-10 judge rewards echoing the user's own words ("the right way to cook rice in a pot") by +1 to +1.5 points with identical content, and penalises per-step times by about 1 point. A judge of record plus a second, symbolic check was the right setup.
6. **Saying it the way it was taught is the biggest wording lever.** Storing each step's taught wording, with facts still carrying every claim, raised the judge score from 4.72 to 5.25. It also raised retention from 14 to 16 of 20, because checklist details like "into lights and darks" survive. Generating sentences from bare facts loses the details that checklists ask for.
7. **Transfer needs mechanisms, and knowledge.** Transfer went from 0 to 6 of 15. Five mechanisms did this:
   - a taught substitute;
   - analogy from another procedure;
   - asking "what do you have that could do the job?";
   - adopting the user's proposal;
   - adding a named extra ingredient.

   The other 9 variants change a time, a plan or a method, and no mechanism can invent that.
8. **Small samples make every number noisy.** With 20 and 15 scenarios, one scenario is 5 to 7 points. Transfer moved between 5 and 6 of 15 between runs of the same build. The judge score has a ±0.35 interval. Changes needed at least two runs to be trusted.
9. **Plan limits and caching shape the work.** A full training run costs about 2,000 calls and a held-out run about 100-230. Shared caches for translation, judging and labels make repeated runs cheap; run 4 needed only 96 new calls. The 95%/70% automatic stop protects the user's plan.

---

## 3. How the work went: timeline, decisions and incidents

### Timeline

All dates are 2026; times are local. The work spans two PCs: Windows on 11-17, Linux from 11-18.

| Phase | When | Commits | What happened |
|---|---|---|---|
| **Design** | 11-17 10:04 | `fe7b901` | Design 2.0 committed; DESIGN.md renamed DESIGN_1.md |
| **M12 Honest measurement** | 11-17 10:22-11:02 | `016feb8` `245609f` | Shared brain (one vault writer), one model per vault state, v1 correctness fixes, turn traces, event stream, outcome labels, 296-turn regression set, stress and bench commands |
| **M13 Knowledge model** | 11:07 | `a6ec974` | Numbers with units and ranges, arithmetic, aggregates, fact ids, the predicate registry, procedural/choice/session/lexical/self layers, procedure and task files |
| **M14 Understanding** | 14:58-15:12 | `6eaa3b2` `c00c538` | Segmenter, three-tier translator (patterns, learned, Haiku), validator with grounding, round trip, retry; gold-set harness |
| **M15 Dialogue manager** | 15:50 | `23b6de0` | Session memory, task frames, slot questions, moves chosen by Datalog policy rules, "teach me", question stack |
| **M16 Generation** | 15:59-16:17 | `9b67613` `daf6ac3` `6792dde` | Planner, microplanner, realizer, frames; 200 plans pass; wording judged; ceiling recorded |
| **M17 Learning** | 16:23-16:38 | `f4c7a50` `0d8923c` | Procedures from teaching, gradual corrections, word queue, rule questions, tentative layer, learned patterns |
| **M18 Curriculum** | 16:46-21:36 | `26a7bdc` … `2d1b87b` | Curriculum runner, scenarios, seeds, hygiene pass; runs 1-7 (second encounter 30% → 65-70%); pairwise 20 of 20 |
| **First training run** | 11-17 evening | `83f5b0f` | 16 teachers, 55 training scenarios: no change in held-out. Found that substitutes were stored as the verb `replace` |
| **Final training runs** | 11-18 morning (new PC) | `ff5ddbc` `b748b6f` | Retrained vault (110 procedures), promoted to `vault/`; second encounter 65-70%, transfer still 0 |
| **Transfer mechanisms** | 11-18 morning | several | Analogy, asking for a replacement, the user's proposal, dropping tool steps; transfer 1 of 15 |
| **Held-out fixes** | 11-18 afternoon | 20+ commits | Lack cues, implied requests, taught wording, framing, step times, prepositions; transfer 6 of 15, judge 5.4 |
| **Close** | 11-18 14:00 | `fe21084` | Judge score and the M15 replay put out of scope by the user; v2.0.0 tagged |

### Decisions and why

The design's six open decisions were confirmed on 2026-11-17: a fresh vault, the vault kept out of git, "probably" answers only in training, staying on the Claude CLI, no bulk import, and retiring `sla_pretrain`. The build added these:

| Decision | Why |
|---|---|
| **The brain owns the vault**: one `Brain` per vault behind a mutex; each conversation is an `Agent` | One in-memory copy, one writer. Translation happens before the lock, so no model call ever runs inside it |
| **One model per vault state**, cached by a generation counter | v1 ran 3-5 full evaluations per message; now at most one per question |
| **Usage counts in `vault/stats/uses.tsv`**, not on fact lines | Writing `@uses` on every answer invalidated the model cache |
| **History evaluated moment by moment**, restricted to dependencies | Facts true at different times are never joined, and the cost is bounded (64 moments) |
| **Rule quarantine** instead of dropping all rules | One bad rule no longer disables the whole program |
| **All-or-nothing changes** (transactions) | A failed refinement leaves the vault untouched |
| **The Haiku checklist judge is the judge of record**; the ground-truth check is reported beside it (user, 11-17) | One agreed measure; hand-label calibration not needed |
| **The teacher is Haiku**, held to the ground truth by a checker (user, 11-17) | Cheap enough for 16 parallel teachers |
| **Model calls stay on the Claude CLI; speed and act accuracy out of scope** (user, 11-17) | The CLI adds about 1.3 s per call; act accuracy mostly reflects the model's own errors |
| **Learned-pattern coverage re-measured after training, not tuned** (user, 11-17) | Teaching is the main input; 300 turns are too varied for templates |
| **Substitutes are `substitute(A, B)`** on every tier, with a migration in consolidation | The pattern tier stored them as the verb `replace(B, A)`, so 15 of 18 taught substitutes were unusable |
| **Steps keep their taught wording** in the procedure's notes (Markdown text, not Datalog) | The facts lose details that checklists ask for. Facts still carry every claim; the wording is used only while it still names the stored action |
| **Judge score out of scope** (user, 11-18) | The remaining low ratings come from what the teacher taught: thin steps, no sample wording, no tips |
| **Milestone 15 help-turn replay out of scope** (user, 11-18) | Its turns ask for things the replay vault was never taught, so the right reply is "teach me", which the judge marks as wrong content |
| **The user adds the git remote and pushes** (user) | |

### Blockers and incidents

| Incident | Effect | Resolution |
|---|---|---|
| A release build in a shell background job was killed | Run 2 measured an old binary | Builds run in the foreground before a run |
| The training run stopped at 80% of the session limit while labelling | The held-out stage was missing | Resumed on the trained vault; the stop is now at 95% |
| Moving to a Linux PC: the trainer used Windows `tasklist` | It could not see other runs | `pgrep -x -r R,S,D sla_eval` (a finished zombie process had counted as running) |
| `promote_vault.py` failed when `vault/` did not exist | No promotion | Backup step skipped when there is nothing to back up |
| The 5% plan limit at the start of the 11-18 session | Work waited for the reset | Resumed after the reset |
| The `/goal` stop hook rejected "finished" while targets were unmet | Forced honest status wording and re-measuring | Status rewritten with measured numbers only |
| `git push` needs credentials this session cannot type | Not pushed | The user pushes |

### Operational findings

- **Cost of a run.** A training run of 55 scenarios with 16 teachers is about 2,000 calls. A held-out run is 100-230 calls, fewer when the caches hit. The judge score (`sla_eval rate`) is about 40 calls and pairwise about 20.
- **Caches make iteration cheap.** The translate, label and judge caches are shared across runs, keyed by exact prompt. After a code change, only the changed turns cost calls.
- **Session budget.** On the plan used, one held-out run plus the judge score is about 5-7% of a session. About ten fix-and-measure cycles fit into one session.
- **Variance.** The same build gave transfer 5 or 6 of 15, and judge scores of 5.10 to 5.42. Treat one-scenario changes as noise.

---

## 4. Architecture: what was built

### Message flow

A message passes six stages. Each stage can fail for one part of the message without losing the rest.

1. **Segmenter**: sentences first, then clauses when a sentence fails.
2. **Translator** (`crates/translate`), three tiers:
   - fixed patterns;
   - learned patterns;
   - one Haiku call for every part still open, with the conversation as context, then one retry with the validator's problems.

   Post-processing: `sentence_rules` (substitutes, glued `until` clauses), `lack_cues` ("no squeegee", "my kettle broke"), and `implied_request`, which handles a stated lack, a how-question naming one task, or items listed after an unknown task.
3. **Validator**: registry vocabulary, argument types, grounding of every constant in the message, session or registry, act type, and round trip.
4. **Dialogue manager** (`crates/agent/src/dialogue.rs`): session facts, task frames, moves chosen by Datalog policy rules, plus the direct paths: needs, duration, adapt to a new situation, a taught procedure by name.
5. **Reasoner** (`crates/reasoner`): v1's engine plus arithmetic, aggregates, quarantine, past moments and proofs.
6. **Generation** (`crates/nlg` and `crates/agent/src/tasks.rs`):
   - planner, microplanner, realizer and frames;
   - task help: needs, steps in partial order, overlaps ("while you…"), taught wording, substitute swaps, dropped tool steps, extra ingredients, waiting-step times, the request's framing, constraint notes and a friendly close.

Every turn writes a **turn trace** with its parts, acts, tier, moves, claims, vault changes, timings and calls. The trace goes to `sessions/` and to the brain's event stream.

### Components against v1

| Component | v1 | 2.0 |
|---|---|---|
| Storage | Several in-memory copies; `@uses` rewrote files | One brain per vault, transactions, stats file, fact ids, `@conv` per fact |
| Knowledge | Facts and rules about entities | Plus procedures (steps, needs, uses, until, overlaps, durations), task schemas, substitutes, numbers with units, a typed registry of 74 predicates |
| Reasoner | 3-5 full evaluations per message | One cached model per state; `sum`/`count`/`min`/`max`; arithmetic; history by moments; quarantine |
| Understanding | Whole message to Haiku | Per-part tiers, validator, retry, clause splitting, lack and request inference |
| Dialogue | One pending slot | Session memory, frames, slot questions, policy moves, question stack, adapt move |
| Generation | Templates | Planned, aggregated, frame-worded, read back; taught wording for steps |
| Learning | Pattern search on every statement | Procedures from teaching (hypotheses until used), gradual corrections, word queue, tentative rules, learned patterns, consolidation (hygiene, substitute migration) |
| Curiosity | Questions in chat | Goals from failed turns (`vault/queue/failures.jsonl`), asked only of teachers |
| Training | Pretraining scripts, friend chats | Curriculum with ground truth, encounters, fix pass, variant teacher, spaced queue |
| Evaluation | Fixed tests plus 1-10 judges | Outcome labels, a checklist judge plus a ground-truth check, judge score, pairwise against M12, stress, bench |

### The curriculum

Each scenario (`data/curriculum/scenarios/*.md`) holds a ground truth in Datalog, teach lines, a checklist and a variant: a change, a request, the situation, and the expected reply. A run goes through these encounters:

| Encounter | What it does |
|---|---|
| `teach` / `teach_block` | The teacher asks for help, the agent fails or asks to be taught, and the teacher sends the teach lines. The **fix pass** then compares what was stored with the ground truth (step actions, objects, `until`) and reteaches the wrong steps, after clearing that session's wrong facts |
| `teach_retry` | The same request again in the same session; checks that the teaching took |
| `repeat` (**second encounter**) | A new session, the request in new words, no teaching. This is the main measure of learning |
| `transfer` / `transfer_retry` | The variant: the same task in a changed situation (no kettle, out of eggs). In training, a failed variant gets up to three teach lines from the variant teacher, then one more try |
| `spaced_repeat` | Failed scenarios come back later from a queue |
| `retention` | The request again at the end, after the other scenarios |
| `curiosity` | The agent asks its own questions from failed turns, answered from the ground truth |

The training stage uses 55 scenarios. The held-out stage uses 20, after training, on the trained vault. Held-out scenarios are taught in their own first encounter; transfer is measured with no teaching.

---

## 5. Results and data analysis

### Milestone acceptance

| # | Milestone | Acceptance result | Status |
|---|---|---|---|
| 12 | Honest measurement and safe ground | Outcome mix with Wilson intervals in one command; the same commit run twice is identical; 16 teachers for 10 minutes on mocks (29,697 conversations, 185,427 turns) with 0 corrupted lines; agent time p95 63-71 ms (was 639 ms); v1 baseline recorded with the new labels | done |
| 13 | Knowledge model | 100 procedure files round-trip; 44 reasoner cases with proofs; 20 ill-typed v1 facts rejected; "used to" never joins times | done |
| 14 | Understanding | Gold F1 0.945 (target 0.93, met); act accuracy 92% (target 95%); long messages 97% recovered with 0 ungrounded; not understood on real turns 11.1% (target 15%); median 4.1 s (target 2 s) | built; accuracy and speed out of scope |
| 15 | Dialogue manager | Dinner dialogue passes with the oracle parser and live; echo 1.5% (met); fitting moves on the v1 replay 24.6% (target 80%) | done; replay out of scope |
| 16 | Generation | 200 plans pass grammar, faithfulness and read-back; no repeats in 12 turns; new wording preferred in 70.3% of pairs (target 65%); ceiling: a model rewrite preferred in 78.4% | done |
| 17 | Learning | Taught-once recipe used later in new words; counterexamples handled; 6 of 6 guessed rules settled; agreement 95.5% (met); learned-pattern coverage 3.3% (target 20%) | built; coverage below target |
| 18 | Curriculum training and hygiene | Second encounter 75-90%; transfer 40%; not understood ≤1.3%; helpful + clarify 74-80%; contradicting facts ≤1.0%; meta requests 0; duplicates after hygiene 0; pairwise 20 of 20; judge score 5.1-5.4 (target 6.0) | done; judge score out of scope |

### Curriculum progression (held-out, Haiku checklist judge)

| Run | Date | Second encounter | Transfer | Retention | Not understood | Helpful + clarify | What changed before it |
|---|---|---|---|---|---|---|---|
| 1 | 11-17 14:51 | 6 of 20 | 0 | 5 | 1.5% | 61.7% | first full run |
| 2 | 15:21 | 9 | 0 | 3 | 0.4% | 62.5% | tasks ranked by fit, plurals (old binary) |
| 3 | 15:48 | 10 | 0 | 6 | 1.3% | 63.4% | needs listed first, request-to-task by what it involves |
| 4 | 16:13 | 10 | 0 | 7 | 1.3% | 63.9% | wording; only 96 new calls |
| 5 | 16:23 | 13 | 0 | 10 | 0.9% | 67.0% | teacher fix pass |
| 6 | 19:13 | 14 | 0 | 8 | 1.7% | 67.0% | curiosity round, duration plus steps |
| 7 | 21:32 | 13 | 0 | 11 | 0.9% | 67.0% | articles and prepositions |
| first training | 11-17 night | 13 | 0 | 10 | 1.3% | 67.2% | 55 training scenarios |
| final training | 11-18 05:11 | 14 | 0 | 11 | 2.1% | 67.4% | substitute fixes |
| analogy | 06:27 | 13 | 1 | 11 | 1.2% | 70.1% | analogy, asking, proposals |
| 10:32 | 11-18 | 13 | 0 | 10 | 0% | 68.6% | |
| 11:09 | | 15 | 6 | 11 | 1.3% | 74.0% | lack cues, implied requests, extra ingredient |
| 11:10 | | 15 | 6 | 9 | 0.4% | 74.7% | confirmation run |
| 11:25 | | 15 | 6 | 14 | 0.9% | 74.0% | how-questions, items after an unknown task |
| 11:28 | | 17 | 6 | 16 | 0.5% | 76.1% | taught wording |
| 11:35 | | 18 | 6 | 17 | 0.5% | 77.2% | the request's framing |
| 11:39 | | 18 | 6 | 17 | 0.9% | 80.5% | step times on waiting steps only |
| 11:44 | | 17 | 6 | 17 | 0.9% | 77.6% | a named procedure beats a same-named schema |

Run reports: `eval/curriculum/<run time>.md`; raw records: `data/training/curriculum/<date>.jsonl`; live transcripts: `data/training/curriculum/live/`.

**Reading the table.**

- Training on 55 other scenarios never moved the held-out numbers. Every gain came from code changes, because the held-out tasks are taught in their own first encounter either way.
- The ground-truth check is usually higher than the Haiku judge: 18-20 of 20 against 13-14 of 20 in runs 5-7.
- The two jumps on 11-18 were understanding (transfer 0 → 6) and wording (retention 11 → 17, judge 4.7 → 5.4).

### Judge score (1-10, 40 conversations: second encounter and retention)

| Build | Score (95% interval) | Change |
|---|---|---|
| v1 best | 4.3 | |
| Run 6 (11-17) | 3.65 (3.30-4.00) | |
| Run 7 | 4.05 (3.72-4.38) | articles, prepositions |
| Final training (11-18 05:11) | 3.80 (3.29-4.31) | |
| 10:32 | 3.58 (3.12-4.03) | |
| 11:10 | 4.03 | transfer fixes |
| 11:15 | 4.33 (3.90-4.75) | prepositions by action |
| 11:19 | 4.45 (4.04-4.86) | friendly close |
| 11:25 | 4.72 (4.37-5.08) | no stray apology, items after an unknown task |
| 11:28 | 5.25 (4.91-5.59) | **taught wording** |
| 11:35 | **5.42 (5.10-5.75)** | **request framing** |
| 11:39 | 5.30 (4.96-5.64) | waiting-step times |
| 11:44 | 5.10 (4.69-5.51) | |

**Probing the judge** (the exact rating prompt, 3 samples per variant, one rice transcript):

| Variant | Scores |
|---|---|
| current wording | 5, 5, 5 |
| "Sure!" opener | 6, 4, 5 |
| echoes the concern ("so it doesn't end up mushy") | 6, 6, 6 |
| echoes the framing ("the right way to cook rice in a pot") | 6, 6, 7 |
| "Enjoy your rice!" as the close | 4, 4, 4 |

On a tire transcript, dropping the per-step times scored 5, 4, 5 against 3, 4, 3. A warmer "You're welcome! Good luck" after thanks made no difference.

Asked why, the judge names content: no sample wording for the thank-you note, no glue and roughening steps in the tire patch, no tips on liquids and chargers when packing. None of it is in the teach lines. This is the evidence behind the user's decision that the remaining gap is a teacher problem.

### Transfer: which variants pass

| Variant | Change | Status | Mechanism |
|---|---|---|---|
| make_tea | kettle broke | pass | lack cue ("my kettle just broke"); a needs question with a lack gets the adapted help |
| make_salad | no lemons | pass | implied request; taught substitute (vinegar) |
| clean_window | no squeegee | pass | lack cue on the item next to the cue |
| hang_picture | no nails allowed | pass | "doesn't allow" cue; the user's proposal (adhesive strips); hammer step dropped |
| make_pancake | out of eggs | pass in most runs (one run answered "Okay.") | asks for a replacement; adopts the banana |
| make_omelette | cheese omelette | pass | the extra ingredient goes in before folding |
| water_plant | no watering can | fails one base check | the jug is adopted; the checklist wants a warning about standing water, which nothing taught |
| do_laundry | no room to hang | fails | the dryer is proposed, but the 45-60 min tumble time was never taught; "sort lights from darks" sometimes lost |
| fix_flat_tire | spare tube | fails | needs "skip finding and patching"; there is no step-replacement mechanism |
| boil_egg | soft-boiled | fails | needs 5-6 min, never taught |
| cook_rice | brown rice | fails | needs 35-40 min, never taught |
| bake_potato | 25 minutes | fails | needs microwave then oven, never taught |
| pack_for_trip | cabin bag | fails | needs a liquids bag, never taught |
| plan_picnic | rain | fails | needs an indoor plan, never taught |
| plan_birthday_party | surprise | fails | needs secrecy steps, never taught |

**6 of 15 pass, and these are the item-swap variants.** The 9 failures need a different time, a different method or new steps. Only teaching, or a source of outside knowledge, can supply those.

### Training-stage figures

| Run | Second encounter | Retention | Transfer after variant teaching |
|---|---|---|---|
| First training (11-17) | 23 of 55 | 8 of 55 | 0 of 36 |
| Final training (11-18) | 19 of 55 | 13 of 55 | 0 of 18 |

The training-stage transfer stayed at 0 because of two bugs, both fixed after these runs:

- the pattern tier stored "B can replace A" as a verb;
- the variant teacher taught step lines into the base procedure.

No training run was repeated after the 11-18 fixes; see section 9, R2.

### Vault

The promoted `vault/` (from the 05-11-26 training run, consolidated) holds 110 procedures (the 30 seeds plus what training taught), 1 task schema and about 116 stored fact lines. The vault is not in git; a copy is in `data/training/curriculum/vaults/2026-11-18T05-11-26/`. It was trained **before** the afternoon fixes. The held-out numbers above use fresh, seeded vaults with the new code, so this vault has no "Taught wording" sections yet.

---

## 6. What still fails, and why

A taxonomy of the failures left in the last runs:

| Class | Examples | Cause | Fixable in the agent? |
|---|---|---|---|
| **Untaught knowledge** | soft-boil time, brown rice, tumble time, surprise party, tips, sample wording | Not in any teach line | No: needs teaching, or outside knowledge (section 9) |
| **Translation variance** | the same pancake answer adapted in one run and said "Okay." in another; "I never get it right" read as a complaint | Model output differs per call; the cache only helps on exact repeats | Partly: more post-validation rules, or learned patterns once seen |
| **Synonyms between task names** | "hang a painting" vs `hang_picture`; "picnic with friends" once matched `invite_friends_to_dinner` | Task matching is by words; no synonym knowledge | Yes: lexical synonyms in the registry and lexicon |
| **Situation phrasing without the item word** | "nowhere to hang them" (no drying rack) | Lack cues need the item named | Partly: map "nowhere to {action}" to that step's tool |
| **Lost teach details** | "check the soil is dry", "lights and darks" before taught wording | Facts hold the action and object only | Mostly fixed by taught wording |
| **Step-level changes** | "skip finding and patching" with a spare tube | Only item swaps, drops of tool steps and one inserted ingredient exist | Yes: replacing steps and taught alternatives (`instead_of_step`) |
| **Wording** | "Ask the allergy with the phone", "Put the passport with the hand luggage" in untaught-wording steps | The realizer's preposition and verb-frame choices | Partly: frames per verb in the lexicon |

---

## 7. Code-level weaknesses and technical debt

1. **Heuristics in Rust code, not in the vault.** The afternoon fixes added many word lists in code:
   - lack cues and HOW cues;
   - the generic task verbs (VERBS) and the state verbs (STATE_VERBS);
   - SURFACES, TOOLS, POUR_FROM, WAIT and HARM;
   - the extended MASS list;
   - the step-time rule.

   This breaks the design's principle 5, "knowledge is data, not code". They work, but they are English-specific, cannot be learned, and grow by hand. They belong in the lexicon (`vault/lexicon/`) as predicates such as `tool(lever)`, `surface(rack)`, `waiting_action(boil)` and `lack_cue("doesn't allow")`.
2. **Taught wording is text, not knowledge.** The wording is shown only while it still names the stored action, and the claims stay factual. But the wording itself is not checked against facts. A misparsed step can now read better than what the agent actually believes. It is also not used by the reasoner, and it bypasses the realizer.
3. **The translator crate carries dialogue logic.** `implied_request` guesses the request from a lack, a how-cue, or items listed after an unknown task. That is a dialogue decision made in the translator, on the text, through `TContext.last_reply`. It should be a dialogue move with a proof.
4. **The held-out set is contaminated** by inspection on 11-18 (see section 1).
5. **`tasks.rs` grew large.** `task_help` now does substitutes, drops, extras, wording, framing, times, dedupe and notes in one function of several hundred lines. Splitting it into planner stages (content plan, then wording) would make it testable stage by stage.
6. **Two judges of different strictness.** The checklist judge and the ground-truth check disagree by up to 8 scenarios. The decision was made (Haiku is the record), but the gap hides real progress and real regressions alike.
7. **Translation cost and speed** are unchanged at about 4 s per turn through the CLI. That was an accepted decision, but it bounds how much can be measured per session.
8. **Learned patterns cover 3.3% of turns.** The design's "learned patterns take over from the model" did not happen at this scale.
9. **Stale documentation.**
   - The README still names the Windows GNU toolchain; the build now also runs on Linux.
   - The trained `vault/` predates the wording format.
   - PROGRESS.md's milestone table for 18 was corrected only with this report.
10. **The dashboard polls** every 2 s and re-reads run files on each request, so it is not real time. This is already noted for 3.0.

---

## 8. Knowledge gained: lessons for the research question

The research question is whether a fully symbolic agent, taught in conversation, can become helpful. 2.0 gives these answers:

1. **Yes, for taught tasks.** Taught once, a procedure is used later, in a new session and in new words: 75-90% by the strict judge, up to 95-100% by the ground-truth check. It is remembered after other scenarios (85%), with every claim backed by a stored fact.
2. **Learning means representation plus matching.** The hard part was not storing the procedure but finding it again: "patch the tube" for `fix_tire`, "a jacket potato" for `bake_potato`, "a picnic" for `plan_picnic`. Task identity across wordings is the core skill.
3. **Adapting to a new situation needs explicit mechanisms.** Each adaptation that works is a symbolic mechanism with a test:
   - a substitute;
   - analogy from another procedure;
   - asking for a replacement and adopting the user's proposal;
   - dropping a step that needed the missing tool;
   - adding a named ingredient.

   None of them invents knowledge. That is why they are trustworthy, and also why they stop at item swaps.
4. **The teacher sets the ceiling.** Both remaining gaps, transfer and the judge score, are bounded by what the teacher says. A symbolic agent can only be as helpful as its teaching is rich: tips, warnings, alternatives, the why. In 3.0 the teacher protocol and the curriculum matter as much as the agent.
5. **Natural replies need the teacher's words.** Generation from bare facts reads like a manual. Keeping the taught wording, and echoing the user's framing, produced the largest naturalness gains. It cost no faithfulness, because the claims still come from facts.
6. **An honest "I don't know" plus a question is a strong move.** "You don't have a watering can: what do you have that could do the job?" turned three failures into passes. Asking is cheaper than knowing.
7. **LLM judges need anchoring.** A symbolic check from the ground truth catches judge errors in both directions. Probing the judge with controlled variants shows what it rewards. Some of it is style that no symbolic content changes.
8. **Measurement discipline pays.** A fixed scenario set, cached judgements, and run-to-run comparison made about 40 fix-and-measure cycles possible in two days. Without the caches, each would cost about 10 times more.

---

## 9. Inputs for Design 3.0

These are recommendations to think about, not a design. They are ordered by priority: P0 means needed first.

#### R1. A fresh, independent held-out set (P0)

The 20 held-out scenarios were inspected on 11-18. Write a new held-out set of 30-50 scenarios with variants, independently of the code, as the v1 test data was: by a sub-agent from a spec only. Report its results as counts only, never transcripts, while tuning. Keep the old 20 as a development set. Larger sets also narrow the intervals (section 2.8).

#### R2. A teacher protocol that teaches richer knowledge (P0)

The ceiling is the teaching (section 8.4). Teach lines should include:

- tips and warnings: `warns(step, standing_water)`;
- reasons: `because(step, …)`;
- alternatives for common situations: `alternative(task, situation, steps)`, `substitute`, `takes_time_if(step, soft, 5, 6)`.

The variant teacher should teach situational changes, not only substitutes. Then retrain, because no training run was made with the 11-18 code.

#### R3. Move word lists from code to the lexicon (P0)

Tools, surfaces, waiting actions, lack cues, how-cues, state verbs, mass nouns and prepositions by action belong in `vault/lexicon/`, as typed predicates the agent can learn and a teacher can correct (section 7.1). This restores "knowledge is data" and opens these to learning.

#### R4. Task identity as knowledge (P1)

Synonyms and paraphrases of task names (`same_task(hang_painting, hang_picture)`), learned when a teacher confirms them. Task matching then becomes a rule with a proof, not a word-overlap score. Implied requests become dialogue moves (section 7.3).

#### R5. Situational adaptation beyond item swaps (P1)

Building on R2:

- conditional durations (soft-boiled, brown rice);
- step replacement (a spare tube instead of a patch);
- conditional steps (indoor or outdoor);
- a curiosity goal that asks the teacher when a variant is unknown ("How is it different for brown rice?").

The fix pass already does a version of this, but only in training.

#### R6. A training lab with a real-time dashboard (P1, user request)

Make the runner, the vault writer and the dashboard one system, with events pushed live over SSE or a WebSocket from the brain's event stream instead of polling files. Show per-encounter outcomes, the fix pass, curiosity questions and knowledge growth as they happen. Already in PROGRESS.md "Notes for Design 3.0".

#### R7. Wording as data (P2)

Generalise taught wording. Store wording per step and per task, check it against the facts (every content word maps to a stored entity or action), and let the realizer use taught phrases as templates for similar steps. Keep the ceiling measurement: a model rewrite was preferred in 78% of pairs in milestone 16.

#### R8. Translator stability (P2)

Translation variance makes runs noisy. Options:

- store every reviewed translation as a learned pattern sooner;
- vote over two calls for low-confidence parts;
- add more post-validation repairs.

Act accuracy and speed stay out of scope unless the CLI decision changes.

#### R9. Split `task_help` into stages (P2)

Content plan (steps, swaps, drops, extras, notes), then wording (taught wording, framing, times), each with its own tests (section 7.5).

#### R10. Housekeeping (P2)

- Retrain and promote a vault with the 11-18 code, so `vault/` has taught wording.
- Update the README's toolchain note.
- Decide whether the ground-truth check should count beside the Haiku judge.

### Questions for the user before 3.0

1. Is the goal of 3.0 broader tasks (more scenarios, other domains), deeper help on the same tasks (tips, alternatives, why), or the training lab?
2. Should outside knowledge enter in bulk, as v1's option E, with review? It is the only route to the untaught-knowledge failures without writing every teach line by hand.
3. Should the 1-10 judge score return as a target, given that it measures teaching richness and style?
4. Should Haiku stay the teacher, or should a stronger model write richer teach lines for the curriculum?

---

## 10. Appendix

### Glossary

- **Second encounter (repeat):** the taught task asked again in a new session, in new words, without teaching.
- **Transfer:** the same task in a changed situation, without teaching (held-out) or after variant teaching (training).
- **Retention:** the taught task asked again at the end of the run, after the other scenarios.
- **Teach block / fix pass:** the teacher sends the scenario's teach lines, then compares what was stored with the ground truth and reteaches the wrong steps.
- **Spaced queue:** failed scenarios that come back later in the run.
- **Checklist judge:** Haiku answers yes or no per checklist item; a scenario passes when every item is yes. Judge of record.
- **Ground-truth check:** symbolic; the needs are named, the step actions are in order, and the total time is right. Reported beside the judge.
- **Judge score:** Haiku rates each conversation from 1 to 10 ("as helpful, correct and natural as a knowledgeable friend").
- **Pairwise:** Haiku picks between this build's reply and the milestone 12 build's reply to the same request after the same teaching.

### Artefacts

| What | Where |
|---|---|
| Design | `research/Symbolic Learning Agent – Design 2.0.md` |
| Build log, decisions, problems | `PROGRESS.md` |
| Scenarios | `data/curriculum/scenarios/` (20 held-out), `data/curriculum/training/` (55), `data/curriculum/seed/` (30) |
| Run reports | `eval/curriculum/*.md` (48 runs), `eval/curriculum/judge_score.md`, `eval/curriculum/pairwise_m12.md`, `eval/curriculum/calibration.md` |
| Raw run records and live transcripts | `data/training/curriculum/*.jsonl`, `data/training/curriculum/live/` |
| Held-out run outputs from 11-18 | `data/training/curriculum/heldout_*.txt` |
| Every model call | `data/training/calls/` (12,626 lines) |
| Caches | `eval/translate_cache.jsonl`, `eval/label_cache.jsonl`, `eval/curriculum_judge_cache.jsonl` |
| Trained vault copy | `data/training/curriculum/vaults/2026-11-18T05-11-26/` |
| Regression set | `data/regression/turns.jsonl` (296 turns), `labels_v1_trainer.tsv` |

### How to reproduce the headline numbers

```
cargo build --release
./target/release/sla_eval curriculum --teachers 16      # held-out stage, 100-230 calls
./target/release/sla_eval rate                           # judge score, about 40 calls
./target/release/sla_eval pairwise                       # against milestone 12, about 20 calls
./target/release/sla_eval curriculum --train --teachers 16   # full training, about 2,000 calls (ask first)
cargo test --workspace                                   # 208 tests, no model calls
```
