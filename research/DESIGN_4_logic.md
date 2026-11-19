# Design 4.0: reasoning and learning layer

**Status.** Design input for the 4.0 design session, written on 2026-11-19 on branch `design-4`. It is not a decision record. Each section gives options and one recommendation, for the user to confirm or change as a D4.x decision.

**Inputs.**
- `research/DESIGN_4_preplanning.md`. Decisions P1-P16 are binding here.
- `research/DESIGN_3_experiments.md`, in particular X4.
- `research/DESIGN_3_alternatives.md`, A3 and A4.
- The code in `crates/reasoner`, `crates/core` and `crates/kb3`.
- An offline benchmark of the existing reasoner (section 6), run with zero model calls.

**What the recommendations add up to.**
- A typed relation set of 11 item relations, plus a procedure schema bridged to the items by `role` and `demands`.
- Exceptions handled by **defeasible inheritance with specificity, compiled to stratified Datalog**. It runs on the existing engine today, and section 2 shows its output on worked examples.
- Above that, a **layered defeasible scheme** for learned rules, where conflicts are skeptical: a conflict becomes a question to the teacher, never an answer.
- About nine base rules, split into logic rules and schema rules. The schema rules are candidates for being learned instead.
- A Rust rule inducer: a bounded FOIL/AMIE-style refinement search plus least general generalisation, with specialisation on counterexamples. Popper and FOLD-R++ serve only as offline yardsticks.
- Proofs as checked trees, with a collapsed rendering and a separate checker.
- Forward closure up to about 5,000 facts, and a demand-driven slice per question above that.
- A union-find vocabulary, with merges only on the teacher's answer.

---

## 0. The problem in one paragraph

X4 failed for a structural reason, not a logical one. The 3.0 knowledge base held procedures: steps, the items they use, and the role each item plays. It held almost nothing about the items themselves. Substitution by function could only reuse a role an item had already played in some other taught procedure, so 81% of the needed changes had no supporting fact anywhere. Induced rules fired on 1.4% because situations rarely repeat.

4.0 therefore has to do three things:
1. teach **relations about things** (kinds, parts, materials, affordances, compatibilities, effects), so that knowledge can be chained;
2. connect procedures to those relations through a small number of bridge relations;
3. reason over them with rules general enough to cross domains, and safe enough to make no unsupported claim when an exception exists.

Everything below serves those three needs.

---

## 1. Knowledge representation for generalisation

### 1.1 Requirements

| Requirement | Why |
|---|---|
| R1. Relations about things are separate from procedures | X4. A fact about `muffin_tray` must be usable by every task, not only the one it was taught in |
| R2. Typed arguments (sorts) | Typing rejects malformed teaching, prunes joins, and is the induction learner's language bias |
| R3. A bridge from procedure steps to item relations | Without one, item knowledge never reaches an answer |
| R4. Negative facts, as first-class facts | Exceptions (P8) and contradictions need them |
| R5. Effects and preconditions at the level of **actions**, not only steps | So that "heat water" taught in tea-making helps in cooking |
| R6. No compound symbols that hide structure | `hold_batter` and `hold_liquid` cannot generalise. `can_do_on(X, hold, batter)` can |

### 1.2 Refining the preplanning's nine relations

The preplanning's list needs four changes:
- `requires` is overloaded: it means a state precondition in one place and an item need in another.
- `can_do` cannot say what a function is applied to.
- The worked examples already use `can_hold` and `can_go_in`, which are not in the nine.
- `part_of` points the wrong way for inheritance. Parts of a kind are inherited by its subkinds, so the subject must be the whole.

**Recommended item relations (11).** All are subject-first, which matters for the engine's first-argument index (section 6).

| # | Relation | Sorts | Inherits down `is_a` by default | Meaning and example |
|---|---|---|---|---|
| 1 | `is_a(X, K)` | item × item | (transitive; strict) | `is_a(muffin_tray, baking_tin)` |
| 2 | `same_as(X, Y)` | item × item | (handled by canonicalisation, section 7) | `same_as(egg_whisk, whisk)` |
| 3 | `has_part(X, P)` | item × item | yes | `has_part(pan, handle)`. `part_of(P, X)` is its view |
| 4 | `made_of(X, M)` | item × material | yes | `made_of(tin, metal)` |
| 5 | `has_property(X, P)` | item × property | yes | `has_property(knife, sharp)` |
| 6 | `can_do(X, F)` | item × function | yes | `can_do(fork, beat)` (an affordance) |
| 7 | `can_do_on(X, F, O)` | item × function × item | yes | `can_do_on(baking_tin, hold, batter)`. It implies `can_do(X, F)` |
| 8 | `used_for(X, F)` | item × function | yes | `used_for(whisk, beat)` (designed function). It implies `can_do` |
| 9 | `can_go_in(X, P)` | item × place | yes | `can_go_in(tin, oven)`; places include `oven`, `microwave`, `dishwasher`, `freezer` |
| 10 | `effect(A, O, S)` | action × item × state | yes, through O | `effect(boil, water, hot)`, `effect(microwave, water, hot)` |
| 11 | `precondition(A, O, S)` | action × item × state | yes, through O | `precondition(whisk_action, egg_white, separated)` |

Negation: every relation can also be taught negated, `not can_go_in(plastic_tub, oven).` It is stored as the strong-negation predicate `-can_go_in`, which the engine already treats as an ordinary predicate (`crates/reasoner/src/lib.rs`, header comment). It is reified as `neg(...)` for inheritance (section 2).

Relation 9 could be folded into `can_do_on(X, go_in, oven)`. I recommend keeping it, for three reasons:
- appliance compatibility is the most frequent constraint in household substitution;
- it is the standard exception example;
- a dedicated relation keeps induced rules short.

The rule "add a relation only when the spike shows a derivation cannot be expressed" from the preplanning stands for anything beyond these.

**Procedure schema (from v3, with two additions).**

| Predicate | Use |
|---|---|
| `task`, `step`, `action(S, A)`, `object_of(S, O)`, `uses(S, I)`, `before`, `until`, `setting`, `amount`, `takes_time` | as in schema v3 |
| `role(S, I, F)` | **bridge 1:** item I serves function F in step S. F is a symbol of the same sort as `can_do`'s F |
| `demands(S, F, Rel, V)` (**new**) | **bridge 2:** whatever fills function F in S must satisfy `Rel(filler, V)`. For example, `demands(banana_bread_s4, hold, can_go_in, oven)` |
| `achieves(S, State)`, `requires_state(S, State)` | step-level effect and precondition, as in v3. **Bridge 3:** `action(S, A)` plus `object_of(S, O)` plus `effect(A, O, State)` derive `achieves(S, State)` |
| situation: `has(user, I)`, `lacks(user, I)`, `excludes`, `time_limit` | as in v3 |

**Why `demands` is needed.** The worked example in the preplanning (muffin tray for a loaf tin) is only correct because a muffin tray can go in the oven. Without `demands`, B7 below would also propose a plastic tub, since the tub can hold batter too. Three alternatives were considered:
- Derive demands from the step ("anything used in a `bake` step must go in the oven"). This is domain knowledge, so it would have to be taught or learned: `demands` is the taught form, and an induced rule can generalise it.
- Leave constraints implicit. This invents a claim.
- Check every relation the original item has. This is far too strong: the loaf tin's shape is not needed.

### 1.3 Typing

**Sorts:** `item` (physical things and their kinds, including food), `material`, `property`, `function`, `place`, `state`, `action`, `step`, `task`, `agent`. Sorts are part of the formal language, so they are built in under P6. They contain no domain content, only the signature of each relation.

**How a symbol gets its sort.**
- A new symbol takes its sort from its first use: `made_of(tin, metal)` makes `metal` a material.
- A later use with a different sort is rejected and becomes a question: "`metal` is a material; did you mean `metal_tin`?". That is step 1 of the five-step message check in preplanning section 3.
- `place` is a subsort of `item`, because an oven is also a thing that can be cleaned.
- `function` and `action` are disjoint sorts, but the same name may appear in both, as `beat` the function and `beat` the action. The link between them is taught: `performs(beat, beat)` is unnecessary when the names agree. When they differ (`cut` the function, `slice` the action), a taught `is_a(slice, cut)` over actions connects them, and actions get their own small `is_a` hierarchy.

