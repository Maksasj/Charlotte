# Symbolic Learning Agent – Design 4.0 (draft)

**Status.** Draft, written on 2026-11-19 on branch `design-4`. It is not yet binding. The decisions P1-P16 in `research/DESIGN_4_preplanning.md` are binding, and this draft does not change any of them. Everything marked **Recommendation (D4.x, to decide)** is a proposal for the design session (milestone 30). The user confirms or changes it, and it is then recorded as decision D4.x in `research/DESIGN_4_decisions.md`. Phase B numbers marked **set after the spike** are deliberately left open: they are set from the spike report, not guessed (P16, the M24/M25 lesson of 3.0).

Inputs:
- the 3.0 results (`paper/sections/*.tex`, `PROGRESS.md`);
- the phase A and phase B findings in `research/DESIGN_3_experiments.md`, especially X4;
- the v3 schema (`research/DESIGN_3_schema.md`);
- the 3.0 crates (`kb3`, `agent3`, `eval3`, `teach3`) and the Datalog reasoner `crates/reasoner`.

Where code is relied on, the file and line are cited.

4.0 is the fourth research iteration. Like 3.0 (D1), it is not planned as the last.

## 1. Summary

Three iterations kept finding the same split:
- learning and recall from correct facts work;
- reading English is the bottleneck;
- generalisation failed even without language.

In 3.0's fact-only experiment X4, substitution by function found 19% of the needed changes and induced rules found 1.4%. The cause was not the logic. The knowledge base held procedures only, with no relations about things, and there was no general inference over relations.

4.0 changes the core question to the part that has never been tested:

> **Taught only through conversation, the agent derives correct answers to multi-hop questions in domains it was not taught directly. Every answer carries its proof, or, for an induced rule, its supporting examples. It makes no claim without one, and it gets better as it is taught more.**

What changes:

1. **Language leaves the core path (P4, P5).** The teacher (Haiku) teaches in the formal language. The agent answers in the formal language, with a proof for every derived claim. Controlled English is the first later layer, and free English comes after it.
2. **Knowledge about things, not only procedures (P9).** A small, domain-free set of relations over items, kinds, materials, functions and states sits beside the v3 procedure schema. It is the knowledge X4 found missing.
3. **Built-in logic, learned knowledge (P2, P6, P7).** A handful of universal base rules (inheritance with exceptions, transitivity, stand-ins by function, step replacement, contradiction) are the only built-in content. Every fact is taught in conversation. New rules are induced from taught examples, layer on layer, with support, confidence and retraction.
4. **Three-layer, replayable knowledge base (P11, P12).** The layers are:
   - raw teaching messages, append-only;
   - extracted facts and rules;
   - provenance.

   Facts and provenance are rebuilt from raw by replay under any agent version.
5. **A derivation benchmark as the measure of record (P15).** It holds 1-, 2- and 3-hop questions, including cross-domain ones and ones in a domain never taught. Its answers are never taught, only their pieces, and a script enforces that.
6. **A learning curve is the main result.** The benchmark is read at 25, 50, 75 and 100% of the curriculum, at several knowledge-base sizes.
7. **Spike first (P16).** Phase A is a design session and a zero-call offline spike. Phase B spends teaching budget only after the user's go.

The three kinds of new knowledge stay apart, as in pre-planning section 1:
- **Derived** knowledge is shown with its proof.
- **Induced** knowledge is shown as a hypothesis with its support, and retracted on a counterexample.
- **Invented** knowledge, a claim with neither, is never produced.

"Invents nothing" in this document means only the third kind.

## 2. What earlier iterations showed

| Iteration | What worked | What failed | Consequence for 4.0 |
|---|---|---|---|
| v1 (`REPORT_DESIGN_1.md`) | Facts, rules and proofs in a small Datalog reasoner | Parsing conversational English; most turns not understood | The reasoner stays; reading English leaves the core path |
| 2.0 (`REPORT_DESIGN_2.md`) | Procedures taught once, recalled 75-90% on second encounter; knowledge-model reasoner cases with proofs (44 cases, `crates/reasoner/tests/knowledge_model.rs`) | About 30% of taught steps stored wrong; about 225 hard-coded English strings; transfer 6/15 by item swap only; work drifted to style | No style metric; no English in the core; transfer must come from knowledge, not swaps |
| 3.0 (`paper/`, `PROGRESS.md`) | Exact fact-level check; sealed held-out set with baselines; no word lists in agent code; learned parser beats the teacher on unsure lines (finding 5); correct facts recalled exactly | No frozen target met. H3: new task 25%, new domain 21%, new situation 8%. Adaptation (X4): 19% / 19% / 1.4% change recall. Distillation did not raise coverage (finding 8). Naming drift between teacher and ground truth (finding 4) | Generalisation needs relations about things plus general inference; naming must be managed as knowledge; thresholds must come from data |

Four 3.0 facts shape this design directly:

1. **X4: most variants need knowledge that is in no other task** (`DESIGN_3_experiments.md`, section X4). Examples are microwaving a potato first, or what a newspaper can do to a window. 4.0 teaches exactly that kind of knowledge, as relations about items, and derives the change.
2. **Rules almost never fired, because situations rarely repeat** (X4; the rules arm in `crates/agent3/src/adapt.rs:252-297` induces only situation-to-change patterns from stored cases). 4.0 induces rules over item relations, which repeat across tasks and domains ("plastic things", "things that can beat"), not over whole situations.
3. **Naming was the largest single source of failure** (phase B finding 4: the second encounter went from 16 to 29 of 71 from a naming fix in the measure alone). 4.0 makes naming part of the conversation: the agent asks `same_as?`, and the answer is taught knowledge.
4. **Targets set before any build were unpassable** (M24, M25 in `PROGRESS.md`). 4.0 sets phase B thresholds only after the spike.

**The reasoner is already most of what 4.0 needs.** `crates/reasoner/src/lib.rs` provides:

| Capability | Where | Limit that matters for 4.0 |
|---|---|---|
| Semi-naive bottom-up evaluation, stratum by stratum | `lib.rs:866-914` | Bottom-up only: no goal-directed (top-down) mode |
| Recursion (transitive closure) | `tests/suite.rs:72` (`is_a` transitive); `lib.rs:897-911` | No depth limit |
| Stratified negation as failure (`not p(X)`) | `lib.rs:5`, `lib.rs:657-681`, `stratify` at `lib.rs:783-856` | Negation through recursion is rejected (`lib.rs:23`, test `unstratifiable_rejected`, `tests/suite.rs:242`) |
| Strong negation as a predicate named `-p` | `lib.rs:5-6`; `crates/core/src/term.rs:3`, `term.rs:358` | Just a separate predicate; consistency is not enforced |
| Contradiction listing (`p` and `-p` both true) | `lib.rs:602-621` | Lists only; no resolution |
| Default inheritance with an exception (`has(X,Y) :- is_a(X,C), has(C,Y), not -has(X,Y)`) | `tests/suite.rs:130`, `tests/suite.rs:185-189` | Works only when the negative fact is given, not derived |
| Proof trees down to given facts, with checked-absent atoms shown | `lib.rs:581-600`, `lib.rs:431-447` | Keeps the **first** derivation only (`lib.rs:745-753`, `lib.rs:935-939`) |
| Confidence: the minimum of the rule's and the premises' | `lib.rs:10`, `lib.rs:938`; test `tests/suite.rs:258` | Confidence of absent atoms is not represented |
| Arithmetic, comparisons, aggregates | `lib.rs:683-741` | Arithmetic in recursion is rejected (`lib.rs:841-848`), so depth counters cannot be written as rules |
| Quarantine: `core/` rules always kept, learned rules added one at a time and set aside if they break safety or stratification | `lib.rs:959-971` | Exactly the guard rule learning needs |
| Dependency analysis (`depends_on`, `naf_sensitive`) | `lib.rs:975-1036` | Usable to prune rules irrelevant to a question |
| Indexing on the first argument only | `lib.rs:306-336` | Argument order matters for speed |

3.0's agent did not use the reasoner at all. `crates/agent3/Cargo.toml` has no `sla_reasoner` dependency; adaptation was hand-written Rust over the vault. 4.0 puts the reasoner back at the centre.

The 2.0 learner (`crates/learner`) already has two pieces 4.0 can reuse:
- support and counterexample counting, with confidence `e / (e + c + k)` (`crates/learner/src/lib.rs:73-75`), where a guessed rule may not support itself (`lib.rs:77-79`);
- correction by adding a condition or an exception (`crates/learner/src/corrections.rs:1-10`, `corrections.rs:60`).

## 3. Principles

1. **Generalisation and learning first (P1).** When a choice trades polish, speed or coverage against the agent's ability to derive new knowledge, derivation wins.
2. **Everything is taught in conversation (P2).** No fact enters the knowledge base except as a message in a dialogue, processed by the agent's own conversation path. That path does five things:
   - schema check;
   - consistency check;
   - provenance;
   - derivation;
   - gap questions.

   There is no injection, no imported resource and no hand-written knowledge base. The only exceptions are the base rules and the schema, which are the system's logic (P6), and the spike's fixture knowledge base (section 9.2), which is a test fixture, delivered through the same conversation path and never used in phase B.
3. **Built-in logic is universal and small (P6).** Base rules mention relation names from the schema and nothing else: no item, material, function or domain symbol. A test enforces this (section 4.3).
4. **Every claim has a reason (P5).** A derived claim carries a proof tree down to taught messages. An induced claim carries its rule, the rule's support and its confidence. A claim with neither is not made: the agent writes a `gap` line and asks.
5. **Taught beats inherited beats induced.** A fact taught about a thing overrides one inherited from a more general kind, and both override a conclusion of an induced rule. This is the exception mechanism (section 5.4).
6. **Statistical parts propose, symbolic parts decide** (3.0 principle 3, kept). Similarity proposes `same_as?` questions, and induction proposes rules. The validator, the teacher's answers and the reasoner decide.
7. **Every model call is reused** (3.0 principle 5, kept). Calls are cached and archived and never deleted (memory: keep training data). The ledger enforces the 5,000-call hard limit (P14).
8. **Measure, then set thresholds (P16).** No phase B threshold is written before the spike's numbers exist.
9. **Sealed measurement** (3.0 principle 6, kept). The held-out benchmark is read a fixed number of times, as counts only.
10. **Household domains only (P13).** The long-term vision (formal-logic counterpart to LLM training) is context, not scope.

## 4. Architecture

