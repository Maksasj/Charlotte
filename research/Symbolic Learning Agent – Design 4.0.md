# Symbolic Learning Agent – Design 4.0

**Status.** Final design, written on 2026-11-19 on branch `design-4`. Milestone 30 (the design session) is done. This document governs phase A's remaining milestones (31 spike build, 32 spike report and go/no-go) and phase B (milestones 33-41). Binding, in this order:
1. the pre-planning decisions P1-P16 in `research/DESIGN_4_preplanning.md`;
2. the design decisions D4.1-D4.11 in `research/DESIGN_4_decisions.md`;
3. this document.

The background documents are the three drafts: `DESIGN_4_draft.md` (structure and milestones, cited as *draft*), `DESIGN_4_logic.md` (reasoning and learning, cited as *logic*) and `DESIGN_4_eval_data.md` (evaluation and data, cited as *eval*). They hold longer arguments, literature and code citations. Where a draft differs from this document, this document wins. Every disagreement between them is resolved in section 15.

Phase B's numeric thresholds are **set after the spike** and frozen in `research/PREREG_4.md` (P16). Everything else marked *decided* is fixed. The few points still open are listed in section 16.

4.0 is the fourth research iteration. Like 3.0, it is not planned as the last.

## 1. Summary

Three iterations kept finding the same split:
- learning and recall from correct facts work;
- reading English is the bottleneck;
- generalisation failed even without language.

In 3.0's fact-only experiment X4, substitution by function found 19% of the needed changes, case transfer 19% and induced rules 1.4%. The cause was not the logic. The knowledge base held procedures only, with no relations about things, and there was no general inference over relations.

4.0 therefore tests the part that has never been tested:

> **Taught only through conversation, the agent derives correct answers to multi-hop questions in domains it was not taught directly. Every answer carries its proof, or, for an induced rule, its supporting examples. It makes no claim without one, and it gets better as it is taught more.**

**Generalisation means creating new knowledge.** Three kinds are kept apart throughout:

| Kind | What it is | Allowed |
|---|---|---|
| **Deduced** | Follows from taught facts by the base rules or by accepted learned rules | Yes, shown with its proof |
| **Induced** | A rule lifted from several taught examples, going beyond what strictly follows | Yes, as a **hypothesis**, shown with its supporting examples and confidence; specialised or retracted on a counterexample |
| **Invented** | A claim with no proof and no supporting examples | **Never** |

"Invents nothing" always means the third kind only: no claim without a reason.

What changes from 3.0:

1. **Language leaves the core path (P4, P5).** Haiku teaches in a formal language; the agent answers in it, with a proof for every derived claim.
2. **Knowledge about things (D4.4).** Eleven typed relations sit beside the v3 procedure schema, bridged to it by `role` and a new `demands`.
3. **Small built-in logic, learned knowledge (P2, P6, P7).** Nine domain-free base rules B1-B9 (D4.6). Every fact is taught. New rules are induced layer on layer, confirmed by one question, specialised or retracted (D4.7).
4. **Exceptions by specificity (D4.5).** The item beats the nearer kind, which beats the farther kind, which beats a learned rule. Incomparable conflicts become questions.
5. **A three-layer, replayable knowledge base (P11).** Raw messages, facts and provenance, rebuilt from raw by zero-call replay.
6. **A derivation benchmark (P15, D4.8, D4.9).** 900 questions (300 dev, 600 sealed) whose answers and proof steps are never taught. A delivered leak invalidates the run.
7. **A learning curve as the main result,** at sizes S, M and, only if scaling pays, L (D4.10).
8. **Spike first (P16, D4.11),** on a sub-agent-written fixture reviewed by the user (D4.3), with a fresh 5,000-call budget after it (D4.1).

## 2. What 3.0 showed

| Iteration | What worked | What failed | Consequence for 4.0 |
|---|---|---|---|
| v1 | Facts, rules and proofs in a small Datalog reasoner | Parsing conversational English | The reasoner stays; English leaves the core path |
| 2.0 | Procedures recalled 75-90% on a second encounter | 30% of steps stored wrong; 225 hard-coded English strings; transfer 6/15 by item swap | No English in the core; transfer from knowledge |
| 3.0 | Exact fact-level check; sealed set with baselines; no word lists; correct facts recalled exactly | No frozen target met (H3: new task 25%, new domain 21%, new situation 8%); X4 19% / 19% / 1.4%; naming drift | Relations about things plus general inference; naming as knowledge; thresholds from data |

Four findings shape this design (draft §2): most variants need knowledge that is in no other task (X4), so 4.0 teaches relations about items; rules almost never fired because whole situations rarely repeat, so 4.0 induces over item relations, which do repeat; naming was the largest single source of failure (a naming fix in the measure alone moved the second encounter from 16 to 29 of 71), so naming becomes part of the conversation; and targets set before any build were unpassable (M24, M25), so thresholds come after the spike.

**The existing reasoner is most of what 4.0 needs** (`crates/reasoner`; capabilities and limits with line citations in draft §2). It has stratified negation, strong negation as `-p`, proof trees with checked-absent atoms, confidence, and a quarantine for learned rules. Its limits shape the design: bottom-up only, no depth limit, first derivation only, first-argument index only, no negation through recursion. 3.0's agent did not use it; 4.0 puts it back at the centre and reuses the 2.0 learner's support counting and corrections (`crates/learner`).

## 3. Principles

1. **Generalisation and learning first (P1).** When a choice trades polish, speed or coverage against the agent's ability to create new knowledge, creating knowledge wins.
2. **Everything is taught in conversation (P2).** No fact enters the knowledge base except as a message processed by the agent's own conversation path: schema check, naming check, consistency check, provenance, derivation, learning, gap questions. There is no injection, no imported resource and no hand-written knowledge base. The exceptions are:
   - the base rules and the schema, which are the system's logic (P6);
   - the spike fixture, which is a test fixture used only in the spike and delivered through the same conversation path (D4.3).
3. **Built-in logic is universal and small (P6).** Base rules name schema relations and sorts only: no item, material, function or domain symbol. A test enforces this.
4. **Every claim has a reason (P5).** A deduced claim carries a proof tree down to raw messages. An induced claim carries its rule, the rule's support set and its confidence, and is marked `hypothesis`. A claim with neither is not made: the agent writes a `gap` and asks.
5. **Taught beats inherited beats learned (D4.5).** Learned rules never override teaching.
6. **Statistical parts propose, symbolic parts decide** (from 3.0): similarity proposes `same_as?` questions and induction proposes rules; the teacher, the validator and the reasoner decide.
7. **Every model call is reused,** cached, archived and never deleted, inside the 5,000-call ledger (D4.1); no call-spending run starts without the user's go-ahead.
8. **Measure, then set thresholds (P16);** only the spike gate is fixed in advance (D4.11). The sealed benchmark is read at most 3 times, as counts only (D4.9).
9. **Household domains only (P13).** The long-term vision is context, not scope.

## 4. Knowledge model

### 4.1 Relations (decided, D4.4)

Eleven item relations, all subject-first, which suits the reasoner's first-argument index:

| # | Relation | Sorts | Inherits down `is_a` | Example |
|---|---|---|---|---|
| 1 | `is_a(X, K)` | item × item (also action × action) | transitive, strict | `is_a(muffin_tray, baking_tin)` |
| 2 | `same_as(X, Y)` | any × same sort | by canonicalisation (4.5) | `same_as(egg_whisk, whisk)` |
| 3 | `has_part(X, P)` | item × item | yes | `has_part(mixer_tap, cartridge)` |
| 4 | `made_of(X, M)` | item × material | yes | `made_of(tin, metal)` |
| 5 | `has_property(X, P)` | item × property | yes | `has_property(lemon, acidic)` |
| 6 | `can_do(X, F)` | item × function | yes | `can_do(fork, beat)` (an affordance) |
| 7 | `can_do_on(X, F, O)` | item × function × item | yes | `can_do_on(baking_tin, hold, batter)` |
| 8 | `used_for(X, F)` | item × function | yes | `used_for(whisk, beat)` (designed function) |
| 9 | `can_go_in(X, P)` | item × place | yes | `can_go_in(tin, oven)` |
| 10 | `effect(A, O, S)` | action × item × state | through O | `effect(microwave, water, hot)` |
| 11 | `precondition(A, O, S)` | action × item × state | through O | `precondition(whip, egg_white, separated)` |

**Negation.** Every relation can be taught negated, `not can_go_in(plastic_tub, oven).` It is stored as the strong-negation predicate `-can_go_in`. A negated `same_as` records "different names, different things", so the agent never asks again.

**The procedure schema** stays as in v3 (`DESIGN_3_schema.md` §2-3.6). It covers:
- tasks and steps: `task`, `step`, `action(S, A)`, `object_of(S, O)`, `uses(S, I)`, `into`, `onto`, `from`;
- order and settings: `before`, `overlaps`, `optional`, `until`, `setting`, `amount`, `takes_time`;
- situations: `has`, `lacks`, `wants`, `excludes`, `time_limit`, `count`;
- differences: `swap`, `drop`, `add_step`, `change`.

Three bridges link procedures to item knowledge:

| Bridge | Predicate | Meaning |
|---|---|---|
| 1 | `role(S, I, F)` | Item I serves function F in step S. F has the sort `function`, shared with `can_do` |
| 2 | `demands(S, F, Rel, V)` (**new**) | Whatever fills function F in S must satisfy `Rel(filler, V)`. For example, `demands(bake_banana_bread_s4, hold, can_go_in, oven)` |
| 3 | `achieves(S, St)`, `requires_state(S, St)` | Step-level effect and precondition, as in v3. `action(S, A)`, `object_of(S, O)` and `effect(A, O, St)` derive `achieves(S, St)` |

**Why `demands`.** The muffin tray stands in for a loaf tin only because it can go in the oven; without `demands`, a plastic tub would qualify too. Deriving demands from the action would be domain knowledge, leaving them implicit would invent a claim, and checking every relation of the lacked item is far too strict. So `demands` is taught, and an induced rule may generalise it. The changes from the pre-planning's nine relations (`requires` split, `can_do_on` instead of compound functions, `has_part` instead of `part_of`) are argued in logic §1.2. A further relation is added only when the spike shows a needed derivation cannot be expressed.

### 4.2 Sorts

The sorts are:
- `item` (physical things and their kinds, including food), with `place` as a subsort;
- `material`, `property`, `function`, `state`, `action`;
- `step`, `task`, `agent`.

Sorts belong to the formal language (P6). They hold no domain content.

A symbol takes its sort from a declaration (`new(sym, sort).`) or its first use; a later use with another sort is rejected with a reason, and the agent asks. `function` and `action` are disjoint but may share a name (`beat`). Actions have their own small `is_a` hierarchy (`is_a(slice, cut)`). Typing rejects malformed facts at entry, restricts joins, and is the induction learner's language bias (logic §1.3).

### 4.3 Base rules B1-B9 (decided, D4.6)

The base rules are written once, as data, in `core/` rule files evaluated on `sla_reasoner`, where quarantine always keeps them. Each is tagged **L** (the logic of the representation) or **S** (the semantics of the procedure schema: domain-free, but a rule that could in principle be learned).

| Id | Tag | Rule |
|---|---|---|
| B1 | L | `is_a` is transitive (`isa`) |
| B2 | L | Default inheritance with specificity over the inheritable relations (4.4) |
| B3 | L | Contradiction: a fact and its negation, or a `conflict` from B2, raise a question; neither is ever answered |
| B4 | L | `same_as` is an equivalence that substitutes everywhere, **implemented by canonicalisation** (4.5) |
| B5 | L | `has_part` is transitive, evaluated on demand only (7.5) |
| B6 | S | Bridges: `role(S, I, F)` → `can_do(I, F)`; `used_for(X, F)` → `can_do(X, F)`; `can_do_on(X, F, _)` → `can_do(X, F)`. A taught negative beats them |
| B7 | S | Stand-in by function, with the `demands` check and an honest gap |
| B8 | S | Step replacement by effect |
| B9 | S | Gap: a lacked item with no stand-in and no replacement becomes `gap(S, I)`; the agent states it and asks |

