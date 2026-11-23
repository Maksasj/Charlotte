# Findings on CaseForge v5

Status as of 23 November 2026. Items marked **probe** are reproduced by
`node prototypes/caseforge-v5/probe.mjs`; items marked **code** were
established by reading the prototype. Nothing here was measured on real
conversations: every input is a hand-written sentence.

## What held up

1. **One pattern serves many domains.** The `gated-request` pattern, never
   edited, grants a refund, a warranty replacement, an expense, a leave
   request, a feature and an upgrade, each from its own atom. *(probe)*
2. **A new domain costs facts, not solutions.** Cancellation gets a cold start
   before mining. After mining a four-sentence article into two atoms, the same
   pattern approves a cancellation bought 5 days ago and offers a pause for an
   activated plan bought 40 days ago. *(probe)*
3. **The miner skips what it cannot formalise.** Two of the four sentences
   carry no condition and produce no atom. *(probe)*
4. **Unknown input is refused, not bluffed.** A message with a greeting and an
   unrelated question goes to the teach path, not to the greeting reply that
   hijacked it in earlier versions. *(probe)*

Point 2 has a cost that is part of the result: the miner labelled both atoms
`refund/order` because the heading contained "refund", and both payloads were
policy prose. The predicate, object and domain had to be corrected by hand and
the two replies rewritten before the atoms were usable.

## What is learned, what is authored, what is dead

| Thing | Status | Evidence |
|---|---|---|
| Trust of patterns, atoms, moves, splits | Learned | Updated on use and on verdict *(code)* |
| Split connectors | Learned | Discovered by anchoring, kept on a resolved verdict *(code)* |
| Which intents share a solution shape | Learned, within existing shapes | `induce` groups traces by step signature *(code)* |
| New moves | Learned | Taught at cold start, retained on close *(code)* |
| Skeletons | Authored | A promoted pattern copies one of three fixed skeletons *(code)* |
| Order of questions (`askOrder`) | Authored | Read when asking, never written by any learning step *(code)* |
| Per-pattern `batch` and `proactive` | Dead | Stored and shown, never read; the global settings are used *(code)* |
| Context window setting (`contextTurns`) | Dead | Has a slider, never read by the engine *(code)* |
| Move `history` | Dead | Stored, not used in ranking *(code)* |
| Intents, predicates, slots, conditions | Authored | Fixed regex tables *(code)* |

In one line: the system learns **where** a pattern applies and **how much** to
trust it, not **what** it does.

Induction is also circular. A trace's shape is the shape of the pattern that
produced it, so induction can only regroup shapes that already exist. A fourth
shape cannot be observed, because nothing in the system can produce one.

The conversational part is thinner than in the earlier versions. Context
reaches a reply only through slots collected over all user messages and
through the list of pending questions. There is no retrieval over the
conversation so far.

## Defects

| # | Defect | Example | Cause | Found by |
|---|---|---|---|---|
| D1 | Slots are shared by all subproblems | "cancel my subscription SUB-12 …, also where is my order 5150?" approves the cancellation **for 5150** | `solveSub` extracts slots from the whole message for every subproblem | Conversation; *probe* |
| D2 | Negation is ignored | "stopped working, … never dropped" is read as accidental damage and the replacement is refused | The extractor tests `dropped` before `never dropped` | *probe*, new |
| D3 | Atoms about another object compete | A sealed order returned after 45 days gets "Has the subscription been activated yet?" | Atoms are selected by predicate only; the object is not bound | *probe*, new |
| D4 | The `offer` step never fires | Over every slot combination of the seed atoms the reply modes are clarify 29, grant 32, deny 5, offer 0 | Atom selection always prefers an unviolated atom, so the alternative is chosen as the main atom and answered through `grant`. A fallback exists only when it would already have been picked | *probe*, new |
| D5 | Promoting some candidates crashes | Three cached-move traces (greeting, pricing, hours) yield a candidate with shape `move`; promoting it throws | No skeleton exists for `move` | Conversation; *probe* |

D4 matters beyond the bug. Alternatives and denials are decided by atom
ranking, outside the skeleton, so the trace signature of an alternative is
`grant`, not `offer`. Signatures are what induction groups on, which means the
traces do not record the distinction a learned skeleton would need.

D3 has a second face: a late return is only denied once the user also says
the item was not activated. Until then the system keeps asking about an atom
that does not concern the request.

## Open problems

Ordered by how much they block the claim "learns from articles and
conversations".

1. **Closed vocabulary.** Intents, predicates and slots are regex tables. A new
   domain needs a new predicate, and a new condition needs a new slot
   extractor; neither can be mined. This is the same bottleneck the first
   research line met with its predicate registry.
2. **Condition extraction.** The article miner recognises seventeen fixed
   phrases. Real policy text will not match them, and a language model doing
   the extraction can invent conditions that the text does not state.
3. **Skeleton learning.** Needs two things the prototype lacks: traces that
   record the gate state at each step (which slots were unbound, which
   condition was violated), so guards can be inferred; and traces that come
   from outside the pattern loop, such as a person resolving a case over
   several turns.
4. **Binding scope.** Slots and objects have to be bound per subproblem (D1,
   D3), which needs entity types: an order and a subscription are different
   subjects.
5. **Pattern competition.** Only one pattern serves each intent today, so the
   specificity and trust ranking has never been exercised.
6. **Evaluation.** There is no test set, no held-out split and no number. The
   first research line had these; this one has demonstrations.
7. **Typed conditions.** Values are compared as numbers or lower-cased
   strings. Dates, durations with units and ordered tiers are not handled.

## Candidate next steps

Not decided. Listed so the choice is explicit.

- **Repair v5** (small): scope slots per subproblem (D1); handle negation
  (D2); bind the object when selecting atoms (D3); decide whether alternatives
  belong to atom selection or to the skeleton and make traces say which (D4);
  guard promotion (D5); wire or remove the dead parameters.
- **The conversation's v5.2 plan**: per-pattern parameters that override the
  globals, a bandit over `askOrder`, gate state in traces so guards become
  inferable.
- **Attack the vocabulary** (open problem 1) before adding more learning on
  top of it, since every other step inherits it.
- **Move the engine out of the page.** `probe.mjs` already runs it headless;
  an engine module with a test set would give this line the measurement the
  first one had.
