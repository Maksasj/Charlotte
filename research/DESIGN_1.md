# Symbolic Learning Agent – Architecture

A minimal, text-based agent that runs on a PC, talks with its user, and learns over time. Its reasoning, memory and learning are symbolic, so they stay deterministic, traceable and editable. A neural model is optional and used only to translate language.

## Goals

- Hold basic conversations in text.
- Learn facts, rules and conclusions from what the user says, shows and corrects.
- Explain why it believes or concludes anything.
- Grow by adding data and modules instead of retraining.

## Design decisions

- **Scope:** a general text agent, not tied to one domain. It learns about many topics over time, so the shared core ontology is the most important design choice.
- **Cold start:** Claude (for example through the Claude CLI) acts as a teacher. It holds basic conversations with the agent and generates synthetic data to seed first facts and rules. A review step checks what gets learned, since mistakes would be stored as truth.
- **Knowledge format:** Datalog-style. Facts are simple statements such as `likes(maksim, coffee)`. Rules are if-then lines built from the same pieces.
- **Storage:** a file-based vault, like Obsidian, so the user can read and edit memories, facts and rules by hand.
- **Time and contradictions:** facts are never deleted when contradicted. Each fact has a start time and, if needed, an end time, so the newest fact counts as current and old facts stay as history. This matches the time-based memory design from the Minecraft agent.
- **Forgetting:** left for later. The agent keeps everything at first. Forgetting is designed once the basic system runs and real data-size trade-offs show up. Track usage counts and last-used time from the start so forgetting is easy to add.
- **Learning rules, not just facts:** rules are the core of reasoning, so the agent must learn new rules as well as new facts. It may guess rules from patterns. Guessed rules start as low-confidence hypotheses and become real rules only after more examples support them or the user confirms them.
- **Proactive curiosity:** the agent may start conversations by itself to ask about gaps or contradictions. Cap this at a few questions a day, and save less urgent questions for when the user is already chatting.
- **Language:** Rust. Most Rust Datalog crates, such as Ascent and Crepe, fix rules at compile time, so the agent needs its own small runtime Datalog interpreter that can load rules learned at runtime from the vault. Claude builds it over a few rounds of coding and testing.

## Built-in core ontology

The agent starts with a tiny built-in core of about 20 predicates, inspired by Anna Wierzbicka's semantic primes. Everything else is learned as combinations of these.

1. **Things:** `thing(X)`, `same(X, Y)`
2. **Categories:** `is_a(X, Y)`
3. **Parts and having:** `part_of(X, Y)`, `has(X, Y)`
4. **Properties:** `property(X, P, V)`, for example colour or size
5. **Time:** `before(A, B)`, `during(A, T)`, plus start and end times on facts
6. **Events and actions:** `event(E)`, `agent_of(E, X)`, `does(X, Action)`
7. **Cause:** `causes(A, B)`
8. **Minds:** `wants(X, Y)`, `knows(X, F)`, `believes(X, F)`, `feels(X, F)`
9. **Truth:** negation, plus a confidence score on every fact
10. **Sources:** `says(X, F)`, so every fact records where it came from

Plain Datalog is deliberately not Turing complete, so every query finishes.

## Testing

Two kinds of tests, tracked as scores over time:

- **Conversation quality:** several Haiku sub-agents chat with the agent, like a Turing test for this system, and score how natural and coherent it is. Useful but subjective, so watch trends rather than single scores.
- **Knowledge and reasoning:** fixed, repeatable tests. Teach it a set of facts or examples, then check that it stored the expected facts and rules and answers specific questions correctly. These are exact and catch regressions.

## Core components

1. **Input parser.** Turns user messages into formal facts, questions or commands. Start with pattern matching (ELIZA-style intents and slots). Optionally use an LLM as a pure translator into the formal language, never as the reasoner. Confirm important new facts back to the user before storing them.
2. **Knowledge base.** A structured vault (for example Obsidian-style Markdown files with a fixed schema) of facts, rules and concepts. Every item has a source, a timestamp and a confidence score.
3. **Reasoner.** Prolog/Datalog-style inference over the knowledge base. Answers questions, derives new facts, and keeps a proof trace for every conclusion.
4. **Learner.** Creates and corrects rules from:
   - examples and demonstrations (before/after traces become new rules),
   - corrections from the user,
   - generalization over labeled examples (Inductive Logic Programming, e.g. Popper),
   - chunking: saving solved reasoning chains as shortcut rules.
5. **Drives and goals.** Small internal needs that generate goals. Curiosity rises when the agent detects gaps or contradictions and drops when it learns something (reward learning progress, not raw novelty). Goals are handled by an HTN/GOAP-style planner.
6. **Output generator.** Grammar-based natural language generation: choose content, choose intent (answer, explain, ask, recall), then realize the sentence with templates, synonyms and aggregation. Imperfect wording is acceptable; it must only say what the agent actually believes.

## Message flow

User message → parser → formal statement → reasoner and learner update or query the knowledge base → drives may add a goal (for example, ask a clarifying question) → output generator → reply.

## Modularity

- Split knowledge into domain modules (for example projects, schedule, people), each with its own rules.
- Modules share a small core ontology of common concepts (time, person, object, cause, quantity) so that cross-domain questions work.
- A router chooses which modules are relevant to a question; cross-domain questions query several and combine results.
- Newly learned rules go into the right module, or start a new one.

## Keeping it healthy as it grows

- Confidence scores rise with confirmation and fall with failure.
- Detect contradictions and ask the user to resolve them.
- Merge duplicate rules, prune unused or low-confidence ones, and group similar items into concepts.
- Store every change with its reason so the knowledge base can be audited and rolled back.

## Known limits

