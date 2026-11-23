---
kind: atom
intent: '[[upgrade]]'
object: account
domain: billing
priority: 1
result: trial_extension
conditions:
- slot: '[[seats]]'
  op: ==
  value: 0
trust: 0.8
source: CaseForge v5 seed
---
No seats remain, so I've added a 14-day trial extension to {subject} instead.
