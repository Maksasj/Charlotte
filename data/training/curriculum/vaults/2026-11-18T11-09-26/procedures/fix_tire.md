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
needs(fix_tire, tire_lever). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
needs(fix_tire, patch_kit). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
needs(fix_tire, pump). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
takes_time(fix_tire, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
step(fix_tire, fix_tire_s2). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
action(fix_tire_s2, remove). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
object_of(fix_tire_s2, tube). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
uses(fix_tire_s2, tire_lever). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
takes_time(fix_tire_s2, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
step(fix_tire, fix_tire_s3). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
action(fix_tire_s3, find). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
object_of(fix_tire_s3, hole). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
uses(fix_tire_s3, pump). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
uses(fix_tire_s3, tube). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
takes_time(fix_tire_s3, 2, 4, minute). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
before(fix_tire_s2, fix_tire_s3). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
step(fix_tire, fix_tire_s4). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
action(fix_tire_s4, patch). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
object_of(fix_tire_s4, hole). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
uses(fix_tire_s4, patch_kit). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
takes_time(fix_tire_s4, 5, 7, minute). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
before(fix_tire_s3, fix_tire_s4). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
step(fix_tire, fix_tire_s5). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
action(fix_tire_s5, fit). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
object_of(fix_tire_s5, tube). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
uses(fix_tire_s5, tire_lever). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
takes_time(fix_tire_s5, 4, 6, minute). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
before(fix_tire_s4, fix_tire_s5). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
step(fix_tire, fix_tire_s6). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
action(fix_tire_s6, pump). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
object_of(fix_tire_s6, tire). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
uses(fix_tire_s6, pump). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
takes_time(fix_tire_s6, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
until(fix_tire_s6, firm). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
before(fix_tire_s5, fix_tire_s6). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
step(fix_tire, fix_tire_s1). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
action(fix_tire_s1, remove). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
object_of(fix_tire_s1, wheel). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
uses(fix_tire_s1, bike). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
takes_time(fix_tire_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1795000166882-640231-17
```
