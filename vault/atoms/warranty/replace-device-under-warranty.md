---
kind: atom
intent: '[[warranty]]'
object: device
domain: warranty
priority: 3
result: replace
conditions:
- slot: '[[term]]'
  op: <=
  value: 12
- slot: '[[misuse]]'
  op: ==
  value: false
trust: 0.8
source: CaseForge v5 seed
---
Replacement approved for {subject} — a new unit ships within 2 business days.
