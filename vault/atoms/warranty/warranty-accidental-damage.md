---
kind: atom
intent: '[[warranty]]'
object: device
domain: warranty
priority: 1
result: paid_repair
conditions:
- slot: '[[misuse]]'
  op: ==
  value: true
trust: 0.8
source: CaseForge v5 seed
---
Accidental damage isn't covered by warranty, but paid repair is available for {subject}.
