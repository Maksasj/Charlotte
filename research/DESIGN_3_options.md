# Design 3.0: results, problems and options to weigh

*2026-11-18. A summary for choosing what Design 3.0 should be. Sources: `research/REPORT_DESIGN_2.md` (sections 1-2, 9-11, 14, 17-19), `research/REPORT_DESIGN_1.md`, `research/Symbolic Learning Agent – Design 2.0.md` and `PROGRESS.md`. A few claims were checked against the code. Nothing was run. The full draft is `research/DESIGN_3_draft.md`.*

How to read it:

- Section 1 gives the numbers we can trust.
- Section 2 sets out **the three axes the user raised**. They are the main choices for 3.0: (A) learning instead of hard-coded heuristics, (B) evaluating learning instead of presentation, (C) where the knowledge comes from (dialogue, an example dataset, or both).
- Sections 3-5 cover the remaining problems, the solution catalogue and the smaller choices.
- Section 6 covers the items 2.0 put out of scope.
- Section 7 is my recommendation.

Effort: **S** is days, **M** is one to two weeks, **L** is several weeks of autonomous build. Expected effects are estimates, not measurements. Intervals are Wilson 95% over scenarios.

---

## 1. Key results, honest numbers only

**v1 (2026-11-16).**

- Fixed tests passed at 98-100%, while the conversation judge stayed at 2.4-4.3 out of 10.
- 0 or 1 of 68 help replies was a real answer, and 44% were not understood.
- Re-labelled with 2.0's labels, v1 replies are "good" in 7.8% of 296 turns [5.2, 11.4].
- 73 rules were guessed and 1 was confirmed.

The lesson: the missing piece was representation, not reasoning.

**v2 (tag v2.0.0).** Three kinds of number exist, and they must not be mixed.

| Measure | Clean held-out (run 1 only) | Development set (last runs; contaminated) | Untuned training stage (55 new tasks, morning build) |
|---|---|---|---|
| Second encounter, Haiku checklist | **6 of 20** (30%, 15-52) | 15-18 of 20 | 19 of 55 (35%, 23-48) |
| Second encounter, symbolic check (lenient) | not measured | 18-20 of 20 | 43 of 55 (78%, 66-87) |
| Transfer | **0 of 15** (0-20) | 6 of 15, all item swaps | 0 of 18 after variant teaching |
| Retention | **5 of 20** | 14-17 of 20 | 11-13 of 55 |
| Helpful + clarify, request turn | 10.1% | 58-63% (official 74-80% counts "Thanks!" turns) | |
| Teach step lines stored correctly | | | 440 of 626 (70%) |

What else the data says:

- **The second encounter mostly measures storage.** When the in-session retry passed, E2 passed 97% of the time (476 of 489); when it failed, 14% (40 of 281).
- **Training never moved a held-out number.** No task is shared, and no training run used the final code.
- **Transfer stops at item swaps.** The 9 time, method and plan variants passed in none of the 39 runs.
- **The translator is the main agent-side failure.** 20% of request-cue calls had no request act, and 71% of repeated messages got more than one translation.
- **Wording moved the judge.** The 1-10 score rose from 3.6 to 5.4 by wording alone; no conversation scored above 7.
- **Cost.** 12,626 Haiku calls. About 2,000 calls fill one plan session.
- **Hand-written gains.** The afternoon gains came mostly from about 225 English strings in hand-written Rust lists and special cases, tuned on held-out transcripts.

**What we can honestly claim.** A symbolic agent can learn a task in one teaching session and reuse it later in new words, when the teaching is stored correctly. That happened about 70% of the time on new tasks. Item swaps transfer through five mechanisms. Nothing else is shown on clean data.

---

## 2. The three axes for choosing 3.0

### Axis A: learn the heuristics, do not code them

**The problem.** 2.0 broke its own principle 5, "knowledge is data, not code":

- 78 `const` word lists and many inline `matches!` lists in the agent crates: tools, containers, surfaces, waiting actions, lack cues, how-cues, generic verbs, state verbs, mass nouns, prepositions by action;
- special-case mechanisms such as the "extra ingredient" rule.

These lists produced most of 2.0's late gains. They also produced its over-fitted misfires, such as "Sprinkle the bike over the tires". A teacher cannot correct them, a proof cannot show them, and the learner never touches them. If they stay, 3.0 would measure the programmer, not the agent.

