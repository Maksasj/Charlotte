---
kind: atom
intent: '[[refund]]'
object: order
domain: retail
priority: 3
result: money_back
conditions:
- slot: '[[days]]'
  op: <=
  value: 30
- slot: '[[packaging]]'
  op: ==
  value: original
trust: 0.8
source: CaseForge v5 seed
---
Refund approved for {subject} — funds return in 3–5 business days.
