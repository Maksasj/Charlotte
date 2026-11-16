# PROGRESS

Build log for the Symbolic Learning Agent (see DESIGN.md).

## Current milestone

**Milestone 8: learning from corrections and examples** (milestones 1-7 pass and are committed).

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
| 8-10 | Corrections/examples, curiosity, eval | not started | - |

## Layout

```
Cargo.toml                 workspace
crates/core      sla_core      types, Datalog syntax, vault files, knowledge base ops
crates/reasoner  sla_reasoner  runtime Datalog engine with proof traces
crates/learner   sla_learner   pattern-counting learner (hypothesis rules, confidence, promotion)
crates/nlg       sla_nlg       reply generator: acts -> English, aggregation, pronouns, why-from-proof
crates/parser    sla_parser    pattern parser (intents + Datalog slots), `Parser` trait
crates/agent     sla_agent     Agent (respond loop), module router, `sla` CLI binary
crates/eval      sla_eval      (placeholder)
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

## Problems

- 2026-11-16: no Rust toolchain at first; the install was blocked by the session's
  permission policy until the user installed rustup (GNU host toolchain). Resolved.
- First build: everything compiled. One suite expectation was wrong: with
  `is_a(penguin, bird)` the *category* `penguin` also satisfies `flies(X)` because
  `abnormal` only matches instances of penguin. The engine was right; the test was fixed.
- The knowledge-base layer (`core/src/kb.rs`) landed in the milestone 1 commit
  because `core/src/lib.rs` already referenced it; its acceptance test is milestone 3's.
