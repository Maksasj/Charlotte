# PROGRESS

Build log for the Symbolic Learning Agent (see research/DESIGN.md).

## Current milestone

**First run done**: milestones 1-10 pass their acceptance tests and the
evaluation baseline is recorded. Next: milestone 11 (optional) and the
conversation-quality issues listed under *Baseline* below.

## How to run

From the repo root (Rust toolchain: stable-x86_64-pc-windows-gnu):

- chat: `cargo run --release -p sla_agent --bin sla` (`-- --vault DIR`, `--script FILE`);
  commands `:facts`, `:rules`, `:log`, `:reload`, `:demo ...`, `:quit`
- all tests: `cargo test`
- full evaluation, appends a row to `eval/history.csv` and writes
  `eval/last_report.md`: `cargo run --release -p sla_eval`
  (`--no-claude` for the fixed tests only; `--judges N`, `--turns N`,
  `--teacher-questions N`, `--model M`, `--notes TEXT`)

Run everything with `cargo test` (Rust toolchain: stable-x86_64-pc-windows-gnu).

## Status by milestone

| # | Milestone | Status | Acceptance test |
|---|-----------|--------|-----------------|
| 1 | Vault and data model | passing, committed | `crates/core/tests/roundtrip.rs` (100 files) + unit tests |
| 2 | Datalog interpreter | passing, committed | `crates/reasoner/tests/suite.rs` (54 cases + 8 tests) |
| 3 | Time and confidence | passing, committed | `crates/reasoner/tests/time_conf.rs`, `core/src/kb.rs` tests |
| 4 | Parser | passing, committed | `crates/parser/tests/dev_set.rs`: dev 200/200 exact; held-out 98/100 exact, 100/100 intent |
| 5 | Reply generator | passing, committed | `crates/nlg/tests/replies.rs`: 100 replies, grammar checks, claims in KB, claims parse back |
| 6 | Agent loop | passing, committed | `crates/agent/tests/session.rs`: 10 facts + 2 rules taught, recalled after restart; history, corrections, hand edits |
| 7 | Learner: pattern counting | passing, committed | `crates/learner/tests/mornings.rs`: hypothesis after 5 mornings, conf drops after a counterexample; promotion at 20 cases; rejection |
| 8 | Corrections and demonstrations | passing, committed | `crates/learner/tests/corrections.rs` over `data/synthetic/corrections/cases.md` (10 cases: right rule changes, nothing else lost); demo rules; agent conversation test |
| 9 | Curiosity drive | passing, committed | `crates/agent/tests/curiosity.rs`: asks about a stated contradiction and an unconfirmed hypothesis; gap questions; daily cap; asked-memory across restarts |
| 10 | Synthetic teaching and evaluation | passing, committed | `crates/eval/tests/one_command.rs`; `cargo run --release -p sla_eval` appended the baseline row |

## Baseline (2026-11-16, commit e16d620)

| Score | Dev | Held-out |
|---|---|---|
| Parser exact accuracy | 100% (200/200) | 98% (98/100) |
| Teaching-dialogue checks | 96/96 | 45/47 |
| Knowledge score (checks + facts + rules) | 100% | 97.8% |
| Conversation quality (3 Haiku judges, 6 turns) | 3.33 / 10 (scores 2, 3, 5) | |

Judges' main complaints, to work on next (watch the trend, not single scores):
- long compound sentences ("X, and she is a teacher at a school near my office")
  are not understood; only short sentence patterns work (a known limit in DESIGN.md);
- "I already knew that" when the user repeats a fact sounds odd in conversation;
- failure replies rotate wording for the same input, which reads as inconsistent;
- multi-word possessives get mangled ("Anna's cat Mia is a nice pet");
- "lives in" is echoed back as "is in".

Live teacher phase: the baseline run asked 0 questions. Two causes: (1) in scripted
dialogues the agent asked its curiosity questions in chat and the scripted next line
ignored them (fixed: no in-chat questions during scripted dialogues); (2) the
dialogues are small worlds that rarely leave a gap or pattern open (partly addressed:
named subjects now form a category for gap questions). A third bug showed up: the
teacher's "I don't know." was stored as `-know(maksim)`; "I don't know" / "no idea"
are now non-answers. After the fixes the live run asked 1 question and learned
nothing false (row 3 of `eval/history.csv`). Richer dialogues are needed for this
metric to mean much.

