# Symbolic Learning Agent – Design 3.0 (draft)

Nov 18, 2026 · @Maksim

**Status of this document.** This is not the final Design 3.0. It is (1) the plan for **phase A**, design and clarification, and (2) a **provisional phase B**, build and test, which phase A confirms or changes. Phase A ends with the final Design 3.0 document (Markdown). There is no plan PDF; the only PDF is the research PDF at the end of phase B (user decision).

- Parts marked **[decide in A]** are proposals, not decisions.
- Unmarked parts follow from decisions or constraints the user has already set.
- The companion summary for weighing the options is `research/DESIGN_3_options.md`.

## Summary

Design 3.0 is the third research iteration. It is not planned as the last one: if it goes well it may be the final iteration, otherwise its results feed a 4.0. It keeps the core rule of v1 and 2.0: reasoning, memory, learning and every reply are symbolic, and a language model only translates input and plays teacher, reviewer and data generator in training. Haiku is the teacher, and model calls stay on the Claude CLI.

2.0 showed that the symbolic core can learn a task once and reuse it later in new words, when the teaching is stored correctly. It also showed three things that 3.0 must change:

1. **Too much of the behaviour was programmed, not learned.** The late gains of 2.0 came mostly from English word lists and special cases in Rust: 78 lists and about 225 strings in the help path alone. They were tuned on the held-out transcripts.
2. **The judges pulled the work toward polish.** Haiku marked down wording and format, so effort went into framing echoes, prepositions and taught wording. The last of these broke the faithfulness rule.
3. **The knowledge itself was thin and badly entered.**
   - 30% of teach lines on new tasks were stored wrong.
   - Task names drifted.
   - Nothing could hold how a situation changes a procedure.

So 3.0 is about **an agent that actually learns, reasons and answers**. It makes five changes:

1. **Learning replaces hard-coded heuristics.** No English word lists in agent code. What the lists did is derived from knowledge or learned from teaching and examples, as hypotheses with evidence.
2. **The agent owns its knowledge entry.** It allocates step ids, checks task identity, validates structure and reads back what it stored. The model fills slots; it does not decide ids, tasks or acts.
3. **Adaptation knowledge becomes a first-class kind of knowledge.** Conditional times, alternative methods, step replacement by effect, warnings, and situational modifiers taught once and applied across tasks.
4. **Knowledge comes from dialogue, from an example dataset, or both.** Whether a symbolic learner trained on a generated dataset generalises is the central research question. It is tested by held-out wording, situations, tasks and a whole domain.
5. **Evaluation measures learning, not presentation.**
   - A reply passes when it contains every fact and every relation the answer needs, and nothing wrong or unsupported. This is checked against the ground truth.
   - Grammar and fluency are not measured.
   - All numbers come from a fresh, sealed held-out set.

Human-like answering is out of scope for 3.0 and moves to the "Later" section. 3.0 ends with a research PDF that summarises all three iterations so far, with every number generated from the repository's data.

## What 2.0 taught us

All numbers are from REPORT_DESIGN_2 (2026-11-18). Three kinds of number exist. Only the first is clean.

| Measure | Clean held-out (run 1) | Development set (contaminated) | Untuned training stage (55 new tasks) |
| --- | --- | --- | --- |
| Second encounter, Haiku checklist | 6 of 20 (30%, 15-52) | 15-18 of 20 | 19 of 55 (35%) |
| Second encounter, lenient symbolic check | | 18-20 of 20 | 43 of 55 (78%) |
| Transfer | 0 of 15 | 6 of 15, item swaps only | 0 of 18 |
| Retention | 5 of 20 | 14-17 of 20 | 11-13 of 55 |
| Helpful + clarify on the request turn | 10.1% | 58-63% | |
| Teach step lines stored correctly | | | 70% (440 of 626) |
| Contradicting facts | 1.0% | 0.8-1.0% (the same 3-4 name mismatches) | 3.2% |

Ten lessons follow.

1. **Procedures plus dialogue state was the right representation.** It took the agent from 0 real answers to answering taught tasks in new words.
2. **The second encounter mostly measures storage.** When the retry after teaching passed, the second encounter passed 97% of the time; when it failed, 14%. Remembering and rewording are nearly solved. Entering the knowledge is not.
3. **Task identity is the core skill.**
   - 30 procedures were stored under drifted names.
   - 193 help replies named another task.
   - A step id became a task, and curiosity asked about it 90 times.
4. **The model made decisions it should not make.**
   - It wrote step ids.
   - It chose task names from a list of 15.
   - It dropped 20% of requests.
   - 71% of repeated messages got different translations.
5. **Code heuristics produced the late gains and the misfires.** Lack cues, how-cues, tool, container and waiting-verb lists, and the "extra ingredient" rule gave transfer 0 → 6. They also produced "Sprinkle the bike over the tires". Nothing learned them, no teacher could correct them, and no proof shows them.
6. **Transfer needs adaptation knowledge.** All five working mechanisms reuse knowledge; none invents it. 9 of 15 variants (time, method, plan) passed in none of the 39 runs.
7. **The agent was blind to changed situations.** In 7 of the 9 failing variant types it answered as if nothing had changed.
8. **Style judges steer the work.** The 1-10 score rose from 3.6 to 5.4 through wording alone, and echoing the user's framing alone gave +1 to +1.5. Honest gaps scored like wrong answers.
9. **Targets met by construction are not results.** Meta requests (scripted lines), duplicates (procedures not checked), contradictions (a constant), "helpful + clarify" (44% of turns were "Thanks!").
10. **Training never moved a held-out number.** The tasks were disjoint, and no reusable knowledge was taught. A curriculum that cannot transfer cannot show generalisation.

## Goals and principles

3.0 has one goal above the others: **knowledge learned once, from teaching or from examples, is used correctly later: in new words, in a new situation, on a new task, and in a new domain.**

**Goals**

- Answer everyday task requests ("how do I", "what do I need", "how long", "what next", "I don't have X") with complete and correct content.
- Learn procedures, adaptation knowledge, task identities, word classes and cue patterns from dialogue and from example data.
- Notice when a situation differs from what it was taught, and say what it knows and what it does not.
- Explain every belief, answer and dialogue move with a proof.
- Stay deterministic, traceable and hand-editable.

**Principles**

1. **Learned, not coded.** Agent code holds the formal language, the teaching-channel frame grammar, segmentation and generic mechanisms. Words and facts about the world are data with a source: taught, derived or learned. A start-up seed is allowed, but it is data, counted, and ablated in the final measurement.
2. **The agent owns its knowledge.** Ids, task identity, act decisions and structure checks are the agent's. Model output is a hypothesis that the agent checks.
3. **Learned means used, and used correctly.** Nothing counts as learned until a later answer uses it correctly, on data nobody tuned for.
4. **Every reply carries its content in full.** The text must state every fact the answer needs and every relation between them. Rough text is acceptable. Missing or unsupported content is not.
5. **An honest gap beats a plausible guess.** A wrong claim costs more than "I don't know". Metrics score them apart.
6. **Everything is a hypothesis first.** Rules, aliases, modifiers, lexicon entries, patterns and imported or generated items start on trial and earn confirmation from evidence counted over distinct examples.
7. **No target met by construction.** Every metric has a negative control that shows it can fail.
8. **Measure on sealed data.** The held-out set is written from a spec, sealed by hash, read only as counts, and read a fixed number of times.
9. **Every number is reproducible.** Each number in either PDF is generated from files in `data/` and `eval/` by a script that calls no model.

