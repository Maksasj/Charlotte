# Symbolic Learning Agent – Design 2.0

Nov 16, 2026 · @Maksim

## Summary

Design 2.0 keeps the symbolic core of v1 and rebuilds everything around it, so that what the agent is taught changes what it says later. The main rule stays: reasoning, memory, learning and every reply are symbolic. A language model only translates input, and plays teacher, reviewer and judge in training.

v1 proved the core works: the Datalog engine, the vault, proof traces and the fixed tests all hold. It also showed the agent cannot help anyone yet. In 68 help-style turns it gave 0 or 1 real answers, and the dinner recipe was taught four times with 0 facts stored.

2.0 makes five changes:

1. **A knowledge model that can hold tasks.** Procedures, steps, needs, durations, numbers with units, and choices among options.
2. **Understanding that never fails as a whole.** A long message is split into parts. Each part is translated and checked alone, and only a failed part is retried or split further.
3. **A dialogue manager.** It keeps a memory per conversation, opens a task frame when someone asks for help, asks for what is missing, and picks every reply move by rules.
4. **Learning that closes the loop.** Procedures, words, sentence patterns and guessed rules all enter as hypotheses, get used on trial, and are confirmed or dropped by results.
5. **Honest measurement.** Every turn is labelled by outcome. Learning counts only when a taught scenario is answered later, in new words, without teaching.

Every failure seen in v1 has a named fix and a test in the section "Every v1 failure, its fix and its test". The design cannot promise that no new failures appear. The per-turn trace exists to find them fast.

## What v1 taught us

The symbolic core passed every fixed test while conversation stayed poor, because the tests and the conversations asked for different things. All numbers below come from REPORT\_DESIGN\_1 (2026-11-16).

| Measure | v1 result |
| --- | --- |
| Fixed knowledge tests | 100% dev, 97.8% held-out |
| Parser on fixed sentence sets | 100% dev, 98% held-out |
| Conversation quality, Haiku judges, 1–10 | 2.4 to 4.3, no real trend |
| Real answers in help-style training | 0 or 1 of 68 replies |
| Not understood in help-style training | 44% (30 of 68) |
| Understanding as input gets longer | 97% single sentences, 86% lessons, 81% friend chats, 56% help requests |
| Answer time | about 5.5 s; 96% of it is the Haiku translation; the symbolic part takes a median of 48 ms |
| Rules the learner guessed by itself | 73 proposed, 1 confirmed |
| Vault | 2,171 facts (374 closed), over 200 distinct predicates, about 200 catch-all `quality` values |

One correction to the report: its section 7 uses "15% real answers" as the baseline. The measured figure is 0 or 1 of 68. This design uses the measured figure.

Eight lessons follow from the data.

1. **The tests measured the wrong thing.** Test sentences have about 4 words. Judges write about 11 words per message and teachers about 44.
2. **The agent stores but does not use.** A training run reported "+44 facts" and gave no helpful reply. Stored knowledge and useful behaviour are not connected yet.
3. **There is nowhere to put the important knowledge.** Steps, needs, durations, quantities, tasks and the state of a conversation have no representation. "Wheel removes from frame" is what a repair procedure becomes.
4. **Translation is all or nothing.** A message over 10 words or longer than one sentence goes to Haiku whole. If one part fails, the whole message is lost. Mixed messages lose their request and keep the "thanks".
5. **The translator became an unchecked parser.** It invents predicates, decides what counts as a fact, and drops "usually". Nothing checks its output against the ontology.
6. **Learning does not close the loop.** Guessed rules never take part in answers. Confidence does not change what the agent says. One correction rewrites a rule for good. Nothing learns words or grammar.
7. **Knowledge gets dirty fast.** 17% of facts are closed, 24 of 40 sampled "names" are ordinary words, and plural and singular forms live side by side (`knive`, `clothe`, `sunglass`).
8. **The engine will not scale as built.** Each message runs 3 to 5 full evaluations and rescans every fact. Parallel training on one shared vault is fragile, and the newest 1,160 lines have no tests.

## Goals and principles

2.0 has one goal above the others: something taught once must be usable later, in a new conversation, in different words.

**Goals**

- Help with everyday tasks: answer "how do I", "what do I need", "how long", "what next", and choose among options.
- Understand long, natural messages part by part.
- Learn facts, rules, procedures, words and sentence patterns from conversation.
- Explain every belief, every answer and every dialogue move with a proof.
- Stay deterministic, traceable and hand-editable.
- Answer in under 2 seconds at the median.

**Principles**

1. **Learned means used.** Nothing counts as learned until it changes a later answer. "+44 facts" is not a result.
2. **No message is lost as a whole.** Each part succeeds or fails alone, and the agent replies to what it understood.
3. **Closed vocabulary, one door.** Every predicate is in a registry. New words enter through a review queue, never silently.
4. **Two memories.** What a user says about their situation lives in the session. Only lasting knowledge goes to the vault.
5. **Knowledge is data, not code.** Tasks, procedures, words, sentence patterns and dialogue moves are vault files the agent can learn and you can edit.
6. **Everything is a hypothesis first.** New rules, procedures, words and patterns start on trial and earn confirmation through evidence or use.
7. **Say only what you believe, and say how sure.** Every clause comes from a believed atom. Low confidence and guessed rules show up as "I think" and "probably".
8. **An honest "teach me" is a designed reply.** Not knowing is fine when the agent says what it needs to learn.
9. **Measure before tuning.** No change is kept without a before and after on the same scenarios.

**What stays from v1**

The Rust workspace, the Markdown vault, the Datalog engine with proof traces, facts that close instead of being deleted, the faithfulness test on replies, the audit log, the `Parser` trait, and Claude as teacher, reviewer and judge.

## Architecture

A message now passes six stages, and each stage can fail for one part of the message without losing the rest.

&#91;embedded content: message flow · 6 stages, 3 memories, 1 learner\]

Tinted boxes are memory. Stages 1 to 4 are new or rebuilt. The reasoner is v1's engine with additions, and every stage writes its result to a turn trace that the learner and the evaluation both read.

| Component | In v1 | In 2.0 |
| --- | --- | --- |
| Segmenter | One split at `. ! ?` and at ", and" | Its own stage: sentences first, clauses on failure |
| Translator | Whole message to Haiku in one call | One answer per part; fixed patterns, learned patterns, then Haiku |
| Validator | Checked spelling of predicate names | Checks vocabulary, argument types, grounding, act type, round trip |
| Dialogue manager | One pending slot, cleared by the next message | Session memory, task frames, moves chosen by rules |
| Reasoner | 3 to 5 full evaluations per message | One model per message, aggregates, sound time, a tentative layer |
| Reply generation | Templates | Planner, microplanner, clause realizer, surface lexicon |
| Learner | Pattern search on every statement | Cheap evidence updates per turn, heavy work in an offline pass |
| Storage | Several in-memory copies of one vault | One writer process owns the vault |

