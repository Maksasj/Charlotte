---
kind: atom
intent: '[[reimburse]]'
object: expense
domain: finance
priority: 1
result: partial
conditions:
- slot: '[[receipt]]'
  op: ==
  value: false
trust: 0.8
source: CaseForge v5 seed
---
Without a receipt we can approve partial reimbursement for {subject} instead.
