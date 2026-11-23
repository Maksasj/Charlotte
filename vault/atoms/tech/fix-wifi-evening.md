---
kind: atom
intent: '[[fix]]'
object: wifi
domain: tech
priority: 2
result: fix
conditions:
- slot: '[[symptom]]'
  op: ==
  value: evening
trust: 0.8
source: CaseForge v5 seed
---
Evening-only drops point to congestion — move the router to 5GHz and test again at the same hour.
