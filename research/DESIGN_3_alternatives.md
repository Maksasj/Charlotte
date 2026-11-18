# Design 3.0: alternatives, unused evidence and architecture options

*2026-11-18. A research-advisor note, written beside the two existing drafts (`research/DESIGN_3_draft.md`, called **D3** below, and `research/DESIGN_3_draft_fable.md`, called **F3**) and the options summary (`research/DESIGN_3_options.md`, **OPT**). It is not a third draft of the same plan. It asks three things: what information we have but do not use, where the drafts are weak, and whether the pipeline itself (translate → Datalog vault → reasoner → nlg) is the right shape for a "learning, not coding" 3.0.*

*Nothing here called a model. Four small offline measurements were run with `python3 -I` over files already in the repository. They are marked **[M1]**-**[M4]**, and their methods are given in Appendix A so they can be rerun. All other numbers about 2.0 cite REPORT_DESIGN_2 (**R2**, with its section numbers) or REPORT_DESIGN_1 (**R1**). Where PROGRESS.md and R2 disagree, R2's corrected numbers are used.*

---

## 0. The argument in one page

The drafts treat 3.0 as "2.0, repaired". They keep one per-message Haiku translator, add a hand-written frame grammar for teaching and a plain English realiser with a read-back parser, move the word lists into seed data, and hope a rule learner will later replace the seed. This is a sound engineering plan. But it leaves the central research question vague, because it mixes **three different learning problems** in one pipeline:

| | Learning problem | What 2.0 shows | Where it can be tested |
|---|---|---|---|
| **L1** | **Language to meaning**: map English (teaching and requests) to atoms | The main failure: 30% of step lines stored wrong (R2 §14.4), 20% of request-cue messages lost their request act, 71% of repeated messages got more than one translation (R2 §14.0) | On (English, atoms) pairs. We already have about 500 gold pairs and 8,288 silver pairs |
| **L2** | **Episodic storage**: keep a taught procedure and find it again | Nearly solved when storage is right: the second encounter passes 97% when the teach retry passed, 14% when it failed (R2 §11.10) | Trivial in atom space; identity is the only hard part |
| **L3** | **Generalisation of knowledge**: substitutes, changed situations, task kinds, identity across names | Item swaps only (6 of 15); time, method and plan variants 0 of 351 encounters (R2 §11.5) | Entirely in atom space, with **zero model calls** |

2.0's curriculum is a loop that hides this. A Claude sub-agent wrote the ground-truth atoms and the English teach lines together. Haiku translated the English back into atoms. The agent "learned" atoms that already existed, through a noisy channel. The 30% storage error is a property of that channel, not of learning.

My main recommendations follow from separating the three problems:

1. **Make L1 a learned component, not a hand-written grammar.** The teach lines are almost pure lexical alignment: **97% of ground-truth action symbols, 96% of objects, 96% of tools and 100% of step times appear word for word in their teach clause [M1]**. A small trained parser (alignment-induced templates plus a role tagger) can learn this from pairs. Haiku then becomes the teacher of the parser, which fits the constraint, and a runtime fallback, instead of a per-message dependency. This removes the drafts' frame grammar, which is hand-written English under another name.
2. **Test L3 in atom space first, cheaply, and decide the adaptation core by experiment.** The candidates are rules induced by ILP, case-based adaptation by structure mapping, or both, with cases consolidated into rules. The 2.0 data warns that real variants rarely repeat across tasks [M2]. So "modifiers over task kinds" may only appear to generalise when the test author designs the situations to repeat.
3. **Make the output content-first.** A reply is a serialised content plan in a fixed, minimal relation format. It is not an English realiser with a parser that reads it back. That meets the user's "rough text is fine, every relation present" literally. It removes the last place where polish can creep back in, and it makes the content check exact.
4. **Add the baselines a reader will ask for.** One is a zero-call retrieval baseline: atoms-only task retrieval already gets **176 of 225 base requests right over 105 tasks with no word list [M3]**. The other is a Haiku-with-teaching-in-context baseline. Without them, the final PDF cannot say what the symbolic machinery adds.

---

## 1. Information we have and neither draft uses

### 1.1 The ground truth is already a parallel corpus for learning L1

Every scenario file holds the ground truth as Datalog **and** the teach lines that express it (`data/curriculum/scenarios/*.md`, `data/curriculum/training/*.md`).

- **[M1]** The 75 curriculum scenarios (20 held-out-turned-development, 55 training) have **361 "Step N of X: …" lines and 150 other teach lines** (needs and total-time lines). Each step line is aligned to its atoms by its step number.
- **[M1]** The atom's symbol appears in the clause (matched on 4-letter stems):

  | Atom | Present in the clause |
  |---|---|
  | `action` | 351 of 361 |
  | `object_of` | 346 of 361 |
  | `uses` | 299 of 311 |
  | `until` | 42 of 44 |
  | step `takes_time` numbers | 312 of 312 |

So parsing a teach step is mostly **choosing which span fills which role**, not inventing symbols. It is a sequence-labelling problem with near-copy outputs. Problems of this shape are learned well from a few hundred examples by a perceptron or CRF tagger (Collins 2002; Lafferty et al. 2001). Neither draft uses these pairs as training data. Both write a frame grammar by hand (D3 "Understanding input"; F3 "Architecture" 2a).

The training-stage archive adds **labelled negative examples**. `live/2026-11-18T05-11-26.jsonl` has 906 teacher lines paired with 906 "stored" lines (checked here by `who` counts). R2 §14.4 found 186 of 626 step lines stored wrong. These are free error cases for a learned parser and for any critic.

### 1.2 The request lines are gold labels for request detection and task identity

Each scenario has three request lines and a variant request, and all of them are known to be requests for a known task: 225 base requests and 52 variant requests [M3]. Neither draft uses them as **labelled data** for the two worst agent-side failures, the dropped request act (R2 §14.1, F3) and the wrong task (F4, F5). D3 proposes to "learn cue patterns" (S11) but does not say from what. These lines are the "what". With the 200-message translator gold set (`data/gold/translator_gold.jsonl`) and the 50 long messages, they form a ready evaluation set for request recall and task-match. No new writing is needed.

### 1.3 8,288 translator calls are a silver corpus, with the noise already measured

`data/training/calls/*.jsonl` holds every translator prompt and answer (R2 §11.12: 8,288 translator calls of 12,623). R2 §14.0 found that 678 of 954 repeated messages got more than one answer. That variance is useful. Messages whose repeated answers agree are high-confidence silver labels. Messages whose answers disagree mark exactly where a learned parser must be careful, or must ask. This is the standard way to distil a teacher into a small student (Hsieh et al. 2023 for LLM distillation). It costs no calls, because the calls were already made and archived. The translation cache (`eval/translate_cache.jsonl`, 9,502 lines) is keyed by hash only, so the calls archive, not the cache, is the source.

### 1.4 The 52 variants are a small corpus of "situation → change" pairs, and they rarely repeat

52 of the 75 scenarios have a variant (`## variant`, `change:` line) [M2]. Their distribution matters for the drafts' central L3 bet:

