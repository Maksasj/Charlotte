---
kind: atom
intent: '[[upgrade]]'
object: account
domain: billing
priority: 3
result: upgrade
conditions:
- slot: '[[seats]]'
  op: '>='
  value: 1
trust: 0.8
source: CaseForge v5 seed
---
Upgrade to Pro is live on {subject} — prorated for the remaining cycle.