- Understanding open-ended language is the weakest point without an LLM translator.
- Generated speech will be stiff and repetitive until the grammar is large.
- Generalization depends on good concepts; building them is the hardest part.
- Interactions between many rules need modularity and tests to stay manageable.

## Suggested first milestones

1. Knowledge base schema and a Datalog-style reasoner with proof traces.
2. Pattern-matching parser for a few intents: tell, ask, correct, why.
3. Template-based reply generator.
4. Learning from corrections and examples, with confidence scores.
5. Curiosity drive that asks about gaps and contradictions.
6. Optional LLM translator for free-form input.

## Build brief for Claude Code

This section is written for an autonomous Claude Code run. Work through the milestones in order. Do not start a milestone until the previous one passes its acceptance tests. Commit after each milestone with a message naming it.

### Working rules

- Language: Rust, a single Cargo workspace with crates `core` (types, vault), `reasoner`, `learner`, `nlg`, `parser`, `agent` (CLI loop) and `eval`.
- Run `cargo test` after every change. Never leave the build broken at a commit.
- Keep a `PROGRESS.md` at the repo root: current milestone, what works, what failed, decisions made and why.
- Use sub-agents freely for side work: generating synthetic data, writing test cases, reviewing code. The main agent integrates and verifies.
- When stuck on one approach for long, write the problem in `PROGRESS.md`, choose the simplest working fallback, and move on.

**No human in the loop for the first version.** The whole prototype runs without the user. Claude teacher sub-agents play the user's role: they hold teaching conversations, state facts and examples, answer the agent's curiosity questions, spot wrong answers and send corrections. Teachers must give answers that are consistent with the expected outcomes in the synthetic data, so learning can be checked exactly.

### Vault layout

- `vault/facts/`: one Markdown file per entity (for example `maksim.md`), with facts in a fenced `datalog` code block, each line carrying `@from`, optional `@until`, `@conf` and `@source`.
- `vault/rules/`: one file per rule or small rule group, with a short plain-English note, the rule in a `datalog` block, its confidence, status (`hypothesis` or `confirmed`) and evidence count.
- `vault/core/`: the built-in core ontology, read-only for the learner.
- `vault/log/`: an append-only log of every change with its reason, for audit and rollback.
- Files must stay readable and hand-editable. The agent reloads the vault on start and after edits.

### Milestones

1. **Vault and data model.** Parse and write the vault format, round-trip without loss. Accept: parse, modify and write back 100 generated files with no diffs except intended ones.
2. **Datalog interpreter.** A runtime engine with semi-naive evaluation, stratified negation, and proof traces for every derived fact. Rules load from files at runtime. Accept: a test suite of at least 50 cases, including recursion (ancestor), negation and time-bounded facts; every answer can print its proof.
3. **Time and confidence.** Facts carry start and end times; queries default to "now" and can ask about the past. Contradictions close the old fact with an end time instead of deleting it. Derived confidence is the minimum of the inputs. Accept: "I don't drink coffee anymore" closes `likes(maksim, coffee)` and the history query still finds it.
4. **Parser, first version.** Pattern-based intents: tell, ask, correct, why, yes/no. Accept: 200 synthetic sentences parsed with at least 90% correct intent and slots. Leave a trait `Parser` so an LLM translator such as Haiku can be plugged in later.
5. **Reply generator.** Content selection, intent (answer, explain, ask, recall, acknowledge) and grammar templates with variation and pronouns. "Why" answers are generated from proof traces. Accept: replies are grammatical on a 100-case test set and never state anything not in the knowledge base.
6. **Agent loop.** A CLI chat: message, parse, update or query, reply, log. Accept: a scripted conversation teaches 10 facts and 2 rules, and a later session recalls them after restart.
7. **Learner, first version: pattern counting.** Find facts that repeatedly co-occur, propose them as hypothesis rules with low confidence, raise confidence with supporting cases, lower it with counterexamples, and promote to `confirmed` past a threshold or on user confirmation. Accept: after 5 synthetic mornings with coffee, a hypothesis rule appears; after a counterexample its confidence drops.
8. **Learning from corrections and examples.** Corrections come from Claude teacher sub-agents that check the agent's answers and correct it when it is wrong. They add exceptions or conditions to rules; demonstrations as before/after state traces become new rules. Accept: a dedicated test set of corrections changes the right rules and nothing else.
9. **Curiosity drive.** Score gaps (unknown properties of known things), contradictions and weak hypotheses; queue questions; in the first version, questions go to a Claude teacher sub-agent instead of the user. Proactive messages to the user, capped at a few per day, come later. Accept: in a scripted session it asks about a stated contradiction and about an unconfirmed hypothesis.
10. **Synthetic teaching and evaluation.** Sub-agents generate teaching dialogues and fact sets as Markdown files in `data/synthetic/`, each with expected facts, rules and answers. The `eval` crate runs fixed tests and reports scores over time to `eval/history.csv`. Haiku judge agents chat with the agent and score conversation quality from 1 to 10 with a short reason. Accept: one command runs the full evaluation and appends a row.
11. **Later, optional:** proper rule induction (ILP, for example a Popper-style learner), an LLM translator behind the `Parser` trait, and forgetting based on usage counts and last-used times.

### Synthetic data guidelines

- Cover many domains (people, places, food, work, tools, events) so the core ontology gets exercised broadly.
- Every generated dialogue comes with its expected outcome, so tests are exact.
- Include contradictions, corrections, vague statements and noise, not only clean examples.
- Keep a held-out set that is never used while developing, to check for overfitting to templates.

### Definition of done for the first run

Milestones 1 to 10 pass their acceptance tests, `PROGRESS.md` is current, and the evaluation report shows a baseline score for both conversation quality and knowledge tests.
