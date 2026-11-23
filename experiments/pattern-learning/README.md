# Experiment: can solution shapes be learned?

23 November 2026. One script, [`run.mjs`](run.mjs), no dependencies, runs in under a second.

## Question

v5 selects a skeleton from three authored ones. Can a skeleton instead be
learned from conversations, and does that still hold for shapes more complex
than "check a rule"?

## Setup

- A hidden **teacher** policy holds conversations in randomly generated
  domains. Understanding is taken as solved: the conversations arrive labelled.
- For each agent turn the **learner** sees a fixed description of the
  conversation state (about 70 yes/no facts: the state of the rule being
  checked, what the agent and the user did last, before that, and ever, plus
  distractors such as the turn number and the domain) and the act the teacher
  chose.
- It induces an ordered rule list, the same form as a v5 skeleton, and names
  each act's argument by a content-free selector ("first unbound slot of the
  rule").
- It is scored on **domains it never saw**: a conversation counts only if every
  agent turn matches the teacher.

Fixed before the first run: the state description, the learner, and the pass
mark of 95% of unseen conversations reproduced from 20 training conversations
(median of 10 seeds).

| Rung | Shape |
|---|---|
| 1 flat | Check a rule: ask what is missing, grant, offer an alternative, deny |
| 2 position | A multi-step procedure with one retry per step; trained on 2 to 4 steps, tested on 5 to 7 |
| 3 revision | Rung 1 plus user corrections; a decision that still stands is confirmed, not repeated |
| 4a composition, shown | Some facts come from a record: ask the id, look it up, then check the rule |
| 4b composition, never shown | Trained on rule checks and on plain lookups separately, tested on the combination |

## Result

Unseen conversations reproduced exactly, median of 10 seeds (worst seed), 6
training domains:

| Rung | No rules | 5 convs | 10 | 20 | 40 | 20 with 10% wrong labels | Verdict |
|---|---|---|---|---|---|---|---|
| 1 flat | 0% | 70% (57) | 75% (61) | **79%** (70) | 90% (75) | 75% (47) | Fail |
| 2 position | 0% | 60% (29) | 87% (56) | **100%** (66) | 100% (77) | 100% (56) | Pass |
| 3 revision | 0% | 40% (13) | 59% (47) | **73%** (45) | 75% (62) | 54% (46) | Fail |
| 4a composition, shown | 0% | 37% (15) | 63% (30) | **71%** (56) | 91% (48) | 55% (25) | Fail |
| 4b composition, never shown | 0% | 1% (0) | 2% (0) | **3%** (0) | 4% (0) | 3% (0) | Fail |

By the mark set in advance, four of five rungs fail, including the first.

## Why, from two follow-up runs

These were added after seeing the table above. They explain the failures and
do not change the verdicts.

With 24 training domains and more conversations (`--more --domains 24`):

| Rung | 20 convs | 80 | 160 | 320 |
|---|---|---|---|---|
| 1 flat | 87% (52) | 99% (91) | 100% (91) | 100% (91) |
| 2 position | 100% (73) | 100% (100) | 100% (100) | 100% (100) |
| 3 revision | 74% (59) | 88% (78) | 92% (81) | 96% (86) |
| 4a composition, shown | 67% (48) | 98% (83) | 100% (93) | 100% (89) |
| 4b composition, never shown | 2% (0) | 6% (2) | 7% (2) | 7% (2) |

- **Rungs 1 and 4a are learnable, but not from 20 conversations.** The rules
  learned at 20 are close to the teacher's; what is missing is a rare branch.
  In the first seed the learner never saw a refusal in a domain without an
  alternative, so it offers one that does not exist (`--rules`). About 80
  conversations over many domains are needed. With only 6 domains, more
  conversations stop helping at about 95%: variety of domains matters more
  than volume.
- **Rung 2 is easy** because the state description already says which steps
  are done. The learner did not have to work out what to remember.
- **Rung 3 improves slowly and is still short at 320.** The teacher remembers
  which decision currently stands. The state description only offers the last
  act, the one before, and "ever", which loses that after two corrections. The
  learner cannot invent the missing memory.
- **Rung 4b does not work at all.** Shapes learned separately do not combine
  on their own; the combination has to be shown.
- **Wrong labels hurt.** One wrong act in ten costs 4 to 19 points at 20
  conversations on the rungs that are not already solved.

## What this does and does not show

It shows that an ordered rule list over a described conversation state can be
induced from examples and carries over to unseen domains, for flat, sequential
and composed shapes, given enough varied examples.

It does not show:

- that it works on real conversations. The teacher is a consistent program;
  people are not;
- that the learner can find its own state description. That part is authored,
  so the closed vocabulary has moved from the skeletons to the state;
- anything about cost in practice. 80 labelled conversations per shape are free
  from a simulator and expensive from people.