| Option | What it means | Pros | Cons |
|---|---|---|---|
| A1 Keep patching in code | As 2.0 | Fastest gains on a known set | Not learning; over-fits; fails the research question |
| A2 Move the lists into the lexicon as data | Same entries, as `tool(lever)` facts, editable and correctable | Visible, correctable, in proofs; S effort | Still hand-written; still not learned |
| A3 **Derive or learn them** | Derive from knowledge where possible (`tool(X) :- uses(S, X)`, `waiting_step(S) :- takes_time(S, ...), until(S, ...)`); learn the rest (lack and request cues, frames, kinds) from examples, as hypotheses with evidence | The agent's behaviour comes from what it was taught; heuristics generalise to new words | Behaviour drops at first; needs examples (axis C) and a real rule learner; M-L effort |

**The boundary to draw.** The code may hold:

- the core formal language;
- the teach-frame grammar of the teaching channel;
- segmentation;
- generic mechanisms (substitute, analogy, modifier application, rule induction).

The code may not hold English words about the world or the task. A test fails if an agent crate gains a word list. A start-up seed of lexicon facts is allowed, but it is data with `@source seed`, counted and reported, and an ablation run removes it.

**How it is tested.**

1. Share of reply-changing decisions whose proof rests on derived or learned knowledge, not on seed data.
2. The precision of learned lexicon entries against a hand-checked sample.
3. Performance on a held-out domain whose words appear in no seed. Coded lists cannot cover it, so only learning can pass.

**Recommendation:** A3, with A2 as the migration step (the lists become seed data on day one, then derived or learned entries replace them).

### Axis B: evaluate learning, not presentation

**The problem.** The Haiku teacher and judges marked down wording, grammar and format, so much of 2.0's late time went into surface polish:

- echoing the user's framing (+1 to +1.5 points with identical content);
- step times shown only on waiting steps;
- prepositions by action;
- taught wording, which broke faithfulness.

The 1-10 score rewards style and plausible guesses. An honest "I don't know the time for soft-boiled eggs" scores like a wrong answer. The real question is whether the taught content came back and was used correctly, in new words and in a new situation.

**The output requirement for 3.0 (user decision).** Human-like answering is out of scope until much later. Rough text is fine: grammar errors, odd word order, missing commas or connectives. But the text **must contain every fact the answer needs, and every relation between them**:

- order;
- overlap ("while");
- cause and reason;
- condition ("if", "until", "when");
- which item or tool goes with which step;
- substitutions ("X instead of Y");
- quantities and times attached to the right step.

The text is a faithful rendering of the content, nothing more. So evaluation checks two things:

- **completeness**: every needed fact and relation is present;
- **correctness**: nothing is wrong or unsupported.

It never checks fluency.

**A fact that makes this cheap.** Every reply is built from claimed atoms. So content can be checked on the claims directly against the ground truth, with no model and no text parsing, provided the faithfulness test guarantees that the text says exactly the claims.

| Option | What it means | Pros | Cons |
|---|---|---|---|
| B1 Haiku checklist plus a 1-10 score (2.0) | As 2.0 | Comparable with 2.0 | Measures style; pushes toward polish and guessing |
| B2 **Symbolic fact-and-relation check on the claims** | The reply's claimed atoms are compared with the ground truth, alias-aware: needs, steps, order and overlap, step-item links, times per step, conditions, substitutions, the adaptation. Scored as recall (completeness) and precision (no wrong or unsupported claims). Honest gaps are scored apart from wrong claims | Measures learning exactly; deterministic; no calls; wording cannot game it | Trusts that the text equals the claims; ground-truth names must match (the alias problem) |
| B3 B2 plus a **text read-back check** | Checks that the text really carries those facts and relations. (a) Symbolic: the agent's own parser reads the reply text back into atoms, which must equal the claims (faithfulness, no calls). (b) A Haiku extraction pass on a sample: list every fact and relation the reply states, then compare symbolically. The model reads; it does not grade style | Catches text that drops a relation the claims hold ("with the jug" on the wrong step); rough text passes as long as nothing is lost | (b) costs some calls and has Haiku extraction errors, measured on anchors |
| B4 Haiku checklist or 1-10 score as a secondary column | For continuity with 2.0 | Bridges the iterations | Style pressure again; **moved to "later"**, not in 3.0 |

**Learning-centred metrics.**

