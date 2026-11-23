---
kind: atom
intent: '[[enable]]'
object: account
domain: license
priority: 3
result: enable
conditions:
- slot: '[[tier]]'
  op: ==
  value: pro
- slot: '[[verified]]'
  op: ==
  value: true
trust: 0.8
source: CaseForge v5 seed
---
Advanced features are now enabled on {subject}.
