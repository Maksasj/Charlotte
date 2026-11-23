# How the idea developed

A condensed account of the conversation in
[`archive/2026-11-22-design-conversation.txt`](../archive/2026-11-22-design-conversation.txt)
(22 to 23 November 2026 by its timestamps). Each step lists the question that drove it and what
it added. Only the last prototype (v5) was kept as code; the earlier ones are
known from the conversation alone.

| Step | Driving question | What it added | What it left open |
|---|---|---|---|
| Case-based reasoning | Can a system learn from single examples? | The cycle: retrieve a similar past case, reuse it, revise it, retain the result. A case is a problem, a solution and an outcome | Adapting a case to a new problem is the hard part |
| Reinforcement view | How do outcomes improve later decisions? | A retained case with an outcome is a value estimate. Retrieval gives a starting preference over actions; feedback corrects it. Asking a clarifying question is an action with a cost and an information gain | Feedback is sparse and arrives at the end of a conversation |
| CaseLab (v1) | Can this be a general tool, not a cooking helper? | One-shot solver: retrieval by text similarity, adaptation notes, a "teach me" path below a match threshold, mining cases from text, trust per case | No conversation: one problem in, one answer out |
| CaseChat (v2) | The goal was a conversational agent | Each reply is the problem; message plus context is the input. Reply templates with slots; a missing slot becomes a clarifying question; a closing verdict credits every memory used | One message is one problem, so the second request in a message is dropped |
| CaseChat v3 | Does it split a message into subproblems? | Segment, solve each part, merge. Shared questions asked once, conflicting requests detected. Merge plans stored as strategy cases | The cut points were a fixed regex |
| v4, v4.1 | Can the split be learned too? | Split cases: connectors that mark a cut, discovered when a fixed split fails and kept when the conversation resolves. v4.1 fixed chat reliability | All knowledge still had to arrive as chat turns |
| Atoms (proposal) | Can it learn from articles, not only chats? | A canonical unit both sources compile into: a claim, typed conditions, slots, provenance, trust. The atom is the unit of trust and editing; the turn is the unit of retrieval | Writing the family of replies per atom multiplies the case base |
| Patterns (proposal) | Can the reply family itself be generic and learned? | Separate the invariant shape of a solution (pattern) from the content (atom); join them at binding time. One pattern times fifty policies gives fifty behaviours | A pattern is a small program, so the system needs an interpreter and a way to learn programs |
| CaseForge v5 | Build it | Three patterns, sixteen atoms, a seven-operation interpreter, an article miner, pattern induction from traces | See [`findings.md`](findings.md) |

## Ideas worth keeping from the discussion

- **Two compilers, one runtime.** Articles compile to atoms, chats compile to
  moves, and the same runtime answers from both.
- **Atoms are the memory, replies are a cache.** A policy change is one edit to
  one atom; replies are regenerated, not hunted down.
- **Knowledge is tested through use.** A reply that resolves a conversation
  raises the trust of the pattern and the atom behind it, so mined knowledge
  earns confidence instead of being assumed correct.
- **Silence is not permission.** Articles say what is allowed; an atom whose
  conditions do not match must not turn into a yes.
- **Three-way debugging.** A wrong answer can come from the pattern, the atom
  or the binding, so the trace has to show the three separately.

## Lineage the conversation named

Recorded as pointers for reading, as stated in the conversation and not
checked against the sources:

- Aamodt and Plaza: the retrieve, reuse, revise, retain cycle.
- Schank: Conceptual Dependency (a language-independent form for meaning),
  MOPs (situated memory), TOPs (abstract cross-domain shapes, closest to
  patterns here).
- Smyth and Keane: retrieve the case that needs the least adaptation, not the
  most similar one.
- Hanney and Keane (1997): learn adaptation knowledge from differences between
  case pairs.
- Plotkin: anti-unification (least general generalisation) as the way to
  abstract a pattern from examples.
- Episodic control (Model-Free Episodic Control, Neural Episodic Control) and
  Heuristically Accelerated Q-Learning: case memory combined with value
  updates.

## Claims from the conversation that are not backed by anything kept

- The simulation comparing cold reinforcement learning, static case retrieval
  and the hybrid (0.543, 0.658 and 0.677 against an optimum of 0.734). The
  script was never shared.
- Match percentages and example replies quoted for v2 to v4. Those versions
  were not kept.
