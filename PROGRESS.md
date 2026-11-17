# PROGRESS

Build log for the Symbolic Learning Agent. The current design is
`research/Symbolic Learning Agent – Design 2.0.md` (milestones 12 to 18);
v1 is `research/DESIGN_1.md` (milestones 1 to 11) with its findings in
`research/REPORT_DESIGN_1.md`.

## Current milestone

**Milestone 12 (honest measurement and safe ground): done.** Every
acceptance check passes (table below), including the v1 baseline labelled by
the Haiku judge (218 calls) and the judge's calibration on the hand labels.
Next: milestone 13 (knowledge model).

## How to run

From the repo root (Rust toolchain: stable-x86_64-pc-windows-gnu):

- chat: `cargo run --release -p sla_agent --bin sla` (`-- --vault DIR`, `--script FILE`, `--llm`);
  commands `:facts`, `:rules`, `:log`, `:reload`, `:demo ...`, `:quit`.
  Turn traces go to `sessions/` beside the vault.
- all tests: `cargo test` (no model calls: every model path runs on a mock)
- outcome mix with 95% intervals, one row appended to `eval/outcomes.csv`,
  report in `eval/outcomes/`:
  - v1 baseline: `cargo run --release -p sla_eval -- outcomes --v1 --gold`
  - this build: `cargo run --release -p sla_eval -- outcomes --parser pattern`
    (`--parser llm[:MODEL]` or `--parser oracle:FILE`; `--judge haiku` labels
    content replies with a cached Haiku judge; `--set trainer|judge`)
- speed on a copy of a vault: `cargo run --release -p sla_eval -- bench --vault DIR`
- parallel writers: `cargo run --release -p sla_eval -- stress --teachers 16 --minutes 10`
  (the real trainer on mock models, then every log, event, trace and cache line is checked)
- training dashboard: `cargo run --release -p sla_eval --bin sla_trainer`, then open the
  printed `http://localhost:8770/?token=...` (the token is required; `--mock` plays every
  model role with a mock, to try the dashboard without spending plan limits)
- v1 knowledge tests and judges: `cargo run --release -p sla_eval` (as in v1)

## Status by milestone

| # | Milestone | Status | Acceptance test |
|---|-----------|--------|-----------------|
| 1-10 | v1 | passing | see the v1 table below |
| 12 | Honest measurement and safe ground | built; model-run parts pending | `crates/agent/tests/safe_ground.rs`, `crates/eval/tests/measurement.rs`, `sla_eval bench`, `sla_eval stress` |
| 13 | Knowledge model | in progress: decimals, arithmetic, aggregates, fact ids, procedural/choice/session/lexical/self layers, registry done | `crates/reasoner/tests/knowledge_model.rs` (44 cases), `crates/lexicon` |
| 14 | Understanding | not started | |
| 15 | Dialogue manager | not started | |
| 16 | Generation | not started | |
| 17 | Learning | not started | |
| 18 | Curriculum training and hygiene | not started | |

### Milestone 12 acceptance