- Second encounter, retention and transfer by content check.
- **Wrong-claim rate**: a wrong claim costs more than a gap.
- **Honest-gap rate** on situations taught nowhere.
- Teaching storage accuracy.
- Task-match accuracy from the trace.
- **Learning curves**: success as a function of how much was taught.
- **Generalisation tiers**: new words, new situation, new task, new domain.

No naturalness, wording or 1-10 score exists in 3.0. The only requirement on the text is that the read-back recovers every fact and relation.

**Recommendation:** B2 + B3 as the measure of record: fact-and-relation recall and precision. B4, the wording judge, framing and polish all move to a "later" section. For comparison, the paper re-scores the archived 2.0 transcripts with the same B2/B3 check, so the iterations stay comparable without any style judge.

### Axis C: where does knowledge come from: dialogue, an example dataset, or both?

**The problem.** 2.0 taught only through scripted teach lines in dialogue, one task at a time:

- 75 tasks;
- 30% of step lines stored wrong;
- no reusable adaptation knowledge;
- teaching thin by construction.

Each task costs translator calls and fix passes. The user's idea is to generate one large dataset of examples and learn from it. Whether a symbolic learner trained that way generalises and learns new things is itself an open research question. 3.0 can make it its central experiment.

| Option | What is learned, and how | Pros | Cons |
|---|---|---|---|
| C1 **Dialogue teaching only** (2.0, fixed) | Procedures and adaptation knowledge from teach lines and elicitation frames, task by task | Tests the real conversational path; the research question as first posed | Low volume; expensive per item; general rules are rarely seen enough times to be induced |
| C2 **Example dataset only** | A generated corpus of, say, 1,000-3,000 examples: procedures with kinds, situations and how each situation changed the procedure, request paraphrases, lack and request phrasings. Two uses: (a) **as knowledge**, stored as procedures (memorisation; generalisation is only coverage); (b) **as training signal**: the learner induces general rules from many examples (kinds, situational modifiers, lexicon classes, cue and translation patterns) | Volume, so rules can be induced; cheap to generate (batched); every example carries gold atoms, so it can be checked | C2a is bulk import, not learning. C2b needs a real rule learner (v1/v2 confirmed 1 and 6 rules). Synthetic regularity may inflate results ("Haiku English"); leakage between train and test |
| C3 **Mix** | The dataset induces general knowledge (C2b); dialogue teaches specific new tasks and corrections, and is the evaluation path | Each source does what it is good at; the comparison C1 vs C3 is the experiment | Two pipelines; harder to attribute effects |

**The dataset can enter in two forms.**

- **As atoms:** this isolates the learner, and its evaluation costs no model calls.
- **As text:** this goes through the translator and tests the whole system.

Run both: atoms first, to answer whether the learner generalises at all; text second.

**How generalisation is tested** (for any option). The dataset and the curriculum are split along four tiers, each held out:

1. **New wording**: held-out paraphrases of seen tasks.
2. **New situation**: a seen task in a situation never paired with it.
3. **New task**: an unseen task in a situation that was seen on other tasks. This is cross-task transfer.
4. **New domain**: a whole domain held out, for example bike repair or gardening when training is cooking and household. Only induced rules that are really general can pass.

On top of the tiers:

- **Learning curves** over dataset size (0, 250, 1,000, 3,000 examples).
- A **leakage check**: near-duplicate detection by atoms and words between the splits.
- A **null baseline**: the same test with induced rules switched off.

If tier 4 stays at the null baseline while tier 1 is high, the learner memorises rather than generalises. That is a publishable answer either way.

**Who writes the dataset.** Haiku, as the teacher (the constraint), generates examples from a spec, in batches. Each example has gold atoms. A symbolic critic checks typing, grounding and consistency, and a sample is reviewed. The test splits (H3) are written separately, by a sub-agent from a spec, and sealed. The training data never sees the H3 tasks or the held-out domain.

**Recommendation:** C3, with **C1 versus C3 as the main experiment** and C2b on atoms as the clean learner test. The learner must become a real rule inducer for this: multi-literal rules over the typed vault, near-miss and difference-based adaptation rules, and evidence counted from distinct examples.

---

## 3. The remaining problems, grouped

