# PROGRESS

Build log for the Symbolic Learning Agent (see DESIGN.md).

## Current milestone

**Milestone 5: reply generator** (milestones 1-4 pass and are committed).

Run everything with `cargo test` (Rust toolchain: stable-x86_64-pc-windows-gnu).

## Status by milestone

| # | Milestone | Status | Acceptance test |
|---|-----------|--------|-----------------|
| 1 | Vault and data model | passing, committed | `crates/core/tests/roundtrip.rs` (100 files) + unit tests |
| 2 | Datalog interpreter | passing, committed | `crates/reasoner/tests/suite.rs` (54 cases + 8 tests) |
| 3 | Time and confidence | passing, committed | `crates/reasoner/tests/time_conf.rs`, `core/src/kb.rs` tests |
| 4 | Parser | passing, committed | `crates/parser/tests/dev_set.rs`: dev 200/200 exact; held-out 98/100 exact, 100/100 intent |
| 5-10 | NLG, agent loop, learner, curiosity, eval | not started | - |

## Layout

```
Cargo.toml                 workspace
crates/core      sla_core      types, Datalog syntax, vault files, knowledge base ops
crates/reasoner  sla_reasoner  runtime Datalog engine with proof traces
crates/learner   sla_learner   (placeholder)
crates/nlg       sla_nlg       (placeholder)
crates/parser    sla_parser    pattern parser (intents + Datalog slots), `Parser` trait
crates/agent     sla_agent     (placeholder)
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

## Problems

- 2026-11-16: no Rust toolchain at first; the install was blocked by the session's
  permission policy until the user installed rustup (GNU host toolchain). Resolved.
- First build: everything compiled. One suite expectation was wrong: with
  `is_a(penguin, bird)` the *category* `penguin` also satisfies `flies(X)` because
  `abnormal` only matches instances of penguin. The engine was right; the test was fixed.
- The knowledge-base layer (`core/src/kb.rs`) landed in the milestone 1 commit
  because `core/src/lib.rs` already referenced it; its acceptance test is milestone 3's.