| Acceptance | Result |
|---|---|
| One command prints the outcome mix with confidence intervals and appends a row | `sla_eval outcomes`: table with Wilson 95% intervals, row in `eval/outcomes.csv`, per-turn report, comparison with the previous comparable row |
| The same commit run twice gives rates within 5 points | identical (0 points): replay on a fixed clock, judge labels cached by turn; `measurement.rs::replay_is_repeatable` |
| 16 parallel teachers for 10 minutes, zero corrupted log lines | `sla_eval stress --teachers 16 --minutes 10`: 600 s, 30 runs, 29,697 conversations, 185,427 turns on mock models; 0 corrupted among 230,494 log lines, 244,881 events, 185,427 trace lines and 1,950 cache lines (the first attempt idled after the trainer's 1000-conversation cap; `stress` now starts runs until the time is used) |
| Agent's own work under 100 ms at the 95th percentile on a copy of the v1 vault | 2,171 facts, 296 regression turns: median 21 ms, 95th percentile 63-71 ms (was 639 ms at the start of the milestone) |
| All v1 tests pass, the v1 baseline recorded with the new labels | all v1 tests pass (one changed on purpose, see decisions); v1 baseline recorded for all 296 turns: hand labels for the 68 trainer turns, templates and the Haiku judge for the rest (`eval/outcomes.csv`, `eval/outcomes/2026-11-17T08-53-10.md`) |

### Baselines with the new labels (2026-11-17)

| Run | Turns | Helpful + clarify | Not understood | Echo | Wrong move | Wrong content | Requests with a fitting move |
|---|---|---|---|---|---|---|---|
| v1 recorded replies, trainer set (hand labels) | 68 | 0.0% [0, 5.3] | 44.1% | 16.2% | 26.5% | 7.4% | 4 of 61 (6.6%) |
| v1 recorded replies, judge set (templates + Haiku judge) | 228 | 10.1% [6.8, 14.7] | 7.9% | | | | |
| **v1 recorded replies, all 296** (hand + templates + judge) | 296 | **7.8%** [5.2, 11.4] | 16.2% | 28.4% | 19.9% | 23.6% | 9 of 80 (11.2%) |
| this build replaying the 296 messages, pattern parser, no judge | 296 | 0.3% | 27.7% | 8.8% | 0.0% | 0.0% | 0 of 80 (63.2% unjudged) |

The judge agrees with the hand labels on 54 of 68 trainer turns (79%). Most
disagreements are between neighbouring failures: hand `wrong_move`, judge
`wrong_content` (6); hand `wrong_content`, judge `echo` (3). It never called a
failure helpful there. Judge labels are cached, so repeated runs agree exactly.

Against the design's starting figures: the help-scenario baseline is as the
report said (0 helpful, 44% not understood). Over all 296 turns echo is 28.4%
(the design said 24%) and wrong content, not measured before, is 23.6%. The
2.0 targets in the design are left as written; only the second-encounter
target (70%) was still open, and it is yours to fix.

The replay uses a fresh vault per conversation and only the pattern parser,
so "not understood" is higher than v1's (v1 used Haiku as translator and a
shared vault). Nothing of 2.0's dialogue is built yet; this row is the
starting point the next milestones are compared with. 80 "requests" are
counted by surface cues over all 296 user messages (61 in the trainer set;
the report's own regex found 57).

## 2.0 decisions

- **The brain owns the vault** (`crates/agent/src/brain.rs`). In one process there is
  one `Brain` per vault behind a mutex (`SharedBrain`); every conversation is an
  `Agent` (a session) holding a handle. This is the design's "single vault writer"
  inside a process: one in-memory copy, one writer of fact files, the log and stats.
  Translation runs before the lock is taken, so no model call ever happens inside it
  (failed translations are cached in memory for the session, so they are not retried
  under the lock either). Two processes on one vault (CLI and trainer at once) are
  still guarded only by the hand-edit fingerprint, as in v1.
- **One model per state of the vault.** The brain caches the evaluated model, keyed by
  a vault generation counter (bumped by every in-memory change) and the interval
  between fact boundaries, so answering, curiosity and the learner share it. A
  question costs at most one evaluation, and none if nothing changed
  (`safe_ground.rs::one_model_per_message`). A statement costs at most two (the
  shared model plus, only when a guessed rule is confirmed, the learner's model).
- **Usage counts moved to `vault/stats/uses.tsv`.** v1 rewrote fact files on every
  answer (`@uses`), which also invalidated the model cache. Old `@uses` on fact lines
  are still read and added.
- **History is evaluated moment by moment** (`reasoner::past_moments`, `Brain::past_models`):
  just before each fact end, and before each start of a fact that a negation depends on.
  Only the facts and rules the asked predicate depends on take part
  (`reasoner::depends_on`), at most 64 moments. A history answer never joins facts that
  were not true at one moment (`safe_ground.rs::history_never_joins_different_times`).
- **Rule quarantine** (`reasoner::quarantine`): a rule that makes the program unsafe or
  unstratifiable is set aside, logged once (`quarantine`) and shown in the trainer; core
  rules always run. v1 dropped every rule.
- **All-or-nothing changes** (`KnowledgeBase::transaction`): a vault snapshot is taken,
  log lines wait until commit, a failed change restores the snapshot. Corrections use it:
  a refinement that would break the program leaves the rule and its counter untouched
  and stores only the exception fact, reported as `Change::RefinementRefused`.
- **"Used to"** goes through `KnowledgeBase::assert_past`: a current copy is closed now;
  otherwise a closed fact is stored once.
- **Corrections retract the old value only for single-valued predicates**
  (`functional(P, K)`; `property(_, quality, _)` stays many-valued). "No, Anna likes
  coffee" keeps "Anna likes tea"; "No, Anna lives in Rome" ends "Paris". The v1 session
  test expected "No, Rex is a dog" to delete "Rex is a cat"; it now says "Rex is not a
  cat" explicitly.
- **The learner works in two passes.** Per message (once, at the end): re-score every
  hypothesis and propose new ones only around the entities the message told about,
  with an inverted feature index. The full pattern search (`Learner::update`) is the
  offline consolidation pass. Over the 296-turn replay on the v1 vault, time in the
  learner fell from 35.6 s to 2.2 s.
- **At most one curiosity question per turn**, chosen at the end of the turn.
- **Turn traces** (`crates/agent/src/trace.rs`): every turn records its parts, acts,
  translator tier, moves, question, reply, claims, vault changes (from the knowledge
  base journal), timings, model calls and evaluations. Written to
  `sessions/<session>.jsonl` and published on the brain's event stream
  (`crates/agent/src/events.rs`), which the trainer also writes to `sessions/events.jsonl`.
- **Every log line, event and trace is one `write_all`**, so parallel writers never
  interleave inside a line.
- **Outcome labels** (`crates/eval/src/outcome.rs`): helpful, partial, clarify,
  teach_me, not_understood, wrong_move, wrong_content, echo, and `unjudged` when no
  judge ran. Fixed-template replies are labelled by exact match; content replies by a
  Haiku judge with a rubric (versioned), cached in `eval/label_cache.jsonl` so runs are
  repeatable. Requests are found by surface cues ("can you", "how do I", "what should
  I", ...). Hand labels for the 68 v1 trainer turns are in
  `data/regression/labels_v1_trainer.tsv`; they reproduce the report's 30 of 68 not
  understood and 0 helpful.
- **The real-turn regression set** (`data/regression/turns.jsonl`, 296 turns) is
  extracted by `tools/extract_regression.py` from the committed v1 artefacts. A replay
  gives each conversation a fresh vault and a fixed clock.
- **Oracle-parser mode** (`crates/parser/src/oracle.rs`): gold acts by message text;
  the gold sets come with milestone 14.
- **Mock models**: `sla_parser::claude::{set_thread_mock, set_global_mock}`. The
  translator, the labeller and the whole trainer are tested on mocks.
- **Trainer API**: a random token (printed in the start-up address) on every `/api/`
  request, and a request with an `Origin` other than this trainer's own page is refused
  (`trainer.rs::refuse`, tested). New endpoint `/api/events?after=N`.
- **Retired** `sla_pretrain` and `tools/live` (the old trend dashboard), as decided; the
  reviewer moved into the trainer.
- **The runtime vault and `sessions/` are not in git** (`.gitignore`); the vault starts
  fresh and core files are seeded from `crates/core/assets/core/`.

## 2.0 problems

- 2026-11-17: the first benchmark on the v1 vault gave a 95th percentile of 639 ms with
  5 evaluations per turn. Causes: answering recorded usage in fact files (invalidating
  the model), history questions evaluated up to 64 full moments, and the learner's
  full pattern search ran on every statement. Fixed by the stats file, dependency-
  restricted history and the two-pass learner.
- The yes/no template for a fact that is no longer true starts with "Yes," ("Yes, Anna
  used to like tea, but not anymore.") when asked in the present. Content is right;
  wording is milestone 16.
- No browser automation is available here, so the dashboard was checked through its
  API (tokens, origin, state, events, a mock run) and by reading the page code, not by
  clicking through it.

# v1 (milestones 1 to 11)

## v1 status by milestone

| # | Milestone | Status | Acceptance test |
|---|-----------|--------|-----------------|
| 1 | Vault and data model | passing | `crates/core/tests/roundtrip.rs` (100 files) + unit tests |
| 2 | Datalog interpreter | passing | `crates/reasoner/tests/suite.rs` (54 cases + 8 tests) |
| 3 | Time and confidence | passing | `crates/reasoner/tests/time_conf.rs`, `core/src/kb.rs` tests |
| 4 | Parser | passing | `crates/parser/tests/dev_set.rs`: dev 200/200 exact; held-out 98/100 exact, 100/100 intent |
| 5 | Reply generator | passing | `crates/nlg/tests/replies.rs`: 100 replies, grammar checks, claims in KB, claims parse back |
| 6 | Agent loop | passing | `crates/agent/tests/session.rs` |
| 7 | Learner: pattern counting | passing | `crates/learner/tests/mornings.rs` |
| 8 | Corrections and demonstrations | passing | `crates/learner/tests/corrections.rs` |
| 9 | Curiosity drive | passing | `crates/agent/tests/curiosity.rs` |
| 10 | Synthetic teaching and evaluation | passing | `crates/eval/tests/one_command.rs` |

v1 baseline (2026-11-16, commit e16d620): parser 100% dev, 98% held-out; teaching
dialogues 96/96 and 45/47; conversation quality 3.33/10 from 3 judges. The full v1
analysis is in `research/REPORT_DESIGN_1.md`.

## Layout

```
Cargo.toml                 workspace
crates/core      sla_core      types, Datalog syntax, vault files, knowledge base ops (journal, transactions, stats)
crates/reasoner  sla_reasoner  runtime Datalog engine with proof traces, quarantine, past moments
crates/learner   sla_learner   pattern-counting learner (per-message pass, full consolidation pass)
crates/nlg       sla_nlg       reply generator: acts -> English, aggregation, pronouns, why-from-proof
crates/parser    sla_parser    pattern parser, LLM translator, oracle parser, Claude calls (mockable)
crates/agent     sla_agent     Brain (vault owner), Agent (session), turn traces, event stream, `sla` CLI
crates/eval      sla_eval      outcome labels, regression replay, bench, stress, trainer + dashboard, v1 evaluation
crates/core/assets/core/   built-in ontology + core rules, seeded into vault/core
data/regression/           v1 real turns and hand labels      data/synthetic/  v1 test sets
eval/                      history.csv (v1 tests), outcomes.csv (2.0), reports
vault/ sessions/           runtime state, not in git
```

## v1 decisions

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

## v1 problems

- 2026-11-16: no Rust toolchain at first; the install was blocked by the session's
  permission policy until the user installed rustup (GNU host toolchain). Resolved.
- First build: everything compiled. One suite expectation was wrong: with
  `is_a(penguin, bird)` the *category* `penguin` also satisfies `flies(X)` because
  `abnormal` only matches instances of penguin. The engine was right; the test was fixed.
- The knowledge-base layer (`core/src/kb.rs`) landed in the milestone 1 commit
  because `core/src/lib.rs` already referenced it; its acceptance test is milestone 3's.