**The output requirement.** A reply's text is a faithful rendering of its content plan, nothing more. Grammar errors, odd word order and missing commas or connectives are fine. The text must express:

- each needed item;
- each step;
- the order and overlaps of the steps;
- which item or tool goes with which step;
- each time or quantity on the right step;
- conditions ("until", "if", "when") and reasons;
- substitutions ("X instead of Y");
- the adaptation to the user's situation;
- honest gaps.

"Need jug instead of watering can. 1 check soil dry with finger. 2 fill jug with water. 3 pour water on soil with jug until drains. 4 empty saucer, warning standing water." passes.

**What stays from 2.0.**

- The Rust workspace, the Markdown vault and the Datalog engine with proofs.
- Facts that close instead of being deleted.
- The single brain per vault, transactions and quarantine.
- The per-part translator with grounding and the six-check validator.
- The dialogue policy in Datalog and procedures as files.
- The curriculum encounters (teach, repeat, transfer, retention).
- The oracle parser, mocks, stress and calibrate tools.
- The archive of every model call in `data/training/`.

**What moves to "Later".** Naturalness, the wording judge, the 1-10 style score, framing echoes, taught wording as free text, paraphrase mining and any LLM rewriting of replies.

## The plan in two phases

| Phase | Milestones | Output | Ends when |
| --- | --- | --- | --- |
| **A. Design and clarification** | 19-21 | Decision log; experiment reports; the sealed H3, T3 and G3 sets; the final Design 3.0 document | The user approves the Design 3.0 document |
| **B. Build and test** | 22-30 (provisional) | The built agent; measurements on the sealed held-out set; the **final research PDF** | Milestone 30's acceptance tests pass |

Phase A answers the open questions with evidence where it is cheap to get. Most phase A experiments use the 2.0 archive and call no model. Phase B is written below as a proposal so that phase A has something concrete to confirm, change or cut.

## Architecture (provisional)

A message passes the same six stages as in 2.0. The changes are in who decides what.

1. **Segmenter.** As in 2.0. Its abbreviation and opener lists move to the lexicon.
2. **Translator.**
   - Teach lines in known frames go through the **frame grammar**, which is data in `vault/grammar/teach_frames.md`. The model fills only the free clause of a frame, for one step at a time.
   - Other messages go through the tiers (fixed patterns, learned patterns, Haiku), as in 2.0.
   - The translator no longer infers requests, lacks or tasks from text. `implied_request`, `lack_cues` and `sentence_rules` are removed from it.
3. **Validator.** The six 2.0 checks, plus:
   - a **task-symbol check**: known task, known alias, or `new_task` with the user's phrase; glued and suffixed symbols are rejected;
   - **structure checks** for teaching: one action per step, order facts only between existing steps, conditions that are registered states;
   - rejection of model commentary and of constants copied from the few-shot examples.
4. **Dialogue manager.** Session state is facts: `active_task`, `at_step`, `open_question`, `said`, `situation`. **Every** move is chosen by policy rules with a proof, including:
   - the request inferred from cues;
   - needs, duration and adapt;
   - the honest gap;
   - task confirmation;
   - resume.
5. **Reasoner.** v1's engine with 2.0's additions, plus:
   - a cached session model;
   - an index on joined argument positions;
   - the rules of the adaptation container.
6. **Generation.**
   - A **content plan**: atoms with roles (need, step, order, tool, time, condition, substitute, added, skipped, warning, gap).
   - A **plain realiser** that renders every atom and relation of the plan.
   - A **read-back** that parses the text into atoms again.
   - The claims are exactly the plan.

| Component | 2.0 | 3.0 (provisional) |
| --- | --- | --- |
| Heuristic knowledge | 78 word lists in Rust | Lexicon facts: seeded, derived by rules, or learned; no lists in agent crates |
| Teaching ingestion | The model writes ids and facts for whole lines | Frame grammar; the agent allocates ids; the model fills one clause; structure validated; read-back |
| Task identity | Word overlap; 15 names in the prompt | Aliases as facts; symbol check; matching as a rule with a proof; confirmation question |
| Request detection | `implied_request` in the translator | Cue patterns (learned) plus policy moves with proofs |
| Adaptation | 5 item-swap mechanisms | Plus the adaptation container: conditional times, alternatives, step replacement by effect, warnings, modifiers over task kinds |
| Learner | Single-feature rules | Multi-literal induction; difference-based adaptation rules; evidence from distinct examples |
| Generation | `task_help` monolith, taught wording, string edits | Content plan, plain realiser, read-back; no free text |
| Evaluation | Haiku checklist; 1-10 score; lenient check | Fact-and-relation check on claims and text; sealed tiers; negative controls |
| Lab | Polls every 2 s | Event log with ids; SSE; runner in-process; drill-down to the proof |

## Knowledge model

### Procedures with integrity

The procedural layer of 2.0 stays. These integrity constraints become Datalog checks run before every commit and in the integrity report:

- a step has exactly one `action`;
- `before` and `overlaps` name existing steps of the same task;
- step ids are `<task>_s<n>`, allocated by the agent;
- no task id has the step-id shape;
- every `uses` and `object_of` value has a type in the registry.

A procedure is confirmed after two successful uses in different sessions, or a teacher review. One accepted recommendation is no longer enough.

### Adaptation container (provisional; names and shapes decided in A)

Adaptation knowledge says how a situation changes a procedure. It lives in `vault/adaptations/` and in procedure files. Each item is a hypothesis until confirmed.

| Predicate | Meaning | Example |
| --- | --- | --- |
| `kind(Task, Kind)` | The task belongs to a kind | `kind(plan_picnic, outdoor_event)` |
| `achieves(Step, Effect)` | What a step is for | `achieves(fix_flat_tire_s3, tube_sound)` |
| `takes_time_if(Step, Cond, L, H, Unit)` | A conditional duration | `takes_time_if(boil_egg_s3, soft_boiled, 5, 6, minute)` |
| `alternative(Task, Cond, Method)` | A second method when a condition holds | `alternative(bake_potato, short_of_time, microwave_then_oven)` |
| `replaces(Method, FromStep, ToStep)` | Which span of steps a method replaces | |
| `warns(Step, Hazard)` | A warning tied to a step | `warns(water_plant_s4, standing_water)` |
| `because(Step, Reason)` | A reason tied to a step | |
| `situation(S)` | A session fact: the user's situation | `situation(rain)` |

**Situational modifiers** are Datalog rules whose heads are restricted to a small set of change predicates:

- `adapt_time(Step, L, H, U)`;
- `adapt_add(Task, Subprocedure, Anchor)`;
- `adapt_skip(Task, Step)`;
- `adapt_replace(Task, Step, Method)`;
- `adapt_need(Task, Item)`;
- `adapt_warn(Task, Hazard)`.

Their bodies range over situations, task kinds and step structure:

```
adapt_add(T, find_indoor_place, first) :- situation(rain), kind(T, outdoor_event).
adapt_need(T, liquids_bag)            :- situation(cabin_bag_only), kind(T, packing).
adapt_replace(T, S, fit_spare_tube)    :- situation(has_spare_tube), step(T, S), achieves(S, tube_sound).
```

A modifier taught or induced on one task applies to every task of the same kind. This is what makes cross-task transfer testable. Each modifier gets negative tests: it must not fire outside its kind or its situation.

