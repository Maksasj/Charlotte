---
kind: atom
intent: '[[cancel]]'
object: subscription
domain: subscriptions
priority: 3
result: cancel
conditions:
- slot: '[[days]]'
  op: <=
  value: 14
trust: 0.8
source: CaseForge v5 seed
---
Cancellation approved for {subject} — access continues to the end of the paid period and no further charges apply.