## Knowledge model

Knowledge stays flat Datalog facts and rules in Markdown files. 2.0 adds four things: numbers with units, an id on every fact, new layers in the core ontology, and a registry that every predicate must be in.

**Values.** v1 had symbols and integers. 2.0 adds decimal numbers and a unit lexicon (minute, hour, gram, piece, person). A duration or amount is a range with a unit: `takes_time(tea, 3, 5, minute)`. "Twenty minutes" stops being the symbol `twenty_minutes`.

**Fact ids.** Every stored fact gets a stable id such as `f1042` and the conversation it came from. Ids make four things possible:

- nested statements: `knows(anna, f1042)` for "Anna knows that Bob likes tea";
- linking loose facts to a task: `context(f1042, fix_flat_tire)`;
- checking a fact against the sentence it came from;
- a reviewer closing exactly the fact it judged, not every copy of that atom.

**Events.** A statement with more than two participants becomes an event entity. "Anna gave Bob a book at Christmas" is stored as `event(e7)`, `action(e7, give)`, `agent_of(e7, anna)`, `object_of(e7, book)`, `recipient(e7, bob)`, `during(e7, christmas)`.

**Core ontology in layers.** The v1 core stays. Five small layers are added, each defined once and shared by everything the agent learns.

| Layer | Example predicates | Used for |
| --- | --- | --- |
| Primes (v1 core) | `is_a`, `part_of`, `has`, `property`, `before`, `causes`, `wants`, `knows` | Facts about the world |
| Procedural | `task`, `step`, `action`, `needs`, `uses`, `takes_time`, `until`, `overlaps`, `method`, `done_when` | How to do things |
| Choice | `solves`, `recommended_for`, `available`, `staple`, `substitute`, `portion` | Picking among options |
| Session | `active_task`, `at_step`, `slot`, `open_question`, `grounded`, `said` | Where the conversation is |
| Lexical | `same`, `same_pred`, `surface`, `plural_of`, `frame` | Words and how to say them |
| Self | `can`, `cannot`, `knows_task` | Honest answers about itself |

**Predicate registry.** `vault/lexicon/` has one entry per predicate: its arity, argument types (entity, task, step, number, unit, fact id), a plain-English definition, synonyms, a sentence frame, and flags (single-valued, open-world, may be a rule head, may be asked about). The flags replace hand lists spread over the v1 crates. Types let the validator reject facts like `is_a(here, first_fact)`.

**Procedures.** One file per task in `vault/procedures/`. Steps are small events ordered by `before`, so two steps can run side by side. Time and source tags are left out of this example.

```
task(garlic_tomato_pasta).
solves(garlic_tomato_pasta, plan_meal).
needs(garlic_tomato_pasta, pasta).
needs(garlic_tomato_pasta, garlic).
needs(garlic_tomato_pasta, olive_oil).
needs(garlic_tomato_pasta, cherry_tomato).
takes_time(garlic_tomato_pasta, 15, 20, minute).

step(garlic_tomato_pasta, gtp_s1).  action(gtp_s1, boil).  object_of(gtp_s1, pasta).
step(garlic_tomato_pasta, gtp_s2).  action(gtp_s2, fry).   object_of(gtp_s2, garlic).
uses(gtp_s2, olive_oil).            takes_time(gtp_s2, 1, 1, minute).
overlaps(gtp_s2, gtp_s1).
step(garlic_tomato_pasta, gtp_s3).  action(gtp_s3, add).   object_of(gtp_s3, cherry_tomato).
until(gtp_s3, soft).                takes_time(gtp_s3, 5, 7, minute).
step(garlic_tomato_pasta, gtp_s4).  action(gtp_s4, mix).   object_of(gtp_s4, pasta).
before(gtp_s2, gtp_s3).  before(gtp_s1, gtp_s4).  before(gtp_s3, gtp_s4).
```

The file's front matter carries its status (hypothesis or confirmed), its source, and how many times it was used with success.

**Hedges.** "Usually", "most" and "some" are kept as a strength on the fact or rule, and replies repeat them. "Cats like fish" becomes a typical rule, not a universal one.

**Time.** A derived fact is valid only while all its premises are. A question about the past evaluates the vault as of one moment, so facts from different times are never joined. "Yesterday", "in 2020" and "every morning" are parsed into start and end times.

**Vault layout**

| Folder | Holds |
| --- | --- |
| `vault/core/` | Ontology layers and core rules; read-only for the learner |
| `vault/lexicon/` | Predicate registry, synonyms, surface forms of words |
| `vault/facts/` | One file per entity, as in v1 |
| `vault/rules/` | One file per rule, with status and evidence |
| `vault/procedures/` | One file per task |
| `vault/tasks/` | Task schemas: slots, slot questions, closing line |
| `vault/grammar/` | Learned input patterns and output patterns |
| `vault/policy/` | Dialogue move rules |
| `vault/queue/` | Review queue: new words, merges, hypotheses |
| `vault/stats/` | Usage counts, so answering never rewrites fact files |
| `vault/log/` | Append-only log, one writer, replayable for rollback |
| `sessions/` (outside the vault) | Per-conversation memory and turn traces |

## Understanding input

No message is translated as one all-or-nothing block any more. A message is split into parts, each part is translated and checked alone, and only a failed part is retried and then split again.

**How a long message is handled**

1. **Structure pass, no model.** Find the greeting, the thanks, numbered or dashed lists, and "first, then, finally" chains. A numbered list on the teaching channel becomes steps directly.
2. **Sentence split.** A real splitter that handles decimals, abbreviations and lists. The v1 rule that turned ", and" into a sentence break is removed.
3. **One call, one answer per sentence.** The model sees the whole message and the session, so "it", "then" and "while it cooks" keep their meaning. It must answer sentence by sentence: `[1] greet`, `[2] inform: ...`, `[3] request: ...`. A sentence it cannot translate comes back as `[4] unknown` and does not hurt the others.
4. **Check each part** with the validator below.
5. **Retry only the failed parts.** Each failed part gets one retry with the previous part's result, the session state and the list of errors.
6. **Split deeper only on failure.** A sentence that still fails is cut at clause words (and, but, then, while, because, so) and each clause is tried again. Pronouns are replaced by what they refer to before the cut. Depth is capped at 2.
7. **Keep what is left.** A part that still fails is stored in the session as an open item. The agent answers what it understood and asks one question about the rest: "I have the first three steps. What do you mean by 'give it a good toss'?"
8. **Keep the links between parts.** "Then" becomes `before`, "while" becomes `overlaps`, "until" becomes `until`, "because" becomes `causes`. The order of the parts is stored.

