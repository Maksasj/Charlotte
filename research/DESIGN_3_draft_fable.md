# Symbolic Learning Agent – Design 3.0 (draft)

Draft, 2026-11-18 · for review before any build work

## Summary

Design 3.0 is the third research iteration of this project. It is not planned as the last one: if it goes well it may be the final iteration. It keeps the rule that has held since v1: reasoning, memory, learning and every reply are symbolic. A language model (Haiku, through the Claude CLI) only translates input. In training it also plays teacher, user and judge.

v1 showed that the symbolic core works but could not help anyone: 0 or 1 real answers in 68 help turns. 2.0 added procedures, dialogue state, a validated translator and a curriculum. On its development set it then answered 75-90% of taught tasks correctly in new words. But only the first held-out run is clean, and it gave 6 of 20 second encounters, 0 of 15 transfers and 5 of 20 retentions. Every later number was tuned on the same 20 scenarios. Section "What 2.0 taught us" gives the details.

The 2.0 report locates the remaining failures at the two ends of the pipeline. The reasoner and the policy almost never failed.

- **Input end.** 30% of step lines were stored wrong on fresh tasks. 20% of translations of request-cue messages had no request act. Tasks were stored under drifted names, and the wrong task was retrieved in 193 replies.
- **Knowledge end.** Nothing taught what a changed situation changes, such as a time, a method, a warning or a step.
- **In between.** 2.0 broke some of its own principles:
  - taught wording is spoken without a check against facts;
  - about 78 English word lists sit in Rust code;
  - dialogue decisions are made in the translator from raw text;
  - several metrics pass by construction.

3.0 makes six changes and ends with one deliverable:

1. **Honest ground.** A fresh held-out set of 60 scenarios, written from a spec and sealed. It is read only a fixed number of times and reported as counts. The metrics measure the request turn, not politeness. A strict symbolic check sits beside the Haiku judge. A user simulator answers the question the agent actually asked.
2. **Deterministic teaching.** Teach lines in the fixed frames are parsed by a grammar, not by the model. The agent owns step and task identity. Every step is validated and read back before it is stored.
3. **Understanding as knowledge.** Request cues, task aliases and the word lists become lexicon facts. Finding the task for a request becomes a scored rule with a proof, and a doubtful match is answered with a confirmation question instead of a seed's recipe.
4. **Adaptation knowledge.** New predicates hold conditional durations, alternative steps, warnings, reasons, completion tests and situation modifiers. The agent notices when a request differs from what it was taught. If it does not know the difference, it says so.
5. **Faithful replies again.** A help reply is a content plan of atoms first and wording second. Taught wording is stored linked to atoms, so every clause again maps to a believed atom.
6. **A training lab.** The runner, the vault writer and the dashboard run as one process. Events are pushed live over server-sent events, and the lab shows a teach log with diffs, turn drill-down with proofs, and run comparison.

**The deliverable** is a research write-up as a PDF built with LaTeX. It summarises all three iterations, with the third run measured on the sealed set. Every number in it is generated from the repository's archived data by one command that makes no model calls.

Every failure, loss and problem found in 2.0 has a named fix and a test in "Every 2.0 failure, its fix and its test". As in 2.0, the design cannot promise that no new failures appear. The final held-out reading of 3.0 is final for 3.0: whatever it shows goes into the write-up as it is, and fixes go to a later iteration.

## What 2.0 taught us

All numbers come from `research/REPORT_DESIGN_2.md` (2026-11-18) and the files it cites.

**Results, read honestly**