Typing pays three times:
1. **Entry:** malformed facts are rejected without a model call.
2. **Reasoning:** joins only bind like sorts, and inheritance only runs over the `item` hierarchy (and the `action` one).
3. **Induction:** the hypothesis language only proposes literals whose variable sorts agree. This is the "mode declarations" of Progol/Aleph and the type bias of Popper. Sort agreement is the main reason the search space in section 4 stays small.

### 1.4 Functional roles versus affordances

Three different notions are easily conflated:

| Notion | Relation | Example | Status |
|---|---|---|---|
| **Role**: what an item does in one step | `role(S, I, F)` | the fork pierces the potato in `bake_potato_s3` | situated, procedural |
| **Affordance**: what an item can do | `can_do(I, F)` | a fork can pierce, beat, stir | disposition of the item; Gibson's term, as used in robotics knowledge bases such as KnowRob (Tenorth & Beetz 2013) and FOON functional units (Paulius et al. 2016) |
| **Designed function**: what an item is for | `used_for(I, F)` | a whisk is for beating | ranking and canonical choice; Houkes & Vermaas's "proper" vs "accidental" function |

Two schema rules connect them (section 3, B6):
- `role(S, I, F) → can_do(I, F)`. **This is the rule that turns procedural teaching into item knowledge.** Every taught procedure now also teaches affordances, which is exactly what X4 lacked. The rule is defeasible: a teacher may say a role was improvised. In that case `not can_do(I, F)` is never taught, because the item did do it.
- `used_for(I, F) → can_do(I, F)`.

The stand-in rule reads only `can_do`. `used_for` orders candidates, so designed tools come first, and it marks "improvised" in the answer.

### 1.5 States and effects for action substitution

There are two levels of effect:
- the **step level** (`achieves(S, State)`), as in v3;
- the **action level** (`effect(A, O, State)`): the action A applied to O leaves O in State.

The action level is what transfers. `effect(boil, water, hot)` was taught in tea-making, and `effect(microwave, water, hot)` in a cooking topic. Together with `demands(S, container, can_go_in, microwave)` on the alternative, they let the agent answer "no kettle" in a coffee task never taught with a microwave.

Step replacement (B8, section 3) is a one-step plan repair (van der Krogt & de Weerdt 2005):
- The precondition of a later step S2 is a state Z, which the earlier step S1 achieves.
- S1 cannot run, because an item is lacking and nothing stands in for it.
- Another action A2 applied to S1's object achieves Z, and the items A2 demands are available.

Then S1 is replaced by A2, with the proof. A full planner is not needed in 4.0; the 16-step limit of R2 §11.13 and a single-step replacement are enough for the household benchmark. Multi-step repair (insert two actions) is a later extension.

**Recommendation 1.** Adopt the 11 relations with sorts, plus the procedure schema with `role` and `demands` as bridges and `effect` at action level. Store item relations reified as `pos(X, Rel, Y)` and `neg(X, Rel, Y)` inside the reasoner (section 2). Keep them as plain relation lines in the facts layer and the vault (P11), so the reification is an internal compilation detail that does not reach the knowledge-base format.

---

## 2. Exceptions and non-monotonic reasoning

### 2.1 What must work

| Case | Required behaviour |
|---|---|
| Penguin | `can_do(bird, fly)`, `not can_do(penguin, fly)`, `is_a(tweety, penguin)`: tweety does not fly, even if `is_a(tweety, bird)` is also taught (the redundant link) |
| Exception to an exception | `can_do(superpenguin, fly)` beats penguin for anything under superpenguin |
| Nixon diamond | two incomparable kinds disagree: **no answer**, and a question to the teacher |
| Plastic tub, taught | `can_go_in(tin, oven)`, `is_a(plastic_tub, tin)`, then `not can_go_in(plastic_tub, oven)`: an exception, not a contradiction |
| Plastic tub, learned | an induced rule "containers can go in the oven" meets the taught negative: the instance answer follows the teacher, and the rule is specialised (section 4) |
| Retraction | when a more specific contrary fact arrives later, the earlier conclusion is withdrawn and the agent says so |

### 2.2 Options

