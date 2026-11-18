---
id: fix_tire
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# fix tire

Learned in conversation.

```datalog
task(fix_tire). @conf 1 @source ""
needs(fix_tire, tire_lever). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
needs(fix_tire, patch_kit). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
needs(fix_tire, pump). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
takes_time(fix_tire, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
step(fix_tire, fix_tire_s2). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
action(fix_tire_s2, remove). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
object_of(fix_tire_s2, tube). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
uses(fix_tire_s2, tire_lever). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
takes_time(fix_tire_s2, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
step(fix_tire, fix_tire_s3). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
action(fix_tire_s3, find). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
object_of(fix_tire_s3, hole). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
uses(fix_tire_s3, pump). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
uses(fix_tire_s3, tube). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
takes_time(fix_tire_s3, 2, 4, minute). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
before(fix_tire_s2, fix_tire_s3). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
step(fix_tire, fix_tire_s4). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
action(fix_tire_s4, patch). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
object_of(fix_tire_s4, hole). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
uses(fix_tire_s4, patch_kit). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
takes_time(fix_tire_s4, 5, 7, minute). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
before(fix_tire_s3, fix_tire_s4). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
step(fix_tire, fix_tire_s5). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
action(fix_tire_s5, fit). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
object_of(fix_tire_s5, tube). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
uses(fix_tire_s5, tire_lever). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
takes_time(fix_tire_s5, 4, 6, minute). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
before(fix_tire_s4, fix_tire_s5). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
step(fix_tire, fix_tire_s6). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
action(fix_tire_s6, pump). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
object_of(fix_tire_s6, tire). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
uses(fix_tire_s6, pump). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
takes_time(fix_tire_s6, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
until(fix_tire_s6, firm). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
before(fix_tire_s5, fix_tire_s6). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
step(fix_tire, fix_tire_s1). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
action(fix_tire_s1, remove). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
object_of(fix_tire_s1, wheel). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
uses(fix_tire_s1, bike). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
takes_time(fix_tire_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1795002260778-760026-42
```

## Taught wording

- task: fix a flat tire
- fix_tire_s2: remove the tube from the tire with the tire levers
- fix_tire_s3: find the hole by pumping a little air into the tube with the pump and listening
- fix_tire_s4: patch the hole with the patch kit
- fix_tire_s5: fit the tube back inside the tire with the tire levers
- fix_tire_s6: pump the tire with the pump until it is firm
- fix_tire_s1: remove the wheel from the bike