| Measure | v1 | 2.0, run 1 (the only clean held-out run) | 2.0, last runs (development set) | 2.0, training stage (untuned tasks, morning build) |
|---|---|---|---|---|
| Real answers to help requests | 0-1 of 68 | | | |
| Second encounter, Haiku checklist judge | | 6 of 20 | 15-18 of 20 | 17-21 of 55 (31-38%) |
| Second encounter, ground-truth check | | n/a | 18-20 of 20 | 40-43 of 55 |
| Transfer to a changed situation | | 0 of 15 | 6 of 15 (item swaps only) | 0 of 37 |
| Retention | | 5 of 20 | 14-17 of 20 | 11-13 of 55 |
| Helpful + clarification, request turn only | 0% | 10.1% | 58-63% | |
| Helpful + clarification, all turns (2.0's official figure) | | 61.7% | 74-80% | |
| Judge score, 1-10 | 2.4-4.3 | 3.65 (run 6) | 5.10-5.42, never above 7 in 480 ratings | |
| Stored facts contradicting ground truth | | 1.0% | 0.8-1.0% (the same 3-4 name mismatches) | 3.2% |
| Step teach lines stored right | | | | 440 of 626 (70%) |

**Twelve lessons**

1. **The representation bet was right.** Procedures, dialogue state and a curriculum turned "no help" into working help on taught tasks. The reasoner and the policy were almost never the cause of a failure.
2. **Tuning on visible transcripts overfits, even with general mechanisms.** The "extra ingredient" rule, written for "cheese omelette", produced "Sprinkle the bike over the tires" in 9 replies. A held-out set must be sealed. Its size and number of reads must be planned, not left to habit.
3. **Second encounter is mostly storage.** When the retry after teaching passed, the second encounter passed 97% of the time (476 of 489). When the retry failed, it passed 14% of the time (40 of 281). Measure storage directly, then retrieval given correct storage.
4. **Knowledge entry is the bottleneck, and the model should not do it.** The model wrote step ids, so "step 2" became step 1 (bake_potato failed 35 of 39 runs on one cached mistranslation). 31 of 110 procedures have a step with two actions. 30 procedures are stored under drifted names. The step id `make_pancake_s4` became a task and drew 90 curiosity questions.
5. **Task identity is the core skill.** "hang a painting" did not reach `hang_picture`. The rice request got the porridge recipe and the picnic request got "invite friends to dinner". Word overlap inside the translator prompt chose the task, with no proof and no confirmation.
6. **Requests must be detected by the agent, not hoped for from the model.** The model dropped the request act in 778 of 3,890 calls with a request cue. The same message got more than one translation in 71% of repeated messages. The 2.0 rescue (`implied_request`) works, but it is a dialogue decision made in the translator from raw text.
7. **Transfer needs adaptation knowledge, not more mechanisms.** All 6 variants that passed were item swaps. The 9 that never passed (0 of 351 encounters) need a time, method, step or warning that nothing taught. In 7 of the 9, the agent answered as if nothing had changed, for example a 7-9 minute boil for a soft-boiled egg.
8. **Taught wording is the biggest naturalness lever, but it broke faithfulness.** Storing the teacher's step wording raised the judge score from 4.72 to 5.25 and retention from 14 to 16. But it is checked only against the first four letters of the action. One test asserts a sentence with an `until` clause that is not stored.
9. **Metrics can pass by construction.** Examples:
   - 44% of scored turns are "Thanks!" answered with "Glad I could help.", labelled helpful;
   - "zero meta requests" counted scripted lines;
   - "zero duplicates" ignored procedures;
   - the contradicting-facts figure was a constant;
   - the symbolic check passed replies with 80% of the step order.
10. **The judges disagree in one direction.** Haiku never passed what the symbolic check failed (0 of 2,999). It failed 712 encounters the symbolic check passed. In the last runs the agent converged on the judge's words. Neither judge is enough alone, and the hand-label calibration was never done.
11. **Training never moved a held-out number.** Training tasks and held-out tasks shared no task id and no situation, and each held-out task was taught in its own first encounter. Training can only matter if what it teaches is reusable across tasks: aliases, cues, modifiers, frames.
12. **Caches make iteration cheap, and hide variance.** About 40 fix-and-measure cycles fit in two days. But a cached wrong translation fails every later run, and stability measured on warm caches overstates what a new user sees.

**What 2.0 put out of scope by user decision, and what 3.0 does with it**

| Item (2.0 decision) | 3.0 proposal | Why |
|---|---|---|
| Translation speed, 2 s median (11-17) | **Stays out.** Reported as a cost, with no target | Model calls stay on the Claude CLI, which adds about 1.3 s per call. 3.0 cuts calls instead of speeding them up: grammar-parsed teach lines and the cue layer remove many translator calls |
| Act accuracy, 95% on the gold set (11-17) | **Stays out as a model measure. Replaced by agent-side measures:** request recall on request-cue messages, and move stability over repeated cold calls | In 3.0 the agent owns the decisions that matter (request, task, step ids). Their accuracy is the agent's responsibility and can be fixed in the agent. The model's own act accuracy cannot be |
| Judge score target 6.0 (11-18) | **Comes back as a reported measure with an interval and a length-and-echo control, not as a target.** One research hypothesis tests the user's diagnosis directly (H4 below) | The write-up needs one overall quality measure that is comparable across v1, v2 and v3. A target would push toward plausible guessing, because the judge scores an honest "I don't know" as low as a wrong answer |
| Milestone 15 help-turn replay, 80% fitting moves (11-18) | **Comes back only as a regression guard with corrected labels**: "teach me" for an untaught request counts as fitting. No 80% target | 3.0 moves request detection into the policy, and the replay of 296 real v1 turns is the only check on untaught, messy input. Its old labels counted the right move as wrong content |

## Goals and principles

3.0 has one goal above the others: on tasks nobody tuned for, what is taught once is stored correctly, found again in other words, adapted to a changed situation when the adaptation was taught anywhere, and said truthfully when it was not.

**Goals**

- Store teaching exactly. Frame lines are stored as the ground truth says. Free-form teaching is read back and confirmed before it is believed.
- Find the right task, or ask. Never answer with another task's procedure without saying so.
- Adapt to changed situations from taught adaptation knowledge, including knowledge taught on other tasks.
- Notice an unknown change and say what is known and what is not.
- Say only what is believed. Faithfulness holds for help replies again.
- Measure on a sealed set, and publish the result with intervals and honest limits in a reproducible PDF.
- Watch training live in one lab.

**Principles** (2.0's nine principles stay; 3.0 adds five and sharpens two)

1. **Learned means used.** (2.0)
2. **No message is lost as a whole.** (2.0)
3. **Closed vocabulary, one door.** (2.0)
4. **Two memories.** (2.0) Personal facts stay in the session unless they are promoted.
5. **Knowledge is data, not code. Now enforced.** A test fails if an agent crate holds an English word list. Every heuristic is a lexicon or policy fact a teacher can correct.
6. **Everything is a hypothesis first.** (2.0) Now true for stated rules and aliases too.
7. **Say only what you believe, and say how sure. Now tested on help replies.** Every spoken clause maps to an atom in the reply's claims.
8. **An honest "teach me" is a designed reply.** (2.0)
9. **Measure before tuning.** (2.0)
10. **The model fills slots; the agent makes decisions.** The model may translate a free clause into atoms. Acts that change the move, task identity, step identity and ordering are decided by symbolic rules with proofs. The model's output is a hypothesis that the dialogue checks.
11. **The validator is the last gate.** No code after validation may set a part to "ok" or delete acts. Later stages can only add hypotheses, and those carry proofs.
12. **A sealed set is read rarely and only as counts.** Development uses the old 20 scenarios and the training set. The sealed set is read a fixed number of times, each read is logged, and nobody reads its transcripts before the final measurement.
13. **No metric may pass by construction.** Every target says how it could be gamed or met trivially and what test prevents that.
14. **A wrong claim costs more than an honest gap.** Measures and judges are set up so that "I was taught hard-boiled eggs; I don't know the time for soft-boiled ones" scores better than a confident wrong time.

**What stays from 2.0**

- the Rust workspace and the Markdown vault;
- the Datalog engine with proofs, quarantine, aggregates and moments;
- procedures as files;
- the segmenter and the three-tier translator with grounding and typing;
- the policy in Datalog;
- session memory;
- the curriculum and its encounter kinds;
- the oracle parser, mocks, stress, bench and calibrate;
- all archives in `data/training/`;
- the Haiku checklist judge as judge of record (unless the user decides otherwise, see Decisions).

## Architecture

The six stages of 2.0 stay. Three things change: where decisions are made, what each stage may do, and how a reply is built.

```
 message
   |
 1 Segmenter (as 2.0)
   |
 2 Understanding
   2a Frame grammar (new): teach frames, request cues, lack cues, item lists
      -> acts and atoms with no model call when a frame matches
   2b Patterns: fixed, then learned (shared in the brain, shadow first)
   2c Haiku for open parts only: fills atoms for free clauses; output is
      stripped of commentary and checked against few-shot constants
   2d Voting: a second call only when the first answer would change the move
   |
 3 Validator: the LAST gate (types, grounding, symbol check on every
   task and step symbol, glue, round trip). Nothing after it sets ok = true
   |
 4 Dialogue manager: every decision is a policy rule with a proof
   - request detection from cue facts plus acts      (was implied_request)
   - task resolution: alias, name, structure, score  (was word overlap)
   - change detection: qualifier, budget, weather, lack, modifier
   - move choice among all moves, including needs, duration and adapt
   - session state as facts (active task, step, open questions, said)
   |
 5 Reasoner: one cached model per (vault generation, session facts hash);
   new rules for adaptation (conditional durations, step replacement,
   modifiers, derivational replacement by effect)
   |
 6 Generation in two stages
   6a Content plan: atoms with roles (need, step, substitute, dropped,
      replaced, warning, tip, gap, question) -> the reply's claims
   6b Wording: realizer, verb frames from the lexicon, taught wording as
      templates linked to atom ids; a faithfulness check on the result
   |
 reply + claims + move proof -> turn trace -> event bus -> lab (SSE)
```

| Component | 2.0 | 3.0 |
|---|---|---|
| Teaching ingestion | The model translates each teach line and writes step ids | Frame grammar parses the frames. The agent allocates step ids from "Step N of X". The model fills only the free clause. Structure is validated and read back |
| Request detection | Model act, plus `implied_request` text rescue in the translator | Cue facts in the lexicon plus acts, decided by a policy rule with a proof. Compound questions get every asked part |
| Task identity | Word overlap in the prompt list and in `similar_task` / `involved_task` | `same_task` aliases, a symbol check, two-stage structural matching with a score and a proof, and a confirmation move below a threshold |
| Heuristics | About 78 `const` word lists and inline `matches!` lists in Rust | Typed lexicon predicates (`tool`, `container`, `surface`, `waiting_action`, `lack_cue`, `request_cue`, `state_verb`, `mass_noun`, `generic_verb`, verb frames) that the agent can learn and a teacher can correct |
| Adaptation | Item swaps: a substitute, analogy, the user's proposal, a dropped tool step, a hard-coded "sprinkle" step | Plus conditional durations, alternative and replaced steps, modifiers shared across tasks, warnings and reasons, change detection, an honest gap and one ranked question |
| Help reply | `task_help` builds text with `format!` and string edits, then the text is post-filtered | Content plan, then wording, with claims taken from the plan and a faithfulness check on help replies |
| Belief | Stated rules confirmed at 0.9; hedges never spoken; "Yes" at any confidence | Stated rules are hypotheses; hedges are spoken; yes/no answers follow confidence |
| Session model | Uncached; 3-4 full evaluations per help turn | Cached by session-facts hash; one evaluation per turn |
| Concurrency | Learned patterns and the registry copied per Agent (lost updates); trainer and curriculum in two processes | Patterns, registry and translation cache owned by the brain; one lab process |
| Evaluation | 20 held-out scenarios read many times; official rate includes "Thanks!" turns | A sealed set of 60 scenarios plus development and training sets; metrics on the request turn; a strict symbolic check; a user simulator |
| Dashboard | Polls every 2 s and re-reads files | Live push over SSE with resume, one process, drill-down, teach diffs, run comparison |

## Knowledge model

The 2.0 model stays: flat Datalog in Markdown, numbers with units, fact ids, the typed registry, and procedures as files. 3.0 adds three groups of predicates and fixes the parts of 2.0's model that were declared but not used.

**Adaptation knowledge** (case-based reasoning calls this the adaptation container; REPORT_DESIGN_2 section 18)

| Predicate | Example | Used for |
|---|---|---|
| `takes_time_if(Step, Cond, Lo, Hi, Unit)` | `takes_time_if(boil_egg_s3, soft_boiled, 5, 6, minute)` | Conditional durations (soft-boiled egg, brown rice) |
| `instead_of_step(Cond, Old, New)` | `instead_of_step(has_spare_tube, fix_flat_tire_s3, fix_flat_tire_s3b)` | Replacing a step in a situation (a spare tube instead of patching) |
| `skip_if(Step, Cond)` | `skip_if(fix_flat_tire_s4, has_spare_tube)` | Dropping steps in a situation |
| `alternative(Task, Cond, Method)` with `method(Method, Task)` | `alternative(bake_potato, short_of_time, microwave_then_oven)` | A second method chosen by the situation |
| `achieves(Step, Effect)`, `because(Step, Reason)` | `achieves(do_laundry_s5, dry(laundry))` | Derivational replacement: a step whose effect is still needed is replaced by another way to reach that effect, for example tumble dry when there is no rack |
| `done_when(Step, Cond)` | `done_when(cook_rice_s4, absorbed(water))` | Completion tests (replaces glued `until` text) |
| `warns(Step, Hazard)` | `warns(water_plant_s4, standing_water)` | Warnings |
| `tip(Task, TipId)` | | Tips, spoken with their taught wording |
| `modifier(Situation, Pattern, Change)` | `modifier(rain, outdoor_step, move_indoors)`; `modifier(hand_luggage_only, liquid_item, liquids_bag)`; `modifier(surprise, inform_guest_step, keep_secret)` | Situations taught once and applied to any task whose steps match the pattern |
| `qualifier(Task, Q)` / `variant_of(Q, Task)` | `variant_of(soft_boiled, boil_egg)` | Knowing that a qualifier names a variant of a taught task |

Patterns such as `outdoor_step` or `liquid_item` are small typed classes in the lexicon (`outdoor(Action)`, `liquid(Item)`). That keeps a modifier from applying outside its pattern.

**Identity knowledge**

- `same_task(T1, T2)`: an alias. Learned from a teacher's confirmation ("Yes, hanging a painting is hanging a picture"). It can also be proposed by the agent from shared needs and step structure, and then it stays a hypothesis.
- `task_phrase(Task, PhraseId)`: how the task was named when it was taught, with the original case kept.
- `same` now propagates. Core rules carry facts across `same` for entities and `same_task` for tasks. In 2.0, facts about `knife` never reached `knive`.

**Quoted detail atoms** (the fix for unchecked taught wording)

A teach line often carries a detail that the action-object atoms cannot hold, such as "into lights and darks", "at least two weeks ahead" or "with your finger". 3.0 keeps such a detail as a stored atom, not as loose text:

- `detail(Step, PhraseId)` holds the phrase. `phrase(PhraseId, "into lights and darks")` holds its exact text, with case kept.
- The atom is created at teach time, read back to the teacher ("Step 1 of doing laundry: sort the laundry into lights and darks. Right?"), and stored with the step's source and confidence.
- The reasoner never reasons over the phrase's content. It is quoted knowledge, and it is said only as part of its step.
- Faithfulness is then checkable. Every content word of a spoken step maps either to an action, object, tool, time or condition atom of that step, or to one of its `detail` atoms.

**Session layer as facts**

`active_task`, `at_step`, `open_question`, `said`, `offered` and `asked_lack` become facts passed to the policy. Today they are Rust fields. As facts, policy rules can mention the step under way, what was already said and the open question. The question stack becomes a real stack of `open_question(N, Q)` facts.

**Declared but unused, resolved.** Every predicate declared in the core is either used by code and tested, or removed. This covers `slot`, `grounded`, `surface`, `plural_of`, `same_pred`, `portion`, `staple` and `recommended_for`. A registry audit test lists each declared predicate with its readers.

**Integrity constraints** (checked by the validator before storing, and by an audit over the whole vault)

- one `action` per step;
- `before` and `overlaps` only between steps of the same task that exist;
- no task id that is a step id or carries a step suffix;
- `until` and `done_when` values are registered conditions, never glued clauses;
- every `detail` phrase belongs to a step that exists.

## Understanding input

**Teach frames parsed by a grammar.** The fixed frames of the teacher protocol are parsed symbolically. These are the 2.0 frames plus the new ones (see "Learning and teaching"):

- "To X you need A, B and C."
- "X takes A to B minutes."
- "Step N of X: …"
- "While step K runs, step N of X: …"
- "B can replace A."
- "If C, step N takes A to B minutes."
- "If C, instead of step N: …"
- "Warning for step N of X: …"
- "Tip for X: …"
- "Step N of X is done when …"

The grammar supplies the task phrase, N, K, the condition C and the free clause. The agent allocates the step id from the task and N. The model is called, if at all, only on the free clause after the colon. It may fill only `action`, `object_of`, `uses`, `until`/`done_when` and `detail` for that one step, and it never writes an id. A frame line the grammar cannot parse is still sent to the model, but its atoms are marked unverified until the read-back is confirmed.

**Cue layer for requests.** Request cues are lexicon facts:

- `request_cue("what are the steps")`, `request_cue("where do i start")`, `request_cue("is there a way around")`, `request_cue("how do i")`;
- `lack_cue("doesn't allow", before)`, `lack_cue("nowhere to {action}", step_tool)`.

They are matched on tokens, not substrings, so "no" no longer matches inside "piano" and "allow" no longer matches "shallow". A cue match produces a `heard_cue(PartId, Cue)` session fact. The model's acts produce `act(PartId, …)` facts. The policy decides from both, with a proof. The translator no longer rewrites parts or deletes acts.

**Compound questions.** "What do I need and how do I cook them?" produces two asked parts. The content plan answers both: needs and steps.

**Cleaning model output.** Commentary lines ("Wait, that's not right …") are stripped before parsing. A constant that appears in a few-shot example but not in the message (`office`, `maksim`) is rejected. In 2.0 this happened in 148 and 10 answers.

**Symbol check.** Every task symbol is checked against known tasks, aliases and the active frame. A step symbol is checked against the steps of the active task. A symbol that is a known task plus a suffix (`make_pancake_s4`) is rejected. So is one that contains a pronoun or manner word (`make_one_properly`, `change_bulb_safely`). The agent then asks instead of inventing.

**Translator stability.**

- *Fewer calls.* Teach frames and cue-covered requests often need no call.
- *Canonical context.* The prompt carries a structured summary of the agent's last reply (its move, task and asked slot), not its raw text. The cache key uses the same summary, so a wording change no longer invalidates every later turn.
- *Failed or rejected translations are never cached as good.* A rejected answer is stored with its rejection, so the same wrong translation is not replayed in every run. This fixes the bake_potato freeze.
- *Voting only where it matters.* If the first answer would change the move (request or not, lack or not, which task), and the cue layer does not settle it, a second cold call is made. If the two calls disagree, the agent asks a short confirmation question.
- *Learned patterns* live in the brain. They count distinct sentences as evidence, not cache replays. They run in shadow until they agree with the model on 20 distinct uses at 95%.
- *Every model call records its model id.* A change of Haiku version between runs is then visible.

## Dialogue

**Every decision is a policy rule.** The direct paths of 2.0 become moves in `policy.md` with priorities and proofs:

- needs, duration and adapt answers;
- the implied request;
- resume;
- the reply filters ("no check-back beside task help", "no stray apology").

The `Act::Note` free-text type is limited to social lines and errors. Curiosity questions and rule answers get structured acts.

**Task resolution with a proof.** A request goes through four stages:

1. Exact name or alias.
2. A lexical filter: name words, needs and objects. This is the cheap retrieval stage.
3. A structural match on actions, objects, effects and situation words, scored by rules.
4. A threshold:
   - above it, the task is used, and the proof names why;
   - between two thresholds, the move is `confirm_task`: "Do you mean planning a picnic? I know how to do that.";
   - below the lower one, `teach_me` with the user's own task phrase.

Every choice can be explained with "why".

**Honest closest option.** When the closest known task is not the one asked for, the move `closest_known` says so: "I don't know how to boil an egg yet. I know how to make scrambled eggs, if that helps." It has lower priority than `teach_me`. A seed's procedure is never presented as the answer to another task. In 2.0 this happened in 25 of 180 first encounters.

**Change detection.** When the request names a task the agent knows, a policy rule compares the situation with the procedure. It flags:

- a qualifier on the goal ("soft-boiled", "brown", "surprise", "hand luggage only");
- a time budget below the total;
- weather against a step with an `outdoor` action;
- a lack of a needed item or of a step's tool, including "nowhere to {action}";
- any modifier whose pattern matches a step.

What follows depends on what is known:

| Situation | Move | Example reply |
|---|---|---|
| Known adaptation (`takes_time_if`, `instead_of_step`, `alternative`, a modifier, a substitute) | `adapt` | The adapted procedure as a diff: "Same as usual, but boil the egg for only 5 to 6 minutes." |
| Item missing, no substitute | `ask_replacement` (2.0's strong move) | "You don't have a kettle: what do you have that could heat the water?" The missing item is not listed as needed, and no step uses it before the question |
| Change detected, no adaptation known | `honest_gap` | "I was taught hard-boiled eggs (7 to 9 minutes). I don't know the time for soft-boiled ones." It files a curiosity goal named in the user's words: `gap(boil_egg, soft_boiled, takes_time)` |
| A qualifier that no taught fact depends on ("a big potato") | none | No flag. The qualifier counts as relevant only when a taught fact or modifier depends on it, which keeps false alarms down |

**Typed guards.** Every mechanism that edits a procedure checks types from the lexicon. This covers substitution, analogy, dropping, adding, replacing and modifiers. Only a food goes into a dish. An "extra" item cannot be the task's own object or tool. "Juice of X" needs a juicy X. The hard-coded "Sprinkle …" step is removed. An added ingredient needs a taught `add_in(Task, Item, Before)` or a modifier.

**Smaller fixes from the transcripts.**

- "You still need X" only when the user said what they have (it appeared in 94 of 789 help replies).
- A real question stack: a side remark no longer drops a pending yes/no.
- Typed slot filling: `available(bike)` does not fill "ingredients".
- Verb frames from the lexicon instead of "make" for every task.
- No cooking-only fallback text.

## Reasoner and storage

The engine stays. The changes close 2.0's gaps between design and code.

| Change | Fixes (2.0) |
|---|---|
| The session model is cached by (generation, session-facts hash, now interval); one evaluation per turn on the help path | 3-4 full uncached evaluations per help turn with session facts |
| An index on every bound argument position used by joins | First-argument index only; joins on a bound second argument scan |
| Procedure facts cached per procedure generation, not cloned on every model build | `procedure_facts()` and `ProcedureFile::facts()` cloned inside loops |
| Analogy through an index of (action, item) to steps | Analogy scanned every step of every procedure |
| Confidence from the best derivation | First derivation found |
| New core rules: conditional duration selection, step replacement and skipping by condition, modifier application by pattern, derivational replacement by `achieves` | No way to represent a time, method or step change |
| Failure ledger and log read incrementally (an index kept in memory, appended on write) | `asked_today` and `goals()` re-read whole files every turn |
| Facts written only through the knowledge base (journal, id, `@conv`), including `proper_name` | `learn_names` bypassed the log and ids |
| Disk writes moved off the brain lock: the brain queues them and a writer thread writes them in order | The whole symbolic turn, including file I/O, held the lock |
| Byte-safe string handling on every symbol (no `&s[..4]`) | A possible panic under the brain lock on a non-ASCII action |

**Budget.** Under 100 ms at the 95th percentile for the agent's own work, measured on the help path with session facts on a 20,000-fact vault. In 2.0 it was measured only on the 296 v1 turns, which rarely have session facts.

## Generating replies

**Content plan, then wording.** `task_help` (about 440 lines with its helpers) is split.

- **Content plan.** A list of atoms with roles: `need`, `need_replaced(new, old)`, `step`, `step_replaced`, `step_dropped`, `step_added`, `warning`, `tip`, `time`, `gap`, `question`, `note(source)`. The plan is pure data. It is computed by rules where possible, and it is the reply's claims.
- **Wording.** The realizer turns the plan into text:
  - verb frames with prepositions per role from the lexicon (VerbNet-style);
  - taught wording as templates whose slots are linked to atom ids;
  - the user's framing ("the right way to …"), from the request, recorded as an echo of the user, not as a claim;
  - step times only on waiting steps (`waiting_action` facts).
- **No string edits on finished replies.** The `replacen`, `swap_word` and prefix filters go away. A substitute is swapped into the plan's atoms before wording, so "fill the jug at the tap" happens only when the atoms say so.

**Faithfulness for help replies.** The v1 test extends to help replies. Every content word maps to an atom in the claims, and every claim is in the plan. The 2.0 test `steps_keep_their_taught_wording`, which asserts an `until` that is not stored, is rewritten so that it fails on unsupported text. Dropped sentences take their claims with them.

**Wording kept as taught, with case.** Taught wording is stored from the original message text, not from the lowercased `heard_text`. Wording is updated or invalidated whenever a step's atoms change by any route.

**Seeds reviewed.** The 30 seed procedures are re-read. Their broken wording is fixed ("Ask the allergy with the phone", "Send the invitation to the guest list and the phone"), and they get taught wording too.

**Ceiling, kept as a measurement.** The LLM-rewrite ceiling (preferred in 78.4% of pairs in milestone 16) is re-run once on the final build. It measures the remaining wording gap and is never shipped. Runtime LLM rewriting (REPORT_DESIGN_2 option D3) stays out, because it would break the principle.

## Learning and teaching

**Teacher protocol 3.0.** Haiku stays the teacher, held to the ground truth.

1. **Ground truth is richer.** Each scenario's ground truth holds needs, steps and times, plus at least one of each knowledge type the checklist asks for: a warning, a tip or detail, a completion test and, for variants, the adaptation. The scenario author writes it. The teacher only phrases it.
2. **Frames for every knowledge type** (see "Understanding input"). One action per step line. A compound line ("dice the tomatoes and tear the basil") is split by the scenario linter before any run.
3. **Elicitation.** After the base teaching, the agent asks structured questions: "What can go wrong?", "How do I know it's done?", "Is there another way if …?". The teacher answers from the ground truth in frames. This is knowledge elicitation (Cooke), driven by the agent's own gaps, so the agent learns to ask for what checklists expect.
4. **Read-back.** Every stored step is read back in the frame's own words. The teacher's fix pass compares the read-back with the ground truth and reteaches only what differs.
5. **A student-aware fix pass.** The teacher sees the agent's stored atoms for the task and the failed probe, and teaches only the missing knowledge type.
6. **The variant teacher teaches situations, not only substitutes.** It uses the conditional frames and modifiers.
7. **Modifiers on purpose.** The training set teaches a set of shared situations on several training tasks each, as near-miss pairs (the same task with and without the situation). The situations are rain, surprise, hand luggage only, whole grain, no room to dry, short of time, a spare part, soft versus firm and group size. The sealed set then tests the same situations on other tasks.
8. **Checker.** Every teach line is checked symbolically against the ground-truth atoms. Lines that leave the ground truth are dropped.

**Teaching in free form, measured apart.** Fixed frames make storage close to correct by construction. So 3.0 also measures storage on teach lines that Haiku paraphrases out of frame, from the same ground truth. Those take the model path and are believed only after read-back. Storage accuracy is reported for both.

**Learning cycle fixes.**

- Stated rules start as hypotheses (`lib.rs:1130` stores them confirmed at 0.9).
- A procedure is confirmed by a second teacher, or by a user who reports doing the task with success. One accepted recommendation is not enough.
- Renames by `similar_task` become alias proposals (`same_task` hypotheses), never silent renames. In 2.0, silent renames split or merged 6 of 75 scenarios (`plan_birthday_party` partly became `plan_baby_shower`).
- Hygiene covers procedure ids and task aliases, not only fact entities.
- Curiosity asks in the user's or teacher's words, after the symbol check, so the teacher can answer. A `gap` goal from change detection is asked as "How is it different for brown rice?".
- The spaced queue re-asks a failed scenario only after a reteach whose read-back was confirmed. Otherwise it reports "taught, not stored" and does not waste the encounter (2.0: 6 of 254 passed).

## Training lab

The user asked for this (PROGRESS.md "Notes for Design 3.0"). The runner, the vault writer and the dashboard become one process, `sla_lab`, which replaces `sla_trainer` and the spawned `sla_eval curriculum` child. Events are pushed live.

**Event stream.**

- Every event has a run id, a monotonic sequence number and a type.
- Event types:
  - message, parts, acts, validator result, cue facts;
  - task resolution with score and proof;
  - move with proof, content plan, reply, claims, faithfulness result;
  - teach line, frame parse, stored atoms, read-back, fix;
  - question asked, question answered;
  - vault diff;
  - judge verdict, symbolic verdict, outcome label;
  - model call (role, model id, seconds, cache hit);
  - plan limits;
  - alert.
- Events go to `data/training/lab/<run>/events.jsonl`, kept forever with the other training data, and to an in-memory ring.
- The UI subscribes over server-sent events (`GET /api/stream?after=N`, with `Last-Event-ID` resume). It stops polling. The disk write happens off the brain lock.
- Each run record stores the git commit, a dirty-tree flag, the binary's hash, the vault's starting fingerprint, the scenario set's hash and the model ids. In 2.0 the run-to-code mapping had to be inferred from commit order.

**Panels** (React and Vite as in 2.0; each panel arrives with the milestone that produces its data)

| Panel | What it shows |
|---|---|
| Live conversations | One lane per teacher; each agent turn with its label, move and failing stage |
| Scenario grid | Encounters by scenario, filling in live; development, training and sealed sets apart. Sealed cells show counts only |
| Teach log | Teach line, frame parse, stored atoms, read-back, fix and vault diff, side by side with the ground truth. This was the chain the 2.0 analyses had to rebuild by hand from three files |
| Turn drill-down | Message, parts, cue facts, acts, validator, task resolution with proof, move with proof, content plan, claims, reply, verdicts |
| Question stream | Curiosity and elicitation questions, who answered, and what was stored |
| Translator health | Tiers, frame-grammar share, votes, disagreements, rejections, cache hits, seconds per call |
| Knowledge health | Integrity counts (multi-action steps, dangling order facts, step-ids-as-tasks), aliases, hypotheses, queue, personal facts in the vault |
| Teacher quality | Knowledge types covered per task, checker rejections, fix lines per scenario |
| Run comparison | Two runs side by side on the same scenarios: verdict flips, reply differences, metric deltas with intervals |
| Plan limits and cost | Session and weekly limits, calls per role, the 95% / 70% automatic stop (kept) |

**Controls.**

- Start and stop stay buttons the user presses. A run spends plan limits, and training runs need the user's go-ahead.
- Pause after a turn, then step through it.
- A `--mock` mode plays every model role, so the lab can be tried without spending anything.
- **Sealed guard.** A run on the sealed set needs an explicit flag and a confirmation, writes a read-ledger entry, and shows only counts until the set is unsealed.

**Security** (kept and tightened). Token and origin check. The token is accepted from the query string only once and then moved to a header or cookie. Token comparison is constant-time.

## Evaluation

**Sets**

| Set | Size | Who writes it | Use | Who may read transcripts |
|---|---|---|---|---|
| D20 (development) | The 20 old held-out scenarios, 15 variants | existing | Development, regression | Anyone |
| T (training) | The 55 old training scenarios plus about 30 new ones that teach shared modifiers as near-miss pairs | a sub-agent from a spec | Training runs | Anyone |
| H3 (sealed) | **60 new scenarios, 45 with variants**: 15 item swaps, 15 cross-task modifiers (situation taught only on T tasks), 15 novel changes (nothing anywhere teaches them; an honest gap is the right answer). Each scenario has 3 request wordings, including idioms ("jacket potato", "put up a painting") | a sub-agent, from a written spec only, without sight of the code, D20 transcripts or T | Baseline, one optional checkpoint, final | Nobody until the final measurement; counts only |
| P-dev, P-H3 (paraphrase sets) | 150 requests each, naming D20/T tasks or H3 tasks in other words, with distractor seeds | same authors | Task-match accuracy, request recall | P-dev anyone; P-H3 sealed with H3 |
| Regression | 296 real v1 turns with corrected labels | existing | Guard (the M15 replay, corrected) | Anyone |
| Gold sets | 200 translator messages, 50 long messages | existing | Translator health, reported | Anyone |

H3 task ids are disjoint from T, D20 and the seeds. Seeds close to H3 tasks are allowed, because in real use they are distractors. The H3 spec, its hash and the author's prompt are committed before H3 is written. The set's files are committed with a SHA-256 manifest. Their transcripts are archived in `data/training/` like all training data and never deleted, but marked sealed.

**Read budget.** H3 is read at most four times before the final measurement:

1. the v2.0.0 baseline;
2. the 3.0 baseline after milestone 19;
3. one optional checkpoint after milestone 23;
4. the final runs (two replicates and the ablations count as one read, because nothing is changed between them).

Every read is logged in `data/training/h3_reads.jsonl` with the commit. A failure-class count per read is allowed. Transcripts are not. This is a discipline, not a technical lock: Claude Code can read any file. The build brief therefore makes it a working rule, and the write-up reports the read ledger.

**User simulator.** 2.0 replayed the scenario's `situation` lines whenever the reply ended with "?", whatever was asked. In 3.0, each situation line is tagged with what it answers (`answers: available`, `answers: time_available`, `answers: replacement`). The simulator answers the slot the agent's move actually asked, from the move's structured data. If nothing answers that question, it says "I'm not sure." The simulator is symbolic and deterministic. A Haiku in-role user is an optional mode, measured apart.

**Judges**

- **Judge of record:** the Haiku checklist judge (2.0 user decision, kept unless changed). Checklist items are now tagged by knowledge type and linked to ground-truth atoms. A scenario linter checks that every item is supported by a teach line or marked as an honesty item before any run. In 2.0, water_plant's "warns against standing water" had no support and failed all 39 runs.
- **Strict symbolic check, reported beside it.** It requires:
  - all needs;
  - 100% step order;
  - no extra or invented step;
  - the right time;
  - every detail atom of the checklist present;
  - no claim that contradicts the ground truth.

  The 83 2.0 replies that passed with a missing or wrong step are its negative tests.
- **Honesty scoring.** For each help reply, the symbolic check counts false claims (claims that contradict the ground truth) and honest gaps (`gap` atoms where the ground truth has no answer). Both are reported. A false claim is counted as worse than a gap.
- **Controlled judge score** (reported, no target). The 1-10 score is reported with an interval over scenarios. A length-and-echo control fits a regression of score on reply length and on echoed user words, and reports the residual. This follows Length-Controlled AlpacaEval. 2.0's probes showed +1 to +1.5 points for echoing the user and -1 for per-step times with identical content.
- **Agreement.** Cohen's kappa between the two judges, and a hand-read sample of disagreements. The 2.0 hand-label sheet was never filled in. 3.0 proposes 60 encounters labelled by the user for the write-up (open decision). Without them, the write-up states that the judge was not validated against people.
- **Self-preference check** (open decision). Haiku translates, teaches, simulates and judges. A second judge from another Claude model through the CLI, run on a 100-encounter sample, would show how much of the verdict is Haiku judging Haiku's English.

**Metrics and proposed targets** (on H3 unless noted; targets are fixed once after the milestone 19 baseline, then frozen)

| Group | Metric | 2.0 value | Could it pass by construction? How 3.0 prevents that | Proposed 3.0 target |
|---|---|---|---|---|
| Storage | Teach lines stored exactly as the ground truth, frame lines | 70% of step lines (T) | Yes, frames make it easy. So the next row is reported too | ≥ 98% |
| Storage | The same, Haiku-paraphrased free-form teaching, after read-back | not measured | No: the model path and the read-back are exercised | ≥ 90% |
| Storage | Vault integrity: multi-action steps, dangling order facts, step-ids-as-tasks, split tasks | 42 / 2 / 1 / 6 of 75 | Audited over the whole vault, procedures included | 0 / 0 / 0 / 0 |
| Learning | Second encounter, Haiku checklist | run 1: 6 of 20; training stage: 31-38% | Sealed set, counts only | ≥ 70%, and better than v2.0.0 on H3 by a paired exact McNemar test |
| Learning | Second encounter, strict symbolic | not measured | Strict check with negative tests | ≥ 80% |
| Learning | Second encounter given correct storage | 97% (476 of 489) | Decomposition reported | ≥ 95% |
| Learning | Retention | run 1: 5 of 20 | | ≥ 70% |
| Transfer | (a) item swaps (15) | 6 of 6 swap variants on D20, tuned | | ≥ 70% |
| Transfer | (b) cross-task modifiers (15), taught only on T tasks | not measurable in 2.0 | The modifier is never taught on an H3 task | ≥ 40% |
| Transfer | (c) novel changes (15): an honest gap, and not answered as if unchanged | 7 of 9 answered as if unchanged | Strict check counts false claims | Honest gap ≥ 80%; answered as unchanged ≤ 10% |
| Understanding | Request recall on request-cue messages, after the cue layer | 80% (model acts only) | Measured on P-H3, written without the cue list | ≥ 98% |
| Understanding | Task-match accuracy on P-H3 | not measured; 193 wrong-task replies in 39 runs | Paraphrases written independently; distractor seeds | ≥ 90% |
| Understanding | Wrong-task help replies per 100 | about 6 in C9 | | ≤ 1 |
| Understanding | Move stability: the same message, 3 cold calls, a different move | not measured; 71% of repeated messages had more than one translation | Cold cache, 100 D20 and T messages | ≤ 2% |
| Dialogue | Helpful + fitting clarification on the request turn (social turns reported in their own row) | run 1: 10.1%; dev: 58-63% | Thanks turns and bare "Okay." excluded | ≥ 65% |
| Dialogue | Another task's procedure given as the answer before teaching | 25 of 180 (C9) | | 0 |
| Dialogue | Mechanism misfires (type-breaking insertions or swaps) | 13 replies (C9) | Symbolic type check over every adapted plan | 0 |
| Faithfulness | Help replies with a content word that maps to no claim | not tested; known to happen | Faithfulness check on every help reply | 0 |
| Faithfulness | False claims per 100 help replies (contradict the ground truth) | not measured | Strict check | ≤ 1 |
| Knowledge | Distinct false beliefs in the vault, after alias normalisation | the same 3-4 name mismatches; 3.2% on T | Counted as distinct false atoms per scenario, not as a share of a large denominator | ≤ 1% on H3 and T |
| Curiosity | Questions the ground-truth teacher can answer | 9 of 67 (T), 71 of 407 (held-out) | | ≥ 80% |
| Overall | Pairwise preference over v2.0.0 on H3, both orders | not measured; 2.0's pairwise against milestone 12 was weak | Both orders, same teaching | ≥ 65% |
| Overall | Judge score, controlled | 5.10-5.42 on D20, tuned | Reported with interval | no target |
| Guard | Regression replay with corrected labels: fitting moves | 24.6% with the old labels | "teach me" for untaught requests counts as fitting | no regression from the 3.0 baseline |
| Guard | D20: second encounter, transfer, retention | 17-18 / 6 / 17 | | within one scenario of the M19 3.0 baseline after each milestone |
| Cost | Calls per encounter; seconds per call, median and p95 | 3.0 s median, 7.1 s p95 | | reported, no target (CLI) |
| Engineering | Agent time p95, help path, 20,000 facts | not measured on this path | Bench with session facts | ≤ 100 ms |
| Lab | Event latency p95 under the 16-teacher mock stress; events lost after a reconnect | 2 s polling | | ≤ 500 ms; 0 |

**Size and intervals.** With 60 scenarios, a 70% rate has a Wilson 95% interval of about 57-80%. With 15 variants per transfer type, 40% has an interval of about 20-64%. So the transfer subtypes are reported with their intervals and are not over-read. A paired comparison over the same H3 scenarios (v2.0.0 against v3) is much stronger than either level alone. It is the main test.

**What the baseline will probably show.** v2.0.0 on H3 will probably land closer to its training-stage figures (31-38% by Haiku, about 75% by the symbolic check) than to the D20 figures. The targets above are proposals until that baseline is read. They are then fixed once and written into the pre-registration file.

**Pre-registration.** Before the first H3 read, `research/PREREG_3.md` is committed. It holds the hypotheses, metrics, targets, statistical tests, read budget and the exact final-run protocol. The write-up compares the result against it and lists every deviation.

**Ablations in the final measurement** (each one H3 run on the frozen build, mostly on warm caches)

1. The oracle parser in place of the translator. This isolates understanding from everything after it.
2. Base teaching only, with the rich frames and elicitation off. This tests H4, the user's diagnosis that the teacher sets the ceiling.
3. A fresh seeded vault against the vault trained on T. This measures the training effect, which was 0 in 2.0.
4. Taught wording off. This measures the wording lever on a clean set.

## Final research write-up

This is the project's closing deliverable: one PDF of about 20-30 pages, built with LaTeX from the repository.

**Research question.** Can an agent whose reasoning, memory, learning and replies are all symbolic, with a language model only translating input (and, in training, teaching, simulating users and judging), learn to help with everyday tasks from conversation? Here "learn to help" means that what is taught once is stored correctly, used later in new words, adapted to changed situations, and stated truthfully.

**Hypotheses** (fixed in the pre-registration)

| | Hypothesis | Test |
|---|---|---|
| H1 | Taught knowledge is reused on tasks nobody tuned for | Second encounter and retention on H3, both judges, with intervals |
| H2 | Storage, not retrieval, limits reuse | Storage accuracy; second encounter given correct storage; v2.0.0 against v3 on H3 |
| H3 | Adaptation knowledge taught on some tasks transfers to others | Transfer (b) on H3 against the fresh-vault ablation |
| H4 | The teacher sets the ceiling of helpfulness | Rich teaching against base teaching on the same build: checklist pass and controlled judge score |
| H5 | A symbolic agent can stay faithful and honest | Unsupported clauses, false claims, honest gaps on novel changes |
| H6 | 2.0's development numbers were partly overfit | v2.0.0 on D20 against H3 |

**Contents**

1. Abstract.
2. Introduction and research question.
3. Related work, condensed from REPORT_DESIGN_1 section 7 and REPORT_DESIGN_2 section 18. Topics: interactive task learning (Rosie, VAL, PUMICE), case-based adaptation (CHEF, the adaptation container, derivational analogy), machine teaching and elicitation, faithful data-to-text, LLM-as-judge biases, and reusable-holdout discipline.
4. The system across three iterations: what each had and why it changed.
5. Method:
   - scenarios and encounters;
   - the teacher protocol and the user simulator;
   - the judges;
   - the sealed set and its read ledger;
   - the pre-registration;
   - statistics: Wilson intervals; exact McNemar for paired scenario outcomes; bootstrap over scenarios for means; run-to-run variance from replicates.
6. Results:
   - v1 (help turns, judges);
   - v2: the clean run 1 against development numbers, and v2.0.0 on H3;
   - v3 on H3, every metric in the table above, with intervals, both judges and both replicates;
   - the ablations;
   - transfer by type;
   - the storage/retrieval decomposition;
   - judge agreement;
   - cost in calls.
7. Failure analysis of the final H3 transcripts, after unsealing, with the same taxonomy as REPORT_DESIGN_2 section 14.
8. Discussion: what symbolic-only can and cannot do, and where the boundary sits (translation, teaching cost, wording).
9. Limits and threats to validity. All of these are stated, none softened:
   - the 2.0 contamination history;
   - one model family in every role;
   - a model-written scenario set and ground truth;
   - English only;
   - fixed teach frames;
   - small samples per transfer type;
   - a single developer agent tuning on D20;
   - CLI and model-version drift;
   - no human-labelled validation unless the user labels;
   - quoted detail atoms that are not reasoned over.
10. Conclusion.
11. Appendices:
    - reproduction (commands, commits, binary hashes);
    - data manifest with SHA-256 sums;
    - prompts by hash (translator, teacher, judge, simulator);
    - the scenario spec;
    - the read ledger;
    - user decisions across all three iterations;
    - the full pre-registration and its deviations.

**How it is produced**

- `paper/` holds the LaTeX sources: `main.tex`, one file per section, `refs.bib`, and figures as `pgfplots` over CSV, so there is no plotting dependency outside TeX.
- `sla_eval paper` (new, no model calls) reads only archived data: `data/training/**`, `eval/**` and the git history of run reports. It writes:
  - `paper/generated/numbers.tex`, one `\newcommand` per number, each with a comment naming its source file, the record and the file's SHA-256;
  - `paper/generated/*.csv` for tables and figures;
  - `paper/generated/manifest.tsv`.
- Results sections use only generated macros and tables. A test (`paper_numbers_are_generated`) fails if a results `.tex` file has a digit outside a macro, table input or citation. A second test recomputes every macro from the data and compares. This prevents the 2.0 kind of error, where the judge score 3.58 was attributed to the wrong run.
- `make -C paper` (latexmk, or tectonic) builds `paper/charlotte_final.pdf`. No TeX toolchain is installed on this machine today (none of pdflatex, latexmk, tectonic or pandoc was found). Installing one is a user decision.
- **Reproduction has three levels, each stated in the paper:**
  1. **Numbers from data:** `sla_eval paper` plus `make -C paper` reproduce every number and the PDF exactly, with no model calls.
  2. **Transcripts from caches:** with the archived caches, the frozen build replays the final runs to identical transcripts and verdicts. This is 2.0's `replay_is_repeatable` property, extended to the lab.
  3. **A fresh re-run:** new model calls give new transcripts. The paper reports the replicate spread as the expected variation.
- **Review.** An independent read-only sub-agent checks every claim in the draft against the generated numbers and the data, as the 2.0 report's analysts did. The user reads and approves the final text.

## Every 2.0 failure, its fix and its test

Codes: **M** measurement, **K** knowledge and learning, **C** conversation, **E** engineering. Sources are REPORT_DESIGN_2 sections (for example 11.3 is section 11, part 3) and PROGRESS.md.

**Measurement**

| 2.0 failure | Fix in 3.0 | Test |
|---|---|---|
| M1 Held-out set contaminated from run 2 on; only run 1 is clean (11.3) | H3 written from a spec, sealed, a read budget, counts only, a read ledger; D20 becomes development | Manifest hash unchanged; ledger shows at most 4 reads; the write-up reports the D20-H3 gap |
| M2 "Helpful + clarify" counts 44% "Thanks!" turns and bare "Okay." (11.4, 14.5) | Request-turn rate is the headline; social turns in their own row; "Okay." labelled by context | Labeller unit tests; the official metric excludes both |
| M3 Targets met by construction: meta requests (scripted lines), duplicates (procedures ignored), contradicting facts (a constant), hygiene (9.2, 11.15) | Meta requests counted only on Haiku-written lines; duplicates over procedures and aliases; false beliefs as distinct atoms after alias normalisation | Injected duplicates, meta requests and false beliefs are detected in a seeded test vault |
| M4 Symbolic check too lenient (passes 80% step order; 83 flawed replies passed) (14.5) | Strict symbolic check | The 83 known flawed replies fail it; the 2.0 "Sprinkle the fruit" reply fails it |
| M5 Judges disagree one way; the agent converged on the judge's words; no human calibration (11.8) | Checklist items linked to atoms; both judges reported; kappa; user hand labels (open decision); optional second-model judge | Agreement table in every report; anchor transcripts give stable verdicts |
| M6 Second encounter mostly measures in-session storage (11.10) | Storage accuracy and retrieval-given-storage reported apart | Both appear in every curriculum report |
| M7 The translation cache hides variance and freezes wrong answers (14.5) | Cold-cache run per milestone; rejected answers never cached as good; move-stability metric | A rejected translation is not replayed (unit test); a cold run per milestone in the ledger |
| M8 Small samples; one scenario = 5-7 points (2.8) | 60-scenario H3, replicates, intervals, paired tests | The report generator refuses a rate without an interval |
| M9 Hand-label calibration never done; calibration covers only runs 2-6 (11.8) | A 60-encounter hand-label sheet for the user, or the limit is stated | The write-up has a judge-validation section either way |
| M10 "Not understood" counted one template (11.4) | Label-based count | Labeller test over the 132 non-template cases |
| M11 Spaced queue ineffective, 6 of 254 (11.2) | Re-ask only after a confirmed reteach; otherwise report "taught, not stored" | Spaced-pass rate and the "taught, not stored" count reported |
| M12 Pairwise against milestone 12 was weak (the M12 build failed every request) (11.14) | Pairwise against v2.0.0 on H3, both orders | Run in milestone 19 and in the final measurement |
| M13 A judge-score label was wrong in the reports (11.2) | Numbers generated from data with sources; run records carry their commit | `paper_numbers_are_generated`; run records have commit hashes |
| M14 Training never moved a held-out number (11.2, 14.4) | Shared modifiers and aliases taught on T; the fresh-vault ablation | The training effect appears as its own result |
| M15 No training run used the final 2.0 code (10) | The final training run uses the frozen build | The ledger shows training and final runs on the same commit |
| M16 Fixed situation lines sent whatever the agent asked (9.2) | User simulator answers the slot actually asked | A test: a wrong question gets "I'm not sure", not the next line |
| M17 Learned-pattern coverage never re-measured after training (11.15) | Measured after the final training run, in shadow | Reported in the final measurement |
| M18 Checklist items without teach support (water_plant item 3), ambiguous items, compound step lines (14.3) | Scenario linter | 0 linter flags before any run |

**Knowledge and learning**

| 2.0 failure | Fix in 3.0 | Test |
|---|---|---|
| K1 30% of step teach lines stored wrong; the model wrote step ids; bake_potato step 2 stored as step 1 in every vault (14.2, 14.4) | Frame grammar; agent-owned step ids; read-back; the fix pass compares the read-back | Storage accuracy ≥ 98% on frame lines (T and H3); the bake_potato teach block stored correctly from a cold cache |
| K2 42 multi-action steps in 31 procedures; dangling `overlaps`/`before` (11.13) | Integrity constraints at the door and in an audit | Vault audit 0/0 after the final training run |
| K3 Teach lines not understood (42 of 55 `not_understood` entries) (11.13) | Frame grammar | 0 unparsed frame lines over every teach line of D20, T and the new frames |
| K4 Name drift: 30 procedures under ids that match no ground truth; 6 of 75 scenarios split across task names (11.13, 14.4) | The task id comes from the frame's task phrase by a deterministic rule; renames become `same_task` hypotheses | 0 split tasks; alias resolution on P-dev |
| K5 A step id became a task (`make_pancake_s4`, 90 questions) (11.13) | Symbol check | Suffixed and manner-word symbols are rejected (unit tests from the 68 `unknown_task` entries) |
| K6 Taught wording not checked against facts; faithfulness broken (9.5.2) | Quoted detail atoms; wording linked to atom ids; faithfulness check on help replies | The rewritten `steps_keep_their_taught_wording` fails on an unstored `until`; 0 unsupported clauses on H3 |
| K7 The "extra ingredient" step had no fact behind it (9.5.2) | Removed; replaced by taught `add_in` or modifiers, typed | Negative suite ("bike", "fruit" in a fruit smoothie) |
| K8 Stated rules stored confirmed at 0.9 (9.2) | Hypothesis with the hedge kept | Rule-status audit test |
| K9 Hedges never spoken; yes/no ignores confidence; a 0.6 procedure answers "Yes" (9.2) | The realizer speaks hedges; yes/no follows confidence | A weak fact gets "I think so"; a hedge test set |
| K10 One accepted recommendation confirms a procedure (9.3.5) | Confirmation by a second teacher or a reported success | Test: an acceptance alone leaves it a hypothesis |
| K11 About 78 English word lists in code; registry flags never read (9.5.1) | Lexicon predicates; flags read | A test fails on English string arrays in agent crates; flags used by validator and learner |
| K12 Session and lexical layers declared, unused (9.2) | Session state as facts; unused predicates removed | Registry audit test: every declared predicate has a reader |
| K13 `same` does not propagate; hygiene ignores procedures (9.2, 11.13) | Propagation rules; hygiene over procedures and aliases | Facts about `knife` reach `knive`; drifted procedure ids are found |
| K14 `proper_name` bypasses log and ids; ordinary words as names; 10 personal facts in the trained vault (9.3.1, 11.13) | Through the knowledge base; session-only by default | Personal facts in the vault = 0 after training unless promoted |
| K15 No representation for a changed time, method, step, warning or reason (9.6) | Adaptation knowledge model | All nine 2.0 never-passing variants solved with hand-written oracle knowledge (shows the mechanisms suffice) |
| K16 Curiosity asked under wrong names; 17% answered (11.13, 14.1 F14) | Asked after the symbol check, in the user's or teacher's words; `gap` goals from change detection | ≥ 80% answerable on T |
| K17 Learned patterns: lost updates, replays counted as evidence, no shadow (9.3.4) | Owned by the brain; distinct sentences only; shadow mode | Concurrency test with 16 sessions; replay does not raise `seen` |
| K18 Consolidation's shared steps unused; re-tests not built (9.2) | Shared steps become `subtask` proposals in the queue; re-tests are not built and the claim is dropped | Design text and code agree (doc check) |
| K19 `similar_task` silently renamed into other tasks (14.4) | Alias proposals only | Test: "plan a birthday party" is never stored as `plan_baby_shower` |

**Conversation**

| 2.0 failure | Fix in 3.0 | Test |
|---|---|---|
| C1 The model dropped the request act in 20% of request-cue calls (14.1 F3) | Cue layer plus a policy rule | Request recall ≥ 98% on P-H3 |
| C2 Two-part questions answered in part (pancakes: 34 of 39 retentions failed) (14.1) | Every asked part answered from the content plan | Test on the pancake message; compound-question cases in P-dev |
| C3 Task not found for a synonym ("hang painting") (14.1 F4) | `same_task`, structural matching, `confirm_task` | Task-match accuracy ≥ 90% on P-H3 |
| C4 Wrong task matched: 193 help replies named another task (porridge, invite friends) (14.1 F5) | Scored resolution with a proof and thresholds | Wrong-task replies ≤ 1 per 100 |
| C5 A seed's procedure given as the answer before teaching, 25 of 180 (14.1 F13) | `closest_known` move | 0 on H3 |
| C6 Changed situation answered as if unchanged, 7 of 9 variant types (14.3) | Change detection; `honest_gap` | Transfer (c) honest gap ≥ 80%, unchanged answers ≤ 10% |
| C7 Over-general mechanisms: "Sprinkle the bike", "juice of the vinegar" (14.1 F12) | Typed guards | Mechanism misfires = 0; negative tests from the 13 replies |
| C8 Transfer reply listed the missing item and steps using it before asking (14.2.3) | `ask_replacement` first, or steps with a gap | Test on the pancake and tea variants |
| C9 "You still need X" when the user listed nothing (94 of 789) (14.6 R10) | Only after the user stated items | Count = 0 when no items were given |
| C10 Model commentary and copied few-shot constants in translations (14.1 F16) | Stripping and the example-constant check | Unit tests from the 148 and 10 cases |
| C11 Translation variance: 71% of repeated messages got more than one translation (14.1 F10) | Frames, cues, canonical context, voting on move-changing parts | Move stability ≤ 2% on 3 cold calls |
| C12 Dialogue decisions in the translator: `implied_request` and `lack_cues` set `ok = true` and delete acts (9.5.3) | Policy moves with proofs; the validator is the last gate | A property test: no stage after the validator changes `ok` or removes acts |
| C13 String coupling: the failure ledger and outcome labels parse reply text; `implied_request` reads the last reply (9.5.3) | Consumers read the reply's move and intent | Test: changing every template's wording leaves curiosity goals and labels unchanged |
| C14 Replies assembled and filtered as text; claims include dropped sentences (9.1, 9.5.8) | Content plan, then wording | Claims equal the plan of the spoken sentences (test) |
| C15 Needs, duration, adapt and resume answers have no proof (9.2) | Policy moves | Every turn trace has a move proof (test over the regression set) |
| C16 Lack cues match substrings ("piano", "shallow") (9.5.8) | Token-level cue facts | Negative tests |
| C17 One pending slot; any message drops a pending yes/no (9.3.5) | Question stack as facts | Resume test after two side remarks, with a pending yes/no |
| C18 Any session fact fills a slot (9.3.5) | Typed slot filling | `available(bike)` does not fill "ingredients" |
| C19 "make" for every task; cooking-only fallback text (9.3.5) | Verb frames and task phrases from the lexicon | "You could make fix flat tire" impossible (test) |
| C20 Broken seed wording ("Ask the allergy with the phone") (14.2.5) | Seed review; verb frames; taught wording for seeds | 0 frame errors in the read-back of all 30 seeds |
| C21 Taught wording lowercased (9.5.2) | Wording from the original text | Case test ("Berlin") |
| C22 A possible panic on non-ASCII action prefixes under the brain lock (9.5.2) | Character-safe handling | Test with a non-ASCII action |

**Engineering**

| 2.0 failure | Fix in 3.0 | Test |
|---|---|---|
| E1 Session model uncached; 3-4 evaluations per help turn; the 100 ms budget never measured on this path (9.3.2) | Cache by session-facts hash; one evaluation per turn | `one_model_per_message` extended to help turns; bench at 20,000 facts ≤ 100 ms p95 |
| E2 Disk I/O under the brain lock; trainer and curriculum as two processes on one vault (9.3.5, 9.5.5) | Writer thread; one lab process | Stress: 16 teachers, 10 minutes, 0 corrupted lines, 0 lost updates in every whole-file store |
| E3 A wording change invalidated the translation cache for every later turn; the cache file parsed per Agent (9.5.6) | Canonical context in prompt and key; one in-memory cache in the brain | Test: a template wording change keeps later cache keys |
| E4 Failed translations remembered only in memory, retried every session (9.5.6) | Persisted with their rejection | Unit test |
| E5 A panicking curriculum thread's encounters vanished silently (9.3.8) | Error event, run marked incomplete | Test with a panicking mock |
| E6 Log and failure ledger re-read whole per turn (9.5.7) | Incremental indexes | Bench: turn time flat as the ledger grows to 100,000 lines |
| E7 First-argument index only; procedure facts cloned; analogy steps² (9.5.7) | Indexes and caches as in "Reasoner and storage" | Bench at 20,000 facts and 500 procedures |
| E8 Dead code and stale docs (Windows toolchain, PROGRESS M17, `tasks.rs` module docs) (9.5.8, 9.5.10) | Removed and updated | `cargo clippy` with dead-code warnings denied in agent crates; a doc check in each milestone's acceptance |
| E9 Dashboard polls every 2 s, re-reads files, has no tests (8, 9.3.8) | Lab with SSE | Latency and reconnect tests under mock stress; unit tests for the event reducer |
| E10 Dashboard panels designed but not built: question stream, teach log, translator health, drill-down, compare runs, step mode (8) | Lab panels by milestone | Each panel checked against the API in the milestone that brings its data |
| E11 Token in the query string, plain string compare (9.3.8) | One-time query token, then header or cookie; constant-time compare | API tests |
| E12 A killed background build made a run measure an old binary (3) | Foreground builds; binary hash in each run record | Run records carry the binary hash |
| E13 Run-to-code mapping had to be inferred from commit order (11) | Commit, dirty flag, binary and set hashes in each run record | Test on the run record schema |
| E14 Model version not recorded per call | Model id in every call record | Schema test |

## Milestones

Eight milestones, numbered on from 2.0's eighteen. Measurement comes first. Then the lab core, so every later cycle is faster. Then structure, knowledge entry, understanding, adaptation and teaching. Last comes the frozen final measurement with the write-up. Runs that spend model calls are marked **[run]**. Each needs the user's go-ahead (see Decisions).

**19. Honest ground: evaluation v3**

Build:

- H3 spec, H3 and P-H3 written by a sub-agent from the spec only; T extension with shared modifiers; P-dev;
- the manifest and read ledger;
- the scenario linter;
- metric changes: request turn, social row, label-based "not understood";
- the strict symbolic check, honesty counts, the controlled judge score, kappa;
- the user simulator with tagged situation lines;
- run records with commit, binary, set and model hashes;
- the report generator with intervals and paired tests;
- `PREREG_3.md` draft.

Accept:

- One command produces every metric with intervals for D20, T and H3. On H3 it gives counts only.
- The 83 known flawed 2.0 replies fail the strict check.
- The linter flags water_plant item 3 and the compound T lines, and passes H3 with 0 flags.
- The same commit run twice on warm caches gives identical numbers.
- **[run]** A cold-cache D20 run of v2.0.0, and the v2.0.0 baseline on H3 (read 1).
- Targets fixed in `PREREG_3.md` and committed before read 2.

**20. Training lab core**

Build:

- `sla_lab` as one process;
- event ids, SSE with resume, the event archive in `data/training/lab/`;
- the writer thread off the brain lock;
- learned patterns, registry and translation cache owned by the brain;
- panic reporting;
- panels: live conversations, scenario grid, turn drill-down, teach log (frame parse, stored atoms, read-back), plan limits;
- the sealed guard;
- the token fixes.

Accept:

- Event latency p95 ≤ 500 ms under the 16-teacher mock stress for 10 minutes.
- 0 lost events after a forced reconnect, 0 corrupted lines, and 0 lost updates in patterns, queue and stats.
- A panicking mock thread marks the run incomplete.
- A sealed run cannot start without the flag and shows only counts.
- All 2.0 tests pass.

**21. Knowledge is data, and every decision has a proof**

Build:

- the word lists moved to typed lexicon predicates;
- registry flags read;
- `implied_request`, the direct paths, resume and the reply filters moved into the policy;
- session state as facts and a real question stack;
- consumers read moves, not reply text;
- `task_help` split into a content plan and wording, with claims from the plan;
- the session model cached;
- belief fixes: rule hypotheses, hedges spoken, confidence in yes/no, procedure confirmation;
- dead code and stale docs removed.

Accept:

- The English-word-list test passes (no lists in agent crates).
- The registry audit passes.
- Every turn of the regression set has a move proof.
- Changing every template's wording leaves curiosity goals and outcome labels unchanged.
- The validator-is-last-gate property test passes.
- Help turns use one evaluation.
- Bench at 20,000 facts ≤ 100 ms p95 on the help path.
- **[run]** D20 cold and warm: second encounter, transfer and retention within one scenario of the v2.0.0 D20 numbers. The move is behaviour-preserving.

**22. Teaching ingestion and vault integrity**

Build:

- the frame grammar for all frames;
- agent-owned step and task ids;
- the free-clause slot filling by the model;
- integrity constraints;
- read-back and the fix pass on the read-back;
- quoted detail atoms with wording linked to atoms;
- the faithfulness check on help replies;
- rejected translations never cached as good;
- alias proposals instead of renames;
- hygiene over procedures.

Accept:

- 0 unparsed frame lines over all D20 and T teach lines.
- Storage accuracy on frame lines ≥ 98% on T from a cold cache.
- The bake_potato block is stored right.
- The vault audit gives 0 multi-action steps, 0 dangling order facts, 0 step-ids-as-tasks and 0 split tasks after a T teaching pass.
- The rewritten taught-wording test fails on unstored text, and 0 unsupported clauses appear on D20.
- **[run]** Haiku-paraphrased teaching on T: storage after read-back ≥ 90%.

**23. Understanding requests and tasks**

Build:

- the cue layer with token matching;
- request detection as a policy rule;
- compound questions;
- commentary and example-constant stripping;
- the symbol check;
- `same_task`, two-stage structural matching with scores and proofs;
- `confirm_task` and `closest_known`;
- canonical prompt context and cache keys;
- voting on move-changing parts;
- learned patterns in shadow;
- curiosity naming.

Accept:

- Request recall ≥ 98% and task-match accuracy ≥ 90% on P-dev.
- A reasoner suite of at least 25 task-identity cases with proofs passes. It includes "hang a painting", "jacket potato", and "picnic with friends" against the dinner seed.
- 0 seed procedures given as answers before teaching on D20.
- Move stability ≤ 2% on 100 messages × 3 cold calls **[run]**.
- Curiosity answerable ≥ 80% on a T run **[run]**.
- Optional H3 checkpoint (read 3), counts only.

**24. Adaptation knowledge and change detection**

Build:

- the adaptation predicates with registry entries, frames, vault format and realizer support;
- the reasoner rules: conditional duration, step replacement and skipping, alternatives, modifier application by pattern, derivational replacement by `achieves`;
- change detection;
- the moves `adapt` (as a diff), `ask_replacement`, `honest_gap`;
- `gap` curiosity goals;
- typed guards on every mechanism;
- removal of the hard-coded "sprinkle" step.

Accept:

- 100 procedure files with the new predicates round-trip.
- At least 40 reasoner cases with proofs pass. They include all nine 2.0 never-passing variants solved with hand-written oracle knowledge.
- A negative suite of 20 cases: no modifier applies outside its pattern, and there are 0 type-breaking insertions. The suite includes the 13 C9 misfires.
- On D20 variants, honest gaps replace every "as if unchanged" answer where nothing is known.

**25. Rich teaching and the training run**

Build:

- the teacher protocol 3.0: elicitation questions, frames for every knowledge type, the student-aware fix pass, the variant teacher for situations, near-miss pairs for modifiers;
- teacher-quality and question-stream panels;
- the remaining lab panels: translator health, knowledge health, run comparison, step mode.

Accept:

- **[run]** One full training run on T with 16 Haiku teachers (with the user's go-ahead). After it:
  - at least 80% of T tasks have their warnings, completion tests and taught adaptations stored and validated;
  - false beliefs ≤ 1%;
  - vault audit 0/0/0/0;
  - curiosity answerable ≥ 80%;
  - learned-pattern coverage and agreement measured in shadow.
- The D20 guard holds.

**26. Final measurement and research write-up**

Build:

- freeze the build (tag `v3.0.0-rc`);
- the final protocol from `PREREG_3.md`;
- `sla_eval paper`, the `paper/` sources and the generated numbers;
- the ceiling re-run;
- unsealing after the final runs;
- the H3 failure analysis;
- the PDF;
- the independent claim review.

Accept:

- **[run]** Read 4:
  - two H3 replicates on the frozen build, cold then warm;
  - the four ablations;
  - pairwise against v2.0.0;
  - the controlled judge score;
  - the corrected regression replay.
- Every metric in the evaluation table is reported against its pre-registered target, with intervals and deviations listed. Missed targets are reported as missed.
- `sla_eval paper` and `make -C paper` reproduce the PDF from data with no model calls.
- `paper_numbers_are_generated` and the recompute test pass.
- The claim review finds no unsupported statement.
- PROGRESS.md, the README and this design's "Decisions" are current.
- Tag `v3.0.0`. The user pushes.

**Rough cost** (estimates from 2.0's figures, not measurements):

- a cold 60-scenario H3 run: about 1,700-2,000 calls, because 2.0's cold 20-scenario run 1 used 583;
- a warm H3 rerun: about 300-600;
- the training run on about 85 T scenarios with elicitation: about 3,000-4,000;
- move stability: about 300.

3.0 in total is likely 12,000-18,000 calls, against 12,626 in 2.0. Grammar-parsed teaching lowers the translator's share.

## Build brief for Claude Code

Written for an autonomous Claude Code run, as in 2.0. 3.0 is built inside this repository on a new branch from `design-2` (the user names it).

**Working rules**

- Work through milestones 19 to 26 in order. Do not start one until the previous one passes its acceptance tests. Commit after each, with a message naming it. Do not push. The user adds remotes and pushes.
- Run `cargo test` after every change. Never leave the build broken at a commit. Build release binaries in the foreground before any run.
- **Every run that spends model calls starts only with the user's go-ahead.** That covers training, the sealed set, cold-cache runs, voting and stability measurements. Before asking, state its expected call count. The automatic stop at 95% of the session and 70% of the week stays.
- **Haiku is the teacher, the simulated user and the judge of record. All model calls go through the Claude CLI.** A second judge model is used only if the user decides so.
- **Keep all training data.** Every call, conversation, run record, event log and vault copy goes to `data/training/` and is never deleted, sealed runs included.
- **The sealed set.** Never open H3 or P-H3 transcripts, records beyond counts, or their scenario files after they are written, until milestone 26 unseals them. Every H3 run is logged in the read ledger. Fixes are found on D20 and T only.
- Keep `PROGRESS.md` current as part of every acceptance: current milestone, what works, what failed, decisions and why.
- End every milestone with a short report: D20 and T metrics before and after, with intervals.
- When stuck for long on one approach, write the problem in `PROGRESS.md`, choose the simplest working fallback, and move on.
- Use sub-agents for side work: the H3 spec and set, P sets, the T extension, the final claim review. Test data is written from a spec alone, without sight of the code.

**Code layout**

- `crates/translate`: the frame grammar and the cue matching. Model calls move here from `sla_parser` (`claude.rs`, `llm.rs`). This finishes 2.0's unbuilt layout item.
- `crates/lexicon`: every typed word class, frames per verb, cues, task phrases.
- `crates/agent`:
  - `dialogue.rs` holds only move execution;
  - `plan.rs` (new) builds the content plan;
  - `tasks.rs` shrinks to procedure queries;
  - `ingest.rs` (new) holds teaching ingestion and read-back.
- `crates/nlg` realizes content plans, help replies included, and reads verbs from `sla_lexicon`.
- `crates/eval`:
  - `lab.rs` replaces `trainer.rs`;
  - `simulator.rs`, `strict.rs` and `paper.rs` are new;
  - `curriculum.rs` loses its dashboard file re-reading.
- `paper/`: LaTeX sources and generated numbers.

**Definition of done for 3.0**

- Milestones 19 to 26 pass their acceptance tests.
- The final H3 measurement is reported against the pre-registration with intervals, met or not.
- The PDF builds from data by one command.
- `PROGRESS.md` is current.

## Known limits and risks

| Risk | Why it stays | How 3.0 holds it down |
|---|---|---|
| One model in every role (translator, teacher, simulator, judge) | The user's constraint: Haiku teaches, through the CLI | A symbolic user simulator; the strict symbolic check beside the judge; an optional second-model judge sample; stated in the write-up |
| The sealed set is model-written, as is its ground truth | No human author is available | Spec and prompt committed first; ground truth reviewed by the linter; "model-written world knowledge" stated as a limit |
| Sealing is a discipline, not a lock | Claude Code can read every file | Read ledger, manifest hash, the brief's rule, the ledger in the write-up |
| Frames make storage accuracy close to correct by construction | Fixed frames are the protocol | Free-form paraphrased teaching measured apart |
| Fixed frames make teaching rigid; it is teaching by protocol, not free conversation | Deterministic ingestion is the main fix of 2.0's worst failure | The model path stays for free-form lines, believed after read-back; reported separately |
| Quoted detail atoms are knowledge the reasoner cannot use | Phrases like "into lights and darks" have no useful predicate | Said only with their step; faithful because stored and read back; counted in the write-up |
| Change detection false alarms | Qualifiers are everywhere ("a big potato") | Relevant only when a taught fact or modifier depends on them; false-alarm count reported on D20 |
| Modifiers over-apply | Pattern classes are coarse | Typed patterns; a negative suite; modifiers are hypotheses until used with success |
| Targets may be out of reach on a clean set | 2.0's clean numbers were low, and the training stage shows 31-38% | Targets fixed after the baseline; missed targets reported as missed. The research value is in the measured gap |
| Small samples per transfer type (15) | Each H3 scenario costs calls | Intervals always shown; the paired v2-v3 test is the main comparison |
| What the final read reveals cannot be fixed inside 3.0 | Sealed-set read budget | The checkpoint read (optional) after milestone 23; the write-up's failure analysis says what a next step would be |
| Model and CLI drift between runs | External | Model id per call; replicates; the drift is reported |
| Scope: eight milestones | Many parts | Each milestone has its own tests; the lab speeds cycles; options B (bulk import) and D3 (runtime rewrite) stay out |
| Haiku English | Teachers, simulator, judge and scenario author write similar English | Idioms in P sets; the corrected v1 regression replay of real turns as a guard |

**Out of scope for 3.0** (proposed):

- bulk import of outside knowledge (REPORT_DESIGN_2 option B);
- runtime LLM rewriting of replies (option D3);
- offline paraphrase mining (D2) beyond taught-wording templates;
- forgetting;
- fully incremental evaluation;
- one grammar shared by parsing and generation;
- messages the agent starts by itself;
- small talk as a measured skill;
- translation speed;
- model act accuracy as a target.

## Decisions

**Constraints carried over (set by the user, kept as written)**

| Constraint | In 3.0 |
|---|---|
| Training runs start only with the user's go-ahead | Every run that spends calls is marked [run] and waits for the go-ahead |
| Haiku is the teacher | Haiku teaches, phrases free-form teaching, and plays the in-role user when that mode is on |
| Model calls stay on the Claude CLI | Translator, teacher, judges and any second model go through the CLI |
| All training data is kept in `data/training/` and never deleted | Lab event logs, sealed runs and vault copies included |
| The user adds git remotes and pushes | Claude commits per milestone on the user's branch; never pushes |
| The real-time training lab comes after 2.0 | Milestones 20 and 25 |
| Out of scope in 2.0: speed, act accuracy, judge score, M15 replay | Speed and act accuracy stay out. The judge score returns as a reported measure without a target. The M15 replay returns as a guard with corrected labels |

**Proposed defaults in this draft** (each is also listed below as an open decision)

| Question | Proposed default | Why |
|---|---|---|
| Size and authorship of the sealed set | 60 scenarios (45 variants in three types of 15) plus P-H3, written by a Claude Code sub-agent from a committed spec | Larger intervals than 2.0 cannot support conclusions; a spec-only author keeps it independent of the code |
| Read budget | 4 reads: v2.0.0 baseline, 3.0 baseline, optional checkpoint, final | Reusable-holdout discipline (Dwork et al.); enough to catch overfitting before the final read |
| Judge of record | Haiku checklist judge, as decided in 2.0, with the strict symbolic check reported beside it | Continuity with 2.0; the strict check guards against both judge failure directions |
| Judge score | Reported with an interval and a length-and-echo control, no target | Needed for comparison across iterations; a target would reward guessing |
| M15 replay | Regression guard with corrected labels, no target | Checks untaught, messy input after request detection moves to the policy |
| Outside knowledge import | Out | Out of 3.0's scope; it weakens the "learned from conversation" claim; licences |
| User simulator | Symbolic, deterministic; Haiku in-role user as an optional measured mode | Reproducible, cheap, and it tests the question actually asked |
| The write-up | LaTeX in `paper/`, numbers generated by `sla_eval paper`, PDF committed with the sources | Reproducible from data with no model calls |

## Open decisions for the user

1. **Sealed set.** Are 60 scenarios (45 variants: 15 item swaps, 15 cross-task modifiers, 15 novel changes) and 150 paraphrase requests the right size for the budget? Should a Claude Code sub-agent write them from a spec, or do you want to write or review the spec yourself?
2. **Read budget and gating.** Is a budget of four H3 reads acceptable, with the optional checkpoint after milestone 23? Which runs need your go-ahead: only training and sealed runs, or every run that spends calls (also D20 cold runs, voting and stability tests)? A per-session call cap for development runs is an alternative.
3. **Total budget.** Is an estimated 12,000-18,000 Haiku calls for 3.0 acceptable, spread over several sessions?
4. **Judge of record.** Keep the Haiku checklist judge as the judge of record, or make the strict symbolic check co-primary for the final write-up?
5. **Human validation.** Will you hand-label about 60 encounters for the write-up's judge validation? Without it, the paper states that the judges were not validated against people.
6. **Second judge model.** May a different Claude model, through the CLI, judge a 100-encounter sample to measure Haiku's self-preference? This adds about 100-200 calls of a larger model.
7. **Judge score and M15 replay.** Agree that the judge score returns as reported-only (no target) and the M15 replay as a corrected regression guard? Or keep both fully out?
8. **Outside knowledge.** Keep bulk import out of 3.0? Alternatively, add a small, separately reported arm (for example a teacher that reads how-to pages for tips and warnings).
9. **Teaching protocol.** Is it acceptable that the scenario author writes rich ground truth (warnings, tips, conditions) and Haiku only phrases it? Should free-form, out-of-frame teaching be measured as described?
10. **User simulator.** A symbolic simulator by default, with Haiku in-role as an optional mode? Or Haiku in-role as the main mode?
11. **The write-up.** Who is it for (a thesis chapter, an internal report, a public preprint)? What length? Whose names go on it? Should the PDF be committed? Which TeX toolchain should be installed (tectonic or TeX Live with latexmk)? None is installed now.
12. **Branch and tags.** Which branch should 3.0 be built on, and should milestones be tagged (`v3.0.0-rc`, `v3.0.0`)?
13. **Autonomy.** Should 3.0 run autonomously between go-aheads, as 2.0 did with `/goal`, or stop for review after each milestone?
14. **Scope cuts if time runs short.** In what order should things be dropped? Proposed: lab extras (step mode, run comparison) first, then the Haiku-paraphrased teaching measure, then the optional checkpoint. The sealed final measurement and the write-up are never cut.
15. **The trained vault.** After the final run, should the trained vault be promoted to `vault/` (outside git, a copy kept in `data/training/`)?