- **[M2]** 20 begin "no X:" (a missing item), and 20 contain "instead" or "in place of". 6 change a time and 3 skip a step.
- **[M2]** **Almost no situation type repeats across tasks.** By hand reading of the 52 lines, only "vegetarian" (2), "no lemon" (2), "surprise" (2) and "only N minutes" (2) occur twice. The rest are task-specific: "no whetstone → the rough ring under a mug", "suede shoes", "mixer tap with a ceramic cartridge", "electric car".
- The variants exist **only as English**. There are no atoms for the change or the expected reply.

What this tells us:

1. **"Modifiers over task kinds" (D3 adaptation container; F3 protocol rule 7) is not how this domain's variants look when written naturally.** Nine shared situations (rain, surprise, cabin bag and so on) can be designed into T3 and H3, but then the T-task tier measures transfer of situations **the spec author chose to repeat**. That is a legitimate test ("is a taught modifier reused on another task?"). It is not evidence of general learning, and the PDF must say so.
2. **The class that does generalise naturally is the missing item.** That is 20 of 52, and the "instead" overlap is larger. It generalises through **function**: which items can fill which role of which action. Function can be induced from the procedures themselves: `uses(S, X)`, `action(S, A)`, `object_of(S, O)` across 105 tasks. 2.0's "analogy" mechanism did this implicitly (R2 §6, `substitutions_why` step 2). A learned affordance relation is the cheapest real L3 result available.
3. **E1 and E3 in D3 cannot score transfer** until the 52 variants are turned into atoms (expected diff: added, removed or replaced atoms). That is a Phase A authoring task, and D3 does not list it.

### 1.5 The ground-truth schema is thinner than the teaching, and that bounds the measure of record

- **[M1]** The ground truth uses only ten predicates: `takes_time` 387, `needs` 373, `step` 361, `action` 361, `object_of` 361, `uses` 311, `before` 288, `task` 75, `until` 44, `overlaps` 19. **There is no `warns`, `because`, condition, quantity or adaptation atom anywhere in the existing ground truth.**
- **[M1]** **103 of 361 step clauses contain "and" or "then" before the duration**, so they describe more than one action. Yet the ground truth gives each exactly one `action`. Example: `repot_plant_s4` is taught as "put a layer of potting soil in the new pot, set the plant in and fill around it with the trowel" and stored as `action(fill)`, `object_of(new_pot)`.

Consequences the drafts miss:

