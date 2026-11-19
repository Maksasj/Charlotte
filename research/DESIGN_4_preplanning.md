# Design 4.0 pre-planning

**Status.** This is pre-planning, written on 2026-11-19. It is not the design. It records:
- the goal;
- what the user decided;
- the risks and how to meet them;
- the questions the design session must settle;
- the spike that must run before the Design 4.0 document is written.

Inputs: the 3.0 results (`paper/main.pdf`, `PROGRESS.md`, `research/DESIGN_3_experiments.md`) and the pre-planning conversation of 2026-11-19.

## 1. Why 4.0 changes focus

Three iterations kept finding the same split.

- **Learning and recall from correct facts work.** Given correct facts, 3.0 stores and recalls them exactly: gold answers reproduce 100%, nothing is forgotten, and retention is close to the second encounter. After one teaching in English, about 92% of core facts are stored right. Almost all errors are misread sentences, not memory.
- **Reading English stayed the bottleneck.** v1's parser failed on conversation, and 2.0 moved reading to Haiku plus about 225 hard-coded strings. In 3.0, Haiku still read 497 of 535 teach lines, and distilling Haiku's readings did not make the agent self-sufficient.
- **Generalisation failed even without language.** In phase A's fact-only experiment X4, substitution by function found 19% of the needed changes, case transfer 19% and induced rules 1.4%. The reason was not the logic. The knowledge base held only procedures, with no relations about things, and the agent had no general inference over relations. Most new situations need knowledge that no other task contains in a form that can be chained.

So 4.0 aims at the part that has never been tested properly: **an agent that creates new knowledge from what it was taught, across domains, with a reason for every piece, and gets better as it is taught more.** Language leaves the core path.

**Generalisation means creating new knowledge. Three kinds are kept apart:**

| Kind | What it is | Example | Allowed |
|---|---|---|---|
| **Derived (deduction)** | New knowledge that follows from taught facts by the rules | taught `is_a(muffin_tray, baking_tin)` and `can_hold(baking_tin, batter)`, so `can_hold(muffin_tray, batter)`: never taught, provable | Yes. Shown with its proof |
| **Induced (a generalised pattern)** | A new rule lifted from several examples, going beyond what strictly follows | fork, spoon and chopsticks each stood in for a whisk because each `can_do(beat)`, so "anything that can beat can stand in for a whisk" | Yes, as a **hypothesis**: shown with its supporting examples and confidence, and retracted if a counterexample is taught |
| **Invented (hallucinated)** | A claim with no basis in anything taught | "use a colander", when nothing taught links a colander to the job | **No** |

"Invents nothing" in this document always means the third kind only: no claim without a proof or supporting examples.

## 2. Goal

> **Taught only through conversation, the agent derives correct answers to multi-hop questions in domains it was not taught directly. Every answer carries its proof, or, for an induced rule, its supporting examples. It makes no claim without one, and it gets better as it is taught more.**

Generalisation and learning are the priority. When a new situation shares parts with known ones, the agent finds a way by deriving it. The worked example:

```
taught (baking):    is_a(muffin_tray, baking_tin)
taught (baking):    can_hold(baking_tin, batter)
derived:            can_hold(muffin_tray, batter)      because is_a + inheritance
asked:              bake banana bread, no loaf tin, user has a muffin tray
answer:             swap loaf_tin -> muffin_tray in step 2,
                    with the proof chain shown
```

The long-term vision is a formal-logic counterpart to LLM training, with facts extracted at scale and an agent trained alongside a growing knowledge base. **It is context only, not in scope.** The user will say explicitly when the project moves toward it. 4.0 stays with household tasks.

## 3. Decisions (user, 2026-11-19)