Splitting everything into small pieces from the start would be worse. It breaks the links between sentences and multiplies model calls. One call with one answer per sentence isolates failures at no extra cost, and deeper splitting is the fallback.

**The formal language gets dialogue acts**

| Act | Example | In v1 |
| --- | --- | --- |
| `tell`, `ask`, `why`, `correct`, `yes`, `no`, `recall` | "Cats like fish." "Does Anna like tea?" | Yes |
| `request` | "Can you help me plan dinner?" | No: dropped or stored as a fact |
| `inform` | "I have pasta and about 20 minutes." | No: stored for good as facts about the speaker |
| `ask_how`, `ask_need`, `ask_duration`, `ask_next` | "How do I fix it?" "What do I need?" "What's next?" | No |
| `done`, `lack` | "I did that." "I don't have garlic." | No |
| `accept`, `reject` | "Sounds great." | No |
| `feedback` | "Hmm, not quite." (about the reply, not about a fact) | No: read as a correction |
| `teach` | A procedure or rule given on the teaching channel | No |
| `greet`, `bye`, `social`, `about_you` | "Thanks!" | Yes; `about_you` is narrowed |

**Translator rules**

- **Three tiers.** Fixed patterns first, then patterns the agent learned, then Haiku. The share of turns that never reach the model should rise over time.
- **Closed vocabulary.** The prompt carries the registry entries and task names that fit the message. A new predicate must arrive as `new_pred` with its arguments, a definition and the source sentence. It is first matched to an existing word by synonyms. If nothing matches it goes to the review queue, and facts that use it are pending, not believed.
- **Session context.** The model sees the last reply, the active task, the missing slots and the things recently mentioned.
- **Examples from its own history.** Each call gets 5 to 10 reviewed translations picked by word overlap.
- **The model does not decide what is true.** It no longer drops opinions or turns "usually" into a plain fact. Hedges and questions pass through as they are.
- **Speed.** The language spec sits in a cached system prompt, thinking effort is low, output is capped at about 200 tokens, and results are cached on text plus session state. Failed translations are cached too, so no model call ever runs inside the vault lock.

**Validator: six checks per part**

1. **Shape.** The line parses, and rules are safe.
2. **Vocabulary.** Predicates are in the registry with the right arity and argument types.
3. **Grounding.** Every constant comes from the message, the session or the registry. The model cannot add knowledge or slip in an answer.
4. **Act.** A sentence ending in "?" cannot become only a `tell`. A request is never dropped because a "thanks" is present.
5. **Entities.** Glued constants such as `good_for_eggs_and_frying` are split or rejected.
6. **Round trip**, for teaching content and lasting facts. The agent says the parse back with its own grammar, and a cheap second check confirms it means the same as the source sentence.

A lasting fact from a user with a weak parse is confirmed in the reply: "So the pasta takes 15 to 20 minutes, right?" v1's design asked for this and never built it.

## Dialogue

A dialogue manager now sits between understanding and reasoning. It keeps a memory for each conversation and picks every reply move by rules.

**Session memory.** It holds what belongs to this conversation only, and it is never written to the vault:

- the active task frame, with filled and missing slots;
- the current step of the procedure in use;
- open questions as a stack, so a side remark no longer erases them;
- the user's situation ("I have pasta", "my bike has a flat tire");
- a ranked list of things recently mentioned, in place of one `last_entity`;
- what the agent already said, and parts of messages it did not understand.

A situation fact moves to the vault only when the user marks it as lasting ("I'm vegetarian"), repeats it in a later session, or a teacher marks it. This replaces v1's patch of closing every fact about `friend` after a chat.

**Task frames.** A `request` opens a frame from `vault/tasks/`. The schema lists the slots, which are required, the question for each slot, and a closing line. Slots can also be derived: the things a task's feasibility rules need to know are its required slots. A newly taught procedure therefore brings its own questions.

**Moves.** One main move per turn, in this order of priority, plus at most one acknowledgement and one question.

| Move | When |
| --- | --- |
| Answer or recommend | The question can be answered, or a feasible option exists |
| Give the next step | A procedure is active |
| Ask for slots | Required slots are missing; all are asked in one turn |
| Clarify | A part of the message was not understood |
| Teach me | The task or word is unknown: "I don't know how to plan dinner yet. What would a good plan include?" |
| Confirm | A lasting fact or a weak parse needs a yes or no |
| Acknowledge | Only new, lasting information; never "I already knew that" |
| Social | Added after content, or alone when nothing else is needed |
| Resume | After a side remark, return to the open task |

Moves are Datalog rules in `vault/policy/`. Each chosen move has a proof, and a wrong move can be corrected like any other rule.

**Choosing among options.** Candidates come from `solves`. Feasibility is plain Datalog, for example `missing(D, I) :- needs(D, I), not available(I), not staple(I).` Ranking runs outside the engine: needed items the user has, time against the budget, confidence, and how often the option was accepted before. When nothing is feasible the agent offers the closest option and names what is missing. Each result is logged as `task_outcome`, which is evidence for the learner.

**Feedback is not correction.** "Not quite" targets the last reply, not a fact. A correction retracts an old value only for single-valued predicates or an explicit "no": "No, Anna likes coffee" no longer deletes "Anna likes tea".

**The dinner dialogue, traced**

| User says | Acts | Session | Reply |
| --- | --- | --- | --- |
| "Hi! Can you help me plan dinner tonight?" | `greet`, `request: plan_meal` | Frame opens; ingredients and time are missing | "Sure! What ingredients do you have, and how much time do you want to spend cooking?" |
| "I have pasta, garlic, olive oil, and some cherry tomatoes. Maybe 20 minutes." | Five `inform` atoms | Slots filled; garlic tomato pasta is feasible | The recommendation, its steps in order, and the total time |
| "Sounds great, thanks!" | `accept`, `social` | Frame closed; outcome logged | "You're welcome! Enjoy your dinner." |

By the report's analysis, v1 answers the first turn with "Hello!" and the second with "You have pasta, garlic, olive oil and cherry tomato. You make tomato soup."

## Reasoner and storage

The engine is kept. It gets faster, sound over time, able to count, and safe to use from many conversations at once.