### Task identity

- `task_name(Task, Phrase)` records the phrasings seen for a task.
- `same_task(A, B)` aliases are taught ("Hanging a painting is the same as hanging a picture."), proposed by the agent from shared structure, or learned from examples. Aliases are hypotheses: until confirmed, the reply confirms the task first ("Do you mean hang picture?").
- Matching a request to a task is a policy rule. It ranks candidates by feature facts (alias, name words, shared items, shared actions and objects), with a proof. Below a threshold, the move is a confirmation question. A seed that is close, but is not the requested task, is offered as "I know Y", never presented as the answer.

### Lexicon as data, derived and learned **[decide in A]**

Every Rust word list becomes one of three kinds of lexicon knowledge:

| Kind | Example | How it is obtained |
| --- | --- | --- |
| **Derived** from knowledge by rules | `tool(X) :- uses(S, X).` `waiting_step(S) :- takes_time(S, _, _, _), until(S, _).` `container(X) :- object_into(S, X).` | Free once procedures exist; correct when the procedures are |
| **Learned** from examples | lack cues ("ran out of", "nowhere to"), request cues ("where do I start"), generic verbs, mass nouns, verb frames | The rule learner, from dialogue and dataset examples labelled with gold acts |
| **Seed** (bootstrapping only) | function words, abbreviations | Hand-written data with `@source seed`; counted; ablated in the final measurement |

Phase A runs an audit of every list (experiment E2 below) and sorts each entry into one of the three kinds. A test fails if an agent crate gains a `&[&str]` of English content words.

### What a reply must carry

The **relation set** is the list of relation types the realiser must express and the evaluation checks:

- `needs`;
- step `action` and `object_of`;
- `uses` (tool per step);
- `before` (order) and `overlaps`;
- per-step and total `takes_time`;
- `until` and conditions;
- `substitute` (instead of);
- the adaptation changes (`adapt_*`);
- `warns` and `because`;
- `gap` (what the agent does not know).

The set is versioned. Each type has one plain realiser frame and one read-back pattern.

## Where knowledge comes from **[decide in A]**

This is the main research axis of 3.0. Three options are on the table. Phase A chooses, with the pilot evidence of experiments E3 and E4.

| Option | What is learned, and how | Tested by |
| --- | --- | --- |
| **C1 Dialogue teaching** | Procedures and adaptation knowledge from teach lines in frames, plus elicitation frames: "what goes wrong", "why this step", "how do you know it is done", "what changes if...", "what kind of task is it". The fix pass is student-aware | The generalisation tiers below, on H3 |
| **C2 Example dataset** | A generated corpus (proposed size 1,000-3,000) of procedures with kinds, situation-change pairs (near misses), request paraphrases, and lack and request phrasings, each with gold atoms. (a) As knowledge: stored as procedures. This is coverage, not learning, and is reported apart. (b) As training signal: the learner induces modifiers, kinds, aliases, lexicon classes and cue patterns | Same tiers; learning curves over size; null baseline |
| **C3 Mix** (proposed) | The dataset induces general knowledge; dialogue teaches the specific tasks and corrections, and is the path that evaluation uses | C1 against C3 on the same H3 is the main experiment |

**The dataset is generated** by Haiku, as the teacher, from a spec, in batches of about 20 examples per call. Each example carries gold atoms. A symbolic critic checks:

- registry typing and grounding;
- consistency with the ground truth of its own example;
- integrity constraints;
- near duplicates.

A random sample of 100 examples is checked by a reviewer pass. The dataset can enter in two forms:

- **as atoms**: this tests the learner alone and costs no calls;
- **as text through the translator**: this tests the whole system.

The proposal is atoms first, text second.

**Generalisation tiers.** These are used for every option, and H3 is built around them.

| Tier | Held out | What passing means |
| --- | --- | --- |
| T-word | Paraphrases of a taught task's request | Identity across wording |
| T-situation | A taught task in a situation never paired with it, whose change was taught on other tasks | Reuse of adaptation knowledge |
| T-task | An untaught task (taught only in its own encounter 1) in a situation seen only on other tasks | Cross-task transfer |
| T-domain | A whole domain absent from training and the dataset (proposal: bike and home repair, or gardening) | General rules, not memorised content |
| T-gap | A situation whose change is taught nowhere | An honest gap, not a guess |

**Leakage guard.** The linter computes near-duplicate overlap between training (dialogue or dataset) and H3 by atoms and by words, and fails above a threshold. No H3 task id, alias or domain appears in T3 or in the dataset.

## Learning

**Rule learner v3 [decide in A: scope and algorithm].** The 2.0 learner proposes single-feature rules. 3.0 needs a learner that can induce:

- **multi-literal rules over the typed vault**, Popper-style search with bounded body size, for kinds, lexicon classes and cue patterns;
- **difference-based adaptation rules** from near-miss pairs: two cases that differ in one situation and in one change of the procedure give a modifier hypothesis;
- **aliases** from structural overlap, confirmed by a teacher or by examples.

Evidence rules:

- Evidence counts **distinct examples or sessions**. Cache replays do not count.
- A hypothesis is used on trial, and the reply says so ("probably", "I was taught this once").
- It is confirmed after N clean uses or a teacher's yes, and retired on counter-evidence.
- Learned input patterns run in shadow until they agree with the model on 5 distinct uses. They live in the brain, so parallel sessions share them and no update is lost.

**Promises 2.0 did not keep, kept now:**

- stated rules start as hypotheses and keep their hedge;
- confidence below the threshold is answered "I think so";
- registry flags (single-valued, open-world) are read, replacing the code lists;
- the failures ledger records moves and reasons from the trace, not reply text.

## Understanding input

- **Teach channel.** Lines in a frame are parsed by the frame grammar. "Step N of X: clause" gives step `X_sN` from the agent. The model translates only the clause, into `action`, `object_of`, `uses`, `until` and `takes_time` for that one step. If the clause has two actions, the agent splits it into two steps with an order fact and reads both back. Every stored step is read back in the frame's words ("Step 2 of baking a potato: wash potato. Right?"). Lasting facts that the patterns do not cover get two model calls and must agree. A rejected translation is never cached.
- **User channel.** The tiers as in 2.0. The translator returns acts and atoms only. Request, lack and situation cues are **lexicon patterns** (seeded at first, then learned), applied by the dialogue manager as evidence, not by the translator as rewrites. When the model's acts and the cues disagree on a move-changing part, the part gets a second call. If the two calls still disagree, the safer move is taken: ask.
- **Situations.** Qualifiers and conditions become `situation(S)` atoms: "soft-boiled", "brown rice", "it might rain", "cabin bag only", "nowhere to hang them". A situation the lexicon does not know enters as a new situation and is flagged.
- **Cache.**
  - Keys are canonical text plus structured context (active task, step, last move id), not reply text, so wording changes no longer invalidate later turns.
  - The cache is loaded once per process and shared in memory.
  - Commentary is stripped before parsing.

## Dialogue

The policy decides every move, with a proof:

- `steps`, `needs`, `duration`, `next_step` and `resume`;
- `adapt`, `ask_replacement` and `adopt_proposal`;
- `gap_situation`: "I was taught hard-boiled, 7-9 minutes. I don't know soft-boiled time.";
- `confirm_task`: "Do you mean plan picnic?";
- `teach_me` and `teach_me_closest`: "I don't know boil egg yet. I know scrambled eggs.";
- `ask_slots`, `recommend`, `closest`, `close` and `social`.