- D3's fix for multi-action steps ("the agent splits it into two steps with an order fact", D3 "Understanding input") will be scored **wrong** against the current ground truth, which has one step. Either the ground truth is re-authored (all 105), or the agent must learn the ground truth's abstraction (head action only). This choice belongs upstream of the measure of record.
- The relation set that D3 makes mandatory in replies (warnings, reasons, conditions, `adapt_*`) has **zero** existing ground truth. E1, "re-score 2.0 with the fact-and-relation check", can therefore check only needs, steps, order, tools, times and `until`. That is close to 2.0's own symbolic check, made strict. It is still worth doing. But it cannot set targets for the new relation types.
- The six checklist items that fail most are about wording details ("sort lights from darks", "at least two weeks ahead"; R2 §11.6). These are **not in the ground-truth atoms**. A content check on atoms will never see them. Either they become atoms (a detail atom, F3's "quoted detail atoms"), or 3.0 states that it no longer measures them.

### 1.6 The archive holds 37 trained vault snapshots and 40 live logs, which give a storage-error curve over builds

`data/training/curriculum/vaults/` holds 37 vault copies and `live/` holds 40 run logs (counted here). R2 analysed two vaults in depth (§11.13). The rest allow, at zero calls:

- storage accuracy per build, which shows whether code changes moved storage at all (R2 suggests they did not: the afternoon fixes "did not change how procedures are stored", §11.13);
- **teaching cost per task**: teach blocks per scenario rose from 7.2 to 8.0 as the fix pass retaught lines (R2 §11.10). This is the "teaching dimension" of R2 §18.4 question 2, which neither draft measures. For a learning paper, "lines of teaching needed per task, and how that falls as knowledge accumulates" is a more natural learning curve than pass rate.

### 1.7 Curriculum records hold text, not claims

The run records (`data/training/curriculum/2026-11-1*.jsonl`) store per turn `user`, `agent` and `label` only (checked: no `claims` or `moves` keys, and 0 lines with "claims" in the last live log). D3's E1 says "using claims where recorded". For curriculum runs, claims were never recorded. E1 must read the 2.0 reply **text** back into atoms. That is actually a useful test: it is the first measurement of the D3 read-back parser on real, messy agent English. Treat it as such.

### 1.8 No text in the project was written by a human

The teach lines, requests, variants, the 296 "real" v1 turns and the friend chats are all Claude-written (R1 §5; R2 §7 calls them the "296 real v1 turns", but their user side came from Haiku teachers in v1's trainer). Every set, D20, T3, H3 and the dataset, shares one author family. F3 lists "Haiku English" as a risk. Neither draft proposes the obvious mitigation: **a small human-written test slice for L1**. For example, 50 to 100 wikiHow step sentences (Koupaee & Wang 2018; CC BY-NC-SA) atomised by a sub-agent. It tests the parser on English nobody in the loop wrote. It costs no Haiku calls at test time, and it is the only defence against a reviewer asking "is this just Haiku talking to Haiku?"

### 1.9 The judge-disagreement corpus labels what a content check misses

The 712 encounters that Haiku failed and the symbolic check passed (R2 §11.8) carry per-item judge notes ("1: yes | 2: no | …"). Joined with the checklist text, they label which checklist items depend on details outside the ground truth. That is a free audit of section 1.5. It shows, before any build, how much of the "content" a strict atom check would ignore.

---

## 2. Improvements to the current drafts

These points apply mainly to D3, because it supersedes F3 on the user's direction. F3 keeps the Haiku checklist judge as the judge of record and keeps taught wording, which the user's notes now rule out (OPT §5b).

### 2.1 Hard-coding that is still smuggled in

1. **The teach-frame grammar is hand-written English.** D3 lets the code hold "the teach-frame grammar of the teaching channel" (OPT axis A, the boundary) and puts it in `vault/grammar/teach_frames.md`. It is data in a file, but it is still written by the programmer: "Step N of X:", "To X you need", "X takes N to M minutes", "instead of", "until". F3 admits the result: "frames make storage close to correct by construction". The headline storage target, 70% to at least 98%, would then measure the grammar's author. *Fix:* induce the frames from the (teach line, atoms) pairs (alternative A1). Keep a hand grammar only as an ablation arm.
2. **The seed lexicon has no learning path with data behind it.** D3 seeds the 78 lists as data and says "derived or learned entries replace them" (D3 milestone 28). But for cue patterns (lack, request, how), no labelled data source is named. Section 1.2 names one. Without it, the seed will still be in place at the final reading, and the "seed off" ablation will show a drop and nothing to replace it.
3. **The "no lists in agent crates" test has a scope gap.** 33 of the 77 `const &[&str]` lists are in `crates/parser/src/lexicon.rs`, including the 267-entry `VERBS` that `sla_nlg` imports (R2 §9.2; counted with grep). The parser crate sits on the runtime path. The test must cover every crate on the runtime path, not only "agent crates".
4. **The plain realiser still needs English word knowledge.** "Articles, inflection and commas are best effort" (D3 "Generating replies") means the 83-entry `MASS` list (`nlg/src/lib.rs:1052`), plurals and verb frames survive in some form. Every one of them is polish. See alternative A6.
5. **Situation noticing will need cue lists, or knowledge.** D3 targets blindness at most 5% and false alarms at most 5%. **[M4]** A generic detector, "content words in the request that are grounded in no atom of the task and are rare across requests", flags 47 to 49 of 52 variant requests. It also flags **169 to 204 of 225 base requests**. A contrastive version, which ignores words seen in the task's other requests, still flags 203 of 225 base requests in a leave-one-out test. Lexical novelty cannot tell "soft-boiled" from "for breakfast". The only routes to 5% false alarms are a hand cue list (hard-coding again) or **knowledge-gated noticing**. In the second route, a qualifier counts only if the vault links it to something that varies: a step with a conditional time, an item with a known substitute, a modifier's trigger. When nothing is linked, the agent should ask a cheap question rather than stay silent, and a false alarm then costs one turn, not a wrong claim. D3 should state this, and set the T-gap target with it in mind.

### 2.2 Measurement choices that will not hold

6. **Strict pass is dominated by alias noise.** "Recall on required atoms is 100%" (D3, measure of record) fails on `new_pot` against "bigger pot", or `tire_lever` against "levers". 2.0's contradiction metric was already "the same 3-4 name mismatches" (R2 §11.15 item 13). *Fix:* the ground truth must carry alias sets per atom, written by the scenario author. Report graded recall beside strict pass, and report strict-pass failures that are only alias mismatches as their own column.
7. **The per-tier intervals are too wide to compare tiers.** H3 is split 12/12/12/6/6 (D3 "Sets"). At 12 scenarios, a Wilson interval at 50% is about ±26 points, and at 6 about ±35. Tier differences will be inside the noise, and tiers are the headline. The cause is cost: each H3 scenario runs the Haiku translator. If L1 runs offline (A1 or A2), an H3 reading costs almost no calls. H3 can then be 150-300 scenarios generated from a spec, while the read budget stays at four for contamination reasons, not cost.
8. **The tiers are hand-authored, and the authoring decides the outcome.** T3 "teaches the kinds and modifiers that H3's T-situation and T-task variants need, on other tasks" (D3 "Training and the curriculum"). The spec author therefore designs the training set against the test set. Use **algorithmic splits** as well. The compositional-generalisation literature builds held-out sets by maximising the divergence of atom combinations between train and test, with similar atom distributions (Keysers et al. 2020, CFQ "maximum compound divergence"; Kim & Linzen 2020, COGS). On the ground-truth atoms, this needs no model call and no author judgement.
9. **The read-back adds almost no information.** The realiser and its inverse share a grammar (D3 lists this risk). A deterministic serialiser read back by its own inverse passes by construction. The real checks are plan against ground truth, which is symbolic, and an independent reader on a sample. Make the read-back a unit test, not a metric.
10. **No baseline isolates what the symbolic agent adds.** Two baselines are missing:
    - **Retrieval baseline (0 calls).** Store the oracle atoms of each taught task. Retrieve by bag-of-atoms with learned idf. Print the retrieved procedure. **[M3]**: top-1 retrieval is **176 of 225 base requests and 42 of 52 variant requests over 105 tasks**, with no word list. Its misses are 2.0's misses: "jacket potato" goes to garlic bread, "hard-boiled egg" to scrambled eggs, "small painting" to patching drywall. With oracle storage, T-word pass is roughly storage times identity. This baseline gives the identity half for free, and any 3.0 identity mechanism must beat it.
    - **LLM-in-context baseline (calls, with go-ahead).** Haiku receives the teach lines of all taught tasks in its context, then answers the H3 requests. Its reply goes through the same content check (with an extraction pass). A reader of a 2026 paper will ask for it first. If Haiku-in-context beats the agent on T-word but loses on T-gap (honesty) and on wrong-claim rate, that is a strong and honest result for a symbolic agent.
11. **Budget allocation.** D3's budget spends about 90% of 15,000-25,000 calls on running the translator inside evaluation and training (D3 "Model-call budget"). Almost none goes to producing learning signal. Under A1, the calls move to **generating supervision** (paired data in batches of 20 to 50 per call), and evaluation becomes nearly free.

### 2.3 Experiments that cannot decide what they claim to decide

12. **E3 has nothing to induce difference rules from.** "Rule-induction prototype on the 105 ground truths: induce kinds, aliases and difference rules" (D3 milestone 20). The ground truths hold no situations, no kinds and no diffs (section 1.5), and the variants are English only (section 1.4). As specified, E3 can test only alias proposals from shared structure. *Fix:* first atomise the 52 variants into (situation atoms, diff atoms) pairs, which is a sub-agent authoring job. Then run E3 leave-one-task-out.
13. **E5 tests a grammar that will be written to pass it.** Replaying the 626 archived step lines through a frame-grammar prototype measures the grammar against lines its author can read. The learned-parser version (train on the 55 training scenarios, test on the 20 development scenarios and the 626 archive lines) measures generalisation. Run both, and the gap between them is itself a result.
14. **E4, the dataset pilot, tests feasibility, not usefulness.** It measures error rate and cost. It should also measure the one thing that decides axis C: **does an induced rule from the pilot change any answer on D20?** If 100 examples induce nothing the agent uses, scaling to 3,000 is a bet, not a plan.

### 2.4 Places where polish work can creep back in

15. **Elicitation frames chase checklist items.** "What goes wrong", "why", "done when" (D3 S19) were motivated by Haiku checklist items, and some of those were wording details. Keep elicitation only for relation types that change an answer downstream: a condition that changes a time, an effect that licenses a substitution, a warning tied to a step's state. A tip that no later answer uses is decoration.
16. **"One plain realiser frame per relation type, in the lexicon"** will grow into a realiser lexicon. Freeze the relation keywords ("instead of", "until", "while", "if", "because", "warning", "unknown") and forbid any other generated function words in 3.0. That is A6.

### 2.5 Smaller points

17. A rule learner "Popper-style, in Rust" (D3 Learning) is an L-sized build with an unknown payoff. Prototype it in Phase A with the real Popper (Cropper & Morel 2021; Python plus clingo) as an **offline** tool. Only port to Rust if it earns its place. Popper also runs offline in consolidation, which keeps the agent's runtime pure Rust.
18. `kind(Task, Kind)` is a hand ontology unless it is induced. If the scenario author assigns kinds, the agent's cross-task transfer is the author's taxonomy. At minimum, report transfer with induced kinds (clusters over shared actions and objects) beside transfer with authored kinds.
19. Fix the confidence promise in the measure. "Probably" and "I was taught this once" are content, a hedge relation, so the content check must score hedges too. Otherwise a hedged correct answer and a confident one look identical, and calibration is never measured.

---

## 3. Fundamental alternatives

For each alternative: the core idea, what it changes in the pipeline, which 2.0 failures it addresses (codes from R2 §14.1: F3 request lost, F4 task not found, F5 wrong task, F6 stored wrong, F7 variant not handled, F10 variance, F11 glued symbols, F12 misfires, F15 teach line not understood), cost, the test on held-out data, and the main risk.

### A1. A learned semantic parser instead of rule-based translation

**Core idea.** Learn L1 from (English, atoms) pairs. Teaching and requests are parsed by a small trained model owned by the agent. Haiku's roles are to produce supervision (paired data, batched) and to act as a logged runtime fallback when the parser is unsure. Every fallback answer that passes the validator becomes a new training pair. Over time, the teacher is distilled into the agent.

**Design, kept deliberately small:**

- **Segmentation** stays as it is: a generic mechanism, with its abbreviation list as data.
- **Act classifier** (request, ask_need, ask_duration, inform-lack, teach_step, teach_needs, teach_time, social, …). Averaged perceptron over token n-grams and position features. Trained on the request lines (section 1.2), the agreed silver calls (section 1.3) and generated pairs.
- **Role tagger for teach clauses and requests.** Spans labelled ACTION / OBJECT / TOOL / UNTIL / TIME / ITEM / TASK. A CRF or perceptron with features such as the previous word, word shape and position. The prepositions that 2.0 hard-coded by action (`tasks.rs:148-153`) become **learned feature weights**.
- **Symbolisation**: span to symbol by the existing grounding rules (`validate.rs`), so every constant is still grounded in the message.
- **Lexicon induction** as an alternative or complement: align words to atoms across pairs, as in unification-based CCG lexicon learning (Zettlemoyer & Collins 2005; Kwiatkowski et al. 2010) or the simpler template induction already in `translate/learned.rs`, but with slots aligned to gold atoms instead of to Haiku's output.
- **Data generation following "overnight" parsing** (Wang, Berant & Liang 2015): generate canonical utterances from ground-truth atoms with a tiny grammar, have Haiku paraphrase them in batches, and train on the paraphrases. This is the cleanest use of "Haiku is the teacher" for L1.

**Pipeline change.** Translate becomes tier 0: the learned parser with a confidence score. Tier 1 is Haiku, only below the confidence threshold, and its output becomes a training pair. The fixed patterns and `implied_request` are removed. The validator stays as the last gate.

**Fixes.**

- F6 and F15: storage becomes a learned, tested function.
- F10: deterministic at inference, so the same message always gets the same answer.
- F3: request detection learned from labelled requests.
- F11: symbols come from aligned spans, not free generation.
- Removes about 40 of the hard-coded lists. HOW, CUES, BEFORE/AFTER, ORDER_WORDS, HEDGES and the preposition lists become features.
- Makes H3 nearly free to run, so it can be large (section 2.2, item 7).

**Cost.**

- Build: M, about 1,500-2,500 lines of Rust. A perceptron and a CRF need no external crates. Alternatively, train offline in Python and load the weights.
- Calls: about 100-300 for 3,000-10,000 paired examples at 20-50 per call, plus fallback calls at run time.

**Test.**

- Train on the T3 and 55-scenario pairs plus generated data. Test on H3's teach lines and requests (sealed), the 626 archive lines, G3, and a human-written slice (section 1.8).
- Metrics: exact storage of a step line; request recall; task-symbol accuracy.
- Compare against the Haiku translator on the same lines (archived answers exist for the 2.0 lines) and against a hand-written frame grammar.

**Main risk.** It generalises only within the phrasing distribution of its training data. Free user English with idioms falls back to Haiku, so the fallback rate must be reported, and it is the honest measure of L1 coverage. A second risk is philosophical: a perceptron is statistical. But it replaces a much larger statistical component (Haiku) in the same layer, and reasoning stays symbolic. Section A9 draws the line.

### A2. The teacher writes structured facts: the LLM as annotator or compiler

**Core idea.** Remove L1 from the teaching channel. The teacher (Haiku) emits atoms directly, in the closed registry vocabulary, with English beside them. Or the teaching dataset is atoms from the start. The agent validates and stores the atoms, and still owns ids, identity and integrity. English parsing remains only on the user channel (requests).

**Pipeline change.** For teaching, translate becomes validate, then store. The rest is unchanged. For the curriculum, the 2.0 round trip (atoms → English → Haiku → atoms) collapses.

**Fixes.**

- F6 and F15 completely, by construction.
- Most of the call cost.
- The noise that made the second encounter measure storage (R2 §11.10).
- It isolates L3: what the agent *does* with knowledge.

**Cost.** S. This is close to D3's "oracle parser" ablation, promoted to a first-class channel.

**Test.** Same H3, with teaching delivered as atoms. Compare against the English channel. The gap is the cost of L1. It is the cleanest decomposition the final PDF can report: *pass = P(stored right) × P(answered right | stored right)*.

**Main risk.** As the only channel, it turns "learning from teaching dialogues" into "learning from structured instruction" (machine teaching). The language-understanding half of the research question disappears from teaching. *Recommendation:* use A2 as the **data channel for L3 experiments and the dataset**, and A1 as the English bridge, measured separately. Not A2 alone.

### A3. Program synthesis and inductive logic programming

**Core idea.** Procedures are programs: HTN methods with preconditions, operators with effects. Adaptation knowledge consists of **induced rules**. Three things can be induced:

1. **Affordances**: `can_fill_role(Item, Action, Role)` from many procedures. This generalises the 2.0 analogy mechanism into a rule with evidence ("X has done A's tool role in N tasks").
2. **Situation modifiers** from near-miss pairs (base procedure, situation, changed procedure), as D3 proposes. These are learned with real ILP (Popper, Metagol; Cropper & Dumančić 2022 survey), not a single-feature counter.
3. **Methods**: when two procedures for a task differ by a sub-sequence under a condition, a method choice with a precondition. HTN-Maker (Hogg, Muñoz-Avila & Kuter 2008) and CaMeL (Ilghami et al. 2002) learn these from traces.

With effects (`achieves(Step, State)`), "no kettle" becomes a **plan repair** problem. The operator "heat water" fails its precondition. The planner finds another operator that achieves the same state with the items at hand (van der Krogt & de Weerdt 2005). This generalises item swaps without word lists, and it handles 2.0's "drop a step whose tool is missing" as a consequence instead of a special case.

**Pipeline change.** The vault gains `achieves`, preconditions and methods. The reasoner gains a small plan-repair search, which is bounded: at most 16 steps per procedure (R2 §11.13). The learner is replaced by an offline ILP pass in consolidation.

**Fixes.**

- F7 for item and function swaps.
- Step replacement (the spare tube): `achieves(s3, tube_sound)` lets "fit spare tube" replace "find and patch".
- F12, because typed preconditions replace untyped special cases.
- The "extra ingredient" class of hard-coded rules.

**Cost.**

- Build: M-L. The representation is M. Plan repair is S-M. ILP offline is M if Popper is used as a tool.
- Teaching: effects must be taught or induced. The ground truth has none (section 1.5), so a "so that…" relation needs authoring.

**Test.**

- Atom space, zero calls: leave-one-task-out on the atomised variants, and affordance prediction for the 20 "no X" variants (is the expected substitute ranked first?).
- Then H3's T-situation and T-task tiers.

**Main risk.** Data sparsity. ILP needs repeated structure, and the natural variants barely repeat (section 1.4). Induced rules with support 1 are memorised cases written as rules. Effects may also be as costly to teach as the adaptations they are meant to derive.

### A4. Case-based and analogical reasoning as the core

**Core idea.** Do not try to state the general rule. Store **cases**: (task, situation, procedure) and adaptation cases (base, situation, diff). For a new request, retrieve the structurally closest cases. Map them by structure: actions, objects, roles and effects, not words (SME, Falkenhainer, Forbus & Gentner 1989; MAC/FAC, Forbus, Gentner & Law 1995). Transfer the diff through the mapping. When the same diff maps cleanly across k cases, **generalise** it into a rule (analogical generalisation, SAGE: McLure, Friedman & Forbus 2015). Rules then grow from cases on demand. This is the CBR cycle (Aamodt & Plaza 1994), and the "adaptation container" that R2 §2.13 already borrows is Richter's (1995) term from the same field.

**Pipeline change.**

- The vault gains a case memory. Procedures already are cases, and variants become adaptation cases.
- Task identity becomes structural retrieval plus confirmation, instead of word overlap. This addresses R2 §18.4 question 7 directly.
- The reasoner still derives and proves. The mapping is an explicit structure, so a proof can say "adapted from `make_salad` / no lemon → vinegar, mapped lemon ↦ lime by role `acid_in_dressing`".

**Fixes.**

- F7, wherever a similar case exists.
- F4 and F5, by structural identity: "jacket potato" shares the oven, potato, prick and bake structure with `bake_potato`.
- The provenance of every transfer is shown, which keeps the honesty principle.
- It fits the evidence: the one adaptation mechanism that worked broadly in 2.0 was analogy from other procedures (R2 §17.3).

**Cost.**

- Build: M. A structure mapper over graphs of at most about 100 nodes is a few hundred lines with greedy SME-style matching.
- Calls: zero at run time.

**Test.** Atom space first: leave-one-case-out on the atomised variants, with a retrieval-only null. Then H3, with T-task reported separately for "a similar case exists" and "no similar case".

**Main risk.** Analogies misfire. "Sprinkle the bike" was a hand rule, but a careless mapper produces the same class of error. *Mitigations:* typed mapping, a minimum structural-evaluation score, a confirmation question below threshold, and negative tests from the 2.0 misfires. A second risk: similarity features can smuggle in hand lists. They must be computed from atoms only.

### A5. Graph representations, learned embeddings, a small neural model

Three versions, from least to most invasive:

- **A5a. Embeddings as a retrieval prior, trained on the agent's own vault.** Node2vec-style embeddings over the procedure graph (items linked by shared action-role slots). They rank substitute and task-identity candidates, and the symbolic layer verifies them, with a proof and a confirmation question. "Learned from what it was taught" holds. With about 100 procedures, the signal is weak. It is only as good as A3's affordance counts, and less explainable.
- **A5b. Pretrained word vectors** (GloVe, fastText) as similarity for identity ("painting" ≈ "picture"). These are cheap and effective for F4. But this is imported knowledge, of the same kind as ConceptNet import (R2 option B). It must be a labelled, ablated arm, and it weakens the "learned" claim.
- **A5c. A small neural model as the whole agent** (a fine-tuned seq2seq from request to answer). Not recommended. One-shot learning from a single teaching dialogue is exactly what weight-based learning is bad at, and what symbolic memory does well (R2 §17.1). There are no proofs and no honest gaps, and it breaks the project's core principle. It is useful only as a **baseline** in the PDF. The Haiku-in-context baseline (section 2.2, item 10) serves that role better, at no training cost.

**Main risk of A5a/b.** Opaque similarity becomes the deciding factor, and failures are hard to explain. Keep it a prior, never a decision.

### A6. Content-first output: structured relations, a dumb template, a symbolic check

**Core idea.** In 3.0 the reply **is** the content plan, serialised in a fixed relation format, plus at most a telegraphic rendering. Example:

```
TASK water_plant  [taught 1x, hypothesis]
NEED jug INSTEAD-OF watering_can  BECAUSE lack(watering_can)  VIA user_proposal
STEP 1 check soil WITH finger UNTIL dry
STEP 2 fill jug WITH water
STEP 3 pour water ONTO soil WITH jug UNTIL drains  AFTER 2
STEP 4 empty saucer  AFTER 3
GAP takes_time(step 3) unknown
```

The telegraphic form, generated by one generic template per relation with no word lists, would read: "need jug instead of watering can (you lack watering can). 1 check soil with finger until dry. 2 fill jug with water. 3 pour water onto soil with jug until drains. 4 empty saucer. time of step 3: unknown."

**Pipeline change.**

- The nlg crate (1,166 lines, the `MASS` list, plurals, frames, aggregation) and the help part of `tasks.rs` are replaced by a serialiser of about 200 lines.
- The content check reads the structured form exactly. This is the same idea as the slot error rate used to score templated data-to-text systems (E2E challenge, Dušek, Novikova & Rieser 2020): content is checked on the plan, not judged on the prose.
- An independent reader (Haiku extraction on a sample) checks the **telegraphic** form, so the paper can show that a reader recovers every relation from the text.

**Fixes.**

- Faithfulness by construction (R2 §9.5.2).
- The last polish surface. No articles, plurals or prepositions are chosen. The relation markers are fixed keywords.
- The about 225 English strings of the help path (OPT §1): most of them are realiser strings.
- The read-back circularity (section 2.2, item 9).

**Cost.** S. It also deletes code.

**Test.** The content check on H3 is exact. A sample with the independent reader measures how much survives telegraphic rendering.

**Main risk.** It reads like a log, not a reply. The user has accepted this for 3.0. A minor risk: the user later wants English, and the serialiser offers nothing to build on. That is acceptable, because human-like output is "much later" by the user's decision.

### A7. Self-supervised learning from a big generated corpus; curriculum versus dataset

**Core idea.** Generate a large corpus. Then be precise about **which learning problem it feeds**, because the answer differs:

| Corpus use | What is learned | Generalises? | Test |
|---|---|---|---|
| Paired English–atoms (A1) | L1, the parser | Yes, within the phrasing distribution | Parser accuracy on H3 lines and the human slice |
| Procedures as knowledge (D3's C2a) | Coverage only | No (memorisation) | Report as coverage, not learning |
| Many procedures, atoms only | Affordances, kinds, script structure | Plausibly: function and role regularities repeat across tasks | Leave-one-task-out affordance and substitute prediction |
| Situation pairs, atoms only | Modifiers and methods (A3/A4) | Only where situations repeat by design (section 1.4) | Tiers plus a null baseline |
| Self-supervised: hide one step, predict it | Procedural priors (scripts; Regneri et al. 2010 induce script graphs by aligning many event sequences; Chambers & Jurafsky 2008) | Partly | Masked-step recall; but only as hypotheses with hedges, never as claims |

**Curriculum versus dataset.**

- For a symbolic learner that counts evidence over distinct examples, the **order** of examples matters little, except for identity (aliases learned early prevent drift).
- The curriculum's value is that it tests the conversational path: understanding, asking and confirming.
- The dataset's value is volume.

So I would not frame C1 versus C3 as the main experiment, as D3 does. Frame it as **learning curves**: performance as a function of the number of distinct examples per learning problem (L1 pairs; procedures, for affordances; situation pairs, for modifiers), on fixed held-out splits. The dialogue path is then the end-to-end check that knowledge learned from the dataset can be used in conversation.

**Main risk.** One author family: Haiku generates, a Claude sub-agent writes H3. Shared priors inflate T-domain. Mitigations: algorithmic splits (section 2.2, item 8), the human slice (section 1.8), and the null baseline.

### A8. Active learning: the agent chooses what to ask

**Core idea.** The agent's uncertainty, not a fixed protocol, chooses the next teaching question.

- **During teaching:** after storing a procedure, ask the one question whose answer would most change future answers. Examples: an effect that would license substitutions for the most-used tool; a condition on the longest waiting step.
- **Over the dataset pool:** when two induced hypotheses disagree (for example, "rain adds an indoor alternative" for all outdoor events, or only for picnics), ask a membership query on a case that splits them ("Does rain change a barbecue?"). This is query-by-committee, or version-space shrinking (Angluin 1988; Settles 2009 survey). In ITL terms (Laird et al. 2017; Rosie, Mohan et al. 2012), the agent drives its own instruction.

**Pipeline change.** Curiosity (`agent/curiosity.rs`) is replaced by a question selector over hypothesis disagreement. It stays offline in training and online only as one ranked question per gap.

**Fixes.**

- 2.0's curiosity waste: 58 of 67 answers were "I don't know." (R2 §14.1, F14), once the questions are named by checked symbols.
- The T-gap move.
- It reduces teaching cost, which is measurable.

**Cost.** M. **Testable with zero calls**: a symbolic teacher that answers from atomised ground truth can answer membership queries.

**Test.** Learning curve against number of questions, compared with random example order and with the fixed elicitation protocol. Headline metric: "teaching lines per correct transfer".

**Main risk.** Question explosion, and questions a real user cannot answer. Cap questions per task, and measure "answerable" as 2.0 did.

### A9. A hybrid neuro-symbolic split, stated as a boundary

The project has drifted into an unstated boundary: "the LLM only translates", while the translator quietly decided tasks, requests and ids (R2 §14.8 risk 1). 3.0 should state the boundary as a table and test it in code.

| Layer | 3.0 | Why |
|---|---|---|
| Memory: vault, procedures, cases, provenance | **Symbolic** | One-shot storage, editability, proofs; the part 2.0 showed works |
| Identity decisions: which task, alias, new task | **Symbolic decision** over learned or statistical candidate scores, with a confirmation question below threshold | Wrong-task replies were the main content error (193 replies, R2 §14.1); the decision needs a proof |
| Reasoning and adaptation: derivation, plan repair, analogy | **Symbolic** | Honest gaps, typed guards, explanations |
| Content plan and output | **Symbolic**, serialised (A6) | Faithful by construction |
| English understanding | **Learned statistical** (A1), with Haiku as a logged fallback | Variability lives here; errors here are recoverable by validation and confirmation |
| Similarity priors for retrieval | **Learned from the vault** (A3 counts, optionally A5a) | Ranks candidates only, never decides |
| Teacher, data generator, independent reader | **LLM (Haiku)**, training and evaluation only | The user's constraint |
| Rule and case generalisation | **Symbolic induction**, offline (ILP or analogical generalisation) | Evidence counted, hypotheses first |

The rule: **anything statistical may rank or propose. Only symbolic layers may decide, store or claim.** A code test can check it. Statistical components return ranked candidates with scores, never acts that bypass the validator. That is the 2.0 `implied_request` failure (R2 §9.5.3) made impossible by type.

### A10 (added). A retrieval-and-recite agent as a control

This is not an architecture to adopt. It is a control arm that should exist in Phase B: oracle atoms stored, retrieval as in [M3], the retrieved procedure printed in A6 format, with no adaptation and no dialogue. It sets the floor for T-word and retention. It shows how much of 2.0's 75-90% development-set second encounter was identity plus storage. It costs zero calls. If the full 3.0 agent does not beat it clearly on T-situation and T-task, the adaptation machinery has not earned its place.

---

## 4. Comparison

Scale: ++ strong, + good, 0 neutral, − weak, −− poor. "Cost" covers both build effort and model calls; ++ means cheap. "Risk" is inverted: ++ means low risk. "Research value" means what it adds to the final PDF.

| Alternative | Generality | No hard-coded English | Content over polish | Cost | Risk | Research value |
|---|---|---|---|---|---|---|
| D3 as drafted (repaired 2.0) | + | − (hand frame grammar, seed lexicon, realiser frames) | + | − (15-25k calls, L build) | 0 | + |
| **A1** learned parser | + (within distribution) | **++** (frames and cues become learned weights) | 0 | + (M build; 100-300 calls) | + | **++** (L1 learning curve; distilling the teacher) |
| **A2** teacher writes atoms | 0 | ++ (no parsing in teaching) | 0 | ++ | + (but narrows the question) | + (clean decomposition; best as an arm) |
| A3 ILP / programs / plan repair | ++ where structure repeats | ++ | + | − (M-L; effects need teaching) | − (sparse repetition, [M2]) | ++ |
| **A4** case-based / analogical | + to ++ | + (features from atoms) | + | + (M; 0 calls) | 0 (misfires) | ++ (CBR framing; provenance) |
| A5a/b embeddings | + | a: +, b: − (imported) | 0 | + | 0 | 0 |
| A5c neural agent | − for one-shot | − | − | −− | −− | − (baseline only) |
| **A6** content-first output | n/a | **++** (deletes nlg lists) | **++** | **++** (S, deletes code) | ++ | + (exact content measure) |
| A7 dataset, framed by problem | + | + | + | + (atoms: 0 calls) | 0 (one author family) | ++ (learning curves) |
| A8 active learning | + | + | + | + (M; 0-call testable) | 0 | ++ (teaching-cost curve) |
| A9 explicit hybrid boundary | n/a | + | + | ++ (a rule and a test) | ++ | + |
| A10 retrieval control | − (by design) | ++ | ++ | ++ | ++ | ++ (makes every result interpretable) |

---

## 5. Recommendation

### 5.1 What 3.0 should be

**Primary architecture: A1 + A4, on the A9 boundary, with A6 output and A10 as a control.**

```
 message ─► segmenter ─► learned parser (A1) ──conf<θ──► Haiku fallback (logged → new training pair)
                              │                                   │
                              └────────► validator (last gate) ◄──┘
                                              │ acts + atoms (never decisions)
                     dialogue policy (Datalog, every move with a proof)
                       │ identity = structural retrieval (A4) + confirm below threshold
                       ▼
     vault: procedures + cases + provenance ◄── offline consolidation: case → rule (A4/A3-lite),
                       │                         affordance counts, alias proposals
     reasoner: derive · adapt by mapped case · minimal plan repair on `achieves`
                       ▼
     content plan ─► serialiser (A6: relation lines + telegraphic form)
                       ▼
     content check vs ground truth (exact) · independent reader on a sample
```

Why this pair:

- **A1 addresses the failure that cost most** (storage and request loss, R2 §11.10, §14.1). It does so by learning, which is the user's binding direction, not by a better hand grammar. It also makes the teacher's role meaningful: Haiku teaches the parser. And it makes evaluation nearly free, which buys the sample sizes the tiers need.
- **A4 addresses transfer with what 2.0 showed works (analogy) and fits the data's shape.** Variants are mostly unique [M2], and cases transfer from one example where rules need many. Rules still appear, but only when cases repeat, which is the honest behaviour for sparse data. A3's affordance counts and a minimal plan repair over `achieves` are taken in as parts of A4, not as a separate ILP system.
- **A6 is nearly free** and closes the polish channel for good.
- **A2 runs as an arm, not as the system.** Teaching as atoms gives the decomposition stored-right × answered-right, and the atoms-first L3 experiments.
- **A8 is the stretch goal.** It is cheap to test once the symbolic teacher answers from atomised ground truth, and it produces the paper's most interesting curve. But it depends on identity working first.

What I would **not** do: build a Popper-class ILP system in Rust (prototype offline only), write a hand frame grammar as the primary teaching path, keep an English realiser, or keep the runtime Haiku translator as the primary path.

### 5.2 Phase A experiments that decide this cheaply

All except X7 use only files in the repository and call no model.

| # | Experiment | Data | Calls | Decides |
|---|---|---|---|---|
| **X1** | **Learned teach parser.** Train a perceptron role tagger plus act classifier on the 55 training scenarios' teach lines (gold atoms). Test on the 20 development scenarios' lines and on the 626 archived L-train lines, scored against ground truth. Compare with Haiku's archived answers on the same lines and with a 1-day hand frame grammar | `data/curriculum/*`, `live/2026-11-18T05-11-26.jsonl`, `calls/` | 0 | A1 against the hand grammar for teaching |
| **X2** | **Learned request detection and identity.** Train on the 55 training scenarios' request lines plus agreed silver calls. Test on the 20 development scenarios' requests, G3-old (200 + 50) and variant requests. Metrics: request recall, task top-1 against the [M3] retrieval baseline (176 of 225) | scenarios, `calls/`, `data/gold/` | 0 | A1 on the user channel; whether Haiku stays primary |
| **X3** | **Atomise the 52 variants** into (situation atoms, diff atoms), done by Claude Code from the English `change`/`expect` lines, plus alias sets for every ground-truth atom | scenario files | 0 Haiku (Claude Code authoring) | A prerequisite for X4, X5 and E1 transfer scoring |
| **X4** | **Adaptation core shoot-out in atom space**, leave-one-task-out on the X3 cases: (a) null (base procedure), (b) A4 case transfer by structure mapping, (c) affordance-count substitution (A3-lite), (d) rules from Popper offline. Metrics: diff recall, wrong-claim rate, honest-gap rate on cases with no analogue | X3 output, 105 ground truths | 0 | A4 against A3; whether rules add anything |
| **X5** | **Situation noticing, knowledge-gated.** Extend [M4]: flag a request qualifier only when it links to a vault element that varies (from X3). Measure false alarms on 225 base requests and hits on 52 variants | scenarios, X3 | 0 | Whether D3's blindness and false-alarm targets are reachable without cue lists |
| **X6** | **Content-check feasibility.** Re-score the 2.0 C9 transcripts by reading the reply text into atoms (claims were not recorded, section 1.7), with alias sets from X3. Report graded recall, strict pass and alias-only failures. Join with the 712 Haiku-fail/symbolic-pass notes to measure how much checklist content lies outside the atoms (section 1.9) | `curriculum/*.jsonl` | 0 | The scoring rules; ground-truth schema changes (detail atoms; compound steps) |
| X7 | **Supervision pilot, with the user's go-ahead.** 300 (English, atoms) paraphrase pairs and 50 situation pairs from Haiku in batches; retrain X1 and X2; measure the gain per 100 pairs | new | about 15-30 | Whether generated supervision helps L1; dataset size for Phase B |
| X8 | **Algorithmic split.** Compute a maximum-compound-divergence split of the 105 ground truths plus X3 cases. Compare its difficulty (X4 metrics) with the hand tiers | ground truths | 0 | Whether H3 uses authored tiers, algorithmic splits or both |

### 5.3 Decision rules (to be pre-registered in Phase A)

1. **Teaching parser (X1).**
   - If the learned parser stores at least 95% of development step lines exactly **and** beats Haiku's archived storage on the 626 lines, adopt A1 for teaching. The hand grammar becomes an ablation only.
   - If it lands between 85% and 95%, adopt A1 with Haiku fallback below the confidence threshold. Report the fallback rate as L1 coverage.
   - Below 85% after X7's data, fall back to D3's frame grammar. State in the PDF that L1 was not learned.
2. **User channel (X2).**
   - If learned request recall is at least 95% and task top-1 beats the retrieval baseline by at least 10 points on development requests, Haiku becomes the fallback only.
   - Otherwise Haiku stays primary for requests, and the learned layer becomes a cue feature for the policy. That is D3's plan, but with learned cues.
3. **Adaptation core (X4).**
   - Choose A4 if case transfer beats the null by at least 15 points of diff recall **and** its wrong-claim rate is at most the null's.
   - Add Popper-induced rules only if they add at least 5 points over A4 on cases with support 2 or more.
   - If nothing beats the null except affordance substitution, 3.0's transfer claim is "function-based substitution plus honest gaps". The T-task tier is then reported as a designed-modifier test, not as evidence of general learning.
4. **Noticing (X5).** If knowledge-gated noticing reaches at most 10% false alarms on base requests, keep the blindness target. Otherwise replace it with "ask on any linked qualifier" and measure the question rate.
5. **Output (A6).** No experiment is needed. Adopt it unless the user wants an English realiser in 3.0 (decision Q4 below).

### 5.4 What this does to Phase B, in brief

The phase B ordering of D3 mostly holds. The changes:

- Milestone 24 (ingestion) becomes "learned parser plus fallback distillation".
- Milestone 26 (realiser) shrinks to the serialiser.
- Milestone 27 (adaptation container) becomes "cases, structure mapping, affordances, minimal plan repair".
- Milestone 28 (rule learner v3) becomes the offline case-to-rule consolidation, cut if X4 says rules add nothing.
- H3 grows to 150-300 scenarios, because readings cost few calls. Four readings remain.
- The final PDF reports L1, L2 and L3 separately, with learning curves, the A2 arm, the A10 control and the Haiku-in-context baseline.

---

## 6. New open decisions for the user

1. **What counts as "the agent learning"?** Does learning the language layer (A1: the agent learns to parse from the teacher) count toward the research question, or only learning world knowledge (L3)?
   *Options:* (a) both count, reported separately (recommended); (b) only L3 counts, and the language layer is infrastructure.
2. **The teaching channel.** May teaching arrive as atoms (A2) for the dataset and for L3 experiments, or must all knowledge pass through English?
   *Options:* (a) English only; (b) atoms for the dataset and as an arm, English for the dialogue path (recommended); (c) atoms everywhere.
3. **Small learned statistical components.** May the understanding layer contain trained models (perceptron or CRF; embeddings trained on the agent's own vault), if they only propose and never decide?
   *Options:* (a) yes, own-data only (recommended); (b) yes, including pretrained word vectors as a labelled arm; (c) no, symbolic only, with Haiku as the only statistical part.
4. **The output format in 3.0.**
   *Options:* (a) relation lines plus a telegraphic form (recommended); (b) relation lines only; (c) D3's plain English realiser with read-back.
5. **The runtime Haiku translator.**
   *Options:* (a) primary, as in 2.0 and D3; (b) fallback below the confidence threshold, each call logged as training data (recommended); (c) training-time only, so the agent runs offline.
6. **The adaptation core.**
   *Options:* (a) rules over task kinds (D3); (b) cases with structure mapping, with rules only from repeated cases (recommended, pending X4); (c) both as equal arms.
7. **The ground-truth schema.** Should the ground truth be re-authored to (i) carry alias sets, (ii) split compound teach clauses into steps or keep head-action abstraction, (iii) add detail atoms and effects (`achieves`)? Who authors it?
   *Options:* (a) a Claude Code sub-agent from a spec, reviewed by the linter; (b) you review a sample of 10.
8. **A human-written test slice.** Should 50-100 wikiHow step sentences (CC BY-NC-SA), atomised by a sub-agent, form a sealed L1 test of English that nobody in the loop wrote?
   *Options:* yes / no / another source you prefer.
9. **Baselines in the final PDF.**
   *Options:* (a) the retrieval control only (0 calls); (b) plus Haiku-with-teaching-in-context on H3 (about 300-600 calls with an extraction pass; recommended); (c) neither.
10. **Held-out construction.**
    *Options:* (a) hand tiers from a spec (D3); (b) algorithmic compound-divergence splits; (c) both, with algorithmic splits as the primary L3 test (recommended).
11. **Offline tools in another language.** May Phase A and B use Python tools offline (Popper with clingo; training scripts) whose outputs are loaded by the Rust agent, or must everything be Rust?
    *Options:* (a) offline Python allowed, runtime Rust only (recommended); (b) Rust only.
12. **Active learning (A8).**
    *Options:* (a) in scope as a stretch milestone, tested with the symbolic teacher; (b) out; (c) core.
13. **The headline learning curve.** Which curve leads the final PDF?
    *Options:* (a) pass rate against amount taught (D3); (b) teaching lines needed per correct answer or transfer (teaching cost); (c) both.

---

## Appendix A. The offline measurements in this note

All four ran with `python3 -I` over files in the repository. They read the scenario Markdown (`## teach`, `## ground truth`, `## requests`, `## variant`), with no model and no network. The scripts were scratch files and are not committed. The methods below are enough to rewrite them in about 60 lines each.

- **[M1] Teach-line alignability.**
  - Data: the 75 curriculum scenarios (`scenarios/` plus `training/`). A step line is one matching `Step N of X: clause`, optionally prefixed by `While step K runs,`. It is aligned to the atoms of `<task>_sN`.
  - An atom symbol is "present" when every `_`-part longer than 2 characters matches, on its first 4 letters, some word of the clause. A step time is present when its low number appears in the clause.
  - Results: 361 step lines and 150 other teach lines. Present: action 351/361, object 346/361, uses 299/311, until 42/44, time 312/312.
  - Compound clauses (the words "and" or "then" before " for "): 103 of 361.
  - Ground-truth predicates counted over the same 75 files: only the ten listed in section 1.5.
  - 164 distinct actions, 92 used once.
- **[M2] Variant distribution.**
  - The 52 `change:` lines of the 75 scenarios.
  - Counts: 20 begin "no ", 20 contain "instead" or "in place of", 6 contain "minutes", 3 contain "skip".
  - The repetition count (section 1.4) is by hand reading.
- **[M3] Atoms-only task retrieval.**
  - Each of the 105 tasks (75 scenarios plus 30 seeds) is a bag of 5-letter stems from the parts of its `task`, `needs`, `action`, `object_of`, `uses` and `until` symbols.
  - A request scores each task by the sum of the idf (log N/df over task bags) of the request stems found in the task's bag. There is no stopword list: frequent stems get low idf.
  - Top-1 is correct for 176 of 225 base requests and 42 of 52 variant requests.
- **[M4] Lexical situation noticing.**
  - A request word (longer than 2 characters) is "unexplained" when its 5-letter stem is in no atom of the true task and occurs in at most *t* of the 277 requests.
  - At t = 2, 3 and 5: variants flagged 47, 49 and 49 of 52; base requests flagged 169, 194 and 204 of 225.
  - A contrastive version also ignores stems seen in the task's other base requests. It flags 48 of 52 variants, and 203 of 225 base requests in a leave-one-out test.

## Appendix B. Related work used here, and what each gives us

Cited from memory and not re-checked by web search for this note. Several are also in R2 §18's checked reference list.

| Work | What it gives 3.0 |
|---|---|
| Zettlemoyer & Collins 2005; Kwiatkowski et al. 2010 (CCG lexicon induction) | Learning a lexicon and grammar from sentence–logical-form pairs: the A1 alternative to a hand frame grammar |
| Wang, Berant & Liang 2015, "Building a semantic parser overnight" | Generate canonical forms from a grammar, collect paraphrases, train: the recipe for Haiku as teacher of the parser |
| Collins 2002 (averaged perceptron); Lafferty et al. 2001 (CRF) | Small, dependency-free sequence taggers for role labelling, trainable in Rust |
| Hsieh et al. 2023, distilling step-by-step | Distilling an LLM into a small task model with limited data: the fallback-to-training-pair loop |
| Aamodt & Plaza 1994; Richter 1995 (knowledge containers) | The CBR cycle and the "adaptation container" vocabulary already used by R2 |
| Falkenhainer, Forbus & Gentner 1989 (SME); Forbus, Gentner & Law 1995 (MAC/FAC); McLure, Friedman & Forbus 2015 (SAGE) | Structure mapping for retrieval and transfer; analogical generalisation from cases to rules |
| Cropper & Morel 2021 (Popper); Cropper & Dumančić 2022 (ILP survey) | Data-efficient rule induction with negative examples, usable as an offline tool |
| Hogg, Muñoz-Avila & Kuter 2008 (HTN-Maker); Ilghami et al. 2002 (CaMeL) | Learning task methods with preconditions from traces |
| van der Krogt & de Weerdt 2005 (plan repair) | Adapting a plan when a precondition fails, the general form of item swap and step replacement |
| Dušek, Novikova & Rieser 2020 (E2E challenge) | Slot error rate: scoring content of templated output on the plan, not on prose (A6) |
| Keysers et al. 2020 (CFQ, MCD splits); Kim & Linzen 2020 (COGS) | Building held-out splits algorithmically by compound divergence |
| Regneri, Koller & Pinkal 2010; Chambers & Jurafsky 2008 | Inducing script structure from many event sequences: what a big procedure corpus can teach |
| Angluin 1988 (queries); Settles 2009 (active learning survey); Laird et al. 2017 (ITL); Mohan et al. 2012 (Rosie) | The agent choosing its own questions; membership queries to split hypotheses |
| Koupaee & Wang 2018 (WikiHow dataset) | A human-written source of procedure sentences for an L1 test slice |