| Change | What it fixes from v1 |
| --- | --- |
| One model per message, shared by answering, learning and curiosity; incremental updates later | 3 to 5 full evaluations per message |
| An index on every argument position that joins use | Only the first argument was indexed |
| Aggregates (`count`, `sum`, `min`, `max`) in their own stratum, and arithmetic that cannot create endless new values | No "how long in total" or "how many" |
| Evaluation as of one moment; derived facts get time intervals | History questions mixed facts that were never true together |
| A tentative layer: guessed rules run in a second pass and mark what they derive | Guessed rules never took part in any answer |
| Confidence from the best derivation, and used in replies | First derivation found; a 0.3 fact answered "Yes" |
| Bad-rule quarantine: a rule that breaks evaluation is set aside and reported, and core rules always stay | One bad rule silently removed all rules |
| All-or-nothing changes: a correction either fully applies or leaves no trace | A reverted correction left a stored negation and a raised counter |
| A small planner that expands a task into methods and steps | The "planner" was one match statement |
| New ids for task instances, events and steps are made by the dialogue manager | Datalog cannot invent objects |
| One writer process owns the vault; chats and teachers send it messages | Several in-memory copies, whole-vault reloads, a corrupted log line |
| Usage counts live in `vault/stats/` | Answering a question rewrote fact files |
| No model call inside the vault lock, ever | A failed translation was repeated while holding the lock |
| The trainer API needs a token and checks the origin | Any local web page could start training |

Budget: the agent's own work stays under 100 ms at the 95th percentile on a vault ten times the size of v1's, about 20,000 facts.

## Generating replies

Replies are still built only from atoms the agent believes. They now pass three steps where v1 had one template.

1. **Reply planner.** Takes the chosen move and its content. Puts the answer first, then the needed detail, then at most one question. Leaves out what the user just said, which ends the echo replies.
2. **Microplanner.** Joins atoms with the same subject. Chooses a name or a pronoun and keeps that choice across turns. Adds connectives from the atoms themselves: `before` gives "then", `overlaps` gives "while it cooks", `until` gives "until they are soft". Numbers more than three steps as a list. Writes ranges ("5 to 7 minutes") and hedges ("usually", "I think", "probably").
3. **Clause realizer.** A small engine in the style of SimpleNLG. It takes subject, verb, object, tense, negation and mood, and handles agreement and word order. It adds the imperative mood for steps ("Boil the pasta.") and proper questions.

**Surface lexicon.** Every entity stores how it is written: `surface(cherry_tomato, "cherry tomato")`, its plural, whether it is a mass noun, and whether it is a name. Every predicate stores a sentence frame such as "X grows on Y". A predicate without a checked frame cannot be spoken, and the agent asks a teacher how to say it. This ends "Spring is an after of Winter", "sunglass" and lowercase "berlin".

**Wording rules**

- The same failure gets the same wording every time.
- No reply is repeated word for word inside one conversation.
- A social line never replaces content.
- Self-descriptions come from facts in the self layer, not from fixed strings.

**Faithfulness.** The v1 test stays and grows. Every clause maps to an atom in the reply's claims, and every claim must parse back to that atom. The test now also covers steps, questions and recommendations. The free-text reply type that skipped this check in v1 is removed.

**Learned wording.** Offline, a teacher paraphrases a few hundred content plans. Patterns are mined from the paraphrases and kept only when their output parses back to the same atoms. They are stored in `vault/grammar/` as hypotheses. At run time only the symbolic realizer speaks.

## Learning

In 2.0 the agent learns ten kinds of thing, and all of them follow one life cycle: proposed, used on trial, then confirmed or dropped by results.

**One life cycle.** A new item is stored as a hypothesis with its source. On trial it is used in a marked way: a guessed rule answers with "probably", a new procedure is offered as "I was taught this once". It is confirmed by a teacher or user, by enough clean evidence, or by a successful use. It is retired by counter-evidence or a failed re-test. Every change is logged with its reason.

| What is learned | Learned from | Confirmed when | In v1 |
| --- | --- | --- | --- |
| Facts | Statements marked as lasting | Repeated by another source and passed by review | Yes, but mixed with the user's situation |
| Stated rules | "If ... then", general statements | Supporting cases and no counterexample; the hedge is kept | Confirmed at once on the teacher's word |
| Guessed rules | Patterns over many entities, with contrast cases required | A teacher says yes to "My rule is ... Is that right?", or 20 clean cases | 73 proposed, never used, 1 confirmed |
| Exceptions | Corrections | The single case is stored at once; the rule itself changes only after two counterexamples share a feature, or a teacher confirms the reason | One correction rewrote the rule for good |
| Procedures | Teaching blocks, steps told over several turns, numbered lists, demonstrations | A second teacher agrees, or a user accepts the result | Not possible |
| Task schemas | Derived from what a procedure needs; "ask about X first" from a teacher | Used with success | Not possible |
| Words | New entities with their written form; synonyms; new predicates through the queue | A reviewer approves, or the word maps to an existing one | Names guessed from capital letters; predicates invented freely |
| Input grammar | Reviewed translations: a sentence shape the model translated the same way many times becomes a pattern | The pattern runs in shadow and agrees with the model in 95% of 20 uses; then it replaces the model call | Not possible |
| Output grammar | Sentence frames from source sentences; patterns mined from paraphrases | Output parses back to the same atoms | Not possible |
| Dialogue moves | Teacher feedback such as "you should have asked what tools I have" | The corrected move rule passes its scenario | Not possible |

**Two learning times.** During a turn the agent does only cheap work: store facts, attach steps to the open task, update evidence for the rules the turn touched. The heavy work runs in an offline consolidation pass between sessions: guessing rules, turning repeated translations into patterns, proposing merges, finding steps shared by several procedures, and scheduling re-tests. v1 ran its full pattern search on every statement.

**Learning from failure.** Every "teach me", every clarification, every tentative answer and every dead end becomes a question for a teacher. The questions are structured: "What is the first step?", "What do I need?", "When is it done?", "Is 'own' the same as 'have'?", "My rule is: mammals have hair. Is that right?"

**Steps told over several turns.** While a task frame is open, each step the user or teacher gives is attached to that task in order. v1 stored such steps as loose facts with no task and no order.

**Re-testing.** Confirmed rules and procedures are tested again at intervals that grow with confidence. A failed re-test lowers confidence. Forgetting stays for later: usage counts and last-used times are still recorded, and unused items would be archived, not deleted.

**Confidence shows.** A weak fact is answered with "I think so". An answer that rests on a guessed rule says "probably" and names the rule when asked why.

## Knowledge hygiene

Hygiene is enforced at the door by the registry and the validator, and repaired in the offline consolidation pass.

- **No catch-all.** `property(_, quality, _)` held about 200 unrelated values in v1. A quality now needs a named property (colour, size, taste, difficulty) or becomes its own predicate through the queue.
- **Word forms.** The lemmatizer gets an exception list, so "knives" gives `knife` and "clothes" stays `clothes`. The form the speaker used is stored as the written form. Names are never made singular.
- **Proper names.** A capital letter at the start of a sentence is not a name. Names come from capitals inside a sentence, from the registry, or from a teacher.
- **Compound constants.** `sunlight_to_grow` and `foods_from_plant` are split into atoms or rejected.
- **Merges are proposals.** Duplicates and synonyms go to the queue and are linked with `same` and `same_pred`. Nothing is rewritten in place, so every merge can be undone.
- **Situation or general.** "The basil is wilted" is a session fact. The reviewer checks that anything entering the vault is general knowledge.
- **Review against the source.** The reviewer compares each fact with the sentence it came from. It can only close facts from its own conversation, by fact id.
- **Rules keep their hedge.** A rule stated by one teacher starts as a hypothesis with the teacher's wording ("usually", "most"), not as a confirmed universal.

