---
kind: atom
intent: '[[refund]]'
object: subscription
domain: retail
priority: 3
result: money_back
conditions:
- slot: '[[activated]]'
  op: ==
  value: false
trust: 0.8
source: CaseForge v5 seed
---
Refund approved for {subject} — the subscription was never activated.
