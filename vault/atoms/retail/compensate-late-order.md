---
kind: atom
intent: '[[compensation]]'
object: order
domain: retail
priority: 3
result: credit
conditions:
- slot: '[[delay]]'
  op: '>='
  value: 3
  source: record
trust: 0.8
source: CaseForge v5 seed
---
Sorry it was late — a 10% credit has been added for {subject}.