| Group | Problem | Evidence |
|---|---|---|
| Measurement | Only run 1 is clean; 20 scenarios (one scenario moves a rate by 5-7 points); some targets met by construction (meta requests, duplicates, contradictions); "Thanks!" turns inflate rates; scripted user turns sent whenever the reply ends in "?"; the cache hides variance and freezes errors | Report 11.3-11.4, 9.2, 14.2.2 |
| Knowledge entry | The translator writes step ids; multi-action steps (31 of 110 procedures); dangling order facts; teach lines not understood (42 of 55 queue entries) | 11.13, 14.4 |
| Task identity | Name drift (30 procedures); 193 wrong-task replies; `make_pancake_s4` became a task; curiosity unanswerable (58 of 67); seeds answered before teaching (25 of 180) | 14.1 |
| Understanding | Requests dropped (20%); variance (71%); dialogue logic inside the translator, setting `ok = true` without re-validating; model commentary and copied examples | 9.3.4, 14.1 |
| Adaptation | No conditional times, methods, step replacement, warnings or reasons; blind to changed situations (7 of 9 failing variant types); misfiring special cases | 14.3, 18.1.2 |
| Reply integrity | Taught wording unchecked against facts; claims and text drift apart; `task_help` monolith | 9.5.2-9.5.4 |
| Design promises | Stated rules confirmed at 0.9; hedges never spoken; registry flags unread; no question stack; moves without proofs; learned patterns without shadow mode and with lost updates | 9.2, 9.5.5 |
| Engineering | Uncached session models (3-4 evaluations per help turn; bench never ran this path); cache keys include reply text; string coupling on reply text; silent thread panics | 9.3, 9.5 |
| Training system | The dashboard polls every 2 s, and most designed panels were never built; the spaced queue works 2% of the time; the hand-label sheet was never filled in | 8, 11.2 |

---

## 4. Solution catalogue

