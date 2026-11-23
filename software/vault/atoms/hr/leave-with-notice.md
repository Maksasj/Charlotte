---
kind: atom
intent: '[[leave]]'
object: request
domain: hr
priority: 3
result: approve
conditions:
- slot: '[[notice]]'
  op: '>='
  value: 14
- slot: '[[coverage]]'
  op: ==
  value: true
trust: 0.8
source: CaseForge v5 seed
---
Leave approved for {subject} — {notice} days notice with coverage arranged.
