---
kind: atom
intent: '[[fix]]'
object: app
domain: tech
priority: 2
result: fix
conditions:
- slot: '[[platform]]'
  op: ==
  value: android
trust: 0.8
source: CaseForge v5 seed
---
On Android: Settings → Apps → our app → Storage → Clear cache (not data), then relaunch.