| # | Topic | Decision |
|---|---|---|
| P1 | Priority | Generalisation and learning, above everything else |
| P2 | Knowledge source | All knowledge is learned through teaching in conversation. Nothing is injected into the knowledge base directly. No imported resources (ConceptNet, WordNet) and no hand-written knowledge base |
| P3 | Teacher | Haiku, in conversation, following a curriculum of topics; the agent also asks the teacher about its own gaps (active learning) |
| P4 | Teaching form | Structured: the teacher's messages are in the formal language (option A). A controlled-English layer (option B) is the first language layer, added once the core works. Free English waits |
| P5 | Output | The formal language, with a proof chain for every derived claim |
| P6 | Built in, not taught | A small set of universal base rules: the logic of the system, with no domain knowledge |
| P7 | Rule learning | Required in 4.0. New rules are derived from base rules and from earlier learned rules, layer on layer. That is the learning |
| P8 | Exceptions | Must be handled; the mechanism is decided in the design stage |
| P9 | Relation set | Proposed below; decided in the design stage |
| P10 | Knowledge-base size | Start small; experiment with several sizes; scale only if scaling is shown to help |
| P11 | Knowledge-base format | Reusable and future-proof, in three separate layers: raw teaching messages, extracted facts, provenance. Built to outlive schema and agent changes |
| P12 | Seed data | The 2.0 and 3.0 data (`data/curriculum`, `data/curriculum3`, `data/dataset3`, the call archives) seeds the raw layer, re-taught through conversation rather than injected |
| P13 | Domain | Household tasks only |
| P14 | Budget | 5,000 Haiku calls, a **hard limit**. It is raised only when the user says so explicitly |
| P15 | Measure of record | A derivation benchmark whose answers are never taught, only their pieces |
| P16 | Stopping rules | Go/no-go gates after the spike and after the first scaling step. Milestone thresholds are set after the spike, not before (the M24 lesson from 3.0) |

### What "teaching through conversation" means here

Knowledge enters only as messages in a dialogue, and the agent processes each one itself. With structured teaching, a message is a formal statement, but the exchange is still a conversation:

```
TEACHER: is_a(muffin_tray, baking_tin).
AGENT:   ok. derived: can_hold(muffin_tray, batter)   [is_a(muffin_tray, baking_tin), can_hold(baking_tin, batter)]
TEACHER: not can_go_in(plastic_tub, oven).
AGENT:   conflict: is_a(plastic_tub, tin), can_go_in(tin, oven). exception for plastic_tub?
TEACHER: yes.
AGENT:   ok. exception stored.
AGENT:   ask: what can_do beat, besides whisk?            <- the agent's own gap
TEACHER: can_do(fork, beat).
```

For every message, the agent does the following, all of which direct injection would skip:
1. checks it against the schema;
2. checks it against what it already knows, and asks about contradictions;
3. records provenance (teacher, session, turn, confidence);
4. derives what follows, and shows it;
5. notices gaps and asks about them.

## 4. Risks and mitigations

| Risk | Mitigation |
|---|---|
| **Naming drift.** Haiku names one thing three ways (`whisk`, `egg_whisk`, `balloon_whisk`); 3.0 finding 5 | The agent keeps one shared vocabulary. When a new symbol looks like a known one, it asks ("same as `whisk`?"). The answer is taught knowledge too (`same_as` or `is_a`) |
| **Learned rules overgeneralise** | A new rule needs enough support (several distinct examples, from more than one task or domain) and no known counterexample. Rules carry confidence and are retracted when a counterexample is taught |
| **Leakage into the benchmark.** The curriculum teaches a benchmark answer directly | Benchmark answers and their trivial variants are on a deny list. A script checks every teaching message against it, and the run fails if one is taught |
| **The base rules are too weak or too strong** | The spike measures it offline, with zero calls, before any teaching budget is spent |
| **Inference explodes on a large knowledge base** (transitive closure) | Typed relations, a depth limit, and derivation on demand from a question, not only forward closure. Measured in the spike at several sizes |
| **Wrong facts spread through chains** | Per-fact confidence that propagates to derived facts; contradiction checks at entry; exceptions; every derived answer shows its chain |
| **"Gets better as taught more" is one number, not a curve** | Benchmark at 25, 50, 75 and 100% of the curriculum, and at each knowledge-base size |
| **Thresholds set by guesswork become unpassable** (3.0 M24, M25) | Thresholds are set after the spike, from measured data |
| **Haiku as teacher spends the budget on volume** | Every call is cached and archived (as in 3.0). Teaching batches several facts per call. The ledger stops runs at the limit and fails fast on the plan usage limit |
| **Structured teaching makes the result look trivial** | The claim is precise: "learns and generalises from formal teaching". Integration, conflicts, questions, rule learning and derivation are all measured, not assumed |