## Layout

```
Cargo.toml                 workspace
crates/core      sla_core      types, Datalog syntax, vault files, knowledge base ops
crates/reasoner  sla_reasoner  runtime Datalog engine with proof traces
crates/learner   sla_learner   pattern-counting learner (hypothesis rules, confidence, promotion)
crates/nlg       sla_nlg       reply generator: acts -> English, aggregation, pronouns, why-from-proof
crates/parser    sla_parser    pattern parser (intents + Datalog slots), `Parser` trait
crates/agent     sla_agent     Agent (respond loop), module router, `sla` CLI binary
crates/eval      sla_eval      evaluation: parser sets, teaching dialogues, Claude teachers, Haiku judges, history.csv
crates/core/assets/core/   built-in ontology + core rules, copied into vault/core on first open
vault/  data/synthetic/  eval/
```

## Decisions

- **Package names `sla_*`**: a package named `core` would clash with Rust's built-in
  `core` crate, so directories follow DESIGN.md (`crates/core`, ...) and packages are
  prefixed `sla_` (symbolic learning agent).
- **Zero external crates** so far (no chrono, no rand, no tempfile): fewer moving
  parts, and the build does not depend on crates.io. Timestamps use Hinnant's civil
  date algorithms; tests use a small LCG.
- **Fact line format**: `likes(maksim, coffee). @from 2026-11-16 @until ... @conf 0.9 @source user @uses 3 @last ...`.
  `@uses`/`@last` track usage from the start so forgetting is easy to add later.
- **Lossless round-trip**: unmodified lines keep their original text; only changed
  items are re-rendered. Invalid lines are kept as written and reported, not dropped.
- **Negation**: `-p(..)` is strong (explicit) negation, stored like any predicate;
  `not p(..)` in rule bodies is negation as failure, stratified.
- **Contradictions**: asserting `-p` closes an open `p` (sets `@until`) and vice versa;
  `functional(P, K)` declarations in core close the old value of a functional key
  (e.g. `property(X, colour, V)`, `located_in(X, Y)`). Facts are never deleted.
- **Confirmation** of an already-current fact halves its remaining doubt
  (`c + (1-c)/2`), rounded to 3 decimals.
- **Derived confidence** = min(rule conf, premise confs), using the first
  derivation found (a later, more confident derivation does not replace it; revisit if
  it matters).
- **Rule statements are one per line** in vault blocks (no multi-line rules yet).
- **Core property inheritance** uses only explicit `-property` exceptions. An earlier
  draft with an "own value overrides" rule created negation through recursion, which
  is unstratifiable.
- **Rust toolchain**: the plan is the `x86_64-pc-windows-gnu` host via rustup, because
  this machine has no MSVC build tools and the GNU toolchain bundles its own linker.

- **Parser test data is independent**: a sub-agent wrote `data/synthetic/parser/dev.md`
  (200) and `held_out.md` (100) from `docs/parser_spec.md` only, without seeing the
  code. The held-out test prints only a score, never the failing sentences, so it
  stays untouched by tuning.
- **Parser output** is an `Intent` with Datalog atoms; `canonical()` gives the one-line
  form tests compare. "I/me/my" = the speaker (default `maksim`). Plural or "every"
  subjects with ordinary verbs become rules over `is_a(A, category)`; with
  have/be they stay facts about the category (core rules inherit them).

- **NLG faithfulness** is checked two ways: every clause is generated from an atom
  listed in `Reply::claims` (the test checks each claim is true in the model), and
  each claim's sentence is parsed back by our own parser to the same atom.
  Variation rotates templates deterministically, so tests stay repeatable.

- **CLI**: `cargo run -p sla_agent --bin sla -- [--vault DIR] [--speaker NAME] [--source LABEL] [--script FILE]`.
  `--script` sends each line and prints the transcript (for teacher/judge sub-agents).
  Commands `:facts`, `:rules`, `:log`, `:reload`. Hand edits are noticed by a vault
  fingerprint (mtimes/sizes) and reloaded before the next message.
- **Proper names**: capitalized words inside a sentence are stored as
  `proper_name(x)` facts (source `parser`) so replies capitalize them after restarts.