| Option | Expressiveness | Cost | Proofs | In Rust? |
|---|---|---|---|---|
| **E1. Explicit exception facts / abnormality literals** (McCarthy 1986): `flies(X) :- bird(X), not ab_fly(X)`, with `ab_fly` taught | Only exceptions someone wrote; no specificity; every default rule needs its own `ab` | stratified, polynomial | simple: "no exception known" | the current engine as is |
| **E2. Default inheritance with specificity** (inferential distance; Touretzky 1986; Horty, Thomason & Touretzky 1990) | Defaults on kinds; the most specific asserting kind wins; incomparable conflicts block | stratified once compiled (below); about linear in the size of the inheritance relation | tree proofs, with "no more specific contrary" checks | **runs on the current engine today** (section 2.4) |
| **E3. Defeasible logic** (Nute 1994; Antoniou et al. 2001) with strict rules, defeasible rules, defeaters and a superiority relation | Defaults on any rule, not only kinds; team defeat; ambiguity blocking or propagating | linear for propositional theories (Maher 2001); grounding first for Datalog | proof theory with explicit +∂/−∂ tags, explainable | an own evaluator (Maher's algorithm over ground rules) or a meta-program, M-L |
| **E4. Answer set programming** (stable models; Gelfond & Lifschitz 1988; clingo) | The most expressive: choice, constraints, unstratified defaults, credulous and skeptical modes | NP-hard in general; grounding blow-up | stable models carry no proofs; explanations need xclingo (Cabalar et al. 2020) or s(CASP) justifications (Arias et al. 2020) | clingo is C++; a Rust binding exists, but it breaks "runtime Rust" in spirit. Offline only |
| **E5. Well-founded semantics** (Van Gelder, Ross & Schlipf 1991) by alternating fixpoint | Unstratified defaults with a third value `unknown`; skeptical | polynomial | proofs need care (unfounded sets) | a Rust extension of the semi-naive engine, M |
| **E6. Belief revision / truth maintenance** (AGM 1985; Doyle 1979; de Kleer 1986) | Not an inference mechanism: it decides **what to give up** when a counterexample arrives | proof-based, cheap with stored derivations | the culprit set is a proof's support | in Rust, on top of any of E1-E5 |

**Notes on the trade-offs.**
- **E1** is what the existing `crates/core/assets/core/rules.md` does (`has(X, Y) :- is_a(X, C), has(C, Y), not -has(X, Y).`). It handles an exception taught on the instance itself. It fails the penguin case once inheritance goes through more than one level, because `-has` is not inherited. And making `-has` inherit too creates negation through recursion, which the engine rightly refuses (`EvalError::Unstratifiable`).
- **Inheriting along direct links only**, with mutual negation evaluated under WFS (E5), gives "nearest wins" on trees. It is known to break on redundant links: `is_a(tweety, bird)` taught next to `is_a(tweety, penguin)` makes the answer `unknown`. Haiku will teach redundant links, so this matters.
- **E4** is attractive for offline checking. P5 asks for a proof chain for every derived claim, and clingo's models give none without an extra layer. ASP's power (choice rules, optimisation) is not needed for skeptical household answers.
- **E3** is the principled superset of E2. Its one advantage over E2 + layers (below) is a single uniform treatment of conflicts between arbitrary rules. Maher's linear algorithm works on ground theories, so in practice it grounds first, like ASP, but stays polynomial.

### 2.3 Recommendation 2: E2 for kinds, a layered E3 subset for learned rules, E6 for counterexamples

**Conclusion classes and precedence.** For one atom `p(a, b)`, the positive and negative conclusions are compared by source, in this order:

| Rank | Source | Example |
|---|---|---|
| T | taught about the subject itself | `not can_go_in(plastic_tub, oven)` |
| I(k) | inherited from an asserted fact on kind K, ordered by specificity: K₁ beats K₂ if `is_a*(K₁, K₂)` | `can_go_in(tin, oven)` reaching `muffin_tray` |
| L | concluded by a learned (induced) rule, layer by layer | `can_go_in(X, oven) :- made_of(X, metal)` |

**Three semantic rules decide between them.**
1. **Specificity.** A conclusion is defeated by a contrary conclusion of strictly higher rank: T beats I, and within I, the more specific kind beats the less specific.
2. **Skeptical blocking.** If both polarities survive at incomparable ranks, the atom is `conflict` and **neither** holds. The agent asks. This is ambiguity blocking in defeasible-logic terms, and it is what "zero inventions" demands.
3. **Hypotheses never override teaching.** A learned-rule conclusion L that meets a contrary T or I conclusion does not hold. The meeting is recorded as a **counterexample** to the rule, which starts specialisation (section 4.4). Between two learned rules, the one created as a specialisation of the other wins (a recorded superiority pair, as in E3). Otherwise they conflict, and the agent asks.

**Layering.** Learned rules sit in numbered layers (section 4.6). Each layer's conclusions are added as asserted-at-rank-L facts, and inheritance (rule 1) is run again over them, so a learned fact about a kind reaches its subkinds. Every layer is a stratified program, so the whole is a stratified sequence. A learned rule that would create a cycle through negation is refused; `reasoner::quarantine` already does that check for learned rules.

**Why not full E3 or E4.** Rules 1-3 give the behaviour the cases in 2.1 need, run on the current engine, and produce ordinary proof trees. Full defeasible logic adds team defeat and superiority between arbitrary rules, which the household benchmark does not need yet. If the spike finds conflicts that the layers cannot express, E3 is the upgrade path: the precedence table is already a superiority relation.

### 2.4 Exact semantics: the compiled rules, and their output

Relations listed as inheritable (`inheritable(can_do)` and so on) are stored as `pos(X, Rel, Y)` and `neg(X, Rel, Y)`. Inheritance with specificity is this stratified program, in the engine's own syntax:

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
```

**Why it is stratified.** Defeat is decided by **asserted** facts and their positions in the `is_a*` order, never by derived conclusions. So `pre` depends only on `isa`, `pos` and `neg`, and every negation points to a lower stratum. This is the key design choice. It lets the program run on the current stratified engine with no change to the evaluator.

**What preemption does, case by case.**
- The positive from K is preempted if X itself has the negative, or if a strictly more specific ancestor K₂ of X has it.
- Preemption by K₂ is correct even when K₂'s own negative is in turn preempted for X by an even more specific positive K₃. In that case K₃'s positive is live and wins, so the most specific asserter always decides along a chain.
- Redundant links (`is_a(tweety, bird)` next to `is_a(tweety, penguin)`) change nothing, because specificity uses `isa*`, not path length.

The program and the examples below were run on `sla_reasoner` as it stands. The program is in `scratchpad/bench/ex1.dl` + `base.dl`; the outputs are quoted verbatim, shortened only by "…".

**Penguin with a redundant link.** Taught:
```
is_a(bird, animal). is_a(penguin, bird). is_a(tweety, penguin). is_a(tweety, bird).
pos(bird, can_do, fly). neg(penguin, can_do, fly). pos(superpenguin, can_do, fly).
is_a(superpenguin, penguin). is_a(pingu, superpenguin).
```
```
? holds(tweety, can_do, fly)  -> 0 answer(s)
? holdsn(tweety, can_do, fly)  -> 1 answer(s)
holdsn(tweety, can_do, fly) by rule: holdsn(X, P, Y) :- liven(X, P, Y, K), not conflict(..), not pos(..).
  liven(tweety, can_do, fly, penguin) by rule: liven(..) :- inhn(..), not pren(..).
    inhn(tweety, can_do, fly, penguin)
      isa(tweety, penguin) <- is_a(tweety, penguin) given
      neg(penguin, can_do, fly) given
    not pren(tweety, can_do, fly, penguin) (no such fact known)
  not conflict(tweety, can_do, fly) (no such fact known)
? holds(pingu, can_do, fly)  -> 1 answer(s)   [live via superpenguin; bird's default preempted]
```

**Nixon diamond.** `is_a(nixon, quaker)`, `is_a(nixon, republican)`, with `pos(quaker, has_property, pacifist)` and `neg(republican, has_property, pacifist)`:
```
? conflict(nixon, P, Y)  -> conflict(nixon, has_property, pacifist)
? holds(nixon, has_property, pacifist)  -> 0 answer(s)
```
The agent's turn: `ask: has_property(nixon, pacifist)? quaker says yes, republican says no.`

**Muffin tray, the preplanning's worked example.**
```
holds(muffin_tray, can_hold, batter) by rule: holds(..) :- live(..), not conflict(..), not neg(..).
  live(muffin_tray, can_hold, batter, baking_tin)
    inh(muffin_tray, can_hold, batter, baking_tin)
      isa(muffin_tray, baking_tin) <- is_a(muffin_tray, baking_tin) given
      pos(baking_tin, can_hold, batter) given
    not pre(muffin_tray, can_hold, batter, baking_tin) (no such fact known)
```

**Plastic tub, taught** (the preplanning's dialogue). The facts are `is_a(plastic_tub, tin)`, `pos(tin, can_go_in, oven)`, and then the teacher's `neg(plastic_tub, can_go_in, oven)`:
- Before the negative arrives, `holds(plastic_tub, can_go_in, oven)` is derived through `tin`.
- At entry, the agent finds that the new negative contradicts a **derived** conclusion whose proof contains one defeasible link (the inheritance from `tin`). This is the E6 moment (section 2.5).
- If the answer is "exception", the negative is stored. The result is `holds(plastic_tub, …) -> 0` and `holdsn(plastic_tub, can_go_in, oven)` given. No other conclusion changes: muffin trays still go in the oven.

**A subkind that inherits a different exception.** With `is_a(silicone_tray, muffin_tray)` and `neg(silicone_tray, made_of, metal)`, the result is `holdsn(silicone_tray, made_of, metal)` given, and `holds(silicone_tray, can_go_in, oven)` through `tin`. The answer happens to be right, but for the wrong reason. A silicone tray goes in the oven because silicone withstands heat, not because it is a tin.

Inheritance cannot express "because of its material"; only a rule over `made_of` can (section 4.3). This is a good spike probe: the benchmark should contain cases where kind-inheritance and material-rules disagree, to see whether the agent asks or answers.

**Plastic tub, learned.** Suppose an induced rule r1 is `can_go_in(X, oven) :- is_a(X, container)` (layer 1, support 6, section 4.3), and `plastic_tub` is a container with no taught oven fact:
- r1 concludes `can_go_in(plastic_tub, oven)` at rank L. The answer is shown as a hypothesis, with r1's support.
- The teacher then says `not can_go_in(plastic_tub, oven)`. That T conclusion beats L (semantic rule 3), so the answer is now `no`.
- r1 gains a counterexample and is specialised to r1′ = `can_go_in(X, oven) :- is_a(X, container), not made_of(X, plastic)`, recorded as superior to r1. With one counterexample, the specialisation is the instance exception `not X = plastic_tub`, which section 4.4 then generalises.

### 2.5 Belief revision when a counterexample is taught

When a taught literal contradicts a derived one, the agent has the derived literal's proof π. The candidates for giving something up are:
- **(a)** add an exception: store the new literal, which beats π by specificity;
- **(b)** retract a taught leaf of π, because the teacher was wrong earlier (for example, the plastic tub is not a tin);
- **(c)** if π uses an induced rule, specialise or retract that rule.

This is kernel contraction (Hansson 1994): give up at least one element of each minimal proof of the unwanted literal. The proofs come from stored derivations, as in a JTMS (Doyle 1979).

**Policy.**
1. Taught on the instance is never silently overridden by anything derived.
2. If π contains an induced rule, choose (c) automatically: hypotheses yield first. Report it: `rule r1 specialised: counterexample plastic_tub`.
3. If π is all taught facts plus inheritance links, ask the teacher, listing (a) and the leaves for (b). The agent proposes (a) first: it is the minimal change, and it keeps every other conclusion. So the dialogue is the preplanning's `exception for plastic_tub? (or is is_a(plastic_tub, tin) wrong?)`.
4. A retraction marks the fact `retracted_by` the message, as the kb3 vault already does. Nothing is deleted.
5. After any revision, the agent recomputes only the strata that depend on the changed predicates (`reasoner::depends_on` exists). It reports conclusions that were withdrawn: "withdrawn: `holds(plastic_tub, can_go_in, oven)`".

---

## 3. The universal base rules

### 3.1 The candidate set

Each rule is tagged **L** (logic of the representation: true of any domain and any schema of this shape) or **S** (semantics of the procedure schema: domain-free, since it names no household constant, but it encodes what `role`, `demands` and `effect` mean; such a rule could in principle be learned). Every rule is written once, as data, in a core rules file. That is P6.

| Id | Tag | Rule |
|---|---|---|
| B1 | L | `is_a` is transitive (`isa` above) |
| B2 | L | Default inheritance with specificity over `inheritable(R)` relations (the 13 rules of 2.4). The `inheritable` list is part of the schema: `has_part`, `made_of`, `has_property`, `can_do`, `can_do_on`, `used_for`, `can_go_in`. `effect` and `precondition` inherit through their object argument (a second, symmetric copy with the subject in third place) |
| B3 | L | Contradiction: `contradiction(X, P, Y) :- pos(X, P, Y), neg(X, P, Y).` and `conflict` from B2 both raise a question; neither is ever answered |
| B4 | L | `same_as` is an equivalence that substitutes everywhere, **implemented by canonicalisation** (section 7), not by Datalog rules |
| B5 | L | `has_part` is transitive, with demand-driven evaluation only (section 6): `has_part*(X, Z) :- has_part(X, Y), has_part*(Y, Z).` |
| B6 | S | Bridges: `holds(I, can_do, F) :- role(S, I, F).`, `holds(X, can_do, F) :- holds(X, used_for, F).`, `holds(X, can_do, F) :- holds(X, can_do_on, F, _)` (the last as a 4-argument reified variant). Defeasible: entered at rank I of the subject itself, so a taught negative beats them |
| B7 | S | Stand-in by function, with the demands check and an honest gap (below) |
| B8 | S | Step replacement by effect (below) |
| B9 | S | Gap: a lacked item with no stand-in and no replacement becomes `gap(S, I)`. The answer states the gap and the agent asks the teacher |

**B7, stand-in (the engine's syntax; reified `holds` from B2):**
```datalog
cand(S, I, J, F) :- lacks(user, I), role(S, I, F), has(user, J), holds(J, can_do, F), J != I.
unfit_known(S, J) :- cand(S, I, J, F), demands(S, F, P, Y), holdsn(J, P, Y).
fit_open(S, J)    :- cand(S, I, J, F), demands(S, F, P, Y), not holds(J, P, Y), not holdsn(J, P, Y).
standin(S, I, J)  :- cand(S, I, J, F), not unfit_known(S, J), not fit_open(S, J).
ask_fit(S, J)     :- cand(S, I, J, F), not unfit_known(S, J), fit_open(S, J).
```
The distinction between `unfit_known` (a proven no) and `fit_open` (unknown) is what keeps B7 honest:
- If nothing is known about whether a muffin tray goes in the oven, the agent does not answer "swap". It asks: `ask: can_go_in(muffin_tray, oven)?`.
- The answer is taught knowledge, and it closes the gap for every later task.

This is the active learning of P3, driven by the agent's own gaps. In the benchmark run (section 6), 16 of 71 candidates at 500 facts end as `ask_fit`.

**B8, step replacement by effect:**
```datalog
needed_state(S1, Z) :- before(S1, S2), requires_state(S2, Z), achieves(S1, Z).
blocked(S1) :- uses(S1, I), lacks(user, I), not some_standin(S1, I).
replace(S1, A2) :- blocked(S1), needed_state(S1, Z), object_of(S1, O),
                   holds_effect(A2, O, Z), not lacks_for(A2, O).
```
`lacks_for(A2, O)` holds when the action's own demands (taught on the action, `demands_action(A2, F, Rel, V)`) cannot be met by the user's items. It runs the same `unfit_known` / `fit_open` split as B7. `achieves(S1, Z)` is itself derivable from `action`, `object_of` and `effect` (bridge 3, part of B6).

**Rejected candidate: "a part can do what its whole can do."** It is false: a pan can fry, its handle cannot. It was dropped before testing.

### 3.2 Which rules are truly domain-free

- **B1-B5 are logic.** They hold whatever the domain, because they are the meaning of `is_a`, negation, defaults, equality and parthood. Transitivity of parthood is known to fail for *functional* parthood: a handle is part of a door, and a door is part of a house, yet the handle is not a functional part of the house (Varzi, SEP "Mereology"). It is kept here because household questions use structural parthood, but its ablation is reported.
- **B6-B9 are schema semantics.** They name no household constant, and they would apply unchanged to gardening or bicycle repair. But they make substantive assumptions:
  - B6 says that doing something once proves an ability.
  - B7 says that one function plus the demands is enough for substitution.
  - B8 says that states are what later steps need.

  Each is exactly the kind of rule the rule learner (section 4) can try to rediscover from taught swaps. **The spike should test that directly:** switch B7 off, teach the swap cases, and see whether induction returns B7 or a near variant. If it does, B7 moves from built-in to learned, which strengthens P7 and makes P6 smaller.

### 3.3 Showing each rule's contribution

Four measures, all zero-call:
1. **Knock-out ablation.** Switch off rule Bk and re-run the benchmark. Report Δcorrect, Δinventions and Δgaps per hop count. B2 off will remove almost everything, so knock-outs alone exaggerate the rules every proof passes through.
2. **Unique contribution.** The number of questions whose every valid proof uses Bk. This is computed from proofs, with no re-run: the engine's `Proof::rules_used` already lists rule ids. A later version should enumerate alternative proofs, not only the first.
3. **Harm.** The number of wrong answers and inventions whose proof uses Bk. A rule that pays must have unique contribution above 0 and harm at 0, or harm below its unique contribution, with each harm case explained.
4. **Strength check for B2.** Run with B2 replaced by E1 (instance-level exceptions only) and by monotone inheritance (no exceptions). The spike's exception questions should separate the three. This answers the "too strong or too weak" risk directly.

The preplanning's rule, "keep only rules that pay", becomes: keep Bk if (unique contribution ≥ 1 question per 100) and (harm = 0), measured on the development part.

---

## 4. Rule learning, layer on layer

### 4.1 What is learned

There are two kinds of target, which need different examples:

| Target | Head | Examples come from | Example rule |
|---|---|---|---|
| **Item rules** | an item relation (`can_go_in`, `can_do`, `has_property`) | the item facts themselves: taught positives and taught negatives | `can_go_in(X, oven) :- made_of(X, metal)` |
| **Adaptation rules** | `swap`, `replace`, `drop` | taught adaptation cases (situation + difference, as in v3 variants) | `swap(S, I, J) :- role(S, I, F), lacks(user, I), has(user, J), can_do(J, F)` |

Item rules are the bigger win. They do not depend on situations repeating, which killed the 3.0 rules arm (X4: 1.4%). Instead, they generalise over the many item facts that the new curriculum teaches. They are rule mining in the sense of AMIE (Galárraga et al. 2013) and AnyBURL (Meilicke et al. 2019), applied to a taught knowledge base.

### 4.2 Options

| Method | How it works | Fit here | Where it runs |
|---|---|---|---|
| Lifted-pattern counting (3.0 rules arm) | count identical diffs, lift constants | needs repetition, which household data lacks | Rust (exists) |
| **Least general generalisation** (Plotkin 1970; Golem, Muggleton & Feng 1990) | bottom-up: the most specific clause covering two examples | very data-efficient (2 examples suffice); overgeneralises without negatives; clauses grow with background size, so bottom clauses must be cut to depth ≤ 2 | Rust, S-M |
| **FOIL-style top-down refinement** (Quinlan 1990) | greedy add literals by information gain over positives and negatives | simple, handles negatives; greedy misses rules that need two literals at once | Rust, M |
| **AMIE-style rule mining** (Galárraga et al. 2013) | enumerate closed rules up to length 3, score by support, head coverage and PCA confidence (open world) | right for item rules on an incomplete knowledge base; exhaustive at this size | Rust, M |
| **Popper** (Cropper & Morel 2021) | generate, test, constrain, with ASP; recursion and optimal (smallest) programs | the strongest generic ILP; Python + clingo + SWI-Prolog | offline only |
| **Metagol** (Muggleton et al. 2015) | meta-interpretive learning from metarules, predicate invention | needs hand-chosen metarules (a form of bias); Prolog | offline only |
| **FOLD-R++** (Wang & Gupta 2022) | learns default rules **with exceptions** (`not ab(X)`) as normal logic programs | matches the exception semantics of section 2 closely | offline (Python) |
| **ILASP** (Law, Russo & Broda 2014) | learns ASP programs, including defaults | powerful, slow, ASP semantics | offline only |
| **Explanation-based generalisation / composition** (Mitchell, Keller & Kedar-Cabelli 1986) | compile a used proof into a macro rule | speeds up, adds no new knowledge (the deductive closure is unchanged) | Rust, S |

### 4.3 Recommendation 3: a Rust inducer of two parts, with Popper and FOLD-R++ as offline yardsticks

**Part A. Item rules by bounded refinement (AMIE/FOIL hybrid).** For each target `R(X, c)` with at least 3 taught positives:
1. Language: the body has at most 3 literals over the item relations and their `holds` forms. Variables are typed by sort, and every variable must appear twice (closed rules). Constants are allowed only in positions where they appear in at least 3 positives. Negated literals are allowed only in the exception step (4.4).
2. Search: breadth-first over refinements (add a literal, bind a variable to a constant). Prune by the minimum support (see 4.5), which is anti-monotone.
3. Scoring, for each candidate body B:
   - **support** s = the number of distinct X with B(X) and taught `R(X, c)`;
   - **known negatives** n = the number of distinct X with B(X) and taught `not R(X, c)`;
   - **PCA confidence** = s / (s + n′), where n′ counts the X that have some taught value of R but not c. This is AMIE's partial completeness assumption: an item with no taught value is unknown, not negative. For the binary-ish relations here (can or cannot go in the oven), n′ = n;
   - **Wilson lower bound** at 95% on s/(s+n), so that 3/3 is not treated as certain.
4. Keep the most general rules that pass the thresholds. Drop a rule whose positives are a subset of a kept rule's, and which has no better confidence.

**Part B. Adaptation rules by lgg over cut bottom clauses.** For each taught case (situation, difference), build its bottom clause:
- the difference literal (`swap(S, I, J)`) as head;
- as body, every fact within 2 hops of S, I and J in the `holds` model: their kinds, roles, can_do, made_of, demands, the user's has and lacks.

Then take pairwise lgg across cases of the same difference type, reduce each result (drop literals whose variables are unconnected to the head), and score as in Part A.

The examples come only from taught cases and the agent's own confirmed answers: when the teacher accepts an answer, it becomes an example. This is the "gets better as taught more" loop.

**Worked example A: what can go in the oven.** The taught facts (a slice of a baking topic):
```
can_go_in: loaf_tin, muffin_tray, roasting_pan, cast_iron_pan (made_of metal);
           pyrex_dish (glass); ramekin (ceramic)                        -> 6 positives
not can_go_in: plastic_tub (plastic), wooden_board (wood)              -> 2 negatives
unknown: steel_bowl (metal), plastic_jug (plastic), glass_jar (glass), silicone_tray
all of these: is_a(_, container) except wooden_board (is_a board)
```

| Candidate body for `can_go_in(X, oven)` | s | n | Wilson lower bound | New predictions |
|---|---|---|---|---|
| `is_a(X, container)` | 6 | 1 (tub) | 0.49 | steel_bowl, plastic_jug, glass_jar, silicone_tray |
| `made_of(X, metal)` | 4 | 0 | 0.51 | steel_bowl |
| `made_of(X, glass)` | 1 | 0 | below support | glass_jar |
| `is_a(X, container), not made_of(X, plastic)` (exception step) | 6 | 0 | 0.61 | steel_bowl, glass_jar, silicone_tray |
| `is_a(X, container), made_of(X, M), heat_safe(M)` | needs a fact `has_property(metal, heat_safe)`, which is not yet taught | | | |

With support ≥ 3 and the starting threshold of 4.5 (Wilson ≥ 0.6), only the exception rule passes; at 0.5, `made_of(X, metal)` would pass too, and `is_a(X, container)` fails either way. Two points follow:
- **The thresholds need data.** 6 examples give at most about 0.61 at 95%. The preplanning's rule that thresholds are set after the spike applies to these numbers too.
- **The last row is the interesting failure.** The general rule would be "things made of a heat-safe material", which needs a fact about materials. The agent can see this. Its best rule has two disjunct-like specialisations (metal, glass, ceramic), and a candidate with a material variable is blocked only by a missing `has_property(M, heat_safe)`. That is exactly a gap question: `ask: has_property(metal, P) and has_property(glass, P) for some shared P?`. Asking the teacher for the common property is how "learning layer on layer" gets a new layer without inventing one.

**Worked example B: stand-ins for a whisk** (adaptation rules). The taught cases:
```
c1 omelette_s2:  lacks whisk, has fork       -> swap(omelette_s2, whisk, fork)
c2 pancake_s1:   lacks whisk, has chopsticks -> swap(pancake_s1, whisk, chopsticks)
c3 scramble_s1:  lacks whisk, has fork       -> swap(scramble_s1, whisk, fork)
c4 gravy_s3:     lacks sieve, has whisk       -> swap(gravy_s3, sieve, whisk)   (role: remove_lumps)
```
The cut bottom clause of c1 contains:
```
role(omelette_s2, whisk, beat), lacks(user, whisk), has(user, fork),
holds(fork, can_do, beat), holds(fork, can_do, pierce), holds(fork, made_of, metal), is_a(fork, cutlery),
object_of(omelette_s2, egg), action(omelette_s2, beat)
```
- lgg(c1, c2) = `swap(S, whisk, J) :- role(S, whisk, beat), lacks(user, whisk), has(user, J), holds(J, can_do, beat), object_of(S, egg), action(S, beat)`. Here `pierce`, `metal` and `cutlery` drop out because chopsticks do not share them.
- lgg with c4 lifts `whisk` and `beat`: `swap(S, I, J) :- role(S, I, F), lacks(user, I), has(user, J), holds(J, can_do, F)`.

**That is B7 without the demands check: the learner rediscovers the stand-in rule from four cases**, from three tasks and two domains. The check in 3.2 is exactly this experiment.

**A counterexample.** The teacher then teaches `not swap(meringue_s2, whisk, fork)`: a fork cannot beat whites to stiff peaks. The rule meets a counterexample, and section 4.4 applies. The distinguishing literal is `until(meringue_s2, stiff_peaks)`, present in the negative and absent from all positives. The specialised rule gets `not until(S, stiff_peaks)`. With one negative, this exception is case-specific. The agent therefore asks why (`ask: what does meringue_s2 demand of the beater that fork lacks?`), and the answer `demands(meringue_s2, beat, can_do, whip_air)` turns an ad-hoc exception into knowledge that B7 already uses.

**Why both parts are Rust.** At household scale (at most a few hundred taught examples per target, at most 50,000 facts), bounded refinement to length 3 with sorts is a few thousand candidate bodies per target. Each one is scored by a join on the existing engine (`Model::query`). That is milliseconds to seconds.

Popper finds globally smallest programs, recursion and invented predicates, but it needs clingo and SWI-Prolog. Neither is installed now (checked: `import clingo` fails, and there is no `swipl` on the path). P-constraints say Python offline only. So the **runtime learner is Rust**, and Popper and FOLD-R++ run **offline on exported spike tasks** as a yardstick: for each target, did the Rust inducer find the rule Popper finds? Any rule only Popper finds is evidence for adding a capability (for example recursion). Popper is not imported into the agent.

### 4.4 Specialisation on counterexamples

The procedure, for a rule r with body B and counterexample e (a taught negative that B covers):
1. Gather the distinguishing literals: literals in e's cut bottom clause whose generalisation (constant → variable bound in B) is **false** for all, or at least 90% of, r's positives (Part A for item rules, lgg for cases). The positive literals that hold for all positives but not for e, such as `made_of(X, metal)` above, come from the opposite direction. Prefer positive literals (they make the rule more specific without negation); use `not L` (FOLD-R style) only when no positive literal separates.
2. Create r′ = B ∧ the literal that keeps the most support. Record `superior(r′, r)` and `specialised_from(r′, r, e)`. Keep r as retracted-by-specialisation in provenance, never deleted.
3. If no literal separates e at all, add the instance exception `not X = e`. It is rendered as "known exception: e". When a second instance exception arrives, run step 1 over the exceptions themselves: lgg(plastic_tub, plastic_jug) gives `made_of(X, plastic)`, and the two instance exceptions become one learned exception literal. **The exception list becomes a learned rule.** This mirrors Shapiro's (1983) contradiction backtracking and FOLD-R++'s nested exceptions.
4. If r′ falls below the thresholds, r′ is kept as a "weak" rule. It is not used in answers, but it is still scored as new examples arrive.

### 4.5 Thresholds and avoiding overgeneralisation

Starting values, to be reset after the spike (P16):

| Guard | Starting value |
|---|---|
| Support | ≥ 3 distinct head subjects |
| Spread | from ≥ 2 distinct taught tasks or ≥ 2 distinct kinds (`is_a` parents) |
| Confidence | Wilson 95% lower bound ≥ 0.6 for use in answers; ≥ 0.8 to answer without "hypothesis" hedging |
| Known counterexamples | 0 uncovered by specialisation |
| Constants in body | only if present in ≥ 3 positives |
| Body length | ≤ 3 literals (≤ 4 after one specialisation) |
| Prediction budget | a rule that would add more than 5× its support in new conclusions is held for a confirming query first |

**The confirming query** is the strongest guard. For a newly accepted rule, the agent asks the teacher one membership query on its most surprising prediction: "`can_go_in(steel_bowl, oven)`?" (Angluin 1988). A "no" is a counterexample, which starts 4.4. A "yes" adds support and records the rule as `confirmed_by` a teaching turn. Each new rule costs about one call. With rules batched several per call, as in the preplanning's batching, 100 rules cost about 30 calls, well within the 20% question share.

### 4.6 How learned rules build on learned rules

Layers:
- **Layer 0** is the base rules.
- A learned rule's layer is 1 + the maximum layer of any learned predicate or rule its body depends on.

The facts it concludes are rank L at that layer (section 2.3), and inheritance runs over them before the next layer. Each induction round adds the conclusions of accepted rules to the background the next round searches over.

A three-layer chain on household data:
1. Layer 1: `can_go_in(X, oven) :- made_of(X, metal)`.
2. Layer 2: `can_do_on(X, hold, batter) :- is_a(X, tray), can_go_in(X, oven)`, induced from taught trays and tins. Its body uses layer 1's conclusions.
3. Layer 3: the stand-in instance `swap(banana_bread_s2, loaf_tin, steel_tray)`, through B7 using layer 2's `can_do_on`. Here `steel_tray` was taught only as `made_of(steel_tray, metal)` and `is_a(steel_tray, tray)`.

The answer is a 3-hop derivation in which no hop was taught for steel trays. That is the measure of record (P15).

**Errors compound.** Confidence of a conclusion is the product of the rule confidences on its path (section 5.3), and conclusions under 0.5 are offered as questions, not answers.

**Reporting.** The rule layer is reported as P7 asks: a table of rules by layer, with `built_from` edges, support, confidence, counterexamples and specialisations. Composition (explanation-based macro rules) is **not** in the spike. It adds speed, not knowledge, and section 6 shows speed is not the problem below 50,000 facts.

---

## 5. Proofs and explanations

### 5.1 Proof records

The engine keeps one derivation per fact (`Derivation::Rule { rule, premises, absent, conf }`) and renders a tree (`Proof::render`). The format to add on top is one JSON record per answer:

```json
{"claim": "swap(banana_bread_s2, loaf_tin, muffin_tray)",
 "kind": "deduced",                     // given | deduced | hypothesis
 "conf": 0.95, "kb_version": "sha256:…", "rules_used": ["B7", "B2", "B1"],
 "tree": {"atom": "standin(banana_bread_s2, loaf_tin, muffin_tray)", "rule": "B7",
          "premises": [ {"atom": "role(banana_bread_s2, loaf_tin, hold)", "given": "raw:s3/t14"},
                        {"atom": "holds(muffin_tray, can_do, hold)", "rule": "B2",
                         "via_kind": "baking_tin", "premises": [ … ],
                         "checked_absent": ["no contrary on muffin_tray or a kind between it and baking_tin"]},
                        {"atom": "holds(muffin_tray, can_go_in, oven)", "rule": "B2", "via_kind": "tin", … } ],
          "checked_absent": ["not holdsn(muffin_tray, can_go_in, oven)"]}}
```

Leaves carry provenance ids into the raw layer (`raw:session/turn`), so every answer traces to the teaching messages that support it (P11).

**For an induced-rule application**, the node also carries:
```json
{"rule": "L2:r17", "layer": 2, "built_from": ["L1:r4"], "support": ["loaf_tin", "muffin_tray", "roasting_pan"],
 "support_msgs": ["raw:s2/t3", "…"], "counterexamples": [{"x": "plastic_tub", "handled_by": "L2:r17 specialised"}],
 "confirmed_by": "raw:s5/t9", "conf": 0.72}
```

### 5.2 Rendering

The raw tree for an inherited fact has five internal nodes (`isa`, `inh`, `live`, `not pre`, `not conflict`), as in the 2.4 output. The rendering collapses them into one line per inheritance:
```
can_hold(muffin_tray, batter)  inherited from baking_tin
    is_a(muffin_tray, baking_tin) [s1/t12]; can_hold(baking_tin, batter) [s1/t7]
    no exception known for muffin_tray or kinds between
```

The "no exception known" line must stay. It is the defeasible assumption, and a later teaching can change it. A hypothesis is rendered with its rule in English-free formal form and its support count:
```
can_go_in(steel_bowl, oven)?  hypothesis (rule L1:r4, 4 examples, conf 0.51)
```

### 5.3 Confidence

| Source | Confidence |
|---|---|
| Taught | the teacher's confidence in the provenance; 1.0 for structured teaching, unless the teacher marks a fact uncertain |
| Base rule | 1.0 |
| Learned rule | the Wilson lower bound from 4.5, re-estimated as examples arrive |
| Derived | min over taught leaves × product of the learned-rule confidences on the path |

The engine today uses min for everything (`conf = prem.iter()…fold(r.conf, f64::min)`). Min is right for taught facts, which are not independent sources of doubt. It is wrong for chains of hypotheses: two 0.6 rules in a chain are weaker than one. So the change is small: tag each rule as base or learned, and multiply only learned confidences. This is the Viterbi (max-times) semiring view of provenance (Green, Karvounarakis & Tannen 2007), restricted to one proof.

**Best proof, not first proof.**
- Semi-naive evaluation inserts a fact the first round it can be derived, so the stored proof has minimal height within a stratum. That matches Soufflé's provenance, which picks minimal-height proofs (Zhao, Subotić & Scholz 2020).
- Because learned layers run after the base strata, any fact provable without hypotheses is stored with a deduced proof first. A hypothesis proof never shadows a deduction.
- Between two hypothesis proofs, the first found may not be the most confident. The spike can live with that; the fix is to re-insert when a higher-confidence derivation arrives, which is safe because confidence is bounded and monotone.

### 5.4 Proof checking

**An independent proof checker is required** for the benchmark metric "proof validity". It is a small Rust function that re-validates a JSON proof against the knowledge base at `kb_version`:
- each leaf is a live taught fact;
- each node is an instance of its rule with the stated bindings;
- each `checked_absent` holds;
- each learned rule meets the thresholds at that version.

The checker shares no code with the evaluator's join loop. That is what makes "0 inventions" a measured claim rather than an assumption.

---

## 6. Scaling

### 6.1 Benchmark

**Setup.** The existing `sla_reasoner` (release build) ran the base program of 2.4 plus B7, on a synthetic household-shaped knowledge base:
- an `is_a` forest of depth ≤ 6 over 22% of the facts;
- inheritable positives on random kinds, with 6% of them followed by a lower exception;
- 4-step procedures with roles and oven/microwave demands;
- one user situation (5 lacked items, 40 owned).

The machine is an AMD Ryzen 5 5500U, single-threaded. The scripts are in the scratchpad (`/tmp/claude-1000/-home-maksasj-Programming-Charlotte/14cb3496-9f01-4ef6-8a28-09423c8bb681/scratchpad/bench/`: `gen.py`, `base.dl`, `demand.dl`, `src/main.rs`). "Demand" is a hand-written magic-set-style rewrite: `isa` is computed only for items in the situation and their ancestors.

| Taught facts | Mode | Derived facts | Evaluation | Peak memory |
|---|---|---|---|---|
| 518 | full closure | 2,935 | 13 ms | n/m |
| 518 | demand | 2,330 | 10 ms | n/m |
| 5,014 | full closure | 44,408 | 268 ms | 73 MB |
| 5,014 | demand | 7,837 | 41 ms | n/m |
| 50,018 | full closure | 556,597 | 3,878 ms | 922 MB |
| 50,018 | demand | 36,319 | 253 ms | 120 MB |

Two stress tests:
- **`same_as` by axioms.** Axiomatising `same_as` (symmetric, transitive, substitution in `pos` and `is_a`) for 10% of the kinds in the 5k knowledge base, in classes of 3 names, gave +38% derived facts (61,456) and 1.8× the time (480 ms).
- **A long chain.** A transitive closure over one `part_of` chain of 2,000 links derived 2,001,000 facts in 10.3 s.

### 6.2 Reading

- **The existing reasoner is enough for the spike and the first scaling steps.** At 500 and 5,000 facts, the full closure takes under 0.3 s. It can be recomputed after every teaching message, so "derived: …" in the dialogue is affordable.
- **At 50,000 facts, the full closure is the wrong tool.** It takes 3.9 s and 0.9 GB per recompute, about 1.7 KB per derived fact: each derivation stores its premises as cloned `GroundAtom`s with `String` symbols. The demand slice answers the same question in 0.25 s with 120 MB. Its answers on the question's items are identical, because inheritance only reads ancestors and B7 only reads the situation's items.
- **Transitive relations over long chains are quadratic.** The `is_a` hierarchy is shallow (household kinds, depth ≤ 8), so B1 is safe. `has_part`, `before` and `same_as` must not be materialised by rules, which is why B4 is canonicalisation and B5 is demand-only.
- **Memory, not time, is the first limit.** Interning symbols (`Value::Sym(String)` → `u32`) and storing premises as fact ids instead of atoms would cut memory roughly 5-10×. That is an estimate, not measured.

### 6.3 Options and recommendation

| Option | What | Effort | When |
|---|---|---|---|
| Forward closure after each message | current engine, as is | 0 | ≤ 5,000 facts |
| Incremental insert | semi-naive with delta = the new fact. Insertions into `naf_sensitive` predicates (which exist in the engine) force recompute of the dependent strata only | S-M | when per-message recompute exceeds ~1 s |
| Deletion / retraction | DRed (Gupta, Mumick & Subrahmanian 1993), or recompute the affected strata | M | with incremental insert |
| Demand slice per question | collect the question's and situation's constants, close upward over `is_a`, `same_as` canonical names and `has_part` to depth d (d = body length × layers), then evaluate on facts about those constants only | S | ≥ 5,000 facts, and for every benchmark question |
| Magic-sets rewriting (Bancilhon et al. 1986; Beeri & Ramakrishnan 1991) | automatic version of the slice | M-L. Magic sets can break stratification under negation and need care | only if hand slicing proves wrong |
| Tabling / SLG (Chen & Warren 1996) | goal-directed with memo tables, as in XSB; also gives proofs top-down | L (a second evaluator) | not needed |
| Engine upgrades | multi-column hash indexes (there is only a first-argument index now: `Rel::by_first`), interned symbols, premise ids | M | before 50,000 facts |
| Replace the engine (Soufflé, `ascent`, `datafrog`, differential Datalog) | faster, but provenance and runtime-loaded rules differ | L | not needed |

**Recommendation 4.**
- For the spike, use the current engine with forward closure.
- From the first scaling step, add the demand slice for questions (S) and interning plus premise ids (M).
- Add incremental maintenance only if the teaching loop becomes slow. At 5,000 facts and 268 ms per recompute, a 1,500-message curriculum costs about 7 minutes of recompute, which is fine.

---

## 7. Naming drift

### 7.1 The problem

Haiku names one thing several ways: 3.0 finding 4 (`shelve_and_drawer` against `shelf`), and the preplanning's `whisk`, `egg_whisk`, `balloon_whisk`. In a reasoning system, drift is worse than in recall. Two names split the support of every rule and break every chain through the item: `can_do(egg_whisk, beat)` never reaches a step whose role filler is `whisk`.

### 7.2 Options

| Option | Effect | Risk |
|---|---|---|
| V1. Prevent at the source: each teaching prompt includes the current vocabulary for its topic ("use these symbols where they fit; introduce a new one only for a new thing") | the cheapest fix; most drift disappears before entry | prompt length, kept to a per-topic list (~100 symbols) |
| V2. Detect and ask: a new symbol is compared with known symbols of the same sort | the agent asks `same_as(egg_whisk, whisk)? or is_a(egg_whisk, whisk)? or different?` | question budget; similarity must not be a word list |
| V3. Learn `same_as` automatically from shared relations | no calls | invents identity; violates the "no claim without support" rule unless confirmed |
| V4. Axiomatise `same_as` in rules | correct | +38% facts and 1.8× time already at 10% aliasing (section 6.1); worse with larger classes |
| V5. Canonicalise: union-find over names; facts rewritten to the class representative at entry (Motik et al. 2015, as in RDFox) | linear, no blow-up; one name per class in reasoning | must keep the original name for the raw layer and for rendering |

### 7.3 Recommendation 5: V1 + V2 + V5, never V3 without the teacher

**Similarity for V2.** It uses only structure and the symbols' own parts; no word list is needed, consistent with the 3.0 no-word-list test:
- same sort;
- a shared head token (`egg_whisk` / `whisk`) or edit distance ≤ 2;
- Jaccard of neighbourhoods: shared `is_a` parents, `can_do`, `made_of`, and roles in steps of the same action.

Ask if the combined score passes a threshold set on the spike's injected-drift test.

**The three answers are different knowledge.**
- `same_as` means one thing with two names. The classes are merged.
- `is_a(balloon_whisk, whisk)` means a kind of whisk. It stays a distinct symbol and inherits.
- "different" is recorded as `distinct(egg_whisk, whisk)`, so the agent never asks again.

**Canonicalisation (V5).**
- Union-find over `same_as`. The representative is the earliest-taught name.
- Facts are rewritten at entry and re-rewritten when classes merge.
- The raw layer keeps the original text (P11), and the facts layer stores both the canonical fact and an alias table `alias_of(name, representative)`.
- A merge is undoable: retracting the `same_as` re-splits the class by replaying entry from raw, which is what the replay tool is for.

**Two guards.**
- A merge that would create a contradiction (`can_go_in(a, oven)` and `not can_go_in(b, oven)` in one class) is refused, and the agent asks again with the conflict shown.
- `same_as` across sorts is rejected by the type check.

**The spike test needs no model calls.** Rewrite 10-20% of the item symbols in the hand-written teaching messages to synonyms, and measure the benchmark with V5 off, V5 on with oracle answers, and V5 with the similarity detector's questions answered by the oracle.

---

## 8. The spike: what to build, in order

The scope is as in preplanning section 6: offline, zero Haiku calls, 300-500 hand-checkable facts in 3-4 domains written as teaching messages and fed through the conversation path, and 50-100 questions.

| # | Piece | Builds on | Difficulty | Done when |
|---|---|---|---|---|
| 1 | **Schema v4:** the 11 relations, sorts, `role`/`demands`/`effect`, negated facts; the type checker; relation-line parsing (extend `kb3::fact::SCHEMA`) | kb3 | S | every hand-written teaching line checks or is rejected with a reason |
| 2 | **Core rules file v4:** B1-B9 in the engine's syntax, the reification `pos`/`neg` ↔ relation lines, the `inheritable` declarations | reasoner, core | S | the 2.4 examples reproduce their outputs as Rust tests |
| 3 | **Teaching path:** per message: check → contradiction and conflict detection → provenance → recompute → show new derivations (top 5) → gap questions (`ask_fit`, `gap`, `conflict`) answered by an oracle file in the spike | 1, 2 | M | the preplanning's dialogue in section 3 runs end to end |
| 4 | **Revision (E6):** exception / retract / specialise choices from the proof; withdrawn conclusions reported | 3 | M | the plastic-tub dialogue and a retraction case pass |
| 5 | **Proof records + collapsed renderer + independent checker** | 2 | M | every benchmark answer has a JSON proof that the checker validates |
| 6 | **Benchmark scorer:** correct, inventions, gaps, wrong, proof validity, per hop and domain; deny-list leakage check | 5 | M | scores the gold answers 100% and an empty agent 0% |
| 7 | **Ablation and size harness:** each Bk off; B2 vs E1 vs monotone; knowledge base at 25/50/75/100%; timing | 6 | S | one table per run, in `research/` |
| 8 | **Rust inducer, part A** (item rules) with thresholds and the confirming query (oracle-answered) | 2, 6 | M | worked example A reproduces |
| 9 | **Rust inducer, part B** (lgg over cases) + **specialisation** (4.4) + layers | 8 | L | worked example B reproduces; B7 off + part B on is measured |
| 10 | **Naming:** union-find canonicalisation + similarity questions, tested on injected drift | 3 | S-M | the drift test in 7.3 |
| 11 | **Offline yardstick:** export the induction tasks; run Popper and FOLD-R++ offline (Python, `python3 -I`, after installing clingo and SWI-Prolog in a separate environment) | 8 | M, optional | a table: rules found by each |
| 12 | **Demand slice** for questions | 2 | S, only if 7 shows > 1 s per question | answers identical to full closure on the benchmark |

**Order and why.**
1. Pieces 1-7 are the deduction core. They answer the gate's first half: at least about 60% of 2-hop questions correct with 0 inventions. Build and run them before any learning work, so that the learned rules' contribution (the gate's second half) is measured against a fixed baseline.
2. Pieces 8-9 are the learning.
3. Piece 10 can run in parallel with 8.
4. Pieces 11-12 are conditional.

**Expected difficulty overall.** About 2-3 weeks of focused work at the 3.0 pace:
- the deduction core is mostly configuration of an engine that already does stratified negation, proofs and confidence;
- the largest single piece is part B with specialisation;
- the largest risk is not code but **content**: whether 300-500 hand-written facts contain enough relations about things for 2-hop questions to be derivable, which is X4's lesson.

The benchmark authors (sub-agents from a spec, P15) should be told the relation set and nothing else, so that the questions test the representation and not the code.

**What the spike must report, beyond the gate numbers.**
1. How often B7 ends in `ask_fit` rather than an answer. This is the honest-gap rate, and its fall as demands are taught is the learning curve in miniature.
2. Whether induction rediscovers B7.
3. The number of `conflict` questions per 100 facts, which tells whether skeptical blocking is too cautious.
4. The cases where inheritance and material-based rules disagree (the silicone tray), and what the agent did.

---

## 9. Decisions this note proposes

| Proposed D4 | Decision |
|---|---|
| D4.a | 11 typed item relations + procedure schema with `role`, `demands`, action-level `effect`; reified `pos`/`neg` inside the reasoner only |
| D4.b | Exceptions: defeasible inheritance with specificity compiled to stratified Datalog; taught > nearer kind > farther kind > learned rule; incomparable conflicts block and ask; hypotheses never override teaching |
| D4.c | Base rules B1-B9; B1-B5 logic, B6-B9 schema rules; keep by unique contribution and zero harm; test whether B7 can be learned |
| D4.d | Rust inducer (refinement for item rules, lgg for cases), Wilson-bounded confidence, support ≥ 3 from ≥ 2 tasks or kinds, specialisation with instance exceptions that generalise, confirming membership queries; Popper/FOLD-R++ offline as yardsticks only |
| D4.e | JSON proof records with provenance ids, collapsed rendering, independent checker; confidence = min over taught × product over learned rules |
| D4.f | Forward closure to 5,000 facts; demand slice and interning before 50,000 |
| D4.g | Vocabulary in prompts, similarity questions, union-find canonicalisation; no automatic merges |

---

## References

Links marked ✓ were checked by web search on 2026-11-19. The others are cited from memory, with standard bibliographic data, and were not re-checked.

**Non-monotonic reasoning and exceptions**
- Alchourrón, Gärdenfors & Makinson 1985, "On the logic of theory change", *J. Symbolic Logic* 50(2). doi:10.2307/2274239
- Antoniou, Billington, Governatori & Maher 2001, "Representation results for defeasible logic", *ACM TOCL* 2(2).
- Arias, Carro, Chen & Gupta 2020, "Justifications for goal-directed constraint answer set programming" (s(CASP)), ICLP. ✓ https://arxiv.org/abs/2009.10238
- Cabalar, Fandinno & Muñiz 2020, "A system for explainable answer set programming" (xclingo), ICLP. ✓ https://arxiv.org/abs/2009.10242
- de Kleer 1986, "An assumption-based TMS", *Artificial Intelligence* 28.
- Doyle 1979, "A truth maintenance system", *Artificial Intelligence* 12(3).
- Gelfond & Lifschitz 1988, "The stable model semantics for logic programming", ICLP/SLP.
- Gebser, Kaminski, Kaufmann & Schaub, clingo. https://potassco.org/clingo/
- Grosof 1997, "Prioritized conflict handling for logic programs" (courteous logic programs), ILPS.
- Hansson 1994, "Kernel contraction", *J. Symbolic Logic* 59(3).
- Horty, Thomason & Touretzky 1990, "A skeptical theory of inheritance in nonmonotonic semantic networks", *Artificial Intelligence* 42.
- Maher 2001, "Propositional defeasible logic has linear complexity", *TPLP* 1(6). ✓ https://arxiv.org/abs/cs/0405090
- McCarthy 1986, "Applications of circumscription to formalizing common-sense knowledge", *Artificial Intelligence* 28.
- Nute 1994, "Defeasible logic", *Handbook of Logic in AI and Logic Programming* vol. 3.
- Touretzky 1986, *The Mathematics of Inheritance Systems*, Morgan Kaufmann.
- Van Gelder, Ross & Schlipf 1991, "The well-founded semantics for general logic programs", *JACM* 38(3).
- Varzi, "Mereology", *Stanford Encyclopedia of Philosophy*. https://plato.stanford.edu/entries/mereology/

**Rule learning**
- Angluin 1988, "Queries and concept learning", *Machine Learning* 2.
- Cropper & Dumančić 2022, "Inductive logic programming at 30: a new introduction", *JAIR* 74. https://arxiv.org/abs/2008.07912
- Cropper & Morel 2021, "Learning programs by learning from failures" (Popper), *Machine Learning* 110. ✓ https://arxiv.org/abs/2005.02259; code https://github.com/logic-and-learning-lab/Popper
- Galárraga, Teflioudi, Hose & Suchanek 2013, "AMIE: association rule mining under incomplete evidence in ontological knowledge bases", WWW. ✓ https://archives.iw3c2.org/www2013/proceedings/p413.pdf
- Law, Russo & Broda 2014, "Inductive learning of answer set programs" (ILASP), JELIA.
- Meilicke, Chekol, Ruffinelli & Stuckenschmidt 2019, "Anytime bottom-up rule learning for knowledge graph completion" (AnyBURL), IJCAI.
- Mitchell, Keller & Kedar-Cabelli 1986, "Explanation-based generalization: a unifying view", *Machine Learning* 1.
- Muggleton 1995, "Inverse entailment and Progol", *New Generation Computing* 13.
- Muggleton & Feng 1990, "Efficient induction of logic programs" (Golem), ALT.
- Muggleton, Lin & Tamaddoni-Nezhad 2015, "Meta-interpretive learning of higher-order dyadic datalog" (Metagol), *Machine Learning* 100. ✓ code https://github.com/metagol/metagol
- Plotkin 1970, "A note on inductive generalization", *Machine Intelligence* 5.
- Quinlan 1990, "Learning logical definitions from relations" (FOIL), *Machine Learning* 5.
- Shapiro 1983, *Algorithmic Program Debugging* (Model Inference System), MIT Press.
- Wang & Gupta 2022, "FOLD-R++: a scalable toolset for automated inductive learning of default theories from mixed data", FLOPS. ✓ https://arxiv.org/abs/2110.07843

**Datalog evaluation, provenance, equality**
- Bancilhon, Maier, Sagiv & Ullman 1986, "Magic sets and other strange ways to implement logic programs", PODS.
- Beeri & Ramakrishnan 1991, "On the power of magic", *J. Logic Programming* 10.
- Chen & Warren 1996, "Tabled evaluation with delaying for general logic programs" (SLG), *JACM* 43(1).
- Green, Karvounarakis & Tannen 2007, "Provenance semirings", PODS. doi:10.1145/1265530.1265535
- Gupta, Mumick & Subrahmanian 1993, "Maintaining views incrementally" (DRed), SIGMOD. doi:10.1145/170035.170066
- Jordan, Scholz & Subotić 2016, "Soufflé: on synthesis of program analyzers", CAV.
- Motik, Nenov, Piro & Horrocks 2015, "Handling owl:sameAs via rewriting", AAAI. ✓ https://ojs.aaai.org/index.php/AAAI/article/view/9187
- Zhao, Subotić & Scholz 2020, "Debugging large-scale Datalog: a scalable provenance evaluation strategy", *ACM TOPLAS* 42(2). ✓ https://arxiv.org/abs/1907.05045

**Functions, affordances, planning**
- Houkes & Vermaas 2010, *Technical Functions: On the Use and Design of Artefacts*, Springer.
- Paulius, Huang, Milton, Buchanan, Sam & Sun 2016, "Functional object-oriented network for manipulation learning" (FOON), IROS.
- Tenorth & Beetz 2013, "KnowRob: a knowledge processing infrastructure for cognition-enabled robots", *IJRR* 32(5).
- van der Krogt & de Weerdt 2005, "Plan repair as an extension of planning", ICAPS.
