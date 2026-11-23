---
kind: atom
intent: '[[refund]]'
object: subscription
domain: retail
priority: 1
result: cancel_instead
conditions:
- slot: '[[activated]]'
  op: ==
  value: true
trust: 0.8
source: CaseForge v5 seed
---
Digital subscriptions are non-refundable once activated, but cancelling stops the next renewal.