Compound questions ("what do I need and how do I cook them?") yield every requested aspect in one reply. A real question stack replaces the single pending slot.

**Changed-situation rule.** A request whose session holds a `situation(S)` that no modifier, alternative or conditional time covers for the active task must get `gap_situation`, never the plain procedure. False alarms are measured on base requests.

**Runtime asking (proposal).** On `gap_situation`, the agent may ask the one question whose answer would fix the gap ("How long for soft-boiled?"). The answer is stored as a user-sourced hypothesis.

## Generating replies

- The **content plan** is the single source. Claims are the plan's atoms, and nothing is added or dropped after it.
- The **plain realiser** renders each atom and relation with one frame from the lexicon. Steps are numbered. "Instead of", "while", "until", "if", "because" and "warning" are always written out when the plan holds those relations. Articles, inflection and commas are best effort.
- The **read-back** parses the reply text with the realiser's inverse grammar and must recover exactly the plan's atoms. This is the faithfulness test, and it runs on every help reply in tests and in evaluation.
- Removed: taught wording as free text, framing echoes, step-time display rules, string swaps after realisation, `Act::Note` for content, and the reply filters that cut sentences after the claims were collected.

## Training and the curriculum

- **Teacher.** Haiku plays every teacher role: the elicitation answers from the ground truth, the variant teacher, the student-aware fix pass, the curiosity answers, and the dataset generator. Ground-truth files and the held-out set are written by a Claude Code sub-agent from a spec, as in 2.0 **[decide in A: confirm]**.
- **User simulator.** It is symbolic. It answers the question actually asked, from the scenario's situation facts: slot questions get the matching situation, a replacement question gets the offered items, anything else gets "not sure". Request wordings, at least three per scenario plus a paraphrase stress set, are written into the scenario files ahead of time.
- **Training set T3.** A new set (proposed 60 tasks) whose tasks teach the kinds and modifiers that H3's T-situation and T-task variants need, **on other tasks**. The 2.0 sets stay: D20 as the development set, and the 55 tasks as extra training.
- **Spaced queue.** It reteaches only the atoms that failed. If it stays under 20% after milestone 28, it is dropped.
- **Training runs start only with the user's go-ahead**, and all data goes to `data/training/` and is never deleted.

## Training lab (real-time)

The user asked for a real-time training lab: the runner, the vault writer and the dashboard as one system.

- **One process.** The lab server hosts the brain and runs curricula in-process. No second process writes the vault.
- **Event log.** It is append-only, in `data/training/lab/<run>/events.jsonl`. Each event has a monotonic id, the run id, the scenario, the encounter, the stage and a payload.
- **Live push.** `GET /api/stream` serves Server-Sent Events with `Last-Event-ID` resume. The UI (the existing Vite and React app) subscribes once and stops polling. Authentication is a token exchanged once for an HttpOnly, SameSite=Strict session cookie, because EventSource cannot send headers. The token is no longer accepted in the query string.
- **Panels.**
  - Live lanes per teacher.
  - The scenario grid by encounter and tier.
  - The teach-line view: line → stored atoms → read-back → fix.
  - Turn drill-down: message → acts → task match → atoms used → move with proof → plan → text → read-back.
  - Fact-and-relation results per relation type.
  - Learning feed: hypotheses, evidence, confirmations.
  - Knowledge integrity.
  - Translator health: tiers, agreement, second calls.
  - Plan limits.
  - Run comparison.
  - Step mode.
  - Alerts.
- **H3 guard.** For H3 runs the lab and the API serve counts only. Transcripts stay on disk in `data/training/`, but no endpoint returns them until the user unseals H3 after the final reading.

## Evaluation

### Sets

| Set | Size (proposed) | Written by | Use |
| --- | --- | --- | --- |
| **H3**, sealed held-out | 48 scenarios: 12 T-word, 12 T-situation, 12 T-task, 6 T-domain, 6 T-gap, each with base encounters and one variant | Sub-agent, from a spec only, never seeing code | Phase B acceptance; at most 4 readings; counts only |
| **T3**, training | About 60 tasks, plus the 2.0 55 | Sub-agent from a spec | Training runs C1 and C3 |
| **Dataset** (if C2 or C3) | 1,000-3,000 examples | Haiku from a spec, with a critic | Learning signal |
| **D20**, development | The 2.0 held-out 20 | existing | Tuning; transcripts may be read |
| **G3**, translator gold | 200 messages, plus 50 long | Sub-agent from a spec | Request-act recall, task-match, agreement |
| Regression | 296 v1 turns, plus the M15 replay guard | existing | No regressions |

**Sealing.** The SHA-256 of every H3 file is committed in milestone 21, and a test fails if a file changes. The build brief forbids opening H3 files. Each H3 reading needs the user's go-ahead **[decide in A]**, and the four readings are:

1. the v2.0.0 baseline;
2. after milestone 25;
3. after the training runs;
4. the final reading.

### The fact-and-relation check (measure of record) **[decide in A: exact scoring]**

Each encounter has an **expected answer**: a set of ground-truth atoms tagged `required` or `optional`, and for variants, the expected changes. The check has three parts.

1. **Claim check** (symbolic, no calls).
   - **Recall** = required atoms present in the reply's claims, alias-aware, out of all required atoms.
   - **Precision** = claims supported by the ground truth or marked as a gap or hypothesis, out of all claims.
   - A **wrong claim** is a claim that contradicts the ground truth: a wrong time, a wrong order, a wrong item on a step, or an unchanged step where a change was expected.
2. **Text read-back** (symbolic, no calls). The reply's text is parsed back into atoms, which must equal the claims. This catches a relation dropped by the realiser.
3. **Independent extraction** (Haiku, a sample of 20% of encounters). Haiku reads the reply text and lists every need, step, order, tool-per-step, time, condition and substitution it states, in a closed format. The list is compared with the claims symbolically. This guards against the circularity of check 2, which shares a grammar with the realiser. Haiku's extraction accuracy is measured on 20 frozen anchor replies.

**An encounter passes** when recall on required atoms is 100%, there are no wrong claims, and the read-back is equal. For a T-gap variant, it passes when the gap is stated and no wrong change is claimed. There is no fluency, wording or style score.

The 2.0 and v1 archives are re-scored with checks 1 and 2 where claims exist, and with check 3 otherwise, so the paper can compare all three iterations on one measure. (That costs some calls; budgeted below.)

### Metrics

| Group | Metric |
| --- | --- |
| Learning | Pass rate by tier and encounter (second encounter, retention, transfer); relation recall by relation type; learning curves over the amount taught |
| Correctness | Wrong-claim rate per 100 claims; unsupported claims; honest-gap rate on T-gap; changed-situation blindness; false alarms on base requests |
| Knowledge entry | Teach lines stored exactly as the ground truth; integrity violations; aliases correct |
| Understanding | Request-act recall after the cue layer; task-match accuracy; move agreement over two cold runs; raw Haiku act accuracy (reported only) |
| Learning versus coding | Share of reply-changing decisions whose proof rests on derived or learned (not seed) knowledge; learned-lexicon precision on a checked sample; drop when the seed is ablated |
| Curiosity | Questions answerable by the ground-truth teacher; goals closed |
| Cost | Calls per encounter; translator median and p95 (reported, not targeted) |

**Negative controls.** Each metric has a test that injects a known fault and checks that it is counted:

- a meta request in a teach line;
- a duplicate procedure;
- a wrong time;
- a dropped relation in the text;
- a modifier firing out of kind.

### Statistics

- The unit is the scenario. Rates get Wilson 95% intervals.
- **The main test** is paired: v2.0.0 against v3 on the same H3 scenarios, with an exact McNemar test. It is stronger than either level alone.
- The final reading is run twice, warm and cold cache, and both are reported.
- Repeated runs are never pooled as if they were independent.
- The second encounter is also reported **given correct storage**, so storage and recall can be told apart. In 2.0 the figures were 97% when storage was right and 14% when it was not.

**Pre-registration.** Before H3 reading 2, `research/PREREG_3.md` is committed. It holds the hypotheses, metrics, frozen targets, tests, read budget and the final-run protocol. The final PDF lists every deviation from it.

**Ablations at the final reading** (on the frozen build, mostly warm caches):

1. the oracle parser in place of the translator (isolates understanding);
2. a fresh seeded vault against the trained vault (the training effect, 0 in 2.0);
3. the seed lexicon off;
4. induced rules off;
5. C1 against C3.

**Free-form teaching.** Besides frame lines, a sample of teaching phrased freely by Haiku is stored through the model path and read back. Its storage accuracy is reported separately (proposed target ≥ 90%), so that the frame grammar does not make storage look solved by construction.

### Targets (provisional; fixed once after the H3 baseline, then frozen)

| Metric (on H3) | 2.0 reference | 3.0 target |
| --- | --- | --- |
| Second encounter pass, T-word | 30% clean (Haiku checklist; not comparable) | 70% |
| Retention pass | 25% clean | 65% |
| Transfer, item-swap and T-situation | 0 of 15 clean | 50% |
| Transfer, T-task (cross-task modifier) | not measurable in 2.0 | 40% (research target) |
| T-domain | none | reported against the null baseline; no target |
| Honest gap on T-gap; blindness | blind in 7 of 9 types | gap ≥ 90%; blindness ≤ 5% |
| Wrong claims | not measured | ≤ 1 per 100 claims |
| Relation recall, all encounters | not measured | ≥ 95% |
| Text read-back equal to claims | not true in 2.0 (taught wording) | 100% |
| Teach lines stored exactly | 70% (training stage) | ≥ 98% on T3 |
| Integrity violations | 42 multi-action steps; dangling order facts | 0 |
| Request-act recall (G3) | 80% | ≥ 97% |
| Task-match accuracy (paraphrase set) | not measured; 193 wrong-task replies | ≥ 95% |
| Move agreement, two cold runs | 29% of repeated messages had one translation | ≥ 95% of moves |
| Word lists in agent crates | 78 | 0 |
| Curiosity questions answerable | 13% | ≥ 80% |
| Agent p95, help path, 20,000 facts | never measured | ≤ 100 ms |

Phase A may drop or change any target after experiment E1 re-scores 2.0 with the new check.

## The research PDF

There is one PDF, the research PDF at the end of phase B. Phase A's output is the Design 3.0 document in Markdown (user decision).

**Layout.**

- `paper/`: the final research PDF.

It holds `main.tex`, `sections/*.tex`, `figures/`, `generated/` and a `Makefile` that runs `latexmk -pdf`. `tools/paper/numbers.py` reads the run records in `data/training/` and `eval/` and writes every number, table and figure data file into `generated/`, as LaTeX macros (`\newcommand{\hThreeWordPass}{34 of 48}`) and table files. It calls no model and runs with `python3 -I`. A `generated/SOURCES.md` lists, for each macro, the file and the query that produced it, and the SHA-256 of the input files. No TeX toolchain is installed now **[decide in A: tectonic or TeX Live]**.

**Final research PDF (end of phase B).** Contents:

1. Abstract.
2. Introduction and research questions:
   - RQ1: is knowledge taught once reused correctly in new words, in later sessions?
   - RQ2: does taught or induced adaptation knowledge transfer across situations, tasks and domains?
   - RQ3: dialogue teaching against an example dataset against a mix;
   - RQ4: can coded heuristics be replaced by learned ones?
   - RQ5: what bounds helpfulness, the teaching or the agent?
3. The system across three iterations.
4. Method: the curriculum, the tiers, sealing, the check, statistics.
5. Results with Wilson intervals: v1 → v2 clean → v2 development → v3 H3, all on the fact-and-relation measure; per tier; ablations (seed lexicon off, induced rules off, C1 against C3, warm against cold cache); cost.
6. Failure analysis.
7. **Honest limits**:
   - the contamination history of D20;
   - one model family as teacher, generator and extractor ("Haiku English", self-preference);
   - scripted request wordings;
   - small samples per tier;
   - one language;
   - no human evaluation unless the anchors are hand-labelled.
8. Related work.
9. Conclusion.
10. Appendix: how to reproduce, with the commit, the tag, the data files with hashes, the commands and the call counts.

Acceptance:

- It builds from a clean checkout.
- Regeneration matches byte for byte.
- Every table states its source.
- Examples from H3 transcripts appear only after the final reading, when H3 is unsealed.

## Every 2.0 failure, its fix and its test

Each failure found in REPORT_DESIGN_2 has a named fix and a test. Fixes marked **[A]** are proposals that phase A confirms. "H3" means the sealed held-out set.

**In measurement**

| 2.0 failure | Fix in 3.0 | Test |
| --- | --- | --- |
| Held-out set contaminated; only run 1 clean | Sealed H3 from a spec; hash-checked; counts only; four readings | H3 hash test; the lab serves no H3 transcript; reading log |
| 20 scenarios: one scenario = 5-7 points | 48 scenarios by tier; intervals; McNemar | Intervals reported in every row |
| "Helpful + clarify" counted "Thanks!" turns | Social turns in their own row; pass is the fact-and-relation check | Negative control: a "Thanks!" turn cannot raise the pass rate |
| Style judges steered the work (framing +1 to +1.5) | No style score; the fact-and-relation check is the record | No metric in the report depends on wording beyond read-back |
| Lenient symbolic check (80% order, extra steps ignored) | Recall 100% on required atoms, precision, wrong claims | Negative controls: a missing step, an extra step and a wrong order each fail |
| Haiku and symbolic judges disagreed, neither validated | Symbolic check of record; Haiku as an extractor only, measured on anchors | Extraction accuracy on 20 anchors ≥ 95% |
| Meta requests 0 by construction | Counted on generated and Haiku-taught lines; negative control | An injected meta line is counted |
| Duplicates 0 by construction (procedures ignored) | Integrity report covers procedures, aliases and step-id tasks | The 2.0 vault yields its 30 drifted names and `make_pancake_s4` |
| Contradictions a constant of name mismatches | Split into name mismatch (alias problem) and false belief | Both reported; an injected false belief is counted |
| Scripted user turns sent on any "?" | Symbolic user simulator answers the asked question | A wrong slot question gets "not sure", and the encounter fails |
| Cache hid variance and froze errors | Cold-cache replicate; rejected translations never cached | bake_potato's frozen line cannot recur (test with the archived answer) |
| Training never moved held-out numbers | T3 teaches modifiers used by H3 on other tasks; T-task tier | T-task pass reported; leakage linter passes |
| Hand-label calibration never done | Anchors for the extractor; optional user labels **[A]** | Anchor accuracy reported |
| Gold F1 tuned on the gold set | Fresh G3; the old gold becomes development data | G3 written from a spec, read only as counts |

