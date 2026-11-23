---
kind: atom
intent: '[[refund]]'
object: order
domain: retail
priority: 1
result: store_credit
conditions:
- slot: '[[packaging]]'
  op: ==
  value: opened
trust: 0.8
source: CaseForge v5 seed
---
Opened items qualify for store credit instead — I've issued a credit note for {subject}.