- **User-stated rules** are stored as confirmed with conf 0.9 in their own rule file,
  filed under a module by the router (`people`, `places`, `things`, `events`, `food`,
  `work`, `tools`, `general`). A rule that makes the program unsafe or unstratifiable
  is refused.
- **"That's wrong"** closes the last answer if stored and records its strong negation.

- **Learner (pattern counting)**: features = an entity's facts with the entity as
  variable. Best single-feature body per head (highest ratio, then most specific,
  then with constants). Confidence `evidence / (evidence + counter + 5)`: starts low,
  rises with cases, drops with counterexamples; promoted at 0.8 (20 clean cases) or
  on user confirmation; rejected when counterexamples are at least twice the support.
  Bookkeeping predicates (event, during, agent_of, is_a, ...) never become heads.
  Missing head facts count as counterexamples (closed world for observed episodes),
  except open-world predicates (knows, wants, has) where only explicit negation counts.
  Learned rules are excluded when counting, so a promoted rule cannot support itself.
- With no contrasting data (only mornings), several bodies predict equally well
  (`event`, `agent_of`, `during`); the learner cannot know which matters. Tests give
  contrast (evenings with tea). Multi-feature bodies wait for ILP (milestone 11).

- **Corrections** refine the rule at the top of the wrong conclusion's proof: a
  `not feature(X)` condition when the wrong case has a feature none of the rule's
  other cases share (preferring `is_a` categories), else the exception `not -head`.
  The negative fact is always stored. Core rules are read-only: only the negative
  fact is stored, leaving a contradiction for the curiosity drive. A wrong *stored*
  fact is closed instead (its consequences disappear with it).
- **Demonstrations** (`:demo ACTION | before: ... | after: ...` in the CLI) become
  effect rules: added facts -> heads, removed facts -> negated heads, action +
  unchanged facts about the participants -> body, action constants -> variables.
  Repeated demonstrations intersect the preconditions (incidental details drop out).
- The correction test set (`data/synthetic/corrections/cases.md`) plays the teacher's
  part with exact expected outcomes; live Claude teacher sub-agents come with
  milestone 10.

- **Curiosity**: goals are contradictions (1.0), hypotheses (0.5-0.9 by evidence) and
  gaps (0.3-0.5: a category member lacks a predicate most peers have). The drive
  level rises with open goals and drops on learning progress (more when the answer
  was to its own question). A small HTN-style `plan` turns a goal into one question
  and a `Pending` answer slot. Questions go to a `Teacher` (`Agent::ask_teacher`),
  are appended to replies while chatting (cap 8/day), or start a conversation
  (`Agent::proactive`, cap 3/day). Asked goals are logged and never asked twice.
- Answering a contradiction question with one side ("Pingu does not fly") is
  handled as a correction of the other side, so the rule gets refined.
- A sentence-initial capitalized word counts as a proper name when followed by a
  singular verb and not a function word ("Pingu is", "Tweety flies").
- Parser: a single capitalized word after "is" is an adjective ("Anna is British"
  -> `property(anna, quality, british)`); dev and held-out scores unchanged.

- **Evaluation**: knowledge tests are exact and repeatable (parser sets, 18
  teaching dialogues written by a teacher sub-agent from `docs/dialogue_format.md`).
  Held-out failures are reported only as counts. Live Claude teachers answer the
  agent's curiosity questions from each dialogue's ground truth only; a teacher
  answer counts as consistent if what the agent learned does not contradict that
  truth. Haiku judges role-play a user for N turns, then score 1-10 with a reason;
  transcripts are in `eval/reports/`.
- **Agent conversation handling** added for the judges: messages with several
  sentences are handled one by one, "he/she/it/they" refer to the last entity
  talked about, "Hi, I'm Maksim" is a greeting, "likes to eat X" -> `likes(_, X)`.

## Problems

- 2026-11-16: no Rust toolchain at first; the install was blocked by the session's
  permission policy until the user installed rustup (GNU host toolchain). Resolved.
- First build: everything compiled. One suite expectation was wrong: with
  `is_a(penguin, bird)` the *category* `penguin` also satisfies `flies(X)` because
  `abnormal` only matches instances of penguin. The engine was right; the test was fixed.
- The knowledge-base layer (`core/src/kb.rs`) landed in the milestone 1 commit
  because `core/src/lib.rs` already referenced it; its acceptance test is milestone 3's.