## 5. Questions for the design session

Each has a recommendation, to be confirmed or changed.

### Q1. Relation set (P9)

A small, domain-free set to start:

| Relation | Meaning |
|---|---|
| `is_a(X, Kind)` | X is a kind of Kind |
| `part_of(P, X)` | P is part of X |
| `can_do(X, Function)` | X can perform a function |
| `used_for(X, Purpose)` | X is used for a purpose |
| `has_property(X, P)` | X has property P |
| `made_of(X, M)` | X is made of material M |
| `achieves(Action, State)` | the action brings about a state |
| `requires(Step or Action, State or Item)` | a precondition |
| `same_as(X, Y)` | two names for one thing |

Procedures (task, step, needs, order), situations and differences stay as in schema v3. **Recommendation:** start with these nine, and add one only when the spike shows a needed derivation cannot be expressed.

### Q2. Exceptions (P8)

**Recommendation:** default inheritance with specificity. A fact about a more specific kind overrides one about a more general kind. Negative facts (`not can_go_in(plastic_tub, oven)`) are taught explicitly. A conclusion is withdrawn when a more specific contrary fact arrives (non-monotonic). Alternatives to weigh: answer set programming (stable models with default negation), or explicit `except` facts. Decided after the spike shows which conflicts occur.

### Q3. Base rules (P6)

A candidate core of about 6-10 rules, written once as Datalog-style rules with variables:
1. `is_a` is transitive.
2. `can_do`, `used_for`, `has_property` and `made_of` inherit down `is_a`, unless a more specific fact contradicts them.
3. `part_of` is transitive.
4. `same_as` is an equivalence: symmetric, transitive, and substitutes in every relation.
5. A stand-in: if step S needs an item I for function F, and the user has J with `can_do(J, F)`, then J may replace I in S. The proof is recorded.
6. A step can be replaced by another action that `achieves` the same state that a later step `requires`.
7. Contradiction: a fact and its negation about the same thing raise a question to the teacher.

**Recommendation:** fix this list in the spike, measure each rule's contribution by switching it off, and keep only rules that pay.

### Q4. Rule learning (P7)

How new rules are derived:
- **Composition:** chain two learned or base rules into a shortcut rule once it has been used often.
- **Induction from taught examples:** lift a pattern seen in several taught cases into a rule with variables. For example, three different tools that each `can_do(beat)` stood in for a whisk, so: "anything that `can_do(beat)` can stand in for a whisk". An offline inductive learner (Popper) or a built-in lifted-pattern inducer, as in the 3.0 rules arm.
- **Specialisation:** a learned rule that meets a counterexample gains a condition, rather than being dropped.

**Recommendation:** induction with a support threshold (number of distinct examples) and specialisation on counterexamples. Composition only if derivation speed needs it. The rule layer is reported as layers, recording which rules were built from which.

### Q5. Knowledge-base format (P11)

Three layers, each in its own files:

| Layer | Content | Changes when the schema changes? |
|---|---|---|
| **Raw** | Every teaching message as sent, with its conversation context; append-only | Never |
| **Facts** | What the agent extracted and stored: facts, learned rules, exceptions | Re-derivable from raw by replaying the conversations |
| **Provenance** | For every fact and rule: source message, session, turn, confidence, what it was derived from, retractions | Re-derivable |