| ID | Solution | Fixes | Effort | Risk | Expected effect |
|---|---|---|---|---|---|
| S1 | Sealed H3 (about 48 scenarios, variants split by generalisation tier, one held-out domain), a fresh gold set G3, counts only, limited readings; old 20 kept as development set D20 | contamination, size | S-M | numbers drop | trustworthy numbers; the D20-H3 gap measures over-fitting |
| S2 | Honest metrics: social turns in their own row, labels from moves not strings, contradictions split into name mismatch and false belief | inflation | S | none | headline ≈ 46%, not 76%, on 2.0 data |
| S3 | Symbolic content check on claims, plus the extraction check (axis B) | judge validity | S-M | trusts faithfulness | learning measured, not style |
| S4 | Symbolic user simulator that answers the question asked | scripted turns | S | none | dialogue really tested |
| S5 | Negative controls: each metric must be shown to fail on an injected fault | by-construction targets | S | none | |
| S6 | Cold-cache runs; translation agreement as a metric | hidden variance | S | calls | |
| S7 | Scenario and dataset linter: checklist ↔ ground truth ↔ teach line; one action per line; split leakage | thin or ambiguous teaching | S | none | |
| S8 | Agent-owned teach ingestion: frame grammar, step ids allocated by the agent, structure validated, read-back, no cached rejects | storage errors | M | rigid frames | storage 70% → 98% target; the largest single E2 lever |
| S9 | Task identity as knowledge: aliases taught and learned, symbol checks, matching as a rule with a proof, confirmation question | identity | M | wrong merges (hypotheses) | wrong-task replies < 1 in 100 |
| S10 | Honest "closest" move ("I don't know X; I know Y") | seed answers | S | | 0 in encounter 1 |
| S11 | Request detection as a policy move; cue patterns learned (axis A); every part of a compound question answered | dropped requests | M | | recall 80% → 97% |
| S12 | Voting only on move-changing parts when the cue layer disagrees | variance | S | calls | |
| S13 | Cache hygiene: canonical keys, structured context, shared, commentary stripped | variance, cost | S | none | |
| S14 | No word lists in agent crates; lists become seed data, then derived or learned (axis A) | heuristics | M-L | early drop | real learning |
| S15 | Rule learner v3: multi-literal induction, difference-based adaptation rules, evidence from distinct examples, shadow mode for patterns | learning | M-L | over-general rules (hypothesis tier) | the mechanism behind axes A and C |
| S16 | Adaptation container: conditional times, alternatives, step replacement by effect, warnings, modifiers as rules over task kinds | transfer | M-L | over-application | cross-task transfer becomes possible |
| S17 | Notice a changed situation; honest gap plus a curiosity goal named by the variant | blindness | M | false alarms | blindness → 0 |
| S18 | Typed guards on adaptation mechanisms | misfires | S | low | 0 misfires |
| S19 | Elicitation frames in dialogue teaching (what goes wrong, why, done when, what changes if, the task's kind); student-aware fix pass | thin teaching | M | generic tips | adaptation knowledge actually taught |
| S20 | Example dataset generator plus critic (axis C) | volume | M | synthetic bias, leakage | the C1 vs C3 experiment |
| S21 | Content plan, then a plain realiser; claims from the plan; faithfulness test on help replies; taught wording removed or linked to atoms | faithfulness | M | plainer text (accepted) | faithfulness holds again |
| S22 | All moves in the policy; session state as facts; a real question stack; consumers read moves, not text | promises, coupling | M | | |
| S23 | Keep the promises: rules start as hypotheses, confidence shown as plain hedges, flags read, procedures confirmed by two successes | promises | S | | |
| S24 | Session-model cache; help-path bench on 20,000 facts; concurrency fixes | engineering | S | | |
| S25 | Real-time training lab: event log with ids, SSE, runner in-process, drill-down to the proof, teach-line view, run comparison, H3 guard | lab (user request) | M | scope creep | faster cycles |
| S26 | Spaced queue: reteach only what failed; drop it if it stays under 20% | | S | | |
| S27 | Outside knowledge import (how-to, ConceptNet), by need, hypothesis tier | untaught knowledge | L | drift, licences | +2 to +5 of 15 transfer (estimate) |
| S28 | Offline paraphrase mining, or a runtime LLM rewrite (options D2 and D3) | fluency | M / S | D3 breaks the principle | style points only |

**Moved to "later" under axis B** (none changes what was learned):

- S28;
- the wording judge and the 1-10 score;
- framing echoes;
- taught wording as text;
- step-time display rules;
- preposition and article tuning;
- the R10 wording fixes.

What stays is a plain realiser that must express every relation. "Use jug instead of watering can. Step 3 pour water onto soil with jug until drains." passes.

---

## 5. Smaller choices

- **Judge of record.** Content check plus extraction check (axis B). Optional: you hand-label 20 anchors, which would be the only human check in the project.
- **H3 size.** 48 scenarios give about ±12 points at 75%. A cold 48-scenario reading is roughly 1,200-1,800 calls. Under axis B, the judge and labeller calls (35% of 2.0's calls) mostly disappear. Recommended: at most four H3 readings.
- **Scope.** Phase A (19-21) plus phase B (22-30), including both PDFs. Import (S27) is optional, as an ablation.
- **Vault.** Start fresh and keep the 2.0 vault archived as an integrity-checker test case.
- **User turns.** The symbolic simulator, with request wordings written into the scenarios ahead of time.

---

## 5b. Against the Fable draft (`research/DESIGN_3_draft_fable.md`)

The Fable draft was written before the user's notes on content over polish, learning over code, the dataset option and the two phases.

**Agreed, and borrowed into my draft:**

- the sealed H3 read at most four times, with a read ledger;
- the symbolic user simulator answering the slot actually asked;
- teach lines parsed by a grammar, with agent-owned ids and read-back;
- every move by a policy rule with a proof;
- the adaptation predicates, and modifiers taught only on training tasks;
- the paired McNemar test against v2.0.0 on the same H3 as the main comparison;
- a committed pre-registration file (`PREREG_3.md`);
- ablations: oracle parser, and fresh against trained vault;
- the "second encounter given correct storage" decomposition;
- a measure of free-form (out-of-frame) teaching;
- the paper's numbers generated by one command with no model calls.

**Disagreed** (the user's notes decide these):

| Point | Fable draft | Mine |
|---|---|---|
| Judge of record | The Haiku checklist judge, with a strict check beside it | The fact-and-relation check on claims and text |
| Judge score and pairwise preference | Reported (with a length-and-echo control) or targeted | Out of 3.0 |
| Taught wording | Kept, linked to atoms | Removed; a plain realiser |
| Word lists | Hand-written lexicon facts | Derived or learned, with a seed that is ablated |
| Teaching source | Dialogue only | Dialogue, a dataset or a mix, tested by tier, including a held-out domain |
| H3 size | 60 scenarios | 48, smaller but with a domain tier |
| Phases | None | Two phases |

**Worth taking from it as open decisions:**

- who the paper is for, and whose names go on it;
- a second Claude model as a self-preference check on a sample;
- autonomy between go-aheads;
- branch and tags.

## 6. Items 2.0 put out of scope

| Item | Recommendation | Why |
|---|---|---|
| Translation speed | **Keep out**; report as cost | The CLI constraint stands |
| Act accuracy | **Bring back, redefined** as agent-level understanding: request-act recall after the cue layer, task-match accuracy, move agreement over two cold runs, on G3 | The translator is the main agent-side failure, and 3.0 takes those decisions back from the model |
| Judge score (1-10) | **Keep out of 3.0 entirely**, and move it to "later" (axis B). Compare iterations by re-scoring archived v1 and v2 transcripts with the fact-and-relation check | It measures style and rewards guessing; human-like answering is out of scope |
| M15 help-turn replay | **Bring back as a non-gating regression guard**, labelled symbolically: "teach me" for a task the vault lacks is a fitting move | Cheap; the 2.0 objection is removed by the rule |

---

## 7. Recommendation

**The thesis of 3.0:** *the agent learns its knowledge, heuristics included, from teaching and examples, and 3.0 measures whether that learning generalises: to new words, new situations, new tasks and a new domain.* Presentation is not a goal. Rough text is fine, but it must carry every fact and every relation the answer needs, and nothing unsupported.

**Two phases (user decision).**

- **Phase A, design and clarification (milestones 19-21).**
  - Brainstorm and settle the decisions above (axes A-C and the section 5 choices).
  - Run small experiments where a decision needs evidence. Most cost no model calls:
    - re-score the archived 2.0 transcripts with the fact-and-relation check;
    - audit how many of the coded word lists can be derived from knowledge;
    - prototype rule induction on the 105 existing ground truths, split by tier;
    - replay the 626 archived teach lines through a frame grammar;
    - a small dataset-generation pilot, only with your go-ahead.
  - Write and seal H3.
  - Write the final Design 3.0.
  - Deliverable: the final Design 3.0 document (Markdown). No plan PDF; the only PDF is the research PDF at the end of phase B.
- **Phase B, build and test (milestones 22-30).** Implementation, each milestone accepted on the fresh held-out set. It ends with the **final research PDF** covering all three iterations.

The list below is the provisional phase B. Phase A may confirm or change it.

**Provisional phase B order:**

1. Honest, learning-centred measurement: S1-S7, axis B, the v2.0.0 baseline on H3.
2. The lab core (S25).
3. The agent owns entry and the code holds no word lists: S8, S13, S14 (lists to seed data).
4. Identity and understanding: S9-S12.
5. Reply integrity, plain: S21-S24, S18.
6. The adaptation container and situation noticing: S16, S17.
7. Rule learner v3 (S15), so that derived and learned heuristics replace the seed.
8. Knowledge sources: elicitation dialogue (S19) and the example dataset (S20). Training runs C1 and C3, with your go-ahead.
9. Final measurement by tier, with ablations: no seed lexicon, no induced rules, dialogue only versus mix.
10. The final LaTeX paper, with every number generated from `data/`.

**Why.** Storage and identity (S8, S9) are where most clean second encounters were lost (97% against 14%). Adaptation plus learning (S15-S17, S20) is the only route past item swaps that does not mean more code. Axis B stops the work drifting into polish again.

**Honest expectation.**

- On H3, the new-wording tier could reach 70-80% if storage and identity work.
- Item-swap transfer: perhaps 50-70%.
- Cross-task and new-domain transfer are open questions. 40% cross-task is a target, not a forecast.
- The new-domain tier may stay near the null baseline. That would be a real finding about the limits of symbolic induction from synthetic examples.

**Decisions for you** (full list at the end of the draft):

1. Axis A: confirm "no word lists in agent crates", with a seed lexicon as data that is ablated.
2. Axis B: the fact-and-relation check (claims plus text read-back) as the measure of record, with no style judge in 3.0; whether the Haiku extraction sample is wanted; whether you hand-label 20 anchors.
3. Axis C: C1 vs C3 as the main experiment; dataset size; atoms first, then text.
4. The held-out domain, and H3 size (48 recommended).
5. The call budget (roughly 15,000-20,000 calls, about 8-10 sessions) and whether H3 readings need your go-ahead (recommended: yes, at most four).
6. Scope: the import ablation in or out.
7. The four out-of-scope items as in section 6.
8. LaTeX toolchain: none is installed (tectonic or TeX Live).
9. Start from a fresh vault.
