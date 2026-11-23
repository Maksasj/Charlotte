---
kind: pattern
applies_to: lookup
rules:
- when:
  - found
  do: report
- when:
  - last_agent=ask
  do: lookup
- when:
  - not subject_known
  do: ask(subject)
- when:
  - not ever_agent=lookup
  do: lookup
otherwise: escalate
support: 74
validated: 1.0
seen_with:
- '[[status]]'
learned_from: data/pretrain-80-per-shape.jsonl
trust: 0.8
status: trusted
---
Learned from 74 conversations. First rule that fits wins.

1. when found → `report`
2. when last_agent=ask → `lookup`
3. when not subject_known → `ask(subject)`
4. when not ever_agent=lookup → `lookup`
5. otherwise → `escalate`
