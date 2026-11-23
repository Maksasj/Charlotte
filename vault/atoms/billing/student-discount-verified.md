---
kind: atom
intent: '[[student-discount]]'
object: account
domain: billing
priority: 3
result: discount
conditions:
- slot: '[[verified]]'
  op: ==
  value: true
trust: 0.8
source: CaseForge v5 seed
---
Student pricing is now active on {subject} — $9 a month.