**The v1 vault.** It is not carried over. 2.0 starts with a fresh vault and replays the v1 teaching transcripts through the new pipeline. This avoids importing the known junk, and it measures 2.0 on the same input v1 had. The v1 vault is kept as an archive.

**Hygiene numbers to watch:** new predicates per conversation, share of predicates used only once (61% in v1), duplicate entity pairs (21 in v1), facts not linked to any category or task, and the reviewer's rejection rate.

## Training and curiosity

Teachers stop chatting at the agent and start teaching by protocol, on a curriculum, and every scenario is met at least twice.

**Teacher protocol**

1. **Two channels.** The teacher writes a user turn (in role, natural, at most 3 sentences). When the agent fails, the teacher adds a separate `[teach]` block. Teach blocks are tagged as teacher knowledge and are never read as the user speaking.
2. **Fixed frames in teach blocks.** One chunk per block, for example: "To make garlic tomato pasta you need pasta, garlic, olive oil and cherry tomatoes." "Step 2 of making garlic tomato pasta: fry sliced garlic in olive oil for 1 minute." "Garlic tomato pasta takes 15 to 20 minutes."
3. **The teacher sees what was stored.** After each turn it gets the acts and atoms the agent kept, and fixes only the wrong atom.
4. **Not allowed:** formatting requests, "say it back", apologies, and playing the assistant.
5. **Retry rule.** After a teach block the teacher asks the original request once more in the same words. If it still fails, the scenario is marked "taught, not learned" and the run moves on.
6. **Ground truth.** Each scenario has a ground-truth file. Teachers answer only from it, and a checker rejects answers that leave it.

**Curriculum.** Stages go from core concepts to objects and tools, then procedures, then social scripts, then mixed help and small talk. Each scenario has a goal checklist. For dinner: proposes a feasible dish, gives the steps in order, gives a time within the budget, closes politely.

**Encounters**

| Encounter | Setup | What it shows |
| --- | --- | --- |
| 1. Teach | Teaching allowed | How many teach blocks a scenario costs |
| 2. Repeat | Fresh session after other conversations, new wording, teaching forbidden | Whether anything was learned |
| 3. Transfer | A variant: no garlic, 10 minutes, rice in place of pasta | Whether it generalizes |
| 4. Retention | Encounter 2 again after many other conversations | Whether it lasts |

A scenario that fails encounter 2 goes back into a spaced queue.

**Curiosity.** v1 asked about contradictions, weak rules and gaps. 2.0 adds unknown tasks, missing steps, needs and durations, unknown words, missing sentence frames and parts of messages it did not understand. Questions are ranked by how many failed turns an answer would fix, and by learning progress per topic.

In a chat with a user the agent may ask only three kinds of question: a missing slot of the active task, a clarification of the current message, and something about the thing under discussion. Rule guesses, merges and frames go to teachers offline. The daily caps stay.

**Seed knowledge.** Teachers write 30 everyday procedures (cooking, household, travel, repair, social plans) from a ground-truth file. They enter as hypotheses and a second teacher reviews them.

## Training dashboard

Training runs on a live dashboard that you configure, so you can watch what the agent is taught, what it asks, what it keeps and whether it is really learning. It replaces v1's trainer page, whose "56% understood" figure hid the fact that almost no reply was helpful.

&#91;embedded content: default training dashboard layout · 4 tiles, 6 panels, alerts\]

This is the default layout. Every panel can be moved, resized, hidden or duplicated, and several layouts can be saved.

**Where the data comes from**

- Every stage writes events to the turn trace: message received, parts, acts, validator result, session change, move with its proof, reply, question asked, question answered, teach block, fact stored, item promoted or dropped, queue entry, alert.
- The vault writer publishes the events on a live stream and keeps them on disk. A finished run can be replayed, and two runs can be compared.
- The dashboard runs only on this computer, behind a token and an origin check. Starting a run stays a button you press, because a run spends plan limits.

**Panels**

| Panel | What you see | Why you want it |
| --- | --- | --- |
| Live conversations | One lane per teacher with the chat as it happens. Each agent turn shows its outcome label, its dialogue act and its move. A failed turn names the stage that failed | See at a glance whether the run is healthy |
| Question stream | Every question the agent asks, as it arrives (details below) | See what the agent cannot do yet |
| Teach log | Every teach block (details below) | See what was taught and what stuck |
| Learning feed | Rules guessed, with their evidence; items promoted or dropped; words and sentence frames waiting in the review queue; patterns learned; confidence moves | See learning as it happens, not as a total |
| Scenario grid | Scenarios as rows, encounters 1 to 4 as columns. Each cell is solved, solved after teaching, or failed. Filter by seen or held-out, and by curriculum stage | The real learning measure: untaught success on the second encounter, transfer and retention |
| Outcome mix | Stacked bars of the turn labels per run or per rolling window, plus echo rate, repeated replies and self-deflection | Shows whether replies are useful, not only understood |
| Translator health | How turns split between fixed patterns, learned patterns and the model; validation failures and retries; grounding violations; speed at median and 95th percentile; cache reads; tokens | Shows the model call shrinking as the agent learns |
| Knowledge health | Facts, closed facts, predicates, new predicates per conversation, predicates used once, duplicate pairs, review queue length, reviewer rejection rate, personal facts in the vault | Catches knowledge getting dirty early |
| Teacher quality | Meta requests, apologies, role drift, repeated messages, facts stored per teach block | Catches a bad teacher before it teaches the agent |
| Plan limits and cost | Session and weekly limits, tokens and calls per role, time left at the current rate | See what the run costs before it stops |
| Speed | The agent's own work per turn, vault writer queue, errors, corrupted log lines | Keeps the 100 ms budget honest |

**Questions pouring in.** Each question shows its type, why it was asked, its score, who answered it and when.

- **Types:** missing slot, clarification of a message, "teach me" for an unknown task or word, a missing step or duration, a sentence frame, a synonym check, a rule question ("My rule is ... Is that right?"), a contradiction.
- **Ranking:** by how many failed turns an answer would fix, then by learning progress on that topic.
- **Status:** open, answered, rejected, expired. The answer and its effect (what was stored, which rule changed) sit under the question.
- **Views:** a live feed, a table sortable by any column, counts by type, and the rate per minute.
- **Flood detection:** a question rate above a threshold is flagged, and curiosity is throttled until it falls. v1 once put an off-topic question into 44 of 48 replies; this panel would show that within a minute.

**Every teach.** Each teach block is one row.