```
 teacher (Haiku, formal messages)                      user / benchmark question
        │  batch of statements                                   │
        ▼                                                        ▼
 ┌──────────────── conversation path (agent4) ─────────────────────────────┐
 │ 1 parse message (formal grammar, kb4)                                  │
 │ 2 schema check: predicate, arity, argument sorts            ─reject──► │ reply "reject: ..."
 │ 3 naming check: new symbol close to a known one?            ─ask────► │ "ask: same_as(a, b)?"
 │ 4 consistency check: contradicts taught / inherited / induced?─ask──► │ "conflict: ... exception?"
 │ 5 store: raw layer (always) → facts layer → provenance layer           │
 │ 6 derive: incremental closure on the reasoner; show new consequences   │ "ok. derived: ..."
 │ 7 learn: induction pass; new hypotheses, specialisations, retractions  │ "hypothesis: ..."
 │ 8 gaps: near-miss derivations → questions (≤ question budget)          │ "ask: can_go_in(x, oven)?"
 └────────────────────────────────────────────────────────────────────────┘
        │                                         │
        ▼                                         ▼
  kb4 store: raw/  facts/  prov/        reason4: base rules (templates) + learned layers
        ▲                                         on sla_reasoner; proof checker
        │ replay (zero calls)                     │
        └──────── any agent version ──────────────┘
                                                  ▼
                                   answer = formal lines + proof / support / gap
                                                  │
                                   eval4: benchmark scorer, proof checker, leak check,
                                          ablations, learning curve (eval only)
```

### 4.1 Crates

The 2.0 and 3.0 crates stay unchanged, so their results can be replayed for comparison (Q8). 4.0 adds:

| Crate | Content |
|---|---|
| `kb4` | Schema v4 (relations, sorts, which relations inherit or are transitive); the formal message grammar (teacher and agent message kinds); the three layers with readers, writers and schema versions; vocabulary and naming store; deny list format |
| `reason4` | Base rule templates and their expansion into `sla_reasoner` programs; learned rule layers; the stratification order of taught, inherited and induced; question pruning with `depends_on` (`crates/reasoner/src/lib.rs:1010`); an independent **proof checker**; proof and support rendering |
| `learn4` | Induction with support and counterexample counting (extending `crates/learner`); specialisation; retraction; the layer record (which rule was built from which) |
| `agent4` | The conversation path (section 4 diagram); questions under a budget; answers |
| `teach4` | The Haiku teacher channel: prompts per topic, batching, the scripted seed teacher, cache and ledger in `data/training/calls4/` (reusing `teach3`'s cache key and fail-fast code, `crates/teach3/src/lib.rs:18-25`, `lib.rs:102-154`) |
| `eval4` | Benchmark loader, scorer, leak checker, replay runner, ablations, learning-curve runner, statistics (reusing `crates/eval3/src/stats.rs:4` Wilson and `stats.rs:18` McNemar), lab events (reusing `crates/eval3/src/lab.rs`) |

Offline Python (in `tools/design4/`) only checks gold proofs and builds numbers for the paper. Only Rust runs at agent runtime.

### 4.2 Why bottom-up plus pruning, not a new engine

The reasoner computes the whole model bottom-up. For the spike's 300-500 facts this is trivial: 2.0 measured a 95th percentile of 69 ms on a 2,171-fact vault (`PROGRESS.md`, milestone 13). For larger knowledge bases, three mechanisms bound the cost. They answer the "inference explodes" risk, and the spike measures each:

1. **Typed relations.** Every argument has a sort (section 5.1). A rule fires only on facts of matching sorts, and the schema check rejects ill-sorted facts at entry.
2. **Question pruning.** For a question, `reason4` keeps only the rules whose heads the question's predicates depend on (`depends_on`, `crates/reasoner/src/lib.rs:1010-1036`). It also keeps only the facts about symbols reachable from the question's constants within the hop limit, plus their `is_a` ancestors. This is derivation on demand without a top-down engine.
3. **A hop limit on answers.** Arithmetic cannot run inside recursion (`lib.rs:841-848`), so depth cannot be counted by a rule. The limit is applied to proofs instead: an answer whose minimal proof has more hops than the limit (default 4) is not claimed. It is written as a gap with the reason `beyond_hop_limit`.

   Semi-naive evaluation finds a derivation in the earliest round it can (`lib.rs:887-911`). So the first derivation kept is short in rounds, though not guaranteed minimal in pieces. The spike reports how often the kept proof differs from the gold proof's length.

If the spike shows any of these is insufficient, magic-set rewriting of the question is the next step (section 13).

### 4.3 Tests that hold the principles

- **No domain symbols in base rules or agent code.** This extends `crates/kb3/tests/no_word_lists.rs` to the 4.0 crates. Every constant in a base-rule template must be a schema relation name, a sort name, a message keyword or `user`.
- **No injection.** The facts layer can be written only by the conversation path. A test builds a knowledge base by every public API and checks that each live fact has a provenance record pointing to a raw message.
- **Proof checker negative controls.** Five proofs must each fail the checker:
  - a proof with a leaf that is not a live fact;
  - a rule instance that does not unify;
  - an "absent" atom that is derivable;
  - a retracted leaf;
  - a proof over a rule in quarantine.
- **Leak check** (section 7.4) runs as a test over every raw-layer file.
- **Replay is exact.** Replaying the raw layer under the same agent version gives byte-identical facts and provenance layers.

## 5. Knowledge model (Q1, Q2, Q3)

### 5.1 Relations (Q1)

**Recommendation (D4.1, to decide).** Start with the nine relations of pre-planning Q1, plus two: `can_hold` and `can_go_in`. The pre-planning's own worked examples (`can_hold(baking_tin, batter)`, `not can_go_in(plastic_tub, oven)`) need these two. Writing them as `can_do(baking_tin, hold_batter)` would hide the second argument from chaining: `is_a(batter, liquid)` could not then feed `can_hold`. Add a further relation only when the spike shows a needed derivation cannot be expressed.

**Taught relations about things (11):**

| Relation | Sorts | Inherits down `is_a` | Transitive | Meaning |
|---|---|---|---|---|
| `is_a(X, K)` | thing, kind | — | yes | X is a kind of K |
| `part_of(P, X)` | thing, thing | in the second argument | yes | P is part of X |
| `can_do(X, F)` | thing, function | yes | — | X can perform function F |
| `used_for(X, P)` | thing, function | yes | — | X is conventionally used for P |
| `has_property(X, P)` | thing, property | yes | — | X has property P |
| `made_of(X, M)` | thing, material | yes | — | X is made of material M |
| `can_hold(X, Y)` | thing, thing | yes | — | X can contain or carry Y |
| `can_go_in(X, Y)` | thing, thing | yes | — | X can be put in or on Y (oven, dishwasher, fridge, microwave) |
| `achieves(A, St)` | action or step, state | — | — | the action or step brings about state St |
| `requires(A, Z)` | action or step, state or thing | — | — | a precondition (replaces v3 `requires_state`) |
| `same_as(X, Y)` | any, same sort | — | equivalence | two names for one thing |

Every taught relation except `same_as` may also be taught negatively, as `not R(x, y).`. It is stored as the strong negation `-R(x, y)` (`crates/core/src/term.rs:3`). A negative `same_as` is how the teacher answers "no, these are different" to a naming question.

**Procedures, situations and differences** stay as in schema v3 (`DESIGN_3_schema.md` sections 2-3.6, `crates/kb3/src/fact.rs:7-17`):
- `task`, `step`, `goal`, `domain`, `needs`;
- `action`, `object_of`, `uses`, `role`;
- `into`, `onto`, `from`;
- `before`, `overlaps`, `optional`, `until`;
- `setting`, `takes_time`, `purpose`, `achieves`, `warning`, `avoid`, `check`.

The situation facts (`lacks`, `has`, `wants`, `excludes`, `time_limit`, `count`) and the difference facts (`swap`, `drop`, `add_step`, `change`, `extra_need`, `no_need`, `cannot`) also stay. `aliases` are not used on the core path, because there is no English.

**Sorts** are inferred from first use and then fixed. For example, `can_do(fork, beat)` makes `fork` a thing and `beat` a function. A later `is_a(beat, tool)` is a sort conflict: the agent rejects the message and says why. Step ids are declared by `step(T, S)`, so the action-or-step slots of `achieves` and `requires` are told apart by declaration, not by spelling.

**Derived only (never taught; written by the agent):**
- `kind/2`, `holds/3`, `holds_not/3`, `overridden/4`;
- `stand_in/3`, `replace_step/2`;
- `conflict/3`, `ask/1`, `gap/2`.

### 5.2 Base rules (Q3)

**Recommendation (D4.3, to decide).** Eight base rule groups, written once as templates. They mention no domain symbol. Two schema declarations parameterise them, and those are part of the schema, not knowledge:
- `inherits(R)` for `can_do`, `used_for`, `has_property`, `made_of`, `can_hold`, `can_go_in`;
- `transitive(R)` for `is_a`, `part_of`.

The rules are evaluated on `sla_reasoner` with ids under `core/` so that quarantine always keeps them (`crates/reasoner/src/lib.rs:959-961`).

The generic rules work on a reified view `holds(X, R, Y)`, with X first so the reasoner's first-argument index stays selective (`lib.rs:306-336`). Only the bridge rules B0 need the relation name in predicate position. They are generated per relation from the schema, so the program is ordinary Datalog with no higher-order features.

```datalog
% ---- B0 bridge (generated per taught relation R from the schema; not counted as a hop)
taught(X, can_do, Y)     :- can_do(X, Y).            % ... one line per relation
taught_not(X, can_do, Y) :- -can_do(X, Y).           % ... one line per relation
entity(X) :- taught(X, _, _).
entity(X) :- taught_not(X, _, _).
entity(X) :- uses(_, X).
entity(X) :- has(user, X).

% ---- B1 kinds: is_a is transitive
kind(X, K) :- is_a(X, K).
kind(X, K) :- is_a(X, M), kind(M, K).
self_or_kind(X, X) :- entity(X).
self_or_kind(X, K) :- kind(X, K).

% ---- B2 inheritance with specificity (exceptions; see 5.4)
decided(X, R, Y, K) :- inherits(R), self_or_kind(X, K), taught(K, R, Y).
decided(X, R, Y, K) :- inherits(R), self_or_kind(X, K), taught_not(K, R, Y).
% a decision at K is overridden for X if a strictly more specific K2 between X and K also decides
overridden(X, R, Y, K) :- decided(X, R, Y, K), decided(X, R, Y, K2), K2 != K, self_or_kind(K2, K).
holds(X, R, Y)     :- taught(X, R, Y).
holds_not(X, R, Y) :- taught_not(X, R, Y).
holds(X, R, Y)     :- inherits(R), kind(X, K), taught(K, R, Y),     not overridden(X, R, Y, K).
holds_not(X, R, Y) :- inherits(R), kind(X, K), taught_not(K, R, Y), not overridden(X, R, Y, K).

% ---- B3 parts: part_of is transitive, and a kind's parts are its members' parts
part(P, X) :- part_of(P, X).
part(P, X) :- part_of(P, Q), part(Q, X).
part(P, X) :- kind(X, K), part(P, K).
holds(P, part_of, X) :- part(P, X).

% ---- B4 same_as: canonical names (see note); the rules below are the reference semantics
eq(X, Y) :- same_as(X, Y).
eq(Y, X) :- same_as(X, Y).
eq(X, Z) :- eq(X, Y), eq(Y, Z).

% ---- B5 function from procedures: an item that served function F in a taught step can do F
holds(I, can_do, F) :- role(S, I, F), not -can_do(I, F).

% ---- B6 stand-in: J may replace I in step S if J can do I's function there
%      and meets every relation I has to the step's object and other items
co(S, Y) :- object_of(S, Y).
co(S, Y) :- uses(S, Y).
req(S, I, R, Y) :- uses(S, I), co(S, Y), Y != I, holds(I, R, Y).
cand(S, I, J) :- lacks(user, I), uses(S, I), has(user, J), J != I.
missing(S, I, J, R, Y) :- cand(S, I, J), req(S, I, R, Y), not holds(J, R, Y).
missing_any(S, I, J) :- missing(S, I, J, _, _).
blocked(S, I, J) :- cand(S, I, J), req(S, I, R, Y), holds_not(J, R, Y).
stand_in(S, I, J) :- cand(S, I, J), role(S, I, F), holds(J, can_do, F), not missing_any(S, I, J).

% ---- B7 step replacement: another action that achieves the state a later step requires
enables(S, St) :- achieves(S, St), before(S, S2), requires(S2, St).
blocked_step(S) :- uses(S, I), lacks(user, I), not has_stand_in(S, I).
has_stand_in(S, I) :- stand_in(S, I, _).
action_blocked(A) :- requires(A, I), lacks(user, I).
replace_step(S, A) :- blocked_step(S), enables(S, St), achieves(A, St), action(S, A0), A != A0,
                      not action_blocked(A).

% ---- B8 contradiction: raise a question, never answer from it
conflict(X, R, Y) :- holds(X, R, Y), holds_not(X, R, Y).
conflict(X, is_a, X) :- kind(X, X).
```

Notes on the rules:

- **Stratification.** `overridden` depends only on taught facts and `kind`, never on `holds`. So the negation in B2 is stratified. The 3.0-era test shows the reasoner rejects anything else (`tests/suite.rs:242`). The obvious alternative, `holds_not` inherited with `not holds` and vice versa, is a cycle through negation and would be rejected. That is why B2 compares decision points instead.
- **B4 is applied as canonical naming, not as rules (recommendation).** Substitution rules for `same_as` add two clauses per relation per argument position and multiply the model. When `same_as(egg_whisk, whisk)` is taught, every fact is stored under one canonical symbol (the earlier-taught one). Provenance keeps the original name, and the proof shows a `same_as` step. The rules above define what canonicalisation must agree with. The spike runs both forms on its fixture and checks that they give the same answers.
- **B5 links procedures to item knowledge.** It is the only bridge from v3 procedures (role facts, 419 of them in `data/curriculum3`, per the `DESIGN_3_experiments.md` ground-truth row) into the relation knowledge. It is a rule about the meaning of `role`, not domain knowledge.
- **B6 requirements are generic.** A stand-in must keep every relation the lacked item has to the step's object and co-items, for example `can_hold(loaf_tin, batter)` and `can_go_in(loaf_tin, oven)`. Nothing about ovens or tins is written in the rule. A requirement that is unknown for J (`missing`, but not `blocked`) is where the agent's gap questions come from (section 6.6).
- **B7** needs taught `achieves` and `requires` for actions as well as steps. Whether this is taught often enough to pay is a spike question (rule ablation).
- **B8** turns contradictions into questions. A conflicting fact never supports an answer: answer extraction excludes any `holds` or `holds_not` whose arguments are in `conflict`.
- **No material rule.** "A thing has the properties of its material" is deliberately **not** a base rule. It is the first thing induction should find (section 6.3). Whether it does is one of the spike's tests.

**Hops.** The hop count of a proof is the number of applications of B1-B3 and B5-B7 (each B1 recursion step counts) and of learned rules along its deepest branch. B0, the helper clauses (`entity`, `self_or_kind`, `co`, `req`, `cand`, `missing*`) and B8 are not counted. The benchmark's hop labels are computed from gold proofs by the same code, so labels and measurement cannot disagree.

### 5.3 Confidence

- **Taught facts.**
  - A fact from the Haiku teacher starts at 0.9.
  - It rises to 0.97 when a second, independent message confirms it (another session, or the answer to the agent's own question).
  - A fact from the scripted seed teacher (v3 ground truth, section 8.3) has 1.0.
  - **Recommendation (D4.3a, to decide):** these values, to be checked on the spike's fixture. The spike cannot measure teacher accuracy, because it makes no calls.
- **Derived facts** take the minimum of their premises and rule (`crates/reasoner/src/lib.rs:938`).
- **Induced rules** have confidence `s / (s + c + 1)`, from support s and counterexamples c, as in `crates/learner/src/lib.rs:73-75` with k = 1.
- **Answers** show their confidence. A conclusion that uses an induced rule is printed as `hypothesis`, never as a plain claim.

### 5.4 Exceptions (Q2)

**Recommendation (D4.2, to decide): default inheritance with specificity, as in B2, with this priority order.**

1. **Taught about the thing itself** (positive or `not`). It always wins.
2. **Inherited from the most specific kind that decides.** A more general kind's contrary decision is overridden (`overridden/4`).
3. **Two decisions at incomparable kinds** (the "Nixon diamond": `is_a(x, a)`, `is_a(x, b)`, a says yes, b says no). Both `holds` and `holds_not` come out, and B8 raises a question. Nothing is claimed until it is answered.
4. **Induced conclusions** (section 6.3) apply only where 1-3 decide nothing. Induced rules are placed in strata above B2, with a guard `not holds_not(X, R, Y)` (or `not holds(...)` for a negative head). So a taught or inherited fact always beats an induced one. This is ordinary stratified negation, and `quarantine` (`lib.rs:959`) rejects any induced rule that would break it.

**Non-monotonic withdrawal.** When a more specific contrary fact arrives, the earlier conclusion simply stops being derived on the next evaluation. The agent compares the model before and after each message. A conclusion it had shown and that is now gone is reported as `withdrawn: ...` with the fact that caused it. Withdrawn conclusions are recorded in provenance as events, not deleted.

**When an exception is taught.** A negative fact that contradicts an inherited positive one is a conflict at entry (section 6.4). The agent asks whether it is an exception or a correction of the general fact:
- an exception keeps both facts;
- a correction retracts the general one, recorded as `retracted_by`, as in `crates/kb3/src/vault.rs:61-71`.

**Alternatives, to weigh in the design session.**
- **Answer set programming** (stable models with default negation) handles cycles through negation and gives several consistent worlds. It needs an outside solver (clingo, not in Rust), loses the existing proof trees (`lib.rs:581`), and makes answers depend on choosing among models. It is not recommended unless the spike shows conflicts that stratified specificity cannot represent.
- **Explicit `except(x, R, y)` facts.** These are equivalent to a negative taught fact here, but add a second way of saying the same thing. Not recommended.

The spike counts which conflicts occur in the fixture: specific-over-general, diamond, induced-against-taught, sort conflicts. That count settles D4.2.

## 6. Learning and the teaching conversation (Q4, protocol)

### 6.1 The message language

Messages are lines in the formal language, parsed by `kb4`. 3.0's fact parser accepts only lower-case predicate names (`crates/kb3/src/fact.rs:46-49`). `kb4` extends it with the `not` prefix and the message keywords below. All keywords are formal vocabulary, listed in the extended word-list test.

**Teacher to agent:**

| Form | Meaning |
|---|---|
| `topic <domain>/<name>.` | Starts a teaching batch (session header) |
| `R(a, b).` | Teaches a fact |
| `not R(a, b).` | Teaches a negative fact |
| `task ... .` block | Teaches a procedure in v3 facts (step ids `<task>_s<N>`) |
| `yes.` / `no.` / `exception.` / `correct R(a, b).` | Answers the agent's last question about a conflict, a name or a hypothesis |
| `unknown.` | The teacher does not know; the agent records the gap as asked |
| `retract R(a, b).` | Withdraws an earlier teaching |

**User or benchmark to agent:**

| Form | Meaning |
|---|---|
| `query: <conjunction>.` | A relation question (`query: can_hold(muffin_tray, batter).`, `query: can_do(X, beat), has(user, X).`) |
| `request(<task>). <situation facts>. given: <facts>.` | A help request. `given:` facts are the question's own context (a procedure for an untaught task). They are used for this answer only and never stored |

**Agent to teacher or user:**

| Form | Meaning |
|---|---|
| `ok.` | Accepted, with no new consequence worth showing |
| `ok. derived: <fact> [<premises>]` | Accepted. Shows up to N new consequences (N = 3 by default), each with its premises |
| `reject: <reason>.` | Schema or sort violation; nothing is stored except the raw message |
| `ask: same_as(a, b)?` | Naming check |
| `conflict: <fact> vs <fact> [<proof>]. exception?` | Consistency check |
| `hypothesis: <rule> support [..] counter [] conf c.` | A newly induced rule |
| `withdrawn: <fact> because <fact>.` | Non-monotonic change |
| `ask: <pattern>?` | A gap question (active learning) |
| `answer:` block | Answer lines, then `proof:` trees, `support:` lists, `gap:` lines |

### 6.2 What the agent does with each teaching message

Each message goes through these steps, in order. All of them are skipped by direct injection, which is why injection is not allowed (P2).

1. **Parse and schema check.**
   - The predicate must exist, with the right arity.
   - Argument sorts must be consistent with earlier use.
   - Symbols must be lower-case.
   - Failures give `reject:` and are kept in raw only.
2. **Naming check (risk: naming drift).** A new symbol is compared with known symbols of the same sort in two ways:
   - by head word, the same 4-letter-stem rule as 3.0's check (`DESIGN_3_schema.md` section 6), which is a generic mechanism, not a word list;
   - by relational context: a shared `is_a` parent and at least one shared relation fact.

   If either is close, the agent asks `same_as(new, known)?` before using the new symbol in derivations. The answer is taught knowledge:
   - `yes` → `same_as`;
   - `no` → `not same_as`;
   - or `is_a(new, known)`.

   The fact is held as pending (stored, but excluded from derivation) until answered.
3. **Consistency check.** The candidate fact is checked against the current model:
   - its negation is taught → conflict, ask which is right;
   - its negation is inherited → conflict, ask "exception?";
   - its negation comes from an induced rule → no question; the induced conclusion is withdrawn and the rule meets a counterexample (section 6.4).
4. **Store.**
   - Raw: always, before anything else.
   - Facts: if accepted.
   - Provenance: teacher, session, turn, cache key of the call, confidence, status.
5. **Derive.** Incremental re-evaluation (the question-free full model in phase A; pruned by topic in phase B if the spike shows it is needed). New consequences are shown with premises. Withdrawn consequences are shown as `withdrawn:`.
6. **Learn.** An induction pass runs at the end of each batch, not after each message (cost). It may print `hypothesis:`, `specialised:` or `retracted:` lines.
7. **Gaps.** Near-miss derivations become questions, ranked and limited by the question budget (section 8.2).

### 6.3 Rule learning (Q4)

**Recommendation (D4.4, to decide): induction with a support threshold, plus specialisation on counterexamples. Composition is added only if the spike shows that answer speed needs it.**

**Induction (lifted patterns over relations).** It is built in Rust, extending `crates/learner` (the 2.0 feature-pattern learner, `lib.rs:1-10`). Popper stays an optional offline comparison, as in 3.0.

1. **Targets.** For each taught relation R and each constant c in its second argument, the positive targets are `{x : holds(x, R, c)}` and the negative targets are `{x : holds_not(x, R, c)}`. For example, `holds_not(_, can_go_in, oven)`.
2. **Features of x.** Every `holds(x, R', y)` and `holds_not(x, R', y)` in the current model, with x replaced by a variable. Because B2 has run, inherited features are included. A rule may therefore use a kind's properties.
3. **Candidate bodies.** Bodies of one feature, then conjunctions of two. A body of three features is not tried in 4.0, to bound the search.
4. **Counts for body B and head H.**
   - **Support s** is the number of distinct x with B and H that are taught or inherited, not induced.
   - **Counterexamples c** is the number of x with B and the contrary of H taught or inherited.
   - **New predictions u** is the number of x with B and H unknown.
5. **Acceptance.** A candidate becomes a hypothesis rule when all of these hold:
   - s ≥ k (recommended k = 3; the spike tests 2, 3 and 4);
   - the supporting x come from at least 2 topics or domains (from provenance);
   - c = 0;
   - u ≥ 1 (otherwise the rule adds nothing);
   - no shorter accepted body exists with the same head and an equal or larger support set (minimality).
6. **Rule form.** The rule is written with the guard that keeps priority:

   `holds_not(X, can_go_in, oven) :- holds(X, made_of, plastic), not holds(X, can_go_in, oven).`

   It gets an id `ind/L<n>/<k>` and confidence `s/(s+c+1)`. It passes through `quarantine` (`crates/reasoner/src/lib.rs:959`).
7. **No self-support.** The evidence model for inducing layer n uses base rules and layers below n only, never the rule's own conclusions. This is the 2.0 rule in `crates/learner/src/lib.rs:77-79`.

**Layer on layer (P7).** The layer of a rule is 1 + the highest layer of any rule used to derive a fact in its body features or support. Base rules are layer 0. An induced rule whose body uses a layer-1 conclusion is layer 2. Each layer is a stratum above the ones it reads, so the program stays stratified. The provenance layer records `built_from` for every rule: the base and learned rules whose conclusions appear in its support. The report shows the rule layers as a graph.

**Composition (only if needed).** When two rules are chained in more than M answers (M set after the spike), a shortcut rule is added with its `built_from` pointing to both. It adds no new conclusions, only speed. It is never counted as learned knowledge in the results.

**Specialisation and retraction on a counterexample.** This follows the 2.0 correction logic (`crates/learner/src/corrections.rs:1-10`):

1. A counterexample is taught: a taught or inherited fact contrary to a hypothesis's conclusion for some x′. Taught beats induced, so the conclusion for x′ is withdrawn at once.
2. The rule now has c = 1. The learner looks for a feature that separates x′ from the support set:
   - a feature all supporting x share but x′ lacks → add it as a condition;
   - a feature only x′ has → add `not` that feature.
3. If a specialised rule still meets the acceptance test, it replaces the old one. The old rule is marked `specialised_by` and its conclusions are recomputed.
4. If no separating feature exists, the rule is **retracted** (status `retracted`, kept in provenance). The agent asks the teacher for the difference (`ask: what differs x′ from {x1, x2, x3}?`). A feature the teacher then teaches can bring the specialised rule back on the next induction pass.

**What is measured** (section 7): the number of hypotheses per layer, their precision on benchmark questions, retractions, and the benchmark with learned rules on and off.

### 6.4 Worked examples of the protocol

The examples use a small fixture in the spike's form. Message ids are `<session>.<turn>`. Proofs use the reasoner's rendering (`crates/reasoner/src/lib.rs:431-447`), extended with the raw message id at each leaf.

**E1. A taught fact and what follows (1 hop, derived).**

```
TEACHER: topic baking/tins.
TEACHER: is_a(loaf_tin, baking_tin).
AGENT:   ok.
TEACHER: can_hold(baking_tin, batter).
AGENT:   ok. derived: can_hold(loaf_tin, batter)   [is_a(loaf_tin, baking_tin), can_hold(baking_tin, batter)]
TEACHER: can_go_in(baking_tin, oven).
AGENT:   ok. derived: can_go_in(loaf_tin, oven)   [is_a(loaf_tin, baking_tin), can_go_in(baking_tin, oven)]
TEACHER: is_a(muffin_tray, baking_tin).
AGENT:   ok. derived: can_hold(muffin_tray, batter)   [is_a(muffin_tray, baking_tin), can_hold(baking_tin, batter)]
         derived: can_go_in(muffin_tray, oven)   [is_a(muffin_tray, baking_tin), can_go_in(baking_tin, oven)]
```

**E2. Naming drift.**

```
TEACHER: can_do(egg_whisk, beat).
AGENT:   ask: same_as(egg_whisk, whisk)?        % head word "whisk", both things that can_do(beat)
TEACHER: yes.
AGENT:   ok. same_as(egg_whisk, whisk) stored; egg_whisk is written whisk from now on.
TEACHER: is_a(balloon_whisk, whisk).
AGENT:   ok.                                     % taught as is_a, no question: the relation names it
TEACHER: can_do(whisk_attachment, beat).
AGENT:   ask: same_as(whisk_attachment, whisk)?
TEACHER: no. part_of(whisk_attachment, stand_mixer).
AGENT:   ok. not same_as(whisk_attachment, whisk) stored.
```

**E3. A conflict and an exception (specificity).**

```
TEACHER: topic cooking/dishes.
TEACHER: can_go_in(dish, oven).
TEACHER: is_a(glass_dish, dish).
TEACHER: is_a(plastic_dish, dish).
AGENT:   ok. derived: can_go_in(glass_dish, oven) [...], can_go_in(plastic_dish, oven) [...]
TEACHER: not can_go_in(plastic_dish, oven).
AGENT:   conflict: not can_go_in(plastic_dish, oven) vs can_go_in(plastic_dish, oven)
           [is_a(plastic_dish, dish) s0007.3, can_go_in(dish, oven) s0007.1]. exception?
TEACHER: exception.
AGENT:   ok. exception stored.
         withdrawn: can_go_in(plastic_dish, oven) because not can_go_in(plastic_dish, oven).
TEACHER: is_a(takeaway_box, plastic_dish).
AGENT:   ok. derived: not can_go_in(takeaway_box, oven)
           [is_a(takeaway_box, plastic_dish), not can_go_in(plastic_dish, oven)]
```

The last line is the specificity rule at work. `takeaway_box` has two decisions, at `plastic_dish` (no) and at `dish` (yes). `plastic_dish` is more specific, so `overridden(takeaway_box, can_go_in, oven, dish)` holds, and only `holds_not` is derived.

**E4. A diamond: no answer until the teacher decides.**

```
TEACHER: is_a(silicone_mould, baking_tin).
TEACHER: is_a(silicone_mould, rubber_thing).
TEACHER: not can_go_in(rubber_thing, oven).
AGENT:   conflict: can_go_in(silicone_mould, oven) [baking_tin] vs not can_go_in(silicone_mould, oven)
           [rubber_thing]; neither kind is more specific. which holds for silicone_mould?
TEACHER: can_go_in(silicone_mould, oven).
AGENT:   ok. derived: (none new; the conflict is resolved by the taught fact about silicone_mould)
```

**E5. A request answered by derivation (2 hops), with a gap question first.**

The procedure `bake_banana_bread` was taught earlier as v3 facts. Step 5 is:
- `action(bake_banana_bread_s5, bake)`;
- `object_of(…_s5, batter)`;
- `uses(…_s5, loaf_tin)`;
- `role(…_s5, loaf_tin, hold)`;
- `uses(…_s5, oven)`.

Then:

```
USER:    request(bake_banana_bread). lacks(user, loaf_tin). has(user, muffin_tray).
AGENT:   answer:
         task bake_banana_bread
         swap 5 loaf_tin muffin_tray
         proof:
         stand_in(bake_banana_bread_s5, loaf_tin, muffin_tray) [conf 0.9] by rule core/B6.stand_in
           cand(bake_banana_bread_s5, loaf_tin, muffin_tray) by rule core/B6.cand
             lacks(user, loaf_tin) given by request
             uses(bake_banana_bread_s5, loaf_tin) [conf 1] given by s0003.12 (seed teacher)
             has(user, muffin_tray) given by request
           role(bake_banana_bread_s5, loaf_tin, hold) [conf 1] given by s0003.14 (seed teacher)
           holds(muffin_tray, can_do, hold) [conf 0.9] by rule core/B2.inherit
             is_a(muffin_tray, baking_tin) [conf 0.9] given by s0042.7
             can_do(baking_tin, hold) [conf 0.9] given by s0041.3
             not overridden(muffin_tray, can_do, hold, baking_tin) (no such fact known)
           not missing_any(bake_banana_bread_s5, loaf_tin, muffin_tray) (no such fact known)
         requirements kept: can_hold(muffin_tray, batter) [B2: s0042.7, s0041.2]
                            can_go_in(muffin_tray, oven) [B2: s0042.7, s0041.4]
```

Had `can_go_in(baking_tin, oven)` never been taught, the requirement `can_go_in(loaf_tin, oven)` would still come from somewhere: here, `is_a(loaf_tin, oven_tin)` with `can_go_in(oven_tin, oven)`. Then the muffin tray would be `missing` one requirement without being `blocked`, and the agent asks before answering:

```
AGENT:   gap: requirement can_go_in(muffin_tray, oven) unknown for swap 5 loaf_tin muffin_tray
         ask: can_go_in(muffin_tray, oven)?
TEACHER: can_go_in(muffin_tray, oven).
AGENT:   ok. answer: swap 5 loaf_tin muffin_tray  (proof as above, requirement now taught: s0060.2)
```

The answer does not claim a new baking time. No taught fact links baking time to the tin. If the benchmark's gold says the time changes, that is a correct gap, not a miss (section 7.3).

**E6. Cross-domain, 3 hops.**

The task `descale_kettle` (cleaning) was taught with step 2:
- `uses(descale_kettle_s2, vinegar)`;
- `role(descale_kettle_s2, vinegar, dissolve_limescale)`.

Separately:

```
TEACHER: topic cleaning/limescale.     can_do(acid, dissolve_limescale).                 (s0090.4)
TEACHER: topic cooking/citrus.         is_a(lemon_juice, citrus_juice).                  (s0121.2)
                                       is_a(citrus_juice, acid).                         (s0121.5)
USER:    request(descale_kettle). lacks(user, vinegar). has(user, lemon_juice).
AGENT:   answer:
         task descale_kettle
         swap 2 vinegar lemon_juice
         proof:
         stand_in(descale_kettle_s2, vinegar, lemon_juice) [conf 0.9] by rule core/B6.stand_in
           cand(...) by rule core/B6.cand  [lacks, uses s0011.6, has]
           role(descale_kettle_s2, vinegar, dissolve_limescale) given by s0011.7 (seed teacher)
           holds(lemon_juice, can_do, dissolve_limescale) [conf 0.9] by rule core/B2.inherit
             kind(lemon_juice, acid) by rule core/B1.trans
               is_a(lemon_juice, citrus_juice) given by s0121.2
               kind(citrus_juice, acid) by rule core/B1.base
                 is_a(citrus_juice, acid) given by s0121.5
             can_do(acid, dissolve_limescale) given by s0090.4
             not overridden(lemon_juice, can_do, dissolve_limescale, acid)
           not missing_any(descale_kettle_s2, vinegar, lemon_juice)
         hops: 3 (B1 ×2, B2, B6 on the deepest branch: is_a → kind → inherit → stand_in)
```

The pieces come from two topics in two domains. No message ever mentions lemon juice and limescale together.

**E7. An induced rule, used, then specialised on a counterexample.**

```
TEACHER: topic cooking/containers.
         made_of(plastic_tub, plastic).    not can_go_in(plastic_tub, oven).       (s0130.1-2)
TEACHER: topic baking/tools.
         made_of(plastic_jug, plastic).    not can_go_in(plastic_jug, oven).       (s0144.3-4)
TEACHER: topic cleaning/tools.
         made_of(plastic_bucket, plastic). not can_go_in(plastic_bucket, oven).    (s0150.1-2)
         made_of(spray_bottle, plastic).                                           (s0150.6)
AGENT:   hypothesis: holds_not(X, can_go_in, oven) :- holds(X, made_of, plastic),
                     not holds(X, can_go_in, oven).
           support [plastic_tub s0130, plastic_jug s0144, plastic_bucket s0150]
           (3 topics, 3 domains) counter [] conf 0.75 id ind/L1/0004
           new: not can_go_in(spray_bottle, oven)  (hypothesis)
USER:    query: can_go_in(spray_bottle, oven).
AGENT:   answer: no (hypothesis, conf 0.75)
         support: ind/L1/0004 from plastic_tub, plastic_jug, plastic_bucket; counter none
         premises: made_of(spray_bottle, plastic) given by s0150.6
TEACHER: topic baking/oven.
         made_of(roasting_bag, plastic).  can_go_in(roasting_bag, oven).            (s0170.1-2)
AGENT:   ok. ind/L1/0004 meets a counterexample: roasting_bag.
TEACHER: has_property(roasting_bag, heat_resistant).                               (s0170.3)
AGENT:   specialised: ind/L1/0004 → ind/L1/0004a
           holds_not(X, can_go_in, oven) :- holds(X, made_of, plastic),
             not holds(X, has_property, heat_resistant), not holds(X, can_go_in, oven).
           support [plastic_tub, plastic_jug, plastic_bucket] counter [] conf 0.75
           (roasting_bag excluded by the new condition)
```

If the teacher had not taught `heat_resistant`, no feature would separate the bag from the support set. The rule would be retracted, and its conclusions withdrawn:

```
AGENT:   retracted: ind/L1/0004 (counterexample roasting_bag s0170.2; no separating feature)
         withdrawn: not can_go_in(spray_bottle, oven) because ind/L1/0004 retracted.
         ask: what differs roasting_bag from {plastic_tub, plastic_jug, plastic_bucket}?
```

**E8. An honest gap.**

```
USER:    request(descale_kettle). lacks(user, vinegar). has(user, baking_soda).
AGENT:   answer:
         task descale_kettle
         gap lacks vinegar in step 2: no known stand-in; baking_soda lacks can_do(dissolve_limescale)
         ask: can_do(baking_soda, dissolve_limescale)?
```

### 6.5 What stays out of the answer

The agent never fills a slot without a proof. In particular, it does not change:
- times;
- settings;
- amounts;
- order;

unless a taught fact or rule derives the change. Answers carry no English. A telegraphic gloss for humans may follow the lines, and it is never scored (3.0 principle 2).

### 6.6 Active learning: which gaps become questions

The question budget is limited (section 8.2), so gap candidates are ranked:

1. **Near-miss stand-ins.** Exactly one `missing` requirement and no `blocked` one.
2. **Conflicts** (always asked; they block answers).
3. **Naming questions** (always asked; they block use of the new symbol).
4. **Unresolved diamonds.**
5. **Hypotheses with u large and s at the threshold.** The agent asks about one of the new predictions, to confirm or refute it.
6. **Empty functions.** A function used in a procedure role that only one thing can do.

Each question is a formal pattern (`ask: can_do(X, beat)?`). It is batched with others into the next teacher call (section 8.2).

## 7. Evaluation (Q6)

### 7.1 The derivation benchmark

**Recommendation (D4.6, to decide).**

- **Sizes.**
  - The spike uses about 100 hand-checkable questions over its fixture (section 9.2).
  - Phase B uses about 600 questions: 150 in a **dev** part, which may be read, and 450 in a **sealed** part, hashed like H3 (`data/heldout3/MANIFEST.sha256`).
  - The pre-planning mentions "about 200 hand-checkable" (Q6) and "50-100" (section 6) for the spike. This draft takes 100 for the spike, and puts the 200-question hand-checkable form in the phase B dev part (150) plus a 50-question pilot used in milestone 35.
- **Question kinds.**

  | Kind | Form | Gold |
  |---|---|---|
  | Relation query | `query: can_hold(muffin_tray, batter).` | yes / no / unknown, plus a proof |
  | Open query | `query: can_do(X, beat), has(user, X).` | the set of bindings, each with a proof |
  | Exception | a query whose naive inheritance answer is wrong | the specific answer, plus a proof |
  | Stand-in | `request(...)` with situation facts | `swap` facts, or a gap |
  | Step replacement | `request(...)` with a lacked item that blocks a step | `replace_step`, or a gap |
  | Induction-needed | a query only an induced rule can answer | a hypothesis answer, plus its support |
  | Honest gap | any of the above with a piece deliberately absent | `unknown` / `gap` |

- **Strata.**
  - Hops 1, 2 and 3 (computed from the gold proof, section 5.2).
  - Domain tiers:
    - **taught**: the question's task and items were taught in their own domain;
    - **cross-domain**: the pieces come from at least two domains;
    - **untaught domain**: gardening, held out of the curriculum as in 3.0 (D15). Its procedures are given in the question's `given:` block, and only item knowledge taught in other domains can answer it.
  - Recommended mix: 30% 1-hop, 40% 2-hop, 30% 3-hop; at least 30% cross-domain; 15% untaught domain; 15% honest gaps.
- **Each question has:**
  - a gold answer;
  - a gold proof (the pieces, and at least one valid chain under the base rules, or a support set for induction questions);
  - the hop count;
  - the domain tier;
  - an honest-gap gold, for the variant with a piece deliberately absent.
- **Who writes it.** Sub-agents write it from a spec (`tools/design4/BENCH_SPEC.md`), without sight of the agent's code, as for H3. They see:
  - the schema;
  - the base rules (needed to write gold proofs);
  - the shared vocabulary (section 8.4);
  - the topic list.

  They do not see teaching transcripts. The gold proofs are checked by the same proof checker over a knowledge base made only of the gold pieces. A question whose gold proof does not check is returned to the authors. The sealed part is then hashed.

### 7.2 Pieces, coverage and why two numbers are reported

The benchmark's pieces are taught by Haiku, not by the benchmark's authors. So a wrong answer can mean two different things:
- the agent did not learn a piece (coverage);
- it learned the pieces but did not derive the answer (reasoning).

These are kept apart:

- **Piece coverage.** For each question, the share of its gold pieces that hold in the agent's knowledge base (taught or derived, matched by canonical name).
- **Accuracy overall,** and **accuracy on questions whose pieces are all present** (reasoning given knowledge).

Gap questions are valid only if the absent piece was in fact never taught. A gap question whose deliberately absent piece appears in the raw layer anyway is **voided**, counted and reported, not scored.

### 7.3 Metrics

All are reported per hop, per domain tier and per question kind, with Wilson 95% intervals (`crates/eval3/src/stats.rs:4`).

| # | Metric | Definition |
|---|---|---|
| N1 | **Correct derivation** | The answer equals the gold, and its proof passes the proof checker |
| N2 | **Invention** | A claim whose proof fails the checker, or a claim with no proof or support. **Target 0** |
| N3 | **Wrong with a valid proof** | The answer differs from the gold, but its proof checks. The cause is a wrong taught piece or an induced rule; each case is attributed by the leaves of its proof |
| N4 | **Correct gap** | The gold is unknown and the agent says gap |
| N5 | **Honest miss** | The gold has an answer and the agent says gap |
| N6 | **Alternative derivation** | On a gap question, a valid proof through pieces other than the deliberately absent one. Audited by hand on dev, not counted as invention |
| N7 | **Proof validity** | The share of all proofs the agent emits that pass the checker |
| N8 | **Hypothesis precision** | Of answers that rely on an induced rule, the share that are correct; plus retractions and specialisations per rule layer |
| N9 | **Piece coverage** | Section 7.2 |
| N10 | **Conversation quality** | Conflicts found or missed (against a scripted set), naming questions asked and confirmed, rejected messages, accepted facts per call |
| N11 | **Cost** | Haiku calls per accepted fact, per stage; inference time and model size per knowledge-base size; the 95th percentile answer time |

### 7.4 Leakage check

- **The deny list** holds every gold answer atom of the dev and sealed parts. It also holds its trivial variants:
  - argument permutations;
  - the negation;
  - forms under every `same_as` the vocabulary declares;
  - the `swap` difference as the fact `stand_in`.

  The sealed part's deny list is stored as hashes only, so the list does not reveal the answers.
- **Before delivery:** `teach4` checks every teacher message against the deny list before the agent sees it. A denied message is **withheld**: it is kept in the raw layer as `withheld`, and the agent never receives it.
- **The run fails** if any denied atom reaches the agent. A test re-scans the raw layer's delivered messages after every run.
- **Recommendation (D4.6a, to decide).** The pre-planning says "the run fails if one is taught". Haiku will sometimes teach a true benchmark answer simply because it is true. Failing the whole run for that would waste calls, while withholding keeps the answer untaught. This draft therefore proposes withhold-then-fail-on-delivery, with the withheld count reported.

### 7.5 Baselines

- **K0, taught facts only.** No rules: an answer is given only if the answer fact itself is in the knowledge base. Under the leak check, this should score near zero, which shows the leak check works and that derivation is the whole result.
- **K1, base rules only** (learned rules off).
- **K2, base rules plus learned rules** (the system).
- **K3, Haiku given the same taught facts** as text, plus the question, answering in the formal language with a proof. This is optional and costs calls (section 10); it is cached and run on dev plus the sealed part, batched 5 questions per call. It shows what a language model does with the same knowledge, including whether it invents.
- **The 3.0 agent** is not a baseline here: it cannot answer relation questions. Its H3 results stay the comparison for procedures (Q8).

### 7.6 The learning curve (the main result)

The main result is a set of curves, not one number.

- **x-axis:** curriculum progress, with snapshots of the knowledge base at 25, 50, 75 and 100% of the teaching messages of a curriculum, in curriculum order.
- **Curves:** one per knowledge-base size (S, M and, if gate G2 says so, L; section 8.1), and one per learner setting (K1 base rules only, K2 with learned rules).
- **y-axis:** N1 (correct derivations), with N2 (inventions) plotted beside it, which must stay at 0. Also N9 (coverage), and N1 restricted to questions whose pieces are all present.
- **Replication:** 2 curriculum orders for S and M (topic order shuffled within stages), so the curve has a spread. The replay tool makes the second order cheap: the same teacher messages, re-ordered, need **no new calls** except for the agent's new questions (replay, section 8.5).
- **Reading:** every snapshot is saved during the runs. The sealed part is read **once for all snapshots together**, in the final reading (section 7.7), so the curve cannot be tuned against it. The dev part is read freely.
- **What counts as "gets better as taught more":** the slope of N1 against progress, with intervals, and the difference between 25% and 100% by a paired McNemar test (`crates/eval3/src/stats.rs:18`) on the same questions. Its threshold is **set after the spike**.

### 7.7 Readings of the sealed part

There are 3 readings, counts only, as in 3.0 (D12):
1. after milestone 36 (S built), a baseline reading;
2. after gate G2;
3. the final reading of every snapshot, ablation and baseline.

Every reading is recorded in `data/bench4/READINGS.md`.

## 8. Knowledge base, curriculum and teacher (Q5, Q7)

### 8.1 Knowledge-base sizes (P10)

**Recommendation (D4.7a, to decide).**

| Size | Taught facts (target) | Domains | Built when |
|---|---|---|---|
| Spike fixture | 300-500 (hand-written, zero calls) | baking, cooking, cleaning, repair | Phase A |
| S | about 1,500 | cooking, baking, cleaning, repair | Milestone 36 |
| M | about 4,000 | S plus laundry, kitchen care | Milestone 38 |
| L | about 10,000 | M plus home care, broader items | Only if gate G2 shows M > S |

Gardening is never taught (the untaught-domain tier). The fact counts are targets for the topic planner, not thresholds.

### 8.2 Curriculum (Q7)

**Recommendation (D4.7, to decide).**

**Stages.** Topics and order come from a script (`tools/design4/topics.py` writes `data/curriculum4/<size>/topics.jsonl`). The stages run in this order:

| Stage | Content | Main relations | Teacher |
|---|---|---|---|
| 1 Kinds and materials | items per domain, their kinds and materials | `is_a`, `made_of`, `part_of`, `same_as` | Haiku |
| 2 Functions and capabilities | what things can do, hold, go in; properties | `can_do`, `used_for`, `can_hold`, `can_go_in`, `has_property` | Haiku |
| 3 Procedures | the v3 procedures of 2.0 and 3.0 tasks | v3 procedure facts | Scripted seed teacher (zero calls), then Haiku for each procedure's item relations |
| 4 Actions and states | what actions achieve and require | `achieves`, `requires` | Haiku |
| 5 Exceptions | negative facts; what does not hold for specific things | `not R(...)` | Haiku |

**A teaching call** asks Haiku for one topic batch of 15-25 statements. The prompt gives:
- the schema (relations, sorts, the message grammar);
- the topic's entities from the shared vocabulary;
- up to 40 known facts nearest the topic, so it extends rather than repeats;
- the agent's pending questions for that topic;
- the instruction to answer in formal statements only.

The answer is split into messages, and each goes through the conversation path one by one. The agent's replies (conflicts, questions) are collected and sent in the **next** call on the same topic. One call is therefore one teacher turn holding many messages.

**Questions** are limited to 20% of the stage's calls, as the pre-planning suggests. Within that share, questions are batched (up to 10 per call) and ranked as in section 6.6.

**Calls per stage are capped** (section 10). Reaching the cap ends the stage, and the run moves to the next stage.

### 8.3 Seed data (P12)

The 2.0 and 3.0 data seeds the raw layer:
- `data/curriculum` and `data/curriculum3` (train, dev, seed; 52 + 19 + seed tasks, re-authored with 419 role facts);
- `data/dataset3` (102 generated tasks);
- the call archives (`data/training/calls3/cache.jsonl`, 4,930 lines).

It is **re-taught through the conversation path, never injected.**

**Recommendation (D4.7b, to decide).**
- The v3 procedure facts are delivered by a **scripted seed teacher**. It is a zero-call component that sends each scenario's ground-truth facts as teaching messages, with sender `seed:<file>`. The agent processes them like any other message (schema, naming, conflicts, derivation, questions), and answers to its questions go to Haiku within the question budget.
- Re-teaching the same facts through Haiku would cost about one call per task (about 300) and add naming drift, for no gain in knowledge.
- **Gardening scenarios and every H3 scenario are excluded** from the seed.
- The call archives are scanned for item facts already stated by Haiku in 3.0 (fallback parses, teacher answers). Those that fit schema v4 are replayed as teacher messages with sender `archive:calls3:<key>`, at zero cost.

Whether "scripted re-teaching" satisfies P3 ("teacher: Haiku") is for the user to decide. The alternative is Haiku re-teaching.

### 8.4 Shared vocabulary

**Recommendation (D4.7c, to decide).** A vocabulary file per domain (`data/curriculum4/vocab/<domain>.txt`) lists symbol names only, no facts. For example: `loaf_tin`, `muffin_tray`, `baking_tin`, `batter`.

- The topic planner gives the teacher the topic's names.
- The benchmark authors use the same names.

This attacks 3.0's naming failure at its source. It is not knowledge (no relation is stated), but it is a design choice the user should confirm. Haiku may introduce new names. They go through the naming check (section 6.2), and the vocabulary grows only from conversation.

### 8.5 Knowledge-base format (Q5)

**Recommendation (D4.5, to decide).** Three layers, each with a schema version on every record. The raw layer is the source of truth.

**Raw layer** (`data/kb4/raw/<curriculum>/<session>.jsonl`). It is append-only and never rewritten. It holds both sides of the conversation, every message as sent:

```json
{"v":"raw/1","schema":"4.0","session":"S-c1-s0042","turn":7,"from":"teacher","sender":"haiku",
 "call":"9f2c…e1","topic":"baking/tins","text":"is_a(muffin_tray, baking_tin).",
 "delivered":true,"in_reply_to":null,"unix":1795000000}
{"v":"raw/1","schema":"4.0","session":"S-c1-s0042","turn":8,"from":"agent","agent":"agent4@3c1a9e0",
 "text":"ok. derived: can_hold(muffin_tray, batter)","in_reply_to":"S-c1-s0042.7"}
{"v":"raw/1","schema":"4.0","session":"S-c1-s0042","turn":9,"from":"teacher","sender":"haiku",
 "call":"9f2c…e1","text":"can_hold(muffin_tray, batter).","delivered":false,"withheld":"deny_list"}
```

**Facts layer** (`data/kb4/builds/<build>/facts.dl` and `rules.dl`). It is Datalog text with `%` comments, which `parse_program` already skips (`crates/reasoner/src/lib.rs:1090`):

```datalog
% schema 4.0  build S-c1@agent4-3c1a9e0  from raw S-c1 (1,512 delivered messages)
is_a(muffin_tray, baking_tin).            % f000311
can_hold(baking_tin, batter).             % f000307
-can_go_in(plastic_dish, oven).           % f000402 exception_of f000398
same_as(egg_whisk, whisk).                % f000120 canonical whisk
```

```datalog
% rules.dl  learned layers (base rules are in the agent, not here)
% ind/L1/0004a  conf 0.75  status hypothesis  layer 1
holds_not(X, can_go_in, oven) :- holds(X, made_of, plastic), not holds(X, has_property, heat_resistant), not holds(X, can_go_in, oven).
```

**Provenance layer** (`data/kb4/builds/<build>/prov.jsonl`). It extends 3.0's `Provenance` fields (`crates/kb3/src/vault.rs:19-29`):

```json
{"v":"prov/1","id":"f000311","kind":"fact","fact":"is_a(muffin_tray, baking_tin)",
 "msg":"S-c1-s0042.7","sender":"haiku","conf":0.9,"confirmed_by":[],"status":"live",
 "names":{"muffin_tray":"muffin_tray"},"retracted_by":null}
{"v":"prov/1","id":"ind/L1/0004a","kind":"rule","layer":1,"built_from":["core/B2","ind/L1/0004"],
 "support":["plastic_tub","plastic_jug","plastic_bucket"],"support_msgs":["S-c1-s0130.2","S-c1-s0144.4","S-c1-s0150.2"],
 "domains":["cooking","baking","cleaning"],"counter":[],"conf":0.75,"status":"hypothesis",
 "history":[{"msg":"S-c1-s0170.2","event":"counterexample","x":"roasting_bag"},
            {"msg":"S-c1-s0170.3","event":"specialised","added":"not holds(X, has_property, heat_resistant)"}]}
{"v":"prov/1","id":"e000017","kind":"event","event":"withdrawn","fact":"can_go_in(plastic_dish, oven)",
 "because":"f000402","msg":"S-c1-s0007.5"}
```

**Replay.** `sla4 replay --raw data/kb4/raw/S-c1 --agent HEAD --out data/kb4/builds/S-c1@<sha>` rebuilds facts and provenance from raw under any agent version, at zero calls.

- Delivered teacher messages are fed in their original order.
- A new agent may ask a question the original run did not ask. It gets no answer in replay; this is recorded as `unanswered_in_replay`. A budgeted `--top-up` option sends those questions live.
- Teacher answers to questions the new agent does not ask are still delivered: they are statements in their own right.

**Schema migration.** A change of schema (a renamed or split relation) is written as data in `data/kb4/migrations/<from>-<to>.dl`, as rename rules applied to raw messages during replay. Raw files are never edited.

This replaces 3.0's single-file vault (`crates/kb3/src/vault.rs:101-112`), which mixed facts, provenance and cases in one JSON file and could not be rebuilt.

## 9. Phase A: design session and spike

Phase A spends **zero Haiku calls**.

### 9.1 Milestone 30: design session

The design session goes through Q1-Q8 with the user, using sections 5-8 and 12 of this draft. Each answer is recorded as a decision D4.1-D4.8 (plus sub-decisions) in `research/DESIGN_4_decisions.md`.

It also agrees the **spike's go/no-go thresholds before the spike runs** (pre-planning section 6). **Recommendation (D4.9, to decide):**

- **Go** if all four hold:
  - on the spike benchmark with the full fixture, base rules derive at least about 60% of 2-hop questions correctly;
  - there are 0 inventions on all questions;
  - proof validity is 100%;
  - learned rules add measurably: at least 5 more correct answers on the induction-targeted questions with rules on than off, and no increase in wrong answers on the other questions.
- **Rethink** otherwise: the relation set, the rules or the approach, before any teaching budget is spent.

### 9.2 Milestone 31: the spike build

**The fixture** (`data/spike4/`). It is written by hand in the design session's follow-up (by the user, or by Claude Code under review). It is a test fixture, not phase B knowledge.

| File | Content |
|---|---|
| `teach/<domain>.msgs` | About 400 teaching messages in the formal language, in curriculum order, for baking, cooking, cleaning and repair, about 100 per domain. Planted inside them: <ul><li>8-10 specificity exceptions</li><li>2-3 diamonds</li><li>4 induction clusters with support 3 and from at least 2 domains</li><li>2 induction traps with support 2 (must not induce)</li><li>1 trap with a counterexample (must not induce)</li><li>3 naming-drift pairs</li><li>2 sort errors (must be rejected)</li></ul> |
| `teach/procedures.msgs` | 12 v3 procedures taken from `data/curriculum3/train` (stand-in and step-replacement targets), delivered by the scripted seed teacher |
| `script/answers.jsonl` | The scripted teacher's answers to the agent's expected questions (naming, conflicts, gaps), keyed by question pattern. An unexpected question is answered `unknown.` and counted |
| `bench/questions.jsonl` | 100 questions:<ul><li>20 1-hop, 30 2-hop, 20 3-hop</li><li>15 honest gaps</li><li>15 induction-targeted</li><li>at least 30 cross-domain</li><li>10 in gardening, with `given:` procedures</li></ul>Each has its gold answer, gold proof, hop count and tier |
| `bench/deny.txt` | The gold answer atoms and their trivial variants |
| `MANIFEST.sha256` | Hashes of the above, so the fixture cannot change silently during the spike |

**The builds** (pre-planning section 6, items 1-6), in buildable order:

1. **`kb4` schema v4.** Relations, sorts, `inherits` and `transitive` declarations, and the message grammar (section 6.1).
   - Tests:
     - every example message in this document parses;
     - the 2 sort errors are rejected;
     - `not` facts round-trip to `-R`.
2. **`reason4` base rules.** The section 5.2 templates are expanded per relation and loaded with ids `core/B0`-`core/B8`. The relation declarations come from the schema.
   - Tests:
     - E1, E3, E4 and E6 give exactly the shown derivations;
     - `check` (`lib.rs:946`) passes, so the program is safe and stratified;
     - each B rule can be switched off by id.
3. **Exceptions.** B2 specificity, the priority order of section 5.4, `withdrawn:` reporting by comparing models before and after each message.
   - Tests:
     - E3 and E4 as written;
     - an induced conclusion never beats a taught or inherited one (a planted case).
4. **`learn4` induction.** The section 6.3 learner, with k ∈ {2, 3, 4}, layers, quarantine, specialisation and retraction.
   - Tests:
     - E7, both branches;
     - the two support-2 traps induce nothing at k = 3;
     - the counterexample trap induces nothing;
     - no rule supports itself.
5. **Proof output and an independent proof checker.** The checker re-unifies every rule instance and confirms every leaf is a live fact with a raw message. It re-evaluates every `not` atom against the model, and computes hop counts.
   - Tests: the five negative controls of section 4.3.
6. **`eval4` scorer and the spike runner.** N1-N11 on the fixture.
   - Commands:
     - `sla4 spike` (full fixture, all rules);
     - `sla4 spike --off core/B6` (one base rule off);
     - `sla4 spike --learn off|k2|k3|k4`;
     - `sla4 spike --prefix 25|50|75|100 --seed 0..2`;
     - `sla4 spike --scale 1|10|50` (section 9.3).
   - The leak check runs on the fixture's teaching files. A planted leak in a test copy must fail it.

The **agent4 conversation path** (section 6.2, steps 1-8) runs in the spike against the scripted teacher, so the fixture is taught, not loaded.

### 9.3 Milestone 32: spike measurements and report

| Measure | How |
|---|---|
| N1-N8 by hop, tier and kind | `sla4 spike`, full fixture |
| Contribution of each base rule | B1-B7 switched off in turn; the change in N1 and N4, and which questions flip |
| Learned rules | Off against k = 2, 3, 4; hypothesis precision; traps |
| The curve on the fixture | Prefixes of 25, 50, 75 and 100% of the teaching messages; 3 random orders as well as curriculum order |
| Inference cost | Evaluation time, model size (derived facts), maximum and median proof depth, at each prefix |
| Scaling stress | The fixture copied 10 and 50 times with renamed symbols plus cross-links between copies (up to about 25,000 facts); time and model size with and without question pruning. Accuracy is not measured on copies |
| Proof length | How often the kept first derivation (`lib.rs:745-753`) is longer than the gold proof |
| Conflicts | The kinds found (section 5.4) and whether each was asked |
| `same_as` | Canonical naming against the B4 rule form, which must agree |

**The spike report** (`research/SPIKE_4.md`) gives these numbers. It also proposes the phase B thresholds of section 11 from them, so it is where every "set after the spike" value is filled in.

**Gate G1.** The user decides go or no-go on the report, against the D4.9 thresholds. On go, the Design 4.0 document is written from this draft plus the report (pre-planning section 7, item 4).

**What the spike cannot tell.** It makes no calls, so it cannot measure:
- Haiku's format compliance;
- its accuracy;
- its naming drift;
- facts per call.

Milestone 34's pilot measures those, with its own small threshold set after the pilot.

## 10. Haiku budget (P14: 5,000 calls, hard limit)

**Ledger.** 4.0 gets its own ledger, `data/training/calls4/ledger.jsonl`. The 3.0 code counts every successful line of the 3.0 ledger against one budget (`crates/teach3/src/lib.rs:92-96`, `lib.rs:117-119`), and 3.0 used 4,486 of its 5,000. The caches are kept separately and never deleted. `teach4` uses `teach3`'s fail-fast behaviour on the plan usage limit (`lib.rs:21-25`, `lib.rs:146-150`).

**Recommendation (D4.10, to decide):** the 5,000 is a fresh budget for 4.0, counted only in `calls4`. If the user means 5,000 in total across 3.0 and 4.0, only about 500 remain, and this plan shrinks to S only.

| Use | Estimate (calls) | Cap |
|---|---|---|
| Phase A (design session, spike) | 0 | 0 |
| Milestone 34 teacher pilot: format, facts per call, naming, accuracy on a hand-checked sample | 50-80 | 100 |
| S curriculum, order 1: teaching plus questions (≤ 20%) | 150-250 | 300 |
| S curriculum, order 2: replay plus top-up of new questions only | 20-50 | 80 |
| M curriculum, order 1 | 350-550 | 650 |
| M curriculum, order 2: replay plus top-up | 50-100 | 150 |
| L curriculum, only if gate G2 says so | 800-1,200 | 1,300 |
| K3 Haiku baseline: 600 questions at 5 per call, dev plus sealed | 120-150 | 200 |
| Reserve: re-runs after fixes, extra replication | the rest | — |
| **Total planned** | **about 1,550-2,400**, up to about 2,800 caps without L | **5,000** |

These estimates assume about 15 accepted facts per call. Milestone 34 measures the real rate, and the caps are re-set then (within 5,000). Benchmark authoring is done by Claude Code sub-agents, not Haiku, and is not counted in this ledger. If the user wants it counted, it is recorded separately.

**Stop rules.**
- The ledger warns at 4,000.
- At 4,500, the remaining work runs on cached calls and replay only.
- At 5,000, `teach4` refuses every live call, until the user explicitly raises the limit.
- A stage that reaches its cap ends.
- A plan-limit refusal stops the run at once (3.0 finding 2).

**No run that spends calls starts without the user's go-ahead** (memory: training runs need go-ahead).

## 11. Phase B: milestones and acceptance tests

Each milestone commits on `design-4` with its acceptance tests passing. Structural acceptance tests are fixed now. Numeric thresholds are **set after the spike** (or after the milestone 34 pilot where stated) and frozen in `research/PREREG_4.md` before milestone 36 starts. They may be lowered once, with the reason written down, after the first sealed reading, as in 3.0.

**33. The three-layer knowledge base, replay and the full conversation path.**
- Build:
  - `kb4` layer writers and readers;
  - `sla4 replay` and migrations;
  - the naming store;
  - the deny-list filter in `teach4`;
  - the full section 6.2 path, beyond the spike's form;
  - lab events (derivations, conflicts, hypotheses) on 3.0's lab stream.
- Accept:
  - replay of the spike fixture is byte-identical to the live run;
  - a migration that renames one relation rebuilds the same answers;
  - the no-injection test passes;
  - the leak test fails on a planted leak and passes otherwise;
  - the word-list test passes on all 4.0 crates.

**34. Teacher channel and pilot (50-80 calls, needs the user's go-ahead).**
- Build: `teach4` prompts per stage, batching, question batching, the scripted seed teacher.
- Run: one topic per stage on two domains.
- Accept:
  - every call is cached and in the `calls4` ledger;
  - the share of messages that parse and pass the schema check is reported;
  - accepted facts per call are reported;
  - teacher accuracy on a 100-fact hand-checked sample is reported;
  - naming questions per 100 facts are reported.
- Thresholds for the format share and accuracy are **set after the spike and this pilot**. If accuracy is low, the teacher prompt is changed before stage S.

**35. Benchmark.**
- Build:
  - `BENCH_SPEC.md`;
  - sub-agent authoring;
  - the gold proof check over gold-piece knowledge bases;
  - the dev part (150) and the sealed part (450) with `MANIFEST.sha256`;
  - the hashed deny list;
  - the gap-question void rule.
- Accept:
  - every gold proof checks;
  - hop labels are recomputed by code and match;
  - the strata mix is within 5 points of D4.6;
  - the hash test passes;
  - K0 run on a gold-piece knowledge base answers 0 non-trivial questions (this shows the answers are not among the pieces).

**36. Curriculum S, with snapshots.**
- Build: the topic planner for S; the curriculum runner with snapshots at 25, 50, 75 and 100%.
- Run: order 1 live, then order 2 by replay plus top-up.
- Accept on dev:
  - N2 (inventions) = 0;
  - N7 (proof validity) = 100%;
  - N1 at 100% of S ≥ **set after the spike**;
  - the paired 25%-against-100% difference ≥ **set after the spike**;
  - piece coverage is reported.
- Then the **first sealed reading** (baseline), and `PREREG_4.md` is frozen.

**37. Rule learning at scale.**
- Build: induction on the S knowledge base, with layers, specialisation and retraction; the agent's hypothesis questions within the question budget.
- Accept on dev:
  - hypotheses per layer are reported;
  - hypothesis precision ≥ **set after the spike**;
  - every retraction is traceable to a teaching message;
  - K2 against K1: N1 gain ≥ **set after the spike**, with no rise in N3;
  - at least one layer-2 rule is accepted, or the report says why none was.

**38. Curriculum M and gate G2 (scaling).**
- Run: M, both orders, with snapshots.
- Accept:
  - the same structural tests as milestone 36;
  - the inference time at the 95th percentile on M ≤ **set after the spike** (from the scaling stress).
- **Gate G2, decided by the user:** does M beat S on dev, overall and on pieces-present questions? By how much is **set after the spike**. If yes, milestone 39 runs. If no, L is skipped, and the result is reported as "scaling did not help at this size".
- Then the **second sealed reading**.

**39. Curriculum L (conditional on G2).** As milestone 38, at L size.

**40. Final measurement.**
- Run, on the sealed part:
  - the final reading of every snapshot of every size and order;
  - K0, K1, K2 and K3;
  - base-rule ablations (B1-B7, each off);
  - induction k = 2, 3, 4;
  - same_as as rules against canonical naming;
  - question budget 0% against 20% (does active learning pay?).
- Accept:
  - every threshold reported with an interval, met or not;
  - the ledger shows 3 readings;
  - the call total is within 5,000.

**41. Research PDF update.**
- Build: a 4.0 section in `paper/`, with every number generated by `tools/paper/numbers.py` from `data/`.
- Accept:
  - `make -C paper` builds from a clean checkout;
  - `make -C paper check` shows no difference.

## 12. Q8: what stays from 3.0

**Recommendation (D4.8, to decide).**

| 3.0 part | 4.0 |
|---|---|
| Provenance fields and "corrections never delete" (`kb3/src/vault.rs:19-71`) | Kept; moved into the provenance layer |
| The single-file vault (`kb3/src/vault.rs:101-112`) | Replaced by the three layers; the 3.0 vaults stay readable for comparison |
| Schema v3 procedure, situation and difference predicates (`kb3/src/fact.rs:7-17`) | Kept unchanged, beside the new relations; `requires_state` is read as `requires` |
| Relation-line answers (`kb3/src/lines.rs`) | Kept for procedure answers; extended with `proof:`, `support:` and `gap:` blocks |
| The fact-and-relation check (`kb3/src/check.rs:158`) | Kept for procedure answers (stand-in requests are scored as `swap` facts); relation questions use the new scorer |
| Head-word matching of names (schema section 6) | Reused as a **proposer** of `same_as?` questions, not as a scoring rule |
| Cached Haiku calls, key, fail-fast, locked appends (`teach3/src/lib.rs`) | Kept; new `calls4` ledger |
| The lab stream (`eval3/src/lab.rs`) | Kept; new event kinds |
| The word-list test (`kb3/tests/no_word_lists.rs`) | Extended to the 4.0 crates and the base-rule templates |
| Statistics (`eval3/src/stats.rs`) | Kept |
| Sealed set procedure (manifest, readings, counts only) | Kept for the 4.0 benchmark |
| Learned English parser (`parse3`) and Haiku fallback parsing | Not on the core path; kept for the later English layer |
| Task identity by overlap (`agent3/src/identity.rs`) | Not needed: requests are formal (`request(<task>)`); kept for the English layer |
| Adaptation arms (`agent3/src/adapt.rs`) | Replaced by base rules B6 and B7 plus induction; kept as a comparison on 3.0's dev variants |
| Situation reading | Formal, in the teaching form |
| The 2.0 crates, including `reasoner` and `learner` | `reasoner` becomes the core engine, unchanged. `learner` is extended by `learn4`, not edited. Everything else is unchanged |

## 13. Risks and their mechanisms

Every risk of pre-planning section 4, with the mechanism in this design and the test that shows it works:

| Risk | Mechanism in 4.0 | Where | Test |
|---|---|---|---|
| **Naming drift** (3.0 finding 5) | <ul><li>Shared vocabulary for teacher and benchmark</li><li>The naming check proposes `same_as?` by head word or relational context, and a new symbol is pending until answered</li><li>Answers are stored as `same_as`, `not same_as` or `is_a`</li><li>Canonical naming keeps the original names in provenance</li></ul> | 6.2 step 2; 8.4; 5.2 B4 | 3 planted drift pairs in the spike; N10 naming questions in the pilot and runs |
| **Learned rules overgeneralise** | <ul><li>Support k ≥ 3, from ≥ 2 topics or domains, and c = 0</li><li>Minimal bodies and no self-support</li><li>Induced conclusions are below taught and inherited ones and are printed as hypotheses</li><li>Specialisation, else retraction, on a counterexample</li><li>Quarantine for unsafe rules</li></ul> | 6.3; 5.4 | Spike traps (support 2, counterexample); N8 hypothesis precision; E7 |
| **Leakage into the benchmark** | <ul><li>Hashed deny list of answers and trivial variants</li><li>Withhold before delivery, and fail on delivery</li><li>The leak test over the raw layer</li><li>Gap questions voided if their absent piece was taught</li><li>K0 near zero</li></ul> | 7.4; 7.2 | Planted-leak test (milestone 33); K0 (milestone 35) |
| **Base rules too weak or too strong** | <ul><li>Every rule can be switched off by id</li><li>Spike ablations with flips listed</li><li>No material rule built in (left to induction)</li><li>Rules that do not pay are dropped in D4.3</li></ul> | 5.2; 9.3 | Spike ablation table; milestone 40 ablations |
| **Inference explodes on a large knowledge base** | <ul><li>Typed relations with sorts checked at entry</li><li>Question pruning by `depends_on` and reachable symbols</li><li>A hop limit on answers</li><li>First-argument index preserved by the `holds(X, R, Y)` order</li><li>Magic sets as the fallback</li></ul> | 4.2 | Scaling stress ×1, ×10, ×50 in the spike; the milestone 38 time threshold |
| **Wrong facts spread through chains** | <ul><li>Confidence per fact (0.9 from Haiku, raised by confirmation) and min-propagation (`lib.rs:938`)</li><li>Contradiction checks at entry, and B8 blocks conflicting facts from answers</li><li>Exceptions</li><li>Every answer shows its chain</li><li>N3 is attributed to its leaves</li></ul> | 5.3; 6.2 step 3; 5.4 | N3 attribution; the milestone 34 teacher-accuracy sample |
| **"Gets better" is one number, not a curve** | <ul><li>Snapshots at 25, 50, 75 and 100%, for every size, both orders, K1 and K2</li><li>All read together, once, on the sealed part</li><li>A paired test of 25% against 100%</li></ul> | 7.6; 7.7 | Milestones 36, 38, 40 |
| **Thresholds set by guesswork** | Every phase B threshold is "set after the spike" or after the pilot and frozen in `PREREG_4.md`; only the spike gate is agreed beforehand, as the pre-planning requires | 9.1; 11 | `PREREG_4.md` written before milestone 36 |
| **Haiku spends the budget on volume** | <ul><li>Batches of 15-25 statements per call, and questions batched up to 10 per call</li><li>Seed procedures by the zero-call scripted teacher</li><li>Replay for second orders</li><li>Caps per stage, with warn/stop at 4,000, 4,500 and 5,000</li><li>Fail fast on the plan limit; every call cached and archived</li></ul> | 8.2; 8.3; 10 | Ledger; N11 calls per accepted fact |
| **Structured teaching makes the result look trivial** | <ul><li>The claim is "learns and generalises from formal teaching"</li><li>Each of integration, conflicts, naming questions, rule learning and derivation is measured (N1-N11)</li><li>K0 shows that taught facts alone answer nothing</li><li>K3 shows what a language model does with the same knowledge</li><li>Answers are never taught (leak check)</li></ul> | 7 | K0, K3, N10 |

Further risks found while drafting:

| Risk | Mechanism |
|---|---|
| **The teacher's world differs from the benchmark's world** (the pieces may never be taught) | Shared vocabulary; topics built from the benchmark's entities (not its pieces); piece coverage and pieces-present accuracy reported separately (7.2) |
| **The kept proof is not minimal**, because the reasoner keeps the first derivation only (`lib.rs:745-753`) | The hop limit is applied to the proof the checker validates; the spike measures how often it is longer than gold. If this matters, a minimal-proof search over `Derivation` alternatives is added in `reason4` |
| **Negation shows "no such fact known" without a confidence** | The proof checker re-evaluates each absent atom; answers that rely on an absence of a fact the agent asked about and got `unknown` for are printed as hypotheses |
| **B6 requirement relations are too strict** (an incidental relation of the lacked item blocks a good stand-in) | `missing` produces a question, not a refusal; the spike counts stand-ins lost to incidental requirements |
| **The scripted seed teacher conflicts with Haiku's facts** (v3 ground truth against teacher naming) | Conflicts and naming checks apply to seed messages too; the seed is delivered first, so later teacher names are checked against it |

## 14. What 4.0 can claim, and what it cannot

- **Can claim, if the thresholds are met:**
  - taught only through formal conversation, the agent derives answers it was never taught, across domains and in an untaught domain, each with a checkable proof or support;
  - it makes no unsupported claim;
  - its accuracy grows with teaching, as a measured curve at several sizes;
  - rules it induces add to what the base rules derive, and are retracted or specialised when contradicted.
- **Cannot claim:**
  - understanding English (there is none on the core path);
  - knowledge Haiku did not teach;
  - that Haiku's facts are true, beyond the hand-checked sample;
  - that synthetic teaching stands for human teaching;
  - that the result scales beyond the largest size measured.

## 15. Later (beyond 4.0)

- **Controlled English (option B).** The first language layer, built on the learned parser of 3.0. It turns controlled sentences into the 4.0 formal messages, and answers into controlled sentences.
- Free English, and human-written text.
- **The real-time training lab.** Design 3.0's dashboard direction applies to the 4.0 event stream: derivations, conflicts and hypotheses live.
- Composition rules for speed, magic sets, a minimal-proof search.
- Answer set programming, if the spike shows conflicts that stratified specificity cannot represent.
- Larger knowledge bases and the long-term vision (a formal-logic counterpart to LLM training with facts extracted at scale). This is context only, and moves only when the user says so (pre-planning section 2).
- Learning numeric changes (times, amounts) that follow from a stand-in.

## 16. Open decisions for the user

The design session (milestone 30) settles these. Each item gives the recommendation of this draft.

| # | Decision | Recommendation |
|---|---|---|
| D4.1 | Relation set | The 9 pre-planning relations, plus `can_hold` and `can_go_in`; sorts fixed on first use; `not` for every relation; add more only on spike evidence |
| D4.2 | Exceptions | Stratified default inheritance with specificity (B2); priority taught > inherited > induced; diamonds become questions; ASP and `except` facts rejected unless the spike shows a need |
| D4.3 | Base rules | B1-B8 as written in 5.2; `same_as` as canonical naming; no material rule; drop any rule that does not pay in the ablation |
| D4.3a | Confidence values | 0.9 for Haiku facts, 0.97 when confirmed, 1.0 for the seed; minimum propagation; induced `s/(s+c+1)` |
| D4.4 | Rule learning | Induction (bodies of 1-2 features, k = 3, ≥ 2 domains, c = 0, u ≥ 1), layers, specialisation, then retraction; composition only for speed |
| D4.5 | Knowledge-base format | Raw JSONL (both sides, append-only), Datalog facts and rules, provenance JSONL, schema version on every record; replay with top-up; migrations as data |
| D4.6 | Benchmark | 100 spike questions; 600 in phase B (150 dev, 450 sealed); kinds, strata and mix as in 7.1; sub-agent authors; gold proofs machine-checked; 3 sealed readings |
| D4.6a | Leakage handling | Withhold denied messages before delivery; fail the run only if one is delivered (the pre-planning says fail on any) |
| D4.7 | Curriculum | 5 stages; batches of 15-25 statements; questions ≤ 20% of calls; caps per stage |
| D4.7a | Knowledge-base sizes | S about 1,500, M about 4,000, L about 10,000 only after G2; gardening never taught |
| D4.7b | Seed re-teaching | A scripted zero-call seed teacher through the conversation path, not Haiku |
| D4.7c | Shared vocabulary | Symbol names per domain, shared by teacher prompts and benchmark authors |
| D4.8 | What stays from 3.0 | The table in section 12 |
| D4.9 | Spike gate | ≥ about 60% of 2-hop correct; 0 inventions; 100% proof validity; learned rules +5 correct on induction questions with no rise in wrong answers |
| D4.10 | Budget accounting | A fresh 5,000 for 4.0 in a separate `calls4` ledger; sub-agent benchmark authoring not counted against Haiku |
| D4.11 | Who writes the spike fixture | Claude Code under the user's review, or the user; it is a test fixture, not phase B knowledge |