**In knowledge entry and identity**

| 2.0 failure | Fix | Test |
| --- | --- | --- |
| 30% of step lines stored wrong | Frame grammar; agent-owned ids; one clause per call | ≥ 98% of T3 step lines stored exactly |
| Translator wrote step ids (`_s1` for step 2, `_s0`) | Ids allocated by the agent from the frame | The 626 archived step lines replayed: 0 id errors |
| Two actions on one step (31 procedures) | Structure check; split into two steps with read-back | Integrity: 0 multi-action steps |
| Dangling `before` and `overlaps` | Order only between existing steps | Integrity: 0 dangling references |
| Teach lines not understood (42 of 55 queue entries) | Frame grammar first, then the model for the clause only | The 18 distinct archived lines all parse |
| Compound teach lines from the teacher | Linter: one action per line; agent splits when needed | Linter flags 0 in T3; split read-back test |
| Glued `until` clauses | `until` must be a registered state; else ask | Archived glued clauses are rejected or asked |
| Name drift (`fix_tire`, `write_note`) | Task-symbol check; aliases; teach under the frame's own name | 0 drifted ids after a T3 run |
| Step id became a task (`make_pancake_s4`) | Step-id shape rejected as a task | Unit test on the archived symbol |
| Wrong task matched (193 replies) | Matching as a rule with a proof; confirmation below threshold | Task-match ≥ 95% on the paraphrase set; wrong-task replies < 1 in 100 |
| Seeds given as the answer before teaching (25 of 180) | `teach_me_closest` move; seeds reviewed | 0 seed procedures given as answers in encounter 1 |
| Curiosity unanswerable (58 of 67 "I don't know") | Questions named by checked task symbols and aliases | ≥ 80% answerable |
| Procedure confirmed after one accepted recommendation | Two successes in distinct sessions, or review | Unit test |

**In understanding**

| 2.0 failure | Fix | Test |
| --- | --- | --- |
| 20% of request-cue calls had no request act | Learned cue patterns plus a policy move | Request-act recall ≥ 97% on G3 |
| 71% of repeated messages translated differently | Second call on move-changing disagreement; canonical cache | Move agreement ≥ 95% over two cold runs |
| `implied_request` and `lack_cues` in the translator, setting `ok = true` | Removed; the dialogue policy decides from acts and cues | No act rewrite after the validator (code test) |
| Two-part questions half answered | Every requested aspect in one reply | The archived pancake question gets needs and steps |
| Model commentary and copied few-shot constants | Stripped or rejected | The 148 and 10 archived cases are handled |
| "nowhere to hang them" not a lack | Lack cues learned, including the action form ("nowhere to {action}") | Learned-cue test on held-out phrasings |
| Substring cue bugs ("piano", "shallow") | Token-level cue patterns | Negative tests on the archived false matches |
| Cache keyed on reply text | Structured context keys | A wording change keeps later turns cached |
| Act accuracy out of scope | Redefined at agent level (above) | G3 metrics |

**In adaptation**

| 2.0 failure | Fix | Test |
| --- | --- | --- |
| No conditional times (soft-boiled, brown rice) | `takes_time_if` | Reasoner case with a proof |
| No step replacement (spare tube) | `achieves` plus `adapt_replace` | Reasoner case with a proof |
| No alternative methods (25-minute potato) | `alternative` | Reasoner case with a proof |
| No plan-level changes (rain, surprise, cabin bag) | Modifiers over task kinds | Applied across kinds in T-task; negative suite of 20 cases |
| No warnings or reasons (standing water) | `warns`, `because`, taught by elicitation | Warning relations recalled |
| Blind to changed situations (7 of 9 types) | `situation` atoms; `gap_situation` move | Blindness ≤ 5%; false alarms ≤ 5% on base requests |
| Over-general mechanisms ("sprinkle the bike", "juice of the vinegar") | The "extra ingredient" rule removed; typed guards on every mechanism | The archived misfires are negative tests |
| The nine 2.0 variants needed untaught knowledge | Elicitation, dataset and modifiers supply it; gaps stated otherwise | All nine solved with oracle knowledge (a mechanism test); on H3 the T-gap tier is honest |

**In replies and design promises**

| 2.0 failure | Fix | Test |
| --- | --- | --- |
| Taught wording unchecked against facts | Taught free text removed; plain realiser from the plan | Read-back equal on 100% of help replies |
| Claims and text drift; claims of dropped sentences | Claims are the plan; no post-filters | Claims = read-back atoms (test on every agent test reply) |
| "Extra ingredient" step with no fact | Removed (adaptation container instead) | Code test |
| `task_help` monolith | Content plan, realiser and read-back as separate stages | Unit tests per stage |
| `Act::Note` free text for content | Only social lines and errors may be notes, typed | Code test |
| Stated rules confirmed at 0.9 | Hypothesis with hedge | Rule status test |
| Hedges never spoken; yes/no ignores confidence | Hedge atoms in the plan; threshold answer | Hedge test set |
| Registry flags unread | Flags replace `functional` and the open-world list | Code test: the hard lists are gone |
| Session state in Rust fields; no question stack | Session facts; stack | Resume after two side remarks; policy rules mention steps and questions |
| Moves without proofs (needs, duration, adapt) | All moves in the policy | Every trace move has a proof |
| String coupling on reply text | Consumers read moves and intent | Changing a reply template changes no curiosity goal or label (test) |
| Byte-slice panic on non-ASCII actions | Code removed with the prefix check | Unit test with a non-ASCII action |
| `proper_name` bypassing log and ids | Through `kb.assert` | Log test |

**In learning and engineering**

| 2.0 failure | Fix | Test |
| --- | --- | --- |
| 78 word lists in agent code | Derived, learned or seed data | Code test: 0 lists; seed ablation reported |
| Learner single-feature only | Rule learner v3 **[A]** | Induction suite: kinds, modifiers and cues recovered from examples, by tier |
| Learned patterns: replays counted, no shadow, lost updates | Distinct-source evidence; shadow mode; shared in the brain | Concurrency test with 16 sessions: no lost observation |
| Coverage 3.3% | Reported; the frame grammar removes most teach-line calls | Calls per teach line reported |
| 3-4 uncached evaluations per help turn | Session-model cache | Help-path bench: p95 ≤ 100 ms at 20,000 facts |
| Registry loaded per Agent | Shared in the brain | Approved-word test across open sessions |
| Panicking curriculum thread vanished | Reported as a failed encounter with the error | Test with an injected panic |
| Trainer and curriculum as two processes on one vault | Runner in the lab process | Code path test |
| Spaced queue 2% | Reteach failed atoms only; drop under 20% | Rate reported |
| Dashboard polls every 2 s; panels missing | Event log, SSE, the panels above | Latency p95 ≤ 500 ms; 0 lost events after reconnect |
| Stale docs, dead code | Housekeeping in milestone 22 | Doc check at each milestone |

## Milestones

Numbering continues from 18. As in 2.0, a milestone starts only when the previous one passes its acceptance tests.

### Phase A: design and clarification

**19. Brainstorm and decision framing**

Build:

- A decision log, `research/design3/decisions.md`. Every open decision of this draft gets its options, the evidence that would settle it, the experiment that would produce the evidence, and a default.
- A brainstorming pass on each of the three axes (learned versus coded, the learning-centred evaluation, the knowledge source), recorded with its arguments.
- The H3, T3, G3 and dataset specs, written as specs only. No scenarios yet.