- **Row:** scenario, teacher, the text, the acts it was parsed into, the atoms stored, their status (hypothesis, confirmed, rejected), the reviewer's verdict.
- **Diff:** opening a row shows exactly what changed in the vault: facts added or closed, rules, procedures, words, with the reason and the source sentence.
- **Measures:** facts per block, blocks until the first success in a scenario, and "taught, not learned" scenarios.
- **Re-test:** a button replays the original request at once, to check that the teaching worked without waiting for encounter 2.

**Turn drill-down.** Any turn opens as one aligned trace: message, parts, acts, validator checks, session before and after, the move with its proof, atoms, reply. "Why" prints the proof. Any turn can be exported as a test case.

**Configuration**

- **Layouts:** drag, resize, hide and duplicate panels; save as a preset such as "Watch live", "Learning progress", "Debug a failure" or "Cost". Layouts are hand-editable files in a `dashboards/` folder beside the vault.
- **Filters:** scenario, teacher, stage, curriculum stage, seen or held-out, question type, outcome label, time window.
- **Run settings:** number of teachers (1 to 24), turns per conversation, scenarios or stage, encounter mode, model and effort, review on or off. A run can be saved as a preset and started again.
- **Compare runs:** pick two runs and see them side by side on the same scenarios and seeds, with differences in each tile and the pairwise preference.
- **Step mode:** pause after every turn or every conversation, then continue one step at a time.
- **Thresholds:** every alert has its own limit and action.

**Alerts.** The defaults below are starting values to tune after the first runs.

| Trigger | Default | Action |
| --- | --- | --- |
| Plan limit used | 80% of session or 70% of weekly | Pause and notify |
| Not understood in a row | 4 | Pause the scenario |
| Question rate | Above 10 a minute | Throttle curiosity |
| Off-topic question in user chat | 1 | Flag |
| Personal fact enters the vault | 1 | Stop the run |
| Reviewer rejection rate | Above 15% over 50 facts | Pause and flag the teacher |
| Grounding violation | 1 | Flag |
| New predicates in one conversation | Above 5 | Flag |
| Same reply twice in one conversation | 1 | Flag |
| Corrupted log line or writer error | 1 | Stop the run |

**Your hands on the run.** From the dashboard you can answer a question yourself in place of the teacher, approve or reject a queue item (a word, a merge, a rule guess, a procedure), mark a turn's label right or wrong, and close or edit a fact. Each action is logged with your reason. Turn labels you correct join the calibration set for the judge. The run does not need you; the teachers answer by default.

**Build order.** Panels arrive with the data they show.

| Milestone | Panels and features |
| --- | --- |
| 12 | Event stream, live conversations, outcome mix, turn drill-down, plan limits, speed |
| 13 | Knowledge health |
| 14 | Translator health, question stream for clarifications |
| 15 | Question stream for slots and "teach me", session view in the drill-down |
| 16 | Repeated-reply and echo flags |
| 17 | Learning feed, teach log with vault diffs, rule questions |
| 18 | Scenario grid, compare runs, saved layouts, run settings, alerts, human actions, teacher quality |

## Evaluation

The headline number becomes "solved without teaching on the second encounter". The 1 to 10 judge score stays only as a trend line.

**What is measured**

- **Every agent turn gets one label:** helpful answer, partial answer, fitting clarification, honest "teach me", not understood, wrong move, wrong content, or echo only. Template replies are labelled by exact match. Content replies are labelled by a Haiku judge with a rubric, calibrated on the 68 turns already labelled by hand.
- **Every conversation gets one result** against its goal checklist: goal reached, reached after teaching, or not reached.
- **Every turn leaves a trace:** message, parts, acts, check results, session changes, the move with its proof, the atoms, the reply. The training UI shows it per turn, so any failure can be placed at one stage.

**Test sets**

| Set | Size | Purpose |
| --- | --- | --- |
| v1 fixed tests | As in v1 | Must not regress |
| Real-turn regression set | The 68 trainer turns and 228 judge turns from v1, labelled | Shows the old failures are gone |
| Translator gold set | 200 free-form sentences with gold acts and atoms, written without sight of the code | Measures translation alone |
| Long-message set | 50 messages of 30 to 80 words with a gold parse per sentence | Measures the segmenter |
| Help scenarios | 40 (20 dev, 20 held-out) | Encounters 1 to 4 |
| Small-talk scenarios | 20 | Chat quality |

**How runs are compared**

- An oracle-parser mode feeds gold formal input, so understanding and answering are measured apart.
- Every build is compared with the previous build on the same scenarios, seeds and judges, with at least 8 judges and pairwise A/B.
- Rates are reported with confidence intervals.
- A ceiling run lets a model rewrite the agent's content into fluent text, for evaluation only. It shows how much of the remaining gap is wording. It is never shipped.

**Targets**

| Metric | v1 | 2.0 target |
| --- | --- | --- |
| Solved without teaching, second encounter | Not measured; 0 of 8 goals reached even with teaching | 70% or more |
| Transfer to variants | Not measured | 40% or more |
| Helpful answers plus fitting clarifications, per turn | 0 or 1 of 68 | 60% or more |
| Requests answered with a fitting move | 4 of 57 (7%) | 80% or more |
| Not understood, help scenarios | 44% | 10% or less |
| Sentences recovered from long messages | Whole message lost on one failure | 90% or more |
| Echo-only replies | 24% | 5% or less |
| Wrong content | Not measured | 2% or less |
| Stored facts rejected by the reviewer | 12% | 5% or less |
| Translation time, median | 5.2 s | Under 2 s |
| Turns handled without the model | None of the long messages | 20% or more after 300 teaching turns, and rising |
| Guessed rules resolved by a test | 1 confirmed and 30 rejected of 73; 42 left hanging | 80% or more confirmed or rejected |
| Judge score, 8 judges | 3.9 | 6.0 or more |

The targets are proposals. The report gives two different goals for the second encounter (50% and 80%); 70% sits between them. Milestone 12 measures the v1 baseline again with the new labels before any target is fixed.

## Every v1 failure, its fix and its test

Each failure the report found has one fix in this design and one test that shows it is gone. The first table uses the report's failure numbers F1 to F12.

**In conversation**

