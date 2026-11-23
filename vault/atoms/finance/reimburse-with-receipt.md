---
kind: atom
intent: '[[reimburse]]'
object: expense
domain: finance
priority: 3
result: reimburse
conditions:
- slot: '[[amount]]'
  op: <=
  value: 500
- slot: '[[receipt]]'
  op: ==
  value: true
trust: 0.8
source: CaseForge v5 seed
---
Reimbursement of ${amount} approved for {subject} — paid with the next cycle.
