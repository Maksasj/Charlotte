---
kind: atom
intent: '[[trial-extension]]'
object: account
domain: billing
priority: 3
result: extend
conditions:
- slot: '[[extended]]'
  op: ==
  value: false
trust: 0.8
source: CaseForge v5 seed
---
I've added 14 more days of trial to {subject}.