| v1 failure | Fix in 2.0 | Test |
| --- | --- | --- |
| F1 Request dropped: "What should I wear?" got "Hello!" | `request` act, task frames, move priority | Requests answered with a fitting move, 80% or more |
| F2 No procedures: "I don't know what you do" | Procedural layer; `ask_how`, `ask_next`; steps as atoms | Dinner dialogue passes end to end |
| F3 Long message not understood (44%) | Segmenter, one answer per sentence, retry and split, partial replies | 90% of sentences recovered; not understood 10% or less |
| F4 Echo: "Okay, I'll remember that. Your bike has flat tire." | Session memory; acknowledge only lasting news | Echo-only replies 5% or less |
| F5 "Glad I could help" as the answer to a request | A social line never replaces content | Zero social-only replies to a message with a request |
| F6 Same self-description again and again | Narrow `about_you`; self layer; no repeated reply | No identical reply twice in a conversation |
| F7 "Not quite" read as a fact correction | `feedback` act | Zero correction misfires on the regression set |
| F8 Garbled facts and predicate soup | Closed vocabulary, typed arguments, grounding, round trip | Reviewer rejections 5% or less; new predicates per conversation falling |
| F9 Facts leaking between conversations: "You make tomato soup" | Session memory; personal facts stay out of the vault | Zero answers proved from another session's facts |
| F10 Stiff or wrong wording: "sunglass", "berlin" | Surface lexicon, sentence frames, clause realizer | New wording preferred in 65% of pairs on the same content |
| F11 Teachers repeat, apologise, ask for formatting | Teacher protocol with two channels | Zero meta requests; facts stored per teach block |
| F12 Off-topic "I noticed a pattern" questions | Curiosity limited in user chat | Zero off-topic questions in user chat |

**In learning and knowledge**

| v1 failure | Fix in 2.0 | Test |
| --- | --- | --- |
| Taught four times, 0 facts stored, 0 help | Procedures as knowledge; steps attach to the open task | Second encounter passes without teaching |
| Guessed rules never used (1 of 73 confirmed) | Tentative layer; "My rule is ... right?" questions | 80% of guesses confirmed or rejected |
| One correction rewrites a rule for good | Exception first; rule changes after two shared counterexamples | Correction test set |
| No learning of words or grammar | Registry and queue; learned patterns; sentence frames | Share of turns without the model rises |
| Confidence ignored in answers | Hedged replies | A weak fact is answered "I think so" |
| "Usually" and "most" stored as always | Hedges kept as strength | Hedge test set |
| Rules confirmed on one teacher's word | Stated rules start as hypotheses | Rule status audit |
| Ordinary words stored as names (24 of 40 sampled) | No names from sentence-initial capitals | 95% of sampled names are real names |
| Bad lemmas and duplicates: `knive`, `clothe`, apple/apples | Exception list, written forms, merge proposals | Zero duplicate pairs after a hygiene pass |
| Over 200 predicates, most used once | Registry, synonyms, review queue | Share of predicates used once falls |
| Personal facts left open in the vault (44) | Session memory | Zero personal facts in the vault unless promoted |
| Reviewer closed facts taught by others | Fact ids; review against the source sentence | Reviewer test |

**In engineering and measurement**

| v1 failure | Fix in 2.0 | Test |
| --- | --- | --- |
| Tests stayed at 98 to 100% while chat stayed poor | Evaluation on real turns, outcome labels, second encounter | Outcome mix reported for every build |
| "Understood 56%" hid 0 helpful replies | Outcome labels replace "understood" | Shown in the training UI |
| Judge scores too noisy to compare | 8 or more judges, same seeds, pairwise A/B | Same commit twice gives rates within 5 points |
| 3 to 5 full evaluations per message | One model per message | Under 100 ms at the 95th percentile on 20,000 facts |
| History questions mixed different times | Evaluation as of one moment | "Used to" test set |
| One bad rule removed all rules | Quarantine; core rules always stay | Bad-rule test |
| Reverted correction left partial changes | All-or-nothing changes | Revert test |
| "No, Anna likes coffee" deleted "likes tea" | Retract only single-valued or explicit negation | Correction test set |
| Open question lost on any next message | Question stack | Resume test after two side remarks |
| ", and" and decimals broke sentence splitting | Real sentence splitter | Long-message set |
| Fragile parallel training; a corrupted log line | Single vault writer | 16 parallel teachers, zero corrupted lines |
| Translation 5.2 s; prompt cache written, almost never read | Spec in a cached system prompt, low effort, learned patterns | Median under 2 s |
| Trainer API open to any local page | Token and origin check | API test |
| No tests for trainer, pretraining and model calls | Mocked model in tests, required per milestone | Test suite |
| PROGRESS.md out of date | Updating it is part of every acceptance test | Checked at each milestone |
| Runtime vault mixed into the code history | Vault kept out of the code repository | Repository check |

## Milestones

Seven milestones, numbered on from v1's eleven. Measurement comes first, then the knowledge model, then one stage of the pipeline at a time, and learning is proved last. The dinner dialogue is the test that runs through all of them.

**12. Honest measurement and safe ground**

Build: the turn trace, the event stream with the first dashboard panels, outcome labels, the real-turn regression set, oracle-parser mode, comparison against the previous build, a fresh vault. Also the single vault writer, one model per message, the v1 correctness fixes (bad-rule fallback, correction revert, "used to", over-retraction), the trainer token, and tests with a mocked model.

Accept:

- One command prints the outcome mix with confidence intervals and appends a row to the history file.
- The same commit run twice gives rates within 5 points.
- 16 parallel teachers run for 10 minutes with zero corrupted log lines.
- The agent's own work stays under 100 ms at the 95th percentile on a copy of the v1 vault.
- All v1 tests still pass, and the v1 baseline is recorded with the new labels.

**13. Knowledge model**

Build: decimal numbers, units and ranges; fact ids; the registry with typed arguments; the procedural, choice, lexical and self layers; procedure and task file formats; events; aggregates and arithmetic; evaluation as of one moment.

Accept:

- 100 generated procedure files round-trip with no unintended changes.
- At least 40 new reasoner cases pass and print their proofs: total duration by `sum`, all needs including sub-tasks, the next step in a partial order, the missing items given what the user has.
- The validator rejects 20 ill-typed facts taken from the v1 vault.
- A "used to" test shows facts from different times are never joined.

**14. Understanding**

Build: the segmenter, the dialogue acts, the three-tier translator, the validator with grounding and round trip, partial understanding, prompt and result caching.

Accept:

- Translator gold set: atom-level F1 of 0.93 or more, act accuracy of 95% or more.
- Long-message set: 90% or more of sentences recovered, and one broken sentence never loses the others.
- Zero stored facts with a constant that is not in the message, the session or the registry.
- Median translation time under 2 s.
- Real-turn regression set: not understood 15% or less.

**15. Dialogue manager**

Build: session memory, task frames, slot questions, move rules, choosing among options, "teach me", the feedback act, the question stack.

Accept:

- With the recipe in the vault, the dinner dialogue passes end to end: first with the oracle parser, then with the real translator.
- After two side remarks, "ok, what's next?" resumes at the right step.
- "Why do I need a jack?" is answered from a proof.
- On dev scenarios: requests answered with a fitting move 80% or more, echo-only 5% or less, zero answers proved from another session's facts.

**16. Generation**

