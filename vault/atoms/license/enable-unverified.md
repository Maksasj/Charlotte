---
kind: atom
intent: '[[enable]]'
object: account
domain: license
priority: 1
result: verify
conditions:
- slot: '[[verified]]'
  op: ==
  value: false
trust: 0.8
source: CaseForge v5 seed
---
The account isn't verified yet — I can start verification for {subject} instead.