**Material properties are not a base rule.** "A thing has the properties of its material" is left to induction, and whether induction finds it is a spike test. The rule "a part can do what its whole can do" was rejected outright: a pan can fry, but its handle cannot.

**Keeping a rule (D4.6).** Bk is kept only if switching it off costs answers and it harms none: unique contribution ≥ 1 question per 100 (questions whose every valid proof uses Bk) and harm = 0 (wrong answers or inventions whose proof uses Bk), on dev (logic §3.3). B2 is also compared against instance-only exceptions and monotone inheritance. The spike also switches B7 off and checks whether induction rediscovers it (logic §3.2, §4.3 example B); if it does, B7 may move from built-in to learned (section 16).

The stand-in, replacement and gap rules, in the engine's syntax (`holds`/`holdsn` are B2's outputs, 4.4):

```datalog
% ---- B6 bridges (stratified: negation only on taught neg and on B2's conflict)
holds(I, can_do, F) :- role(_, I, F), not neg(I, can_do, F), not conflict(I, can_do, F).
holds(X, can_do, F) :- holds(X, used_for, F), not neg(X, can_do, F).
holds(X, can_do, F) :- holds3(X, can_do_on, F, _), not neg(X, can_do, F).

% ---- B7 stand-in by function, with demands
cand(S, I, J, F)  :- lacks(user, I), role(S, I, F), has(user, J), holds(J, can_do, F), J != I.
unfit_known(S, J) :- cand(S, I, J, F), demands(S, F, P, Y), holdsn(J, P, Y).
fit_open(S, J)    :- cand(S, I, J, F), demands(S, F, P, Y), not holds(J, P, Y), not holdsn(J, P, Y).
stand_in(S, I, J) :- cand(S, I, J, F), not unfit_known(S, J), not fit_open(S, J).
ask_fit(S, J)     :- cand(S, I, J, F), not unfit_known(S, J), fit_open(S, J).
some_stand_in(S, I) :- stand_in(S, I, _).

% ---- B8 step replacement by effect
needed_state(S1, Z) :- before(S1, S2), requires_state(S2, Z), achieves(S1, Z).
blocked(S1)         :- uses(S1, I), lacks(user, I), not some_stand_in(S1, I).
replace_step(S1, A2) :- blocked(S1), needed_state(S1, Z), object_of(S1, O),
                        holds_effect(A2, O, Z), not lacks_for(A2, O).
replaced(S1) :- replace_step(S1, _).

% ---- B9 gap
gap(S, I) :- uses(S, I), lacks(user, I), not some_stand_in(S, I), not replaced(S).
```

Notes:
- **`unfit_known` against `fit_open`** keeps B7 honest: a proven "no" excludes the candidate, and an unknown demand produces `ask: can_go_in(muffin_tray, oven)?` instead of a swap. The answer closes that gap for every later task.
- `lacks_for(A2, O)` applies the same split to the action's own demands (`demands_action(A2, F, Rel, V)`).
- B6's role bridge concludes ability for the item that served, not for its subkinds.
- Ternary relations are reified as `pos3`/`neg3`/`holds3` with the same B2 template; `effect` and `precondition` inherit through their object.

### 4.4 Exceptions: most specific wins (decided, D4.5)

**Precedence.** For one atom `p(a, b)`, positive and negative conclusions are ranked by source:

| Rank | Source |
|---|---|
| T | Taught about the item itself |
| I(K) | Inherited from a taught fact on kind K; K₁ beats K₂ if `isa(K₁, K₂)` (the nearer kind wins) |
| L | Concluded by a learned rule, layer by layer |

**Three semantic rules decide between them:**
1. **Specificity.** A conclusion is defeated by a contrary one of strictly higher rank.
2. **Skeptical blocking.** If both polarities survive at incomparable ranks (a diamond), the atom is a `conflict` and **neither** holds. The agent asks the teacher.
3. **Hypotheses never override teaching.** A learned conclusion that meets a contrary T or I conclusion does not hold. The meeting is recorded as a counterexample to the rule, which starts specialisation (7.2). Between two learned rules, the specialisation beats the rule it came from. Otherwise they conflict, and the agent asks.

**The compiled program** (logic §2.4) was run on `sla_reasoner` as it stands. It reproduces the penguin case with a redundant link, the exception to an exception, the Nixon diamond and the muffin-tray example. Inheritable relations are stored reified as `pos(X, Rel, Y)` and `neg(X, Rel, Y)`. The reification is internal: the facts layer keeps plain relation lines.

```datalog
isa(X, K) :- is_a(X, K).
isa(X, K) :- is_a(X, M), isa(M, K).
inh(X, P, Y, K)  :- isa(X, K), pos(K, P, Y), inheritable(P).
inhn(X, P, Y, K) :- isa(X, K), neg(K, P, Y), inheritable(P).
% preempted: a contrary assertion on X itself, or on a kind more specific than K
pre(X, P, Y, K)  :- inh(X, P, Y, K), neg(X, P, Y).
pre(X, P, Y, K)  :- inh(X, P, Y, K), inhn(X, P, Y, K2), isa(K2, K).
pren(X, P, Y, K) :- inhn(X, P, Y, K), pos(X, P, Y).
pren(X, P, Y, K) :- inhn(X, P, Y, K), inh(X, P, Y, K2), isa(K2, K).
live(X, P, Y, K)  :- inh(X, P, Y, K), not pre(X, P, Y, K).
liven(X, P, Y, K) :- inhn(X, P, Y, K), not pren(X, P, Y, K).
conflict(X, P, Y) :- live(X, P, Y, K1), liven(X, P, Y, K2).
holds(X, P, Y)  :- pos(X, P, Y).
holds(X, P, Y)  :- live(X, P, Y, K), not conflict(X, P, Y), not neg(X, P, Y).
holdsn(X, P, Y) :- neg(X, P, Y).
holdsn(X, P, Y) :- liven(X, P, Y, K), not conflict(X, P, Y), not pos(X, P, Y).
% B3
contradiction(X, P, Y) :- pos(X, P, Y), neg(X, P, Y).
```

**Why it is stratified.** Defeat is decided by **taught** facts and their places in the `isa` order, never by derived conclusions, so every negation points to a lower stratum and the engine runs it unchanged. Redundant links change nothing, because specificity uses `isa`, not path length.

**Learned layers.** Each layer's conclusions become rank-L facts in their own predicates, and a copy of the B2 template runs over them, so a learned fact about a kind reaches its subkinds. The guard against teaching reads only lower strata, so the whole is a stratified sequence; quarantine refuses any learned rule that would create a cycle through negation.

**Withdrawal.** The agent compares the model before and after each message. A shown conclusion that is gone is reported as `withdrawn: <fact> because <fact>` and recorded as a provenance event. Nothing is deleted.

**When a taught fact contradicts a derived one** (logic §2.5, kernel contraction over the stored proof π): if π uses a learned rule, the rule yields (specialised or retracted, and reported). If π is taught facts plus inheritance, the agent asks, proposing "exception" first and listing π's taught leaves as alternatives to retract. An exception keeps both facts; a correction marks the general fact `retracted_by`. Only dependent strata are recomputed.

**Alternatives rejected** (logic §2.2): answer set programming (no proofs, outside solver), well-founded semantics over direct links (breaks on redundant links, which Haiku will teach), explicit `except` facts (duplicate negatives). Full defeasible logic is the upgrade path if the spike finds conflicts the layers cannot express.

### 4.5 Naming and `same_as` (B4)

Naming drift splits the support of every rule and breaks every chain through an item. The measures, from logic §7 and eval §2.5:

1. **Prevent at the source.** Curriculum stage 0 builds a symbol registry through conversation (9.1). Every prompt carries the topic's slice, and the benchmark uses registry symbols only.
2. **Detect and ask.** A new symbol is compared with known symbols of the same sort by a shared head token or edit distance ≤ 2, and by Jaccard similarity of relational neighbourhoods. No word list is needed. Above a threshold (set on the spike's drift test), the agent asks `same_as(egg_whisk, whisk)? or is_a(egg_whisk, whisk)?`, and the fact is pending, excluded from derivation, until answered.
3. **The answers are different knowledge.** `same_as` merges the names; `is_a` keeps a distinct symbol that inherits; `not same_as` records a different thing and the question is never asked again.
4. **Canonicalisation, never automatic merges.** Union-find over `same_as`, with the earliest-taught name as representative; facts are rewritten at entry; raw keeps the original text and the facts layer an alias table; a retracted `same_as` re-splits by replay. Axiomatising `same_as` cost 38% more facts and 1.8 times the time (Appendix B). The spike checks that both forms give the same answers.
5. **Guards.** A merge that would create a contradiction is refused and re-asked with the conflict shown; `same_as` across sorts is rejected.

### 4.6 Confidence

| Source | Confidence |
|---|---|
| Taught by Haiku (live or archive3) | 0.9; 0.97 when independently confirmed (another session, or the answer to the agent's own question) |
| Spike fixture | 1.0 |
| Base rule | 1.0 |
| Learned rule | s / (s + c + 1) from support s and unhandled counterexamples c (as in `crates/learner`); the Wilson 95% lower bound is reported beside it |
| Derived | the minimum over taught leaves × the product of the learned-rule confidences on the path |

The taught values are starting values, checked against the milestone 34 audit. The engine today takes the minimum everywhere; the change is to multiply learned-rule confidences only, since two 0.6 rules in a chain are weaker than one. A conclusion below 0.5 is offered as a question, not an answer (starting value).

## 5. Architecture

```
 teacher (Haiku, formal messages; or archive3 / fixture replay)   user / benchmark question
        │  one call = one teacher turn of many statements                │
        ▼                                                                ▼
 ┌──────────────── conversation path (agent4) ────────────────────────────────┐
 │ 0 leak filter (teach4): denied statements withheld before delivery          │
 │ 1 parse + schema check: predicate, arity, sorts            ─reject──► "reject: …"
 │ 2 naming check: new symbol close to a known one?           ─ask─────► "ask: same_as(a, b)?"
 │ 3 consistency check: vs taught / inherited / learned        ─ask─────► "conflict: … exception?"
 │ 4 store: raw (always, first) → facts → provenance                           │
 │ 5 derive: recompute (closure ≤ 5,000 facts; demand slice above)             │ "ok. derived: …"
 │ 6 learn (end of batch): hypotheses, confirmations, specialisations          │ "hypothesis: …"
 │ 7 gaps: near-miss derivations → ranked questions (≤ 20% share)              │ "ask: …?"
 └─────────────────────────────────────────────────────────────────────────────┘
        │                                         │
        ▼                                         ▼
  kb4: raw/ builds/{facts.dl, rules.dl,    reason4: B1-B9 + learned layers on sla_reasoner;
       vocab.jsonl, provenance.jsonl}       proof records; self-check
        ▲ replay (zero calls, any agent version)  │
        └─────────────────────────────────────────┘
                                                  ▼
                              answer = formal lines + proof / support / gap
                                                  │
                     eval4 + tools/design4/ref_reason.py: scorer, independent proof
                     checker, KB-relative gold, leak audit, ablations, curve (eval only)
```

### 5.1 Crates and tools

The 2.0 and 3.0 crates stay unchanged, so their results can still be replayed. `reasoner` becomes the core engine, unchanged. `learner` is extended by `learn4`, not edited. 4.0 adds:

| Crate | Content |
|---|---|
| `kb4` | Schema v4 (relations, sorts, `inheritable` and transitive declarations); the message grammar; the three layers with readers, writers and schema versions; the registry and alias table; the deny-list format |
| `reason4` | B1-B9 rule files and their expansion; learned layers and their strata; question slicing; proof records and rendering; a runtime self-check that withholds any answer whose proof fails |
| `learn4` | Item-rule refinement, case lgg, specialisation, retraction, confirming questions, the layer record with `built_from` |
| `agent4` | The conversation path, the question queue, answers |
| `teach4` | The Haiku teacher channel: stage prompts, batching, the archive3 and fixture replay channels, the deny filter. Cache and ledger in `data/training/calls4/`, reusing `teach3`'s key, fail-fast behaviour and locked appends |
| `eval4` | Benchmark loader and scorer, replay runner, ablations, curve runner, statistics (Wilson, McNemar, cluster bootstrap), lab events on 3.0's stream |

Offline Python in `tools/design4/` holds `ref_reason.py` (the independent reference reasoner and **proof checker of record**), `leak_check.py`, `check_bench.py`, `import_archive3.py`, `haiku.py` and the versioned stage prompts. Only Rust runs at agent runtime.

### 5.2 Tests that hold the principles

- **No domain symbols** in base rules or agent crates (extends `crates/kb3/tests/no_word_lists.rs`).
- **No injection.** The facts layer has one writer, the conversation path; every live fact's provenance points to a raw message.
- **Proof-checker negative controls**, each of which must fail: a leaf that is not a live delivered fact; a rule instance that does not unify; a derivable `checked_absent` atom; a retracted leaf; a step through a defeated inheritance; a learned rule below threshold or in quarantine; a hypothesis without its marker.
- **Leak audit** over every delivered raw record (10.5).
- **Replay is exact:** same raw, build and flags give byte-identical outputs.
- **Readings never teach:** the facts-layer hash is unchanged across every reading.

## 6. Teaching protocol

### 6.1 Message grammar

Messages are lines in the formal language, parsed by `kb4`. 3.0's fact parser is extended with `not` and the keywords below, all of which are formal vocabulary.

**Teacher to agent:**

| Form | Meaning |
|---|---|
| `topic <domain>/<unit>.` | Starts a teaching batch |
| `new(sym, sort).` | Declares a new symbol |
| `R(a, b).` / `not R(a, b).` | Teaches a fact or a negative fact |
| v3 procedure facts | Teach a procedure (step ids `<task>_s<N>`) |
| `answer N: <statement or yes/no/exception/unknown>.` | Answers the agent's question N |
| `retract R(a, b).` | Withdraws an earlier teaching |

**User or benchmark to agent:**

| Form | Meaning |
|---|---|
| `query: <goal>. given: <facts>.` | A relation or open question. `given:` facts (a situation, or a procedure fragment for an untaught domain) are used for this answer only and never stored |
| `request(<task>). <situation facts>.` | A help request, answered with procedure lines plus differences |

**Agent to teacher or user:**

| Form | Meaning |
|---|---|
| `ok.` / `ok. derived: <fact> [<premises>]` | Accepted; shows up to 3 new consequences |
| `reject: <reason>.` | Schema or sort violation; kept in raw only |
| `ask: same_as(a, b)? or is_a(a, b)?` | Naming check |
| `conflict: <fact> vs <fact> [<proof>]. exception?` | Consistency check |
| `hypothesis: <rule> support [..] counter [] conf c.` then `ask: <prediction>?` | A new learned rule and its confirming question |
| `specialised:` / `retracted:` / `withdrawn:` | Revisions |
| `ask: <pattern>?` | A gap question |
| `answer:` block | Answer lines, then `proof:`, `support:` or `gap:` |

### 6.2 What the agent does with each message

Each message goes through these steps, in order; direct injection would skip all of them (P2).

1. **Parse and schema check** (predicate, arity, sorts, lower case). A failure gives `reject:` and stays in raw only.
2. **Naming check** (4.5).
3. **Consistency check.** A taught negation: ask which is right. An inherited negation: ask "exception?" (4.4). A negation from a learned rule: no question; the learned conclusion is withdrawn and the rule meets a counterexample (7.2).
4. **Store:** raw first, always; facts if accepted; provenance (sender, channel, session, turn, call key, confidence, status).
5. **Derive:** recompute; show new and withdrawn consequences with premises.
6. **Learn:** an induction pass at the end of each batch.
7. **Gaps:** near-miss derivations become ranked questions (6.4).

### 6.3 Worked examples

Message ids are `<session>.<turn>`. Proofs are shown in the collapsed rendering (7.4).

**E1. Taught facts and what follows (1 hop, deduced).**

```
TEACHER: topic baking/tins.
TEACHER: is_a(loaf_tin, baking_tin).
TEACHER: can_do_on(baking_tin, hold, batter).
AGENT:   ok. derived: can_do_on(loaf_tin, hold, batter)  [is_a(loaf_tin, baking_tin), can_do_on(baking_tin, hold, batter)]
TEACHER: can_go_in(baking_tin, oven).
TEACHER: is_a(muffin_tray, baking_tin).
AGENT:   ok. derived: can_do_on(muffin_tray, hold, batter) [...]
         derived: can_go_in(muffin_tray, oven) [is_a(muffin_tray, baking_tin), can_go_in(baking_tin, oven)]
```

**E2. Naming drift.**

```
TEACHER: can_do(egg_whisk, beat).
AGENT:   ask: same_as(egg_whisk, whisk)? or is_a(egg_whisk, whisk)?   % head token, shared can_do(beat)
TEACHER: answer 1: same_as(egg_whisk, whisk).
AGENT:   ok. egg_whisk is written whisk from now on.
TEACHER: is_a(balloon_whisk, whisk).
AGENT:   ok.                                  % the relation names it; no question
TEACHER: can_do(whisk_attachment, beat).
AGENT:   ask: same_as(whisk_attachment, whisk)?
TEACHER: answer 2: not same_as(whisk_attachment, whisk). has_part(stand_mixer, whisk_attachment).
AGENT:   ok.
```

**E3. An exception by specificity, then a diamond.**

```
TEACHER: topic cooking/dishes.
TEACHER: can_go_in(dish, oven).  is_a(glass_dish, dish).  is_a(plastic_dish, dish).
AGENT:   ok. derived: can_go_in(glass_dish, oven) [...], can_go_in(plastic_dish, oven) [...]
TEACHER: not can_go_in(plastic_dish, oven).
AGENT:   conflict: not can_go_in(plastic_dish, oven) vs can_go_in(plastic_dish, oven)
           [is_a(plastic_dish, dish) s0007.3, can_go_in(dish, oven) s0007.1].
           exception? (or is is_a(plastic_dish, dish) wrong?)
TEACHER: answer 1: exception.
AGENT:   ok. withdrawn: can_go_in(plastic_dish, oven) because not can_go_in(plastic_dish, oven).
TEACHER: is_a(takeaway_box, plastic_dish).
AGENT:   ok. derived: not can_go_in(takeaway_box, oven) [is_a(takeaway_box, plastic_dish), not can_go_in(plastic_dish, oven)]
TEACHER: is_a(silicone_mould, baking_tin).  is_a(silicone_mould, rubber_thing).
         not can_go_in(rubber_thing, oven).
AGENT:   conflict: can_go_in(silicone_mould, oven) [via baking_tin] vs not [via rubber_thing];
           neither kind is more specific. ask: can_go_in(silicone_mould, oven)?
```

`takeaway_box` has two decisions, at `plastic_dish` (no) and at `dish` (yes). `plastic_dish` is nearer, so the positive is preempted. For `silicone_mould`, nothing is claimed until the teacher answers.

**E4. A stand-in request with demands, and a gap question first (3 hops).**

The procedure `bake_banana_bread` was taught earlier, by archive3 replay. Step 4 is:
- `action(…_s4, pour)`, `object_of(…_s4, batter)`;
- `uses(…_s4, loaf_tin)`, `role(…_s4, loaf_tin, hold)`;
- `demands(…_s4, hold, can_go_in, oven)`.

```
USER:    request(bake_banana_bread). lacks(user, loaf_tin). has(user, muffin_tray). has(user, plastic_tub).
AGENT:   answer:
         task bake_banana_bread
         swap 4 loaf_tin muffin_tray
         proof:
         stand_in(bake_banana_bread_s4, loaf_tin, muffin_tray)   B7  [conf 0.9]
           role(bake_banana_bread_s4, loaf_tin, hold)            taught a3-bake_banana_bread.4
           can_do(muffin_tray, hold)   B6 from can_do_on(muffin_tray, hold, batter)
             inherited from baking_tin: is_a(muffin_tray, baking_tin) [s0042.7];
             can_do_on(baking_tin, hold, batter) [s0041.2]; no exception known for muffin_tray or kinds between
           demands met: can_go_in(muffin_tray, oven)
             inherited from baking_tin [s0042.7, s0041.4]; no exception known
         excluded: plastic_tub   unfit_known: not can_go_in(plastic_tub, oven) [s0130.2]
         hops: 3 (inherit can_do_on, inherit can_go_in, B7)
```

Had `can_go_in(baking_tin, oven)` never been taught, the muffin tray would be `fit_open`. The agent then asks before answering:

```
AGENT:   gap: demand can_go_in(muffin_tray, oven) unknown for swap 4 loaf_tin muffin_tray
         ask: can_go_in(muffin_tray, oven)?
```

The answer does not change the baking time, because no taught fact links time to the tin (6.5).

**E5. Cross-domain (3 hops).** In cleaning, `role(descale_kettle_s2, vinegar, dissolve_limescale)` was taught, together with `can_do(acid, dissolve_limescale)`. In cooking, `is_a(lemon_juice, citrus_juice)` and `is_a(citrus_juice, acid)` were taught. The request `request(descale_kettle). lacks(user, vinegar). has(user, lemon_juice).` gives:
- `swap 2 vinegar lemon_juice`;
- proof: inheritance down two `is_a` links (2 hops), then B7 (1 hop).

No message ever mentions lemon juice and limescale together.

**E6. An induced rule: proposed, confirmed, used, then specialised.**

```
TEACHER: topic cooking/containers.  made_of(plastic_tub, plastic).    not can_go_in(plastic_tub, oven).
TEACHER: topic baking/tools.        made_of(plastic_jug, plastic).    not can_go_in(plastic_jug, oven).
TEACHER: topic cleaning/tools.      made_of(plastic_bucket, plastic). not can_go_in(plastic_bucket, oven).
                                    made_of(spray_bottle, plastic).  made_of(food_bag, plastic).
AGENT:   hypothesis: not can_go_in(X, oven) :- made_of(X, plastic).
           support [plastic_tub, plastic_jug, plastic_bucket] (3 tasks/kinds, 3 domains) counter [] conf 0.75 id L1/0004
         ask: can_go_in(spray_bottle, oven)?                    % the one confirming question
TEACHER: answer 1: not can_go_in(spray_bottle, oven).
AGENT:   ok. L1/0004 confirmed_by s0150.9; support 4.
USER:    query: can_go_in(food_bag, oven).
AGENT:   answer: no (hypothesis, L1/0004, support 4, conf 0.8)
           premise: made_of(food_bag, plastic) [s0150.7]
TEACHER: topic baking/oven.  made_of(roasting_bag, plastic).  can_go_in(roasting_bag, oven).
AGENT:   ok. L1/0004 meets a counterexample: roasting_bag.
TEACHER: has_property(roasting_bag, heat_resistant).
AGENT:   specialised: L1/0004 → L1/0004a
           not can_go_in(X, oven) :- made_of(X, plastic), not has_property(X, heat_resistant).
           support 4, counter [] (roasting_bag excluded by the new condition)
```

If no feature separated the bag from the support set, the rule would be retracted, its conclusions withdrawn, and the agent would ask `ask: what differs roasting_bag from {plastic_tub, plastic_jug, plastic_bucket}?`.

**E7. An honest gap.**

```
USER:    request(descale_kettle). lacks(user, vinegar). has(user, baking_soda).
AGENT:   answer:
         task descale_kettle
         gap lacks vinegar in step 2: no known stand-in; can_do(baking_soda, dissolve_limescale) unknown
         ask: can_do(baking_soda, dissolve_limescale)?
```

Further examples are in draft §6.4, logic §2.4 (engine outputs verbatim) and eval §5.4 (benchmark questions with gold proofs).

### 6.4 Active learning: which gaps become questions

The question budget is limited (9.3), so candidates are ranked:
1. conflicts and contradictions (always asked; they block answers);
2. naming (always asked; they block the new symbol);
3. the confirming question of each new learned rule (D4.7);
4. near-miss stand-ins (`ask_fit` with one open demand);
5. unresolved diamonds;
6. typing (a symbol with no `is_a`);
7. thin functions ("what else can_do beat?");
8. missing common properties, such as two rule specialisations differing only in a material. Asking for the shared property is how a new layer arises without inventing one (logic §4.3, example A).

A question matching a denied atom is dropped silently by the harness (10.5).

### 6.5 What stays out of an answer

No slot is filled without a proof: times, settings, amounts and order change only when a taught fact or rule derives the change. Answers carry no English; a gloss for humans may follow and is never scored.

## 7. Reasoning and learning

### 7.1 Rule learning (decided, D4.7)

There are two kinds of target, both learned in Rust (logic §4.1-4.3):

| Target | Head | Examples | Method |
|---|---|---|---|
| **Item rules** | an item relation (`can_go_in`, `can_do`, `has_property`, …) | taught positives and negatives of that relation | Part A: bounded top-down refinement (an AMIE/FOIL hybrid) |
| **Adaptation rules** | `swap`, `replace_step`, `drop` | taught cases (situation plus difference) and the agent's answers that the teacher confirmed | Part B: least general generalisation over cut bottom clauses (depth ≤ 2) |

Item rules are the main win. They do not depend on whole situations repeating, which is what killed 3.0's rules arm.

**Candidate language.** Bodies of 1-2 literals over `holds`/`holdsn` features of the subject (specialisation may add up to 2 more); variables typed by sort, each appearing twice; a constant only where it occurs in at least 3 positives.

**Counts** for body B and head H: **support s**, the distinct subjects with B and H taught or inherited (never learned); **counterexamples c**, subjects with B and the contrary of H; **new predictions u**, subjects with B and H unknown.

**Acceptance as a hypothesis** (D4.7): s ≥ 3; the supporting subjects come from at least 2 distinct tasks or kinds (`is_a` parents), read from provenance; no unhandled counterexample; u ≥ 1; minimality (no shorter accepted body with an equal or larger support set); no self-support (evidence for layer n uses only base rules and layers below n). The rule then passes quarantine and gets an id `L<layer>/<n>` and confidence s/(s+c+1).

**One confirming question per new rule** (D4.7): a membership query about the rule's most surprising prediction not on the deny list (E6). "Yes" adds support and records `confirmed_by`; "no" is a counterexample (7.2). Until answered, conclusions are shown as unconfirmed hypotheses. A rule that would add more than 5 times its support in new conclusions (starting value) waits for the answer before any is used. These questions ride in teaching calls, within the 20% share.

The spike tests k = 2, 3, 4 and the body length. The Wilson bound is reported, not gated: 3 of 3 gives only 0.44, so a Wilson gate would contradict D4.7. Any confidence threshold for dropping the `hypothesis` marker is set after the spike.

### 7.2 Counterexamples: specialise or retract

Following the 2.0 corrections and logic §4.4:

1. A counterexample is a taught or inherited fact contrary to a hypothesis's conclusion for some x′; that conclusion is withdrawn at once.
2. **Specialise:** add a literal that all supporting subjects share and x′ lacks (preferred), or `not` a literal only x′ has. The new rule r′ must pass acceptance; it records `specialised_from(r, x′)` and beats r, which stays in provenance as `specialised`.
3. **Instance exceptions generalise.** If nothing separates x′, an instance exception `X != x′` may be added ("known exception: x′"). When a second arrives, step 2 runs over the exceptions themselves: lgg(plastic_tub, plastic_jug) gives `made_of(X, plastic)`, so an exception list becomes a learned literal.
4. **Retract** when no specialisation passes: status `retracted`, conclusions withdrawn, and the agent asks the teacher for the difference. A feature taught later can bring a specialised rule back.

### 7.3 Layers and `built_from`

Layer 0 is the base rules. A learned rule's layer is 1 + the highest layer of any learned rule whose conclusions appear in its body or support, and each layer is a stratum above those it reads. Provenance records `built_from` for every rule. A three-layer chain (logic §4.6): layer 1 `can_go_in(X, oven) :- made_of(X, metal)`; layer 2 `can_do_on(X, hold, batter) :- is_a(X, tray), can_go_in(X, oven)`; then B7 lets `steel_tray`, taught only as metal and a tray, stand in for a loaf tin. The rule layer is reported as P7 asks: rules by layer with `built_from` edges, support, confidence, confirmations, counterexamples, specialisations and retractions.

**Composition** adds speed, not knowledge. It is built only if the spike or milestone 38 shows that answer time needs it, and it never counts as learned knowledge. **Popper and FOLD-R++** run offline on exported spike tasks as an optional yardstick; neither is imported into the agent.

### 7.4 Proofs

Every answer has one JSON proof record (logic §5.1):

```json
{"claim": "stand_in(bake_banana_bread_s4, loaf_tin, muffin_tray)", "kind": "deduced",
 "conf": 0.9, "kb_build": "b-…-S100", "rules_used": ["B7", "B6", "B2", "B1"],
 "tree": {"atom": "stand_in(bake_banana_bread_s4, loaf_tin, muffin_tray)", "rule": "B7",
          "premises": [{"atom": "role(bake_banana_bread_s4, loaf_tin, hold)", "given": "raw:a3-bake_banana_bread-04#3"},
                       {"atom": "holds(muffin_tray, can_do, hold)", "rule": "B6", "premises": [ … ]},
                       {"atom": "holds(muffin_tray, can_go_in, oven)", "rule": "B2", "via_kind": "baking_tin",
                        "checked_absent": ["no contrary on muffin_tray or kinds between it and baking_tin"]}],
          "checked_absent": ["not unfit_known(…)", "not fit_open(…)"]}}
```

`kind` is `given`, `deduced` or `hypothesis`; leaves carry raw message ids; a learned-rule node also carries `layer`, `built_from`, `support`, `support_msgs`, `counterexamples`, `confirmed_by` and `conf`. The rendering collapses each inheritance to one line (E4) and keeps the "no exception known" line, because it is the defeasible assumption.

**Hops (decided here).** The number of rule-application nodes in the minimal valid proof: an inheritance down n `is_a` links counts n; B5, B6's `role` bridge, B7, B8 and each learned rule count 1; reification, the definitional bridges (`used_for`/`can_do_on` → `can_do`), helper clauses and leaves count 0. Benchmark labels are computed by `ref_reason.py` from gold proofs with base rules only; a learned rule may shorten a proof but never changes the label. **Hop limit** (starting value 4): an answer whose minimal proof is longer becomes a gap, `beyond_hop_limit`.

**First proof, not best proof.** Semi-naive evaluation keeps the first derivation, which has minimal height within a stratum; learned layers run after the base strata, so a hypothesis never shadows a deduction. The spike reports how often the kept proof is longer than gold; if it matters, `reason4` adds a minimal-proof search.

**Proof checking.** The checker of record is `ref_reason.py`, a Python implementation of the frozen base rules written from this document, not from `reason4`'s code. It checks that each leaf is a live delivered statement or a `given` premise, each node is an instance of its rule, each `checked_absent` holds, no step uses a defeated inheritance, and each learned rule meets acceptance at that build. `reason4` also self-checks every proof and withholds a failing answer as a gap; the two must agree on the whole spike. Independent checking makes "0 inventions" a measured claim.

### 7.5 Scaling

Measured on the existing engine (Appendix B): full closure takes 13 ms at about 500 facts, 268 ms at 5,000 and 3.9 s / 922 MB at 50,000, where a demand slice takes 253 ms / 120 MB. Decided:

| KB size | Mode |
|---|---|
| Spike, S (about 1,500), M (about 4,000) | Forward closure after each message (≤ 0.3 s), with derivations shown in the dialogue |
| L (about 10,000), and every benchmark question at any size | Per-question **demand slice**: the question's and situation's constants, closed upward over `isa`, canonical names and `has_part` to depth d (body length × layers). Then evaluation over facts about those constants only, plus rules pruned by `depends_on` |
| Before L | Interned symbols and premises stored as fact ids (an estimated 5-10 times less memory) |

`has_part`, `before` and `same_as` are never materialised by rules. Incremental maintenance is added only if a recompute per message exceeds about 1 s; magic sets are the fallback if hand slicing proves wrong. The spike checks that slice answers equal closure answers.

## 8. Knowledge base layers and replay

### 8.1 Layout (P11)

```
data/kb4/
  raw/
    sessions/<session_id>.jsonl   append-only, one record per message, every speaker
    imports/<source>.jsonl        import manifests (archive3), never edited
    INDEX.jsonl                   per session: id, channel, stage, block, first/last unix, sha256
  builds/<build_id>/
    facts.dl                      taught facts, negatives, accepted exceptions (Datalog text)
    rules.dl                      learned rules with id, layer, status (Datalog text with % headers)
    vocab.jsonl                   the registry and alias table
    provenance.jsonl              one record per fact, rule, exception, retraction, withdrawal
    BUILD.json                    agent commit, schema versions, raw prefix, base-rule hash, counts
  migrations/<from>-<to>.dl       relation renames or splits, as data, applied in replay by `lang`
  MANIFEST.json
data/spike4/                      the spike fixture and its own raw layer (never merged into data/kb4)
```

Only stored knowledge goes into a build. Derived facts are recomputed, never stored. The agent's `derived:` lines are kept in raw as conversation.

### 8.2 Records

**Raw** (eval §4.1). One record per message. A teacher turn holds many statements, each with its own delivery flag:

```json
{"rec":"raw","schema":"raw4/1","lang":"formal4/1","msg_id":"s0007-t012","session":"s0007","turn":12,
 "unix":1795000000,"speaker":"teacher","sender":"haiku","channel":"live","stage":3,"block":2,
 "topic":"baking/tins","in_reply_to":"s0007-t011",
 "text":"answer 1: same_as(egg_whisk, whisk).\nis_a(muffin_tray, baking_tin).\ncan_do_on(muffin_tray, hold, batter).",
 "statements":[{"i":0,"atom":"same_as(egg_whisk, whisk)","answers":"s0007-t011#q1","delivered":true},
               {"i":1,"atom":"is_a(muffin_tray, baking_tin)","delivered":true},
               {"i":2,"atom":"can_do_on(muffin_tray, hold, batter)","delivered":false,"quarantine":"deny"}],
 "call":{"key":"3f9a…","ledger":"calls4","prompt_version":"stage3_v1"}}
{"rec":"raw","schema":"raw4/1","lang":"formal4/1","msg_id":"s0007-t013","session":"s0007","turn":13,
 "speaker":"agent","agent_build":"a4-9c1e2d0",
 "text":"ok. derived: …\nask: is_a(plastic_tub, X)?",
 "asks":[{"q":"q1","canonical":"is_a(plastic_tub, _)","priority":"typing"}]}
{"rec":"raw","schema":"raw4/1","lang":"v3facts","msg_id":"a3-bake_potato-03","session":"a3-bake_potato",
 "turn":3,"speaker":"teacher","sender":"haiku","channel":"archive3","stage":6,
 "text":"step(bake_potato, bake_potato_s3)\naction(bake_potato_s3, prick)\nuses(bake_potato_s3, fork)\nrole(bake_potato_s3, fork, pierce)",
 "source":{"file":"data/training/calls3/cache.jsonl","key":"<cache key>","tag":"parse_fallback"}}
```

Raw records are never edited or deleted. A correction is a new message (`retract`, or a human annotation pointing to a `msg_id`). Quarantined statements are kept for audit and never replayed. `lang` names the grammar version, so old messages are read with their own grammar.

**Facts and rules** (engine syntax; `%` comments are skipped by `parse_program`):

```datalog
% schema facts4/1  build b-2026-12-02-S100  agent a4-9c1e2d0  raw 5,412 msgs (prefix 100%)  base B1-B9 sha256:7be1…
is_a(muffin_tray, baking_tin).            % f:0001a2
-can_go_in(plastic_dish, oven).           % f:000402 exception_of f:000398
same_as(egg_whisk, whisk).                % f:000120 canonical whisk
role(bake_potato_s3, fork, pierce).       % f:0020b5
```

```datalog
% L1/0004a  layer 1  status hypothesis  conf 0.8  confirmed_by s0150.9
hypn_1(X, can_go_in, oven) :- holds(X, made_of, plastic), not holds(X, has_property, heat_resistant).
```

The agent shows learned rules in relation form (E6); `rules.dl` holds their compiled layer form.

**Provenance** (extends 3.0's provenance fields):

```json
{"rec":"prov","schema":"prov4/1","id":"f:0001a2","kind":"fact","atom":"is_a(muffin_tray, baking_tin)",
 "source":{"msg_id":"s0007-t012","i":1},"sender":"haiku","channel":"live","conf":0.9,
 "confirmed_by":[],"status":"live","names":{"muffin_tray":"muffin_tray"},"retracted_by":null}
{"rec":"prov","schema":"prov4/1","id":"L1/0004a","kind":"rule","layer":1,"built_from":["B2","L1/0004"],
 "support":["plastic_tub","plastic_jug","plastic_bucket","spray_bottle"],"support_msgs":["s0130-t002#1","…"],
 "domains":["cooking","baking","cleaning"],"counter":[],"confirmed_by":"s0150-t009#0","conf":0.8,"status":"hypothesis",
 "history":[{"msg":"s0170-t002","event":"counterexample","x":"roasting_bag"},
            {"msg":"s0170-t003","event":"specialised","added":"not holds(X, has_property, heat_resistant)"}]}
{"rec":"prov","schema":"prov4/1","id":"e:000017","kind":"event","event":"withdrawn",
 "fact":"can_go_in(plastic_dish, oven)","because":"f:000402","msg":"s0007-t005"}
```

### 8.3 Versioning and replay

Every record carries `schema` (`<layer><major>/<minor>`; a minor version adds optional fields, a major one needs a new reader), and raw also carries `lang`. Raw is never migrated: replay reads every raw version ever written, tested on one frozen session per version. A build is a function of (raw prefix, agent build, base-rule hash), recorded in `BUILD.json`; a schema change means a rebuild by replay, with renames from `migrations/`.

```
sla4 replay --raw data/kb4/raw --agent <build> [--upto 25%|<msg_id>] [--blocks 1,3,2,4]
            [--channels live,archive3] [--answers raw|none|live] --out data/kb4/builds/<id>
```

Every delivered statement is fed, in order, through the live entry point; there is no bulk-load path. Questions a new agent asks that are not in raw are answered per `--answers`: `raw` (default) only from an identical canonical question answered at or before that point; `none` not at all; `live` by Haiku, counted, with the answer appended to raw as a new session. Teacher answers to questions the new agent did not ask are still delivered, as statements in their own right. A replay with more than 5% unanswered questions is marked "under-answered". Replay is deterministic, so curve points and ablations cost zero calls.

### 8.4 Seeding from 2.0 and 3.0 (decided, D4.2)

Cached formal readings of 3.0 procedures and situations are **replayed as teacher messages** through the conversation path, at 0 calls, where they fit the 4.0 schema; they are **never injected**, and readings tied to H3 are excluded. Counts from read-only scripts (eval §2.3, §4.5):

| Source | Channel | Count | Used |
|---|---|---|---|
| calls3 `parse_fallback`, train and dev (71 tasks) | `archive3` | 1,385 messages, 6,735 fact lines | Yes, curriculum stage 6, after the deny check. Ablation: channel off |
| calls3 `situation_fallback` matching non-H3 lines | `archive3` | 295 | Yes, stage 7, paired with their task session |
| calls3 `parse_fallback` on the 90 unknown (H3) tasks | — | 948 | Never |
| calls3 gardening readings | — | 34 | Never (gardening is never taught) |
| other `situation_fallback` | — | 1,053 | Quarantined |
| `baseline_b2`, `teacher_answer`, `x7_paraphrase`, `smoke` | — | 454 | No (H3 answers, or English) |
| `data/curriculum3`, `data/dataset3` (203 tasks) | — | — | Topic seeds and agreement check only |
| 2.0 data and call archives | — | 12,623 calls | Kept for the English layer |

"Where they fit": a reading whose predicates and sorts map onto schema v4 is replayed; one that does not is re-taught live in stage 6 or dropped, with counts reported. The v3 ground truth only scores agreement with the taught procedures (like 3.0's M3) and is never delivered. Archive readings carry no `demands`; those are taught live in stages 6 and 7.

## 9. Curriculum and teacher

### 9.1 Stages

Topics and order come from a script, `tools/design4/topics.py`, which writes `data/curriculum4/<size>/topics.jsonl`.

| Stage | Relations taught | Topic unit | Seeded from |
|---|---|---|---|
| 0 Vocabulary | `new(sym, sort)` declarations, `same_as`, `not same_as` | domain × entity group | 3.0's 412 items, 285 actions and 76 role symbols, plus Haiku's additions |
| 1 Kinds | `is_a` | entity group | the registry |
| 2 Materials and properties | `made_of`, `has_property`, and negatives | entity group | the registry |
| 3 Functions | `can_do`, `can_do_on`, `used_for`, `can_go_in` | entity group; each role symbol ("what can pierce?") | the 3.0 roles (32 symbols have 3 or more items) |
| 4 Parts | `has_part` | appliances, fixtures, tools | the registry |
| 5 Actions and states | `effect`, `precondition`, action `is_a` | each action in the seed procedures | 3.0 actions and `until` states |
| 6 Procedures | v3 procedure facts plus `demands` | one task | archive3 replay (71 tasks, 0 calls); dataset3 tasks live, given the task name and step outline only |
| 7 Situations and exceptions | situation cases with their resolution; negative facts on specific kinds | task × situation kind | archive3 situations, plus the 3.0 variant kinds |
| Agent questions | any | asked by the agent | interleaved and budgeted |

**Order of work** (eval §1.9): stage 0 runs first for all sizes; then the benchmark is authored from the registry, sealed and deny-listed; only then do stages 1-7 run. The benchmark cannot be shaped by what Haiku taught, and teaching is shaped by the benchmark only through the deny list, which only removes.

**Quarter blocks** (eval §1.10). Each size's topic units are assigned at random, stratified, to four blocks, each running stages 1-7 for its units. Every prefix is then a representative sample, and the curve measures quantity of teaching, not stage order.

### 9.2 Knowledge-base sizes (decided, D4.10)

| Size | Taught facts (planner target) | Domains | Built when |
|---|---|---|---|
| Spike fixture | about 420 statements | baking, cooking, cleaning, repair | Milestone 31 (test only) |
| S | about 1,500 | baking, cooking, cleaning, repair | Milestone 36 |
| M | about 4,000 | S plus errands, travel, crafts, personal care and planning | Milestone 38 |
| L | about 10,000 | M at greater depth (more entities per group) | Milestone 39, **only if** the scaling gate G3 passes |

The sizes are nested. **Gardening is never taught**; it exists only as question context. The H3 task names are on the teaching name-level deny list.

### 9.3 Teaching calls and questions

**A teaching call** carries one topic batch (eval §2.2, prompt sketch there). One fixed, versioned system prompt per stage keeps cache keys stable. The user prompt holds the stage and unit, the unit's registry slice (≤ 40 symbols plus the 30 most frequent kinds), up to 10 pending agent questions answered first under `answer N:`, "teach 20-30 statements", and the last 2 exchanges. It never holds benchmark material (leak check P1). The reply is split into statements that go through the conversation path one by one; the agent's replies go into the next call on that topic, so one call is one teacher turn.

**Yield.** The plan assumes 15 accepted facts per call (stages 1-5) and one task per call (live stage 6). Below 10 in the pilot, the plan is re-cut before stage 1.

**Questions** ride along, up to 10 per call. Question-only calls are capped at 20% of a stage's calls. When the queue is full, the lowest-ranked questions are dropped and logged. If questions yield fewer facts than topic teaching after stage 3, their share drops to 10%.

**Teacher audit.** 100 random delivered statements per stage are hand-checked for world truth.

## 10. Evaluation

### 10.1 The derivation benchmark (decided, D4.9)

**Sizes.**
- **900 questions**, written as one pool by Claude Code sub-agents and split at random (with a committed seed, stratified by type × hop × `kb_scope`) into **300 dev** (readable) and **600 sealed** (hashed, at most 3 readings).
- The **spike has its own 100** hand-checked questions, which become dev-only afterwards.

**Question types** (eval §1.2):

| Code | Type | Gold |
|---|---|---|
| K | Kind reasoning: inheritance and transitivity over `is_a`, `has_part`, `same_as` | yes/no or bindings, plus proof |
| SUB | Substitution (B7), with demands | bindings plus `excluded` look-alikes, or a gap |
| X | Cross-domain: the procedure is given in an untaught domain (gardening), or the pieces come from other domains | as SUB or K |
| EXC | Exception: naive inheritance gives the wrong answer | usually `no`, plus a proof of the defeat |
| STEP | Step replacement (B8) | `replace_step` bindings plus proof |
| NAME | Needs `same_as` | as K or SUB |
| IND | No base-rule proof exists, but a pattern with enough support does | `hyp` with `min_support` |
| GAP | A needed piece is deliberately withheld; a quarter are near-misses that share entities with answerable questions | `gap` with the missing pieces |

**Sealed composition.** Dev has the same mix at half size.

| Type | Total | 1-hop | 2-hop | 3-hop |
|---|---|---|---|---|
| K | 130 | 40 | 50 | 40 |
| SUB | 130 | 25 | 55 | 50 |
| X | 90 | 10 | 45 | 35 |
| EXC | 60 | 20 | 25 | 15 |
| STEP | 40 | 10 | 15 | 15 |
| NAME | 20 | 5 | 10 | 5 |
| **Answerable deductive** | **470** | **110** | **200** | **160** |
| IND | 50 | | | |
| GAP (20 near-miss) | 80 | | | |
| **Total** | **600** | | | |

`kb_scope` targets are about 40% S, 30% M and 30% L. If L is not built, the L-scope questions are reported as uncovered, never dropped.

**Record format** (eval §1.3, schema `bench4/1`): `query`, `given`, `gold` (yes/no, bindings with `excluded`, gap with `missing`, or hyp with `min_support`), `gold_proof` (any other valid proof is also accepted), `pieces` (the leaves), `deny` (the answer and every intermediate node), `withheld` (GAP only), `hops`, `type`, `domains`, `kb_scope`, `rules_needed`, and a reviewer field.

**Authoring** (eval §1.9). Four writer sub-agents, one per domain group, work from `tools/design4/BENCH_SPEC.md`. They see only the spec, the relations and base rules as logic, the registry (names only) and the 3.0 procedure ground truth for step ids and roles: no agent code, prompts, teaching, output or H3. A reviewer sub-agent checks each gold's world truth, and a question is kept only when both agree. `check_bench.py` checks that `ref_reason.py` derives each gold from `pieces` plus `given` at the stated hops; that proofs are minimal; that GAP questions are underivable without their withheld atoms and derivable with them; that no denied atom of one question is another's piece; that no piece is in more than 5% of a split's proofs; and that no question repeats another up to renaming. Authoring is logged in `data/bench4/AUTHORING.md` and uses no Haiku calls.

### 10.2 Two golds

Haiku teaches from its own knowledge, so some gold pieces will be untaught or taught in another form. Each question therefore has a **world gold** (authored, sealed, the **measure of record**) and a **KB-relative gold** computed at each reading by `ref_reason.py` over the statements actually delivered (eval §1.4). Every reading reports **coverage** (world gold derivable from what was taught: the curriculum), **conditional accuracy** (correct given derivable: the reasoning, the cleanest measure of generalisation) and **headline correct** (against the world gold, with a valid proof).

### 10.3 Scoring classes and metrics

Each answer falls in exactly one class:

| Class | Definition |
|---|---|
| **Correct** | Matches the world gold, and every proof passes `ref_reason.py`. For bindings, all gold answers are found and no `excluded` one is claimed. For IND with gold `hyp`, the answer is marked `hypothesis` with a support set that checks |
| **Wrong** | The proof is structurally valid, but the conclusion is not the world gold. Either **defeated** (a missed exception) or **wrong premise** (a taught statement is false in the world, traced to its message) |
| **Invented** | A claim with no proof, a leaf never delivered, a step that is no rule instance, or a hypothesis stated without its marker. **This includes correct answers with an invalid proof.** Target 0 |
| **Gap, correct** | No claim where the world gold is `gap`, or where the KB-relative gold says the answer is not derivable (a "justified gap", reported separately and given no credit in the headline). Gap precision (is the named missing piece one that would complete a proof?) is reported |
| **Gap, missed** | No claim where the world gold was derivable from what was taught |

On IND questions with gold `hyp`, a gap counts as a justified gap: neither correct nor wrong.

**Metrics**, per hop, type, domain tier, `kb_scope`, curve point and size, with Wilson 95% intervals and a cluster bootstrap over entities (primary if more than 1.3 times wider): correct, invention (share of claims), wrong (defeated, wrong-premise), correct and missed gaps, proof validity, coverage, conditional accuracy, hypothesis precision with confirmations, retractions and specialisations per layer; conversation quality (planted conflicts found, naming questions, rejections, accepted facts per call); cost (calls per accepted fact, milliseconds, facts touched and proof depth per question, p95 per size). Power is in Appendix C: sealed 2-hop ±6.9 points, paired contrasts about 2.6 points overall, and the spike's 2-hop stratum ±16 points.

### 10.4 The learning curve (the main result)

Replays of raw prefixes at 25, 50, 75 and 100% for S, M and L if built give up to 12 points at zero calls. Each reports correct, invented (which must stay 0), coverage and conditional accuracy, with learned rules on and off. Robustness comes from order-shuffled replays (0 calls) and one teacher-variance replicate of S (300 calls). The sealed part is read for all snapshots together in R3, so the curve cannot be tuned against it. "Gets better as taught more" is the paired McNemar difference between 50% and 100% at the largest size built, with the slope over all points; its threshold is set after the spike.

### 10.5 Leakage (decided, D4.8)

**The deny list** holds every dev and sealed question's gold answer, intermediate proof nodes and withheld pieces, plus their variants: argument permutations of symmetric relations, the `same_as` closure (recomputed as the KB grows) and the `stand_in` form of a `swap`. The negation is flagged "contradicts gold" and counted, not blocked. The 48 X4 variant differences used by K5 are added too.

**Normalisation levels:**

| Level | Normalisation | On a hit |
|---|---|---|
| L0 | exact atom, after whitespace and case | Block |
| L1 | symbol form (case, singular, separators) plus the `same_as` closure | Block |
| L2 | head-word form | Warn only; counted, and a sample is hand-checked |

**Sealed atoms are stored as salted SHA-256 hashes** (`data/bench4/sealed/deny.sha256`). The check needs no clear text and reports counts only.

**Check points** (`tools/design4/leak_check.py`):

| Point | Check | On a hit |
|---|---|---|
| P1 | Every outgoing teacher prompt | The call is not made; harness error |
| P2 | Every teacher statement, before delivery | **Withheld**: kept in raw as `delivered: false, quarantine: "deny"` and counted; the rest of the turn is delivered |
| P3 | Every agent question | Dropped silently (a reply of "unknown" would tell the agent something) |
| P4 | Every archive3 import | Withheld, as at P2 |
| P5 | Reading audit over the whole delivered raw layer, under the current closure | **Any delivered leak invalidates the run**: find the path, fix it, re-teach from the hit point |

**Alarms.** K0 (retrieval only) must score 0 on answerable deductive questions; any hit is a leak the script missed. During a reading, gap questions are discarded and the facts-layer hash must not change.

### 10.6 Sealing and readings

At most **3 sealed readings**, counts only, logged in `data/bench4/sealed/READINGS.md`:

| Reading | When | What runs |
|---|---|---|
| R1 | After S (milestone 36) | All S curve points, K0, K1, ablations, K3 at S100 |
| R2 | After M (milestone 38) | M's points; S again on the S-scope subset |
| R3 | Final (milestone 40), one frozen build | Every size, point, ablation and baseline |

No sealed transcript is read before R3, changes between readings use dev only, and a fourth reading needs an explicit, logged user decision. H3 stays sealed and unused.

### 10.7 Baselines

All baselines run on the same questions with the same scorer, so every comparison is paired.

| Id | Baseline | Shows | Calls |
|---|---|---|---|
| K0 | Retrieval only: the answer atom itself is stored | The floor and the leak alarm | 0 |
| K1 | Base rules only (learned rules off) | What rule learning adds | 0 |
| K1−Bk | Base rules minus Bk; B2 replaced by instance-only exceptions, and by monotone inheritance | Each rule's contribution and harm; the cost of the exception mechanism | 0 |
| K2 | The full system | — | 0 |
| K3 | Haiku open-book: the delivered statements relevant to the question (its symbols and their ancestors to 3 levels, at most about 1,500), answering in the formal language with a proof, 10 questions per call. The agent is also run on the same retrieved subset | A language model on the same knowledge, and its inventions | 90 per run |
| K4 | Haiku closed-book | How much of the world gold is common knowledge; correctness reported apart, since its proofs cannot cite teaching | 90, once |
| K5 | 3.0's X4 comparison: 4.0 on the 48 train and dev variant cases (3.0: 19.2% change recall), with situations as formal facts, at the largest size | Direct comparison with 3.0's generalisation number | 0 |
| K6 | Naive sibling stand-in: any item sharing an `is_a` parent | A reference for overgeneralisation | 0 |

**Pre-registered primary contrasts at R3** (Holm-corrected), at the largest size built:
1. the learning curve, 50% against 100%;
2. learned rules on against off (K2 against K1), with IND reported apart;
3. invention rate, agent against K3 (one-sided), with correct rates beside it.

### 10.8 Gates

| Gate | When | Form |
|---|---|---|
| **G1 spike** (decided, D4.11) | Milestone 32 | Go to phase B only if, on the spike's 100 questions with the full fixture, all four hold: **2-hop correct ≥ 60%** (≥ 22 of 36); **0 inventions** over all claims; **100% valid proofs**; **learned rules add correct answers without adding wrong ones** (on against off: at least one more correct, zero more wrong, and no question moves from correct to wrong). The **user decides** go or no-go on the spike report. Otherwise, rethink the relations, the rules or the approach before any teaching budget is spent |
| **G0 benchmark ready** | Milestone 35, before stage 1 | ≥ 900 validated; writer-reviewer agreement ≥ 90% before drops; split, manifest and `deny.sha256` committed; K0 = 0 on answerable deductive questions against the stage 0 KB. Pilot yield ≥ 10 net facts per call and ≥ 90% of statements in schema |
| **G2 after S** | Milestone 36, then R1 | Delivered leaks = 0; 0 inventions on dev; proof validity 100%; teacher audit error per stage ≤ a threshold; dev 2-hop at S100 ≥ s2 − m₁; positive curve slope S50→S100 on dev; coverage at S100 ≥ c₁ on S-scope questions. A failure means fix on dev and re-teach S within the reserve; a second failure means stop and report |
| **G3 scaling** (D4.10) | Milestone 38, then R2 | **L is built only if coverage rises from S to M** (paired, S-scope and full set). Guards: correct on the S-scope subset at M100 not below S100 − m₂; 0 inventions; p95 answer time ≤ t₁. The user decides. If it fails, L is skipped and the result is reported as "scaling did not help at this size" |
| **G4 final** | Milestone 40, R3 | Every pre-registered target reported with an interval, met or not |

s2 is the spike's 2-hop rate. m₁, m₂, c₁, t₁ and the audit threshold are **set after the spike**: the spike report proposes them, and `PREREG_4.md` freezes them. Eval §7.1 proposes the starting forms: m₁ = 15 points, c₁ = 50%, audit error ≤ 10%, m₂ = 2 points and t₁ = 2 s.

**Hard stops:** the ledger reaches 5,000; the first plan-limit refusal (fail fast); a phase exceeds its cap; a delivered leak; the facts-layer hash changes during a reading; the sealed reads are used up.

**Soft stops (review first):** the new-symbol rate rises for 2 stages running; the audit error exceeds its threshold; question yield falls below half the topic yield; defeated answers exceed 10% of EXC on dev; more than 3 retractions per 100 teacher messages.

## 11. Milestones

Each milestone commits on `design-4` with its acceptance tests passing. Structural tests are fixed now. Numeric thresholds marked **set after the spike** go into `research/PREREG_4.md`, written from the spike report before milestone 36. After R1 they may be lowered once, with the reason written down, as in 3.0. They are frozen after that.

### Phase A (zero Haiku calls)

**30. Design session (done).** P1-P16 and D4.1-D4.11 are recorded, the three drafts are written, and this document is merged from them.

**31. Spike build.**
- **Fixture.** Written by a sub-agent from Appendix A and **reviewed by the user** before any run (D4.3). It covers relations, demands, the oracle answers and the 100 questions, with procedures from archive3 replay. `data/spike4/MANIFEST.sha256` fixes it during the spike.
- **Build, in this order** (logic §8; draft §9.2):
  1. `kb4` schema v4 and grammar;
  2. `reason4` with B1-B9 and the exception program;
  3. the `agent4` conversation path against the oracle teacher, with revision and withdrawal;
  4. proof records, the renderer, `reason4::check` and `ref_reason.py`;
  5. the `eval4` scorer and leak check;
  6. the ablation and size harness;
  7. `learn4` part A, then part B with specialisation and layers;
  8. naming canonicalisation and the drift test.

  Items 1-6 are the deduction core. They are built and run before the learning work, so that learning is measured against a fixed baseline.
- **Accept:** every example message here parses and the 2 planted sort errors are rejected; the logic §2.4 outputs (penguin with a redundant link, exception to an exception, diamond, muffin tray) reproduce as Rust tests; E1-E7 give exactly the shown lines; the program passes the engine's safety and stratification check and each Bk switches off by id; every negative control of 5.2 fails and `reason4::check` agrees with `ref_reason.py` on every spike proof; the scorer gives gold 100% and an empty agent 0%; a planted leak fails the leak check; the traps induce nothing at k = 3; no rule supports itself; fixture replay is byte-identical to the live spike run.

**32. Spike measurements, report and go/no-go.**

| Measure | How |
|---|---|
| G1 numbers by hop, type and tier | `sla4 spike` with the full fixture |
| Contribution and harm of each base rule | B1-B9 off in turn; B2 against instance-only and monotone; questions that flip |
| Learned rules | Off against k = 2, 3, 4; hypothesis precision; traps; confirming questions; whether part B rediscovers B7 with B7 off |
| The curve on the fixture | Prefixes of 25, 50, 75 and 100%, in curriculum order and 3 random orders; the `ask_fit` rate falling as demands are taught |
| Conflicts | Kinds found (specific-over-general, diamond, learned-against-taught, sort) and whether each was asked; `conflict` questions per 100 facts |
| Naming | 10-20% of item symbols rewritten to synonyms: canonicalisation off, on with oracle answers, and on with the detector's questions; canonical against rule-form `same_as` |
| Inheritance against material rules | The silicone-tray cases: did the agent ask or answer? |
| Inference cost and scaling stress | Time, derived facts and proof depth at each prefix; the fixture copied ×10 and ×50 with renamed symbols and cross-links (up to about 25,000 facts), with closure against demand slice, and slice answers equal to closure answers |
| Proof length | How often the kept proof is longer than the gold proof |

- **Report:** `research/SPIKE_4.md`. It gives the numbers above and proposes every **set after the spike** value: G2 and G3 margins, hypothesis precision, the K2-K1 gain, the curve threshold, p95 limits, body length and the hop limit.
- **Gate G1:** the user decides go or no-go (D4.11). On go, `PREREG_4.md` is drafted from the report.

### Phase B (budget, each call-spending milestone needs the user's go-ahead)

**33. Three-layer knowledge base, replay and full conversation path.**
- Build: `kb4` layer readers and writers; `sla4 replay` with `--answers` and migrations; the registry and alias table; the `teach4` deny filter (P1-P4) and P5 audit; the archive3 importer; lab events on 3.0's stream.
- Accept: replay byte-identical to the live run; a migration renaming one relation rebuilds the same answers; the no-injection test passes; leak tests fail on planted leaks at P1-P5 and pass otherwise; the word-list test passes on all 4.0 crates and rule files; the archive3 import reports mapped, re-teach and dropped counts.

**34. Teacher channel, pilot and stage 0 (about 100 calls).**
- Build: stage prompts, batching, question batching, `haiku.py` with the `calls4` ledger and caps.
- Run: pilot T0 (2 prompt versions × 4 stages × 4 topic units, plus 8 question round-trips); then stage 0 for all sizes.
- Accept: every call cached and in `calls4`; reported: schema pass share, accepted facts per call, teacher accuracy on a 100-fact hand-checked sample, naming questions per 100 facts, new-symbol rate; G0's pilot criteria met, else the prompt is changed before stage 1.

**35. Benchmark (gate G0).**
- Build: `BENCH_SPEC.md`, sub-agent authoring and review, `check_bench.py`; the 300/600 split with `MANIFEST.sha256`, a hash test in `cargo test`, and the hashed deny list.
- Accept: every gold proof checks and recomputed hop labels match; the strata mix is within 5 points of 10.1; writer-reviewer agreement ≥ 90%; K0 = 0 on answerable deductive questions.

**36. Curriculum S, snapshots, R1 (gate G2).**
- Build and run: the S planner and quarter-block runner; S live (archive3 for stage 6); then the S replicate.
- Accept on dev: 0 delivered leaks, 0 inventions, 100% proof validity; 2-hop at S100 ≥ **set after the spike**; S50→S100 paired gain ≥ **set after the spike**; coverage, conditional accuracy and the audit reported. Then sealed reading R1, and `PREREG_4.md` is frozen.

**37. Rule learning at scale.**
- Build: induction over S with layers, confirming questions, specialisation and retraction within the question share.
- Accept on dev: hypotheses per layer reported; hypothesis precision ≥ **set after the spike**; every retraction traceable to a teaching message; K2 over K1 correct gain ≥ **set after the spike** with no rise in wrong answers; at least one layer-2 rule, or the report says why none.

**38. Curriculum M, R2 (gate G3).** M live in quarter blocks. Accept: the structural tests of 36; p95 answer time ≤ **set after the spike**. Then gate G3, decided by the user (L only if coverage rises from S to M, D4.10), and sealed reading R2.

**39. Curriculum L (only if G3 passes).** As 38, at L, with the demand slice and interning in place first.

**40. Final measurement (R3, gate G4).**
- Run on the sealed part: every snapshot of every size; K0-K6; the base-rule ablations; induction at k = 2, 3, 4; canonical against rule-form `same_as`; question share 0% against 20% (does active learning pay?, by replay with `--answers none`); archive3 off.
- Accept: every target reported with an interval, met or not; the three primary contrasts, Holm-corrected; ≤ 3 readings in the ledger; ≤ 5,000 calls.

**41. Research PDF update.**
- Build: a 4.0 section in `paper/`, with every number generated from `data/` by `tools/paper/numbers.py`.
- Accept: `make -C paper` builds from a clean checkout, and `numbers.py --check` shows no difference.

## 12. Haiku budget (decided, D4.1: a fresh 5,000, hard limit)

4.0 has its own ledger and cache in `data/training/calls4/`; 3.0's 4,486 calls do not count (its ledger shows 4,533 successful lines, eval §3.1). The 3.0 caches are kept; their prompts differ, so reuse comes through archive3 replay (8.4), not cache hits. Benchmark authoring by sub-agents is not a Haiku call and is logged separately.

| Use | Milestone | Plan | Cap |
|---|---|---|---|
| Design session, spike | 30-32 | 0 | 0 |
| Pilot T0 | 34 | 60 | 100 |
| Stage 0 vocabulary (all sizes) | 34 | 40 | 60 |
| S: stages 1-5 (about 1,500 facts at 15 per call, plus re-asks, rejections and duplicates) | 36 | 170 | |
| S: stage 6 (archive3 at 0 calls; about 39 dataset3 tasks live, plus retries) | 36 | 50 | |
| S: stage 7 | 36 | 60 | |
| S: question-only calls (≤ 20%) | 36-37 | 55 | 400 for S in all |
| S replicate (teacher variance) | 36 | 300 | 330 |
| M: stages 1-7 and questions | 38 | 400 | 480 |
| K3 Haiku open-book: at S100, M100, and at the largest size's 100% and 50% | 36-40 | 360 | 400 |
| K4 Haiku closed-book, once | 40 | 90 | 100 |
| Replay top-up for changed agents (`--answers live`) | 36-40 | 195 | 250 |
| **Planned without L** | | **1,780** | |
| L: stages 1-7 and questions (only if G3) | 39 | 750 | 900 |
| **Planned with L** | | **2,530** | |
| Reserve: retries, prompt fixes, re-teaching after a failed audit or a G2 failure, archive readings that do not fit | | the rest (≥ 2,470) | |
| **Total** | | | **5,000** |

The caps are re-set after the pilot's measured yield, always within 5,000.

**Stop rules.** The ledger warns at 4,000; from 4,500 the remaining work runs on cached calls and replay only; at 5,000 `haiku.py` refuses every live call until the user explicitly raises the limit (P14, D4.1). A phase at its cap stops and asks for a re-plan; the first plan-limit refusal stops the run; reserve use is logged with its reason.

## 13. Risks and their mechanisms

| Risk | Mechanism | Test |
|---|---|---|
| **Naming drift** | Registry in every prompt; similarity questions; pending facts; canonicalisation; no automatic merges | Spike drift test; new-symbol rate; NAME questions |
| **Learned rules overgeneralise** | D4.7 acceptance (s ≥ 3, ≥ 2 tasks or kinds, no unhandled counterexample) plus u ≥ 1, minimality, no self-support; one confirming question; conclusions below teaching and marked `hypothesis`; specialise or retract; quarantine; prediction budget | Spike traps; hypothesis precision; K6 |
| **Leakage** | Deny list over answers, intermediate nodes and withheld pieces, hashed for sealed; checks P1-P5; any delivered leak invalidates; benchmark written before stages 1-7 | Planted-leak tests (31, 33); K0 = 0 |
| **Base rules too weak or too strong** | Every Bk can be switched off; unique contribution and harm; B2 against weaker variants; no material rule; B7 rediscovery test | Spike and milestone 40 ablations |
| **Inference explodes** | Typed relations; demand slices; no materialised `has_part`/`before`/`same_as`; interning before L; hop limit | Scaling stress ×1, ×10, ×50; p95 at 38 |
| **Wrong facts spread** | Confidence (min over taught × product over learned); checks at entry; skeptical blocking; every chain shown; wrong-premise answers traced | Teacher audit; wrong-premise attribution |
| **"Gets better" is one number** | 12 curve points by zero-call replay, rules on and off, shuffled replays, a teacher replicate, all read in R3 | Milestones 36, 38, 40 |
| **Thresholds by guesswork** | Only G1 fixed in advance; the rest set after the spike in `PREREG_4.md` | `PREREG_4.md` before 36 |
| **Budget spent on volume** | 20-30 statements per call; questions ride along; archive3 at 0 calls; replay for curves; caps and hard stop | Ledger; calls per accepted fact |
| **Result looks trivial** | Integration, conflicts, naming, learning and derivation each measured; K0 shows taught facts alone answer nothing; K3 shows a language model on the same knowledge | K0, K3 |
| **Teacher's world differs from the benchmark's** | Shared registry; world and KB-relative golds; coverage and conditional accuracy apart | Coverage at G2 |
| **`demands` under-taught** | B7 asks rather than refuses; demands taught in stages 6-7; near-miss questions ranked high | `ask_fit` rate on the curve |
| **Archive readings conflict with live teaching** | All checks apply to archive3 messages; the channel can be ablated | Archive3-off ablation |

## 14. What 4.0 can claim, and what it cannot

**Can claim, if the thresholds are met:** taught only through formal conversation, the agent creates knowledge it was never taught, across domains and in an untaught domain, deduced with a checked proof or induced with its supporting examples; it makes no claim without a reason, measured by an independent checker; its accuracy grows with teaching, as a curve at several sizes; and the rules it learns add correct answers, are confirmed, specialised or retracted by teaching, and build on one another in recorded layers.

**Cannot claim:** understanding English (there is none on the core path); knowledge Haiku did not teach; that Haiku's facts are true beyond the audited samples; that synthetic teaching stands for human teaching; that results hold beyond the largest size measured.

## 15. Resolved between drafts

| # | Topic | Draft positions | Resolution (source) |
|---|---|---|---|
| 1 | Relation set | draft: 9 plus `can_hold`, `can_go_in`. eval: the pre-planning's 9. logic: 11 typed plus `demands` | logic's (D4.4): `can_hold` → `can_do_on(X, hold, Y)`, `part_of` → `has_part`, action-level `effect`/`precondition`, step-level `requires_state` kept. Draft and eval examples restated in these relations |
| 2 | Stand-in requirements | draft: keep every relation of the lacked item to the step's object and co-items. eval: `requires(action, P)` checked as `has_property` | Taught `demands` with the `unfit_known`/`fit_open` split (D4.4, D4.6) |
| 3 | Base rules | draft B0-B8; logic B1-B9; eval R1-R9 with candidates R8 (material) and R9 (role → can_do) | logic's B1-B9 (D4.6). R9 is B6; R8 is left to induction |
| 4 | Exception program | draft: `decided`/`overridden`, diamonds yield both polarities. logic: `pos`/`neg`/`pre`/`live`, skeptical | logic's, which was run on the engine; diamonds yield neither polarity and a question (D4.5) |
| 5 | "Different" naming answer | draft `not same_as`; logic `distinct` | `not same_as` (every relation can be negated, D4.4) |
| 6 | Rule acceptance | draft: k = 3, ≥ 2 topics, c = 0, u ≥ 1, bodies ≤ 2. logic: ≤ 3 literals, Wilson ≥ 0.6, confirming query | D4.7 plus draft's u ≥ 1, minimality and bodies ≤ 2 (+ specialisations). Wilson reported, not gated (it would contradict s = 3) |
| 7 | Confidence | draft: s/(s+c+1), min everywhere. logic: Wilson; min × product of learned | s/(s+c+1) for rules; min over taught × product over learned for conclusions |
| 8 | Adaptation rules | draft: item rules only. logic: plus lgg over cases | Both; part B is needed for the B7 rediscovery test |
| 9 | Seed procedures | draft: scripted seed teacher delivering v3 ground truth. eval: archive3 replay | archive3 replay where readings fit; ground truth never delivered (D4.2) |
| 10 | Spike fixture | draft: user, or Claude Code under review, 12 seed procedures. eval: sub-agent, 8 archive3 procedures | Sub-agent writes, user reviews, spike only (D4.3); 8 archive3 procedures (D4.2) |
| 11 | Benchmark size | draft 600 (150/450). eval 900 (300/600). Pre-planning about 200, then 500+ | 900 (300/600), ≤ 3 readings; spike 100 (D4.9) |
| 12 | Leak handling | Pre-planning: fail if taught. draft: withhold, fail on delivery. eval: five check points | Withhold before delivery; any delivered leak invalidates the run; deny list covers answers, proof steps and withheld pieces (D4.8) |
| 13 | Spike gate | draft: +5 on induction questions. eval: +3 on IND, plus p95 < 1 s | D4.11 as in 10.8; timing reported, not gated |
| 14 | Budget | draft: about 1,550-2,400. eval: 2,510 plus a conditional XL of 1,490 | Fresh 5,000 (D4.1); no XL (D4.10); section 12 |
| 15 | Sizes and domains | draft: M adds laundry and kitchen care. eval: M adds five dataset3 domains, L adds depth, plus XL | D4.10 sizes; eval's domains; no XL |
| 16 | Scaling gate | draft: M beats S on correct. eval: non-inferiority, gain, p95 | Coverage rises S → M (D4.10), with eval's guards; the user decides |
| 17 | Gate and baseline ids | draft G1-G2, K0-K3. eval G0-G4, B0-B6 (clashing with base-rule ids) | G0-G4; K0-K6 |
| 18 | Hop counting | draft: deepest branch. eval: nodes in the minimal proof | eval's, made precise in 7.4 |
| 19 | Proof checker | draft, logic: Rust, independent of the evaluator. eval: Python `ref_reason.py` | `ref_reason.py` of record; `reason4::check` at runtime; both must agree |
| 20 | Curriculum | draft: 5 stages, 2 shuffled orders, authored name files. eval: stages 0-7, quarter blocks, stage 0 registry | eval's; draft's second order becomes a zero-call shuffled replay plus one teacher replicate |
| 21 | Batch size | draft 15-25; eval 20-30 | 20-30 asked, 15 accepted assumed, re-cut after the pilot |
| 22 | `PREREG_4.md` | draft: before 36, lowered once after R1. eval: set after R1, never lowered | Written from the spike report before 36; may be lowered once after R1 with a reason; then frozen |
| 23 | KB syntax and records | draft: `-R`, one statement per raw record, migrations as data. eval: `not R`, `rule(id):` pseudo-syntax, `statements` arrays, mappings in the reader | Engine syntax in `facts.dl`/`rules.dl`; eval's raw records; migrations as data selected by `lang` |
| 24 | Primary contrasts | eval: at L | At the largest size built |
| 25 | B6 subject | logic: role-derived ability at rank I of the item | Holds for that item, beaten by a taught negative, not inherited |

## 16. Open points

These are the only points not settled by P1-P16, D4.1-D4.11 or this document:

1. **Confirm the adopted pre-planning recommendations for Q5, Q7 and Q8.** They have no D4 number: the knowledge-base format details (section 8), the curriculum stages and question share (section 9), and what stays from 3.0 (crates unchanged, `reasoner` as the engine, 3.0's parser kept for the English layer). This document adopts them. They become binding with it unless the user changes them.
2. **The S teacher-variance replicate (300 calls).** It is planned in section 12. The user may cut it, in which case the curve's spread comes from zero-call order-shuffled replays only.
3. **`demands` across steps.** A demand sits on the step where the item fills the role (`demands(s4, hold, can_go_in, oven)`), although the oven is used in a later step. The spike settles whether the fixture's demands are enough or whether demands must follow an item along `before` and the same object (eval §5.4, E3).
4. **Whether B7 moves from built-in to learned,** if the spike shows that part B rediscovers it. This is the user's decision on the spike report.
5. **Starting values to be fixed by the spike or the pilot** (no user decision needed unless the user wants one): body length, the prediction budget, the hop limit (4), the naming-similarity threshold, taught confidences (0.9 / 0.97) and the 0.5 answer floor. The support threshold itself is fixed at 3 by D4.7; k = 2 and 4 are ablations only.
6. **The offline Popper and FOLD-R++ yardstick** needs clingo and SWI-Prolog installed in a separate environment. It is optional; the user may skip it.

## 17. Later (beyond 4.0)

- **Controlled English (option B).** The first language layer, built on 3.0's learned parser. It turns controlled sentences into 4.0 messages, and answers into controlled sentences. Free English and human-written text come after it.
- **The real-time training lab** (Design 3.0 dashboard direction) on 4.0's event stream: derivations, conflicts and hypotheses, live.
- Composition rules, magic sets, a minimal-proof search, incremental maintenance.
- Full defeasible logic or ASP, if conflicts appear that the layers cannot express.
- Multi-step plan repair; learning the numeric changes that follow from a stand-in (times, amounts).
- Larger knowledge bases and the long-term vision. These are context only, and move only when the user says so.

---

## Appendix A. Spike fixture specification (D4.3)

The fixture is written by a sub-agent from this appendix, then reviewed by the user. It is test-only. It lives in `data/spike4/` and is never merged into the phase B raw layer. It is delivered through the conversation path, by channel `fixture`, never loaded.

**Files.**

| File | Content |
|---|---|
| `teach/<domain>.msgs` | About 100 formal statements per domain (baking, cooking, cleaning, repair), in curriculum order |
| `teach/procedures.list` | 8 train or dev tasks (2 per domain) replayed from archive3, chosen by script; any that do not fit are written in `teach/procedures_fixture.msgs`, marked |
| `script/answers.jsonl` | Oracle answers to expected questions, keyed by canonical question; an unexpected question gets `unknown.` and is counted |
| `bench/questions.jsonl` | The 100 questions (`bench4/1`), gold proofs citing fixture line numbers |
| `bench/deny.txt`, `MANIFEST.sha256` | Deny atoms with variants; hashes of everything |

**Target composition per domain** (about 104 statements; repair has more `has_part`):

| Relations | Per domain |
|---|---|
| `is_a` (3-4 levels deep, so 3-hop chains exist) | about 20 |
| `made_of`, `has_property`, with negatives | about 16 |
| `can_do`, `can_do_on`, `used_for` | about 16 |
| `can_go_in`, with negatives | about 6 |
| `has_part` (repair: about 12) | about 4 |
| `effect`, `precondition`, action `is_a` | about 6 |
| `demands` on the procedure steps | about 4 |
| `same_as` | about 2 |
| procedure facts (from archive3) | about 30 |

**Planted cases** (each listed in a fixture index with its line numbers):
- 8-10 specificity exceptions (a general fact plus a more specific negative), including one exception to an exception and one redundant `is_a` link;
- 2-3 diamonds;
- 3-4 induction clusters of support 3-4 from at least 2 domains. One is material-based (plastic → not oven) and one property-based (acidic → can remove limescale);
- 2 induction traps with support 2 (must not induce at k = 3);
- 1 cluster with a taught counterexample (must not induce, or must specialise);
- swap cases from at least 3 tasks in 2 domains, enough for part B to try to rediscover B7;
- 12 cross-domain links (an item's kind taught in one domain and used in another);
- 2 decoy kinds (a shared parent, but failing a demand, like the plastic tub);
- 2 silicone-tray cases, where kind inheritance and a material pattern disagree;
- 3 naming-drift pairs;
- 2 sort errors (must be rejected).

The 10-20% synonym rewrite for the drift test is made by script from the fixture, not written by hand.

**Questions (100).**

| Type | n | 1-hop | 2-hop | 3-hop |
|---|---|---|---|---|
| K | 24 | 8 | 10 | 6 |
| SUB | 20 | 4 | 10 | 6 |
| X (gardening context, `given:` procedures) | 10 | 0 | 6 | 4 |
| EXC | 10 | 4 | 4 | 2 |
| STEP | 4 | 2 | 2 | 0 |
| NAME | 6 | 2 | 4 | 0 |
| IND | 10: 4 with support (gold `hyp`); 3 with a counterexample (gold `gap`); 3 below the threshold (gold `gap`) | | | |
| GAP | 16, of which 6 are near-miss | | | |
| **Total** | **100** | **20** | **36** | **18** |

G1's 2-hop stratum is the 36 answerable deductive 2-hop questions. 22 of 36 (61%) has a Wilson interval of 0.45-0.75, and the verdict is read with that interval in view. Ten example questions with gold proofs are in eval §5.4. When they are written into the fixture, they must be restated in the 4.0 relations: `can_do_on` for holding, `demands` for oven requirements, `has_part` for parts.

## Appendix B. Scaling measurements (logic §6.1)

These were taken on the existing `sla_reasoner` (release build), running the B2 program plus B7 on a synthetic household-shaped knowledge base:
- an `is_a` forest of depth ≤ 6 over 22% of the facts;
- 6% of inheritable facts followed by a lower exception;
- 4-step procedures with demands;
- one situation (5 items lacked, 40 owned).

The machine was a single-threaded AMD Ryzen 5 5500U.

| Taught facts | Mode | Derived facts | Time | Peak memory |
|---|---|---|---|---|
| 518 | full closure | 2,935 | 13 ms | — |
| 518 | demand slice | 2,330 | 10 ms | — |
| 5,014 | full closure | 44,408 | 268 ms | 73 MB |
| 5,014 | demand slice | 7,837 | 41 ms | — |
| 50,018 | full closure | 556,597 | 3,878 ms | 922 MB |
| 50,018 | demand slice | 36,319 | 253 ms | 120 MB |

Stress tests:
- `same_as` as axioms, at 10% aliasing in classes of 3: 61,456 derived facts (+38%), 480 ms (1.8 times).
- One 2,000-link `part_of` chain: 2,001,000 facts in 10.3 s.

Memory, at about 1.7 KB per derived fact, is the first limit. A 1,500-message curriculum at 268 ms per recompute costs about 7 minutes in all.

## Appendix C. Power (eval §1.6)

Assumptions: α = 0.05 two-sided and power 80%. The half-width of one proportion at p = 0.5 is 1.96·√(0.25/n):

| n | 36 | 100 | 200 | 300 | 600 |
|---|---|---|---|---|---|
| ± points | 16.3 | 9.8 | 6.9 | 5.7 | 4.0 |

Zero inventions in n claims bounds the rate at about 3/n (0.5% at 600). Paired contrasts (exact McNemar) need n = (1.96·√p_d + 0.84·√(p_d − δ²))² / δ²: 155 questions for δ = 0.10 at p_d = 0.20, and 312 for δ = 0.05 at p_d = 0.10. With p_d = 2δ the minimum detectable effect is 15.3 points at n = 100, 7.8 at 200 and 2.6 at 600. Unpaired comparisons would need 388 per arm for 50% against 60%, so every comparison is paired. Clustering is limited by the 5% piece cap and checked by cluster bootstrap.