Build: reply planner, microplanner, clause realizer, surface lexicon and sentence frames, wording rules.

Accept:

- 200 content plans: all pass the grammar checks, all claims are in the knowledge base, all claims parse back.
- No identical reply twice in a 12-turn conversation.
- On the same content, judges prefer the new wording over the old in 65% or more of pairs.
- The ceiling run is recorded, with the gap between it and the symbolic replies.

**17. Learning**

Build: procedures from teach blocks and from steps told over several turns; the tentative layer and rule questions; gradual corrections; words and frames through the queue; learned input patterns; the consolidation pass.

Accept:

- Garlic tomato pasta is taught in one session. After a restart and one unrelated session, the dinner dialogue passes in new words with no teaching.
- One counterexample stores only an exception. Two with a shared feature change the rule.
- In a scripted teacher run, 80% or more of guessed rules end confirmed or rejected.
- After 300 teaching turns, learned patterns handle 20% or more of turns and agree with the model 95% of the time.
- A new word enters as pending and cannot be spoken until it is approved.

**18. Curriculum training and hygiene**

Build: the teacher protocol, the curriculum, the four encounters, the spaced queue, the new curiosity goals, the hygiene pass, 30 seed procedures, the full training dashboard.

Accept:

- 20 held-out scenarios: second encounter solved without teaching in 70% or more, not understood in 10% or less of turns, and helpful answers plus fitting clarifications in 60% or more.
- 15 variants: transfer in 40% or more.
- Stored facts that contradict ground truth: 1% or less.
- Zero teacher meta requests, and zero duplicate entity pairs after a hygiene pass.
- Judge mean of 6.0 or more with its interval above v1's best 4.3, and a pairwise preference of 70% or more over the milestone 12 build.

**Later, optional:** multi-condition rule learning in the style of Popper, forgetting, fully incremental evaluation, one grammar shared by parsing and generation, bulk import of outside knowledge, and messages the agent starts by itself.

## Build brief for Claude Code

This section is written for an autonomous Claude Code run, like the brief in v1. 2.0 is built inside the v1 repository; the engine, the vault code and the test harness are extended, not rewritten.

**Working rules**

- Work through milestones 12 to 18 in order. Do not start one until the previous one passes its acceptance tests. Commit after each, with a message naming it.
- Run `cargo test` after every change. Never leave the build broken at a commit.
- Keep `PROGRESS.md` current. Updating it is part of every milestone's acceptance: current milestone, what works, what failed, decisions and why.
- End every milestone with a short report: the outcome mix before and after on the same scenarios and seeds.
- When stuck on one approach for long, write the problem in `PROGRESS.md`, choose the simplest working fallback, and move on.
- Use sub-agents for side work: test data, ground-truth files, code review. Test data is written from specifications alone, without sight of the code. Held-out results are shown only as counts.

**Code layout**

- New crates: `sla_dialogue` (session memory, frames, moves), `sla_translate` (segmenter, model calls, validator), `sla_lexicon` (registry, written forms, frames).
- Model calls move out of `sla_parser`. The parser crate holds only fixed and learned patterns.
- Generation reads verbs and word forms from `sla_lexicon`, not from the parser crate.
- `sla_agent` becomes a thin loop over the six stages, and every stage writes to the turn trace.
- One process owns the vault. The chat, the trainer and the evaluation talk to it.

**Model use**

- A model translates input and plays teacher, reviewer and judge. It never writes a reply.
- Calls stay lean: a short system prompt of our own, no tools. The language spec goes in the system prompt so it is cached. Translation uses low effort.
- Every code path that calls a model has a test with a mocked model.
- No human in the loop while building: teacher agents play the user. Starting a training run that spends the plan's limits stays a user action.

**Data**

- The runtime vault and the session files are not committed with the code.
- Start from a fresh vault. Replay the v1 teaching transcripts through the new pipeline once milestone 14 passes.
- Ground truth, scenarios, checklists and gold parses live in `data/` and are versioned.

**Definition of done for 2.0**

Milestones 12 to 18 pass their acceptance tests. `PROGRESS.md` is current. The evaluation report shows the second-encounter rate, the transfer rate and the outcome mix next to the v1 baseline on the same scenario set.

## Known limits and risks

2.0 has a fix for every failure v1 showed, but a reply path that is fully symbolic keeps some limits by design.

| Risk | Why it stays | How 2.0 holds it down |
| --- | --- | --- |
| Unknown tasks | Everyday help is open-ended; each task must be taught | A graceful "teach me" with structured questions; 30 seed procedures; the curriculum |
| Assembled-sounding replies | Sentences are built, not written freely; small talk suffers most | Planner, microplanner and learned patterns; naturalness is measured apart from helpfulness |
| Vague conditions | "Until soft" and "a splash" have nothing to reason over | Stored and repeated as given; only the numbers are reasoned over |
| The translator slipping in knowledge | More context makes it easier to add an answer to a translation | Grounding check on every constant; an audit log of every translation |
| Wrong procedures | One wrong step spoils a whole answer | Procedures are hypotheses until a second teacher or a successful use |
| Learned patterns freezing an error | A pattern copies what the model did | Patterns come only from reviewed translations and run in shadow first |
| Learning "Haiku English" | Teachers, judges and paraphrasers are the same model | Held-out scenarios, different prompts per role, a small set labelled by a person |
| Seven milestones of new parts | More parts, more ways to break | Each stage has its own test, and oracle mode shows which stage failed |
| Targets set before measuring | The v1 baseline was never measured with the new labels | Milestone 12 measures it first; targets are adjusted once, then fixed |

## Decisions

All six choices were confirmed on 2026-11-17 as written below. The build follows them.

| Decision | Decision (confirmed) | Why |
| --- | --- | --- |
| Fresh vault, or migrate the v1 vault? | Fresh vault; replay the v1 teaching transcripts; keep the old vault as an archive | The old vault carries the known junk, and a replay measures 2.0 on the same input |
| Keep the runtime vault in git with the code? | No; keep it beside the repository with its own snapshots | It holds personal facts from early chats and mixes knowledge history with code history |
| May a real user see "probably" answers that rest on guessed rules? | Only in training until milestone 18 has numbers | A wrong guess shown to a user costs more trust than "I don't know" |
| Translator through the Claude CLI on your plan, or through the API? | Stay on the CLI behind the `Parser` trait | The validator with one retry already enforces the closed vocabulary; revisit if the median stays above 2 s |
| Import outside knowledge in bulk? | Not in 2.0; only the 30 seed procedures written by teachers | Bulk imports bring errors nobody reviewed, and they do not show learning from conversation |
| Retire `sla_pretrain` and the old trend dashboard? | Retire the dashboard; fold pretraining into the trainer | The trainer and its UI already replace both |

Still open: the second-encounter target of 70%. The report suggests 50% in one place and 80% in another.