**Recommendation:** JSONL for raw and provenance, Datalog text for facts and rules, and a schema version on every record. A "replay" tool rebuilds the facts layer from raw under any agent version, so the knowledge base outlives the schema. That is the reuse requirement.

### Q6. The derivation benchmark (P15)

- **Questions** need 1, 2 or 3 hops, in household domains, with some situations in domains that were not taught directly. Their answers are never taught, only their pieces, checked by script (risk "leakage").
- **Each question has:**
  - a gold answer;
  - a gold proof (the pieces, at least one valid chain);
  - an honest-gap gold, used when the pieces are deliberately absent.
- **Metrics:**
  - correct derivations;
  - inventions (an answer without a valid proof; target 0);
  - correct gaps;
  - wrong answers;
  - proof validity;
  - all per hop count and per domain.
- **The learning curve:** the benchmark at 25, 50, 75 and 100% of the curriculum, and at each knowledge-base size.
- **Who writes it:** sub-agents from a spec, without sight of the agent's code, as for H3, then sealed by hash.

**Recommendation:**
- about 200 questions in the spike's hand-checkable form, then 500 or more for the main measurement;
- a separate sealed held-out part;
- a development part, which may be read.

### Q7. Curriculum (P3)

- Topics and order come from a script (tools, materials, kinds, functions, then procedures, then situations).
- Haiku teaches within each topic in formal statements, batched several per call.
- The agent asks about its gaps, and the teacher answers.
- The 2.0 and 3.0 data seeds topics and procedures (P12).

**Recommendation:** a fixed topic list per stage, a cap on calls per stage, and the agent's questions limited to a share of the budget (for example 20%).

### Q8. What stays from 3.0

| 3.0 part | Recommendation |
|---|---|
| Vault with provenance and retraction, relation lines, the fact-and-relation check, cached Haiku calls with the ledger, the lab stream, the word-list test | Keep and extend |
| The learned English parser | Not on the core path. Kept for the later English layer |
| Situation reading | Formal, in the teaching form |
| The 2.0 crates | Kept unchanged for comparison |

## 6. The spike (before the Design 4.0 document)

**Purpose.** Answer one question cheaply: does a small base rule set over a small relation set derive correct, proven answers across domains, and do learned rules add to it?

**Scope.**
- Offline; **zero Haiku calls**.
- A hand-checkable knowledge base of about 300-500 facts in 3-4 household domains (baking, cooking, cleaning, repair), written as teaching messages and fed through the agent's conversation path, not injected.
- About 50-100 derivation questions with gold answers and proofs, at 1, 2 and 3 hops, including cross-domain ones and ones with deliberately missing pieces.

**Builds.**
1. A relation schema extension (Q1).
2. The base rules (Q3) on the existing Datalog reasoner.
3. Exceptions by specificity (Q2), in a first form.
4. One rule-learning method (Q4, induction with support).
5. Proof output.
6. The benchmark scorer.

**Measures.**
- correct derivations, inventions, gaps, wrong answers, and proof validity;
- each base rule switched off in turn;
- learned rules on and off;
- the knowledge base at 25, 50, 75 and 100%;
- the inference cost at each size.

**Go/no-go gate.** Thresholds are agreed before the spike runs:
- **Go** to the full design if the core derives at least about 60% of 2-hop questions correctly, with **0 inventions**, and the learned rules add measurably.
- **Otherwise,** rethink the relation set, the rules or the approach before spending any teaching budget.

**Output.** A short spike report in `research/` with the numbers, which then sets the Design 4.0 document's thresholds and milestones.

## 7. Order of work

1. **Design session.** Settle Q1-Q8 with the user, using this document; record each as a decision D4.x.
2. **The spike** (section 6), with no calls, then its report.
3. **Go/no-go,** by the user, on the spike report.
4. **The Design 4.0 document:** phases, milestones and acceptance tests set from the spike's numbers; the curriculum and budget plan within 5,000 calls; the benchmark spec, sealing and readings.
5. **Build and measure,** with the learning curve as the main result.