Accept:

- Every item under "Open decisions" has an entry with a default and a named experiment, or "no experiment needed".
- The user reviews the log.

**20. Experiments for the decisions** (no model calls, except E4)

Build and report in `research/design3/experiments/`:

| # | Experiment | Data | Calls | Decides |
| --- | --- | --- | --- | --- |
| E1 | Re-score the archived 2.0 runs with a prototype of the fact-and-relation check, using claims where recorded and a symbolic read of the text otherwise. Compare with the Haiku checklist verdicts | `data/training/curriculum/*.jsonl`, live transcripts | 0 | The exact scoring rules; whether targets are reachable; the 2.0 baseline on the new measure |
| E2 | Audit every word list in the agent crates: derivable from knowledge, learnable, or seed | code, 2.0 vaults | 0 | The lexicon plan of axis A |
| E3 | Rule-induction prototype on the 105 ground truths (20 + 55 + 30) as atoms: induce kinds, aliases and difference rules; split by tier | `data/curriculum/` | 0 | Whether the dataset route can generalise at all; learner scope |
| E4 | Dataset pilot: 100 examples from Haiku in batches, through the critic | new | about 10-20 (with the user's go-ahead) | Dataset feasibility, error rate, cost |
| E5 | Replay the 626 archived step lines and the 18 not-understood lines through a frame-grammar prototype | L-train transcript | 0 | The ingestion design |
| E6 | Call-budget model from the 2.0 call archive | `data/training/calls/` | 0 | Set sizes and reading count |

Accept: each experiment has a report with its numbers generated by a script, and the decision it informs is updated in the log.

**21. Final Design 3.0**

Build:

- the final Design 3.0 document (`research/Symbolic Learning Agent – Design 3.0.md`);
- H3, T3 and G3 written by sub-agents from the specs, and H3 sealed by hash;
- targets fixed from E1, provisionally (frozen after the H3 baseline).

Accept:

- every open decision of this draft is either decided or explicitly deferred;
- the H3 hash test passes;
- the linter passes on H3, T3 and the dataset spec (leakage, checklist coverage);
- the user approves.

### Phase B: build and test (provisional; confirmed or changed by phase A)

**22. Measurement v3 and the H3 baseline**

Build:

- the fact-and-relation check (claims, read-back, extraction with anchors);
- honest metrics and negative controls;
- the symbolic user simulator;
- the cold-cache mode;
- the reading log;
- the M15 guard with symbolic labelling;
- housekeeping.

Accept:

- one command prints all metrics with intervals;
- every negative control is detected;
- the same commit run twice gives identical numbers;
- **H3 reading 1**: v2.0.0 on H3, cold, with the user's go-ahead;
- targets frozen in `research/PREREG_3.md`, committed before reading 2.

**23. Training lab core**

Build:

- the event log with ids;
- SSE with resume;
- the runner in-process;
- the turn drill-down and the teach-line view;
- the scenario grid by tier;
- the H3 guard;
- the cookie authentication.

Accept:

- event latency p95 ≤ 500 ms under the 16-teacher mock stress;
- 0 lost events after a reconnect;
- the H3 guard test;
- the token is refused in the query string.

**24. The agent owns its knowledge entry; no word lists in code**

Build:

- the frame grammar as data;
- agent-owned step ids;
- the structure validator and the integrity report;
- read-back to the teacher;
- the cache rules;
- every word list moved to lexicon seed data, with derivation rules for the derivable entries (from E2);
- the code test against lists.

Accept:

- the archived 626 step lines replay with ≥ 98% stored exactly and 0 integrity violations;
- 0 word lists in agent crates;
- D20 numbers on the new check do not fall by more than one scenario.

**25. Task identity and understanding**

Build:

- aliases;
- the task-symbol check;
- matching as a rule with a proof;
- the `confirm_task` and `teach_me_closest` moves;
- request, lack and situation cue patterns as lexicon data used by the policy;
- compound questions;
- second calls on disagreement;
- canonical cache keys;
- patterns shared in the brain with shadow mode.

Accept:

- task-match ≥ 95% and request-act recall ≥ 97% on G3;
- move agreement ≥ 95% over two cold runs on 50 D20 turns;
- at least 20 identity cases with proofs ("hang a painting", "jacket potato", "picnic" against a dinner distractor);
- **H3 reading 2**, with the user's go-ahead.

**26. Plain, complete replies and the kept promises**

Build:

- content plan, realiser and read-back;
- all moves in the policy, session facts, the question stack;
- hedges, rule hypotheses, confidence, registry flags;
- typed guards;
- the session-model cache;
- the 20,000-fact help-path bench.

Accept:

- read-back equal on 100% of help replies in all agent tests;
- claims equal the plan;
- every move has a proof;
- p95 ≤ 100 ms on the help path;
- the archived misfires are rejected.

**27. Adaptation container and situation noticing**

Build:

- the adaptation predicates and modifier rules;
- the change predicates in the plan and realiser;
- `situation` atoms and the `gap_situation` move;
- runtime asking **[A]**.

Accept:

- all nine 2.0 failing variants solved with hand-written oracle knowledge, each with a proof;
- a negative suite of 20 cases where no modifier fires;
- blindness 0 and false alarms ≤ 5% on D20 variants.

**28. Rule learner v3** **[A: scope]**

Build:

- multi-literal induction;
- difference-based modifiers from near-miss pairs;
- alias proposals;
- cue and lexicon-class learning;
- evidence from distinct examples;
- the use, confirm and retire cycle.

Accept:

- on the E3 split, the learner recovers the held-out kinds and modifiers above the null baseline, with each rule's evidence;
- learned lexicon precision ≥ 90% on a checked sample;
- the share of reply-changing decisions resting on seed data falls (reported).

**29. Knowledge sources and training** **[A: C1, C2, C3]**

Build:

- the elicitation frames and the student-aware fix pass;
- the dataset generator and critic (if chosen);
- training runs on T3 with the chosen sources, each with the user's go-ahead: C1 and C3 for the main comparison.

Accept:

- ≥ 98% of T3 teach lines stored exactly;
- ≥ 80% of T3 tasks have warnings, reasons and completion conditions stored;
- dataset error ≤ 5% on the reviewed sample;
- contradictions (false beliefs) ≤ 1%;
- **H3 reading 3** on each trained vault, with the user's go-ahead.

**30. Final measurement, full lab and the final research PDF**

Build:

- the remaining lab panels (learning feed, run comparison, translator health, step mode, alerts);
- **H3 reading 4**, the final, warm and cold;
- the ablations listed under Statistics;
- the paired McNemar test against v2.0.0, compared with `PREREG_3.md`;
- unsealing H3;
- the final research PDF in `paper/`;
- the v3.0.0 tag (the user pushes).

Accept:

- the targets as frozen in milestone 22, each reported with its interval, met or not;
- the final PDF acceptance above;
- PROGRESS.md current.

## Build brief for Claude Code

**Working rules**

- Phase A first. Phase B starts only after the user approves the Design 3.0 document. Within a phase, work in order and commit after each milestone with a message naming it. **Do not push.** The user adds remotes and pushes.
- Run `cargo test` after every change, and never leave the build broken at a commit. Keep PROGRESS.md current: it is part of every acceptance.
- **Never open H3 files or H3 transcripts.** Report H3 only as counts. Tune only on D20 and T3.
- **No English word lists in agent crates.** A behaviour that needs words gets lexicon data, a derivation rule, or a learning path.
- **No polish work.** A reply is done when the read-back recovers every fact and relation. Do not tune wording to any judge.
- When stuck, write the problem in PROGRESS.md, choose the simplest fallback, and move on.

**Model use**

- Haiku translates, teaches, generates the dataset and extracts in evaluation, always through the Claude CLI behind the `Parser` trait. It never writes a reply.
- Every code path that calls a model has a mocked test.
- **Training runs and H3 readings start only with the user's go-ahead.** Development runs on D20 follow the 2.0 practice.

**Data**

- Every model call, run record, transcript, dataset batch and event log goes to `data/training/` and is never deleted. Superseded data is archived beside it, never removed.
- The runtime vault stays out of git. Scenarios, specs, ground truth, H3 hashes and generated paper numbers are versioned.

**Code layout** (provisional)

- `crates/dialogue`: session facts, policy and moves, split out of `agent`.
- Model calls move from `sla_parser` into `crates/translate`.
- `crates/learner` gains the v3 inducer.
- `crates/eval` gains the fact-and-relation check.
- `crates/lab` (or `eval/src/trainer.rs` grown) holds the lab server.
- `tools/paper/` holds the number generator.

**Definition of done for 3.0.** Phase A's Design 3.0 document is approved. Milestones 22-30 pass their acceptance tests. The final research PDF builds from a clean checkout and reports every frozen target on H3 with intervals, met or not.

## Model-call budget (estimate from the 2.0 archive)

About 2,000 calls filled one plan session in 2.0. Judge and labeller calls were 35% of 2.0's calls; the symbolic check removes most of them.

| Item | Calls (estimate) |
| --- | --- |
| Phase A: dataset pilot E4 | 10-20 |
| Re-scoring v1 and v2 archives with the extractor (sample) | 200-400 |
| G3 measurements (two cold runs) | 500 |
| H3 readings 1-4 (cold, about 48 scenarios, plus a cold replicate of the final) | 5,000-8,000 |
| Development cycles on D20 (about 40, translator only, warm cache) | 3,000-5,000 |
| Dataset generation (3,000 examples in batches of 20, plus the critic sample) | 200-400 |
| Dataset as text through the translator (if chosen) | 3,000-6,000 |
| Training runs on T3 (C1 and C3) | 4,000-6,000 |
| **Total** | **about 15,000-25,000 (8-12 sessions)** |

## Known limits and risks

| Risk | Why it stays | How 3.0 holds it down |
| --- | --- | --- |
| Numbers drop on H3 | 2.0's numbers were development figures | Expected; reported honestly; the D20-H3 gap is a finding |
| The learner does not generalise beyond wording | Symbolic induction from synthetic data may only memorise | The null baseline and T-domain tier make that a reportable answer, not a hidden failure |
| One model family everywhere | Haiku teaches, generates, translates and extracts | The symbolic check is the record; the extractor is measured on anchors; the limit is stated in the paper |
| The read-back is circular | The realiser and reader share a grammar | Haiku extraction on a sample as an independent reader |
| Rough text that humans find hard to read | Polish is out of scope | Content-completeness is required; readability is "Later" |
| Removing word lists drops behaviour first | 2.0's gains lived in them | Seeds as data on day one; derivation and learning replace them; seed ablation measured |
| Over-applied modifiers | Rules over kinds generalise too far | Hypothesis tier; negative suites; kinds are taught or confirmed |
| Ground truth drives the check | The check is only as good as the expected-answer atoms | Linter; required versus optional tags; the extractor on a sample |
| Scope of the iteration | 12 milestones, one PDF | Phase A may cut; import, runtime asking and text-form dataset ingestion are optional |
| Call budget | 15,000-25,000 calls | Four H3 readings maximum; atoms-first dataset; caches |

## Decisions

**Constraints kept as set by the user.**

- Training runs start only with the user's go-ahead.
- Haiku is the teacher.
- Model calls stay on the Claude CLI.
- All training data is kept in `data/training/` and never deleted.
- The user adds remotes and pushes.

**Items 2.0 put out of scope, revisited.**

| Item | 3.0 proposal | Why |
| --- | --- | --- |
| Translation speed | Stays out; reported as cost | The CLI constraint stands |
| Act accuracy | Back, redefined as agent-level understanding (request-act recall, task-match, move agreement) on G3; raw Haiku act accuracy reported only | The translator was the main agent-side failure, and 3.0 moves these decisions to the agent |
| Judge score (1-10) | Stays out of 3.0; moved to "Later" | It measures style and rewards guessing; human-like answering is out of scope |
| M15 help-turn replay | Back as a non-gating regression guard with symbolic labelling ("teach me" for a task the vault lacks is fitting) | Cheap, and the 2.0 objection is removed |

## Later (beyond 3.0)

- Natural, human-like replies: wording judges, naturalness scores, paraphrase mining checked by read-back, taught wording linked to atoms, framing.
- Small talk and the self-model.
- Outside knowledge import at scale (how-to resources, ConceptNet), unless phase A adds it as an ablation.
- Other languages; forgetting; fully incremental evaluation; one grammar shared by parsing and generation.

## Open decisions for the user

1. **Axis A (learned versus coded).** Confirm "no English word lists in agent crates", with seed lexicon data that is counted and ablated. How strict is the boundary for language infrastructure (frame grammar, segmentation)?
2. **Axis B (evaluation).** Confirm the fact-and-relation check as the measure of record, and no style score in 3.0. Is the Haiku extraction sample wanted as the independent reader? Will you hand-label 20 anchors?
3. **Axis C (knowledge source).** Dialogue only, dataset only, or a mix. Is C1 against C3 the main experiment? Dataset size (1,000-3,000) and form (atoms first, then text, or atoms only).
4. **The held-out domain** for the T-domain tier (bike and home repair, gardening, or another).
5. **H3 size and readings.** 48 scenarios recommended; at most four readings; each reading with your go-ahead.
6. **Call budget.** About 15,000-25,000 calls (8-12 sessions). What ceiling?
7. **Who writes ground truth and H3.** A Claude Code sub-agent from a spec, as in 2.0, with Haiku for all teacher roles. Confirm.
8. **Scope cuts if needed.** Which go first: runtime asking, text-form dataset ingestion, full lab panels, or an import ablation (currently out)?
9. **The out-of-scope items** as proposed: speed stays out; act accuracy back, redefined; judge score to "Later"; M15 replay back as a guard.
10. **LaTeX toolchain.** None is installed: tectonic (one binary) or TeX Live with latexmk. Is the PDF committed, or only built?
11. **Vault.** Start fresh with reviewed seeds (recommended), keeping the 2.0 vault archived as an integrity test case.
12. **Phase gate.** Does phase B start only on your approval of the Design 3.0 document (recommended)?
13. **The final paper's audience** (thesis chapter, internal report, public preprint), its length, and whose names go on it.
14. **Self-preference check.** May a second Claude model, through the CLI, extract on a 100-encounter sample to measure how much of the result is Haiku reading Haiku's English? It costs about 100-200 calls of a larger model.
15. **Autonomy and git.** Run autonomously between go-aheads, as 2.0 did with `/goal`, or stop after each milestone? Which branch, and should milestones be tagged (`v3.0.0-rc`, `v3.0.0`)? Should the final trained vault be promoted to `vault/`?
