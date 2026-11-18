---
id: bake_potato
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# bake potato

Learned in conversation.

```datalog
task(bake_potato). @conf 1 @source ""
needs(bake_potato, potato). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
needs(bake_potato, olive_oil). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
needs(bake_potato, salt). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
needs(bake_potato, fork). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
needs(bake_potato, baking_tray). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
needs(bake_potato, oven). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
takes_time(bake_potato, 60, 70, minute). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
step(bake_potato, bake_potato_s1). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
action(bake_potato_s1, heat). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
object_of(bake_potato_s1, oven). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
takes_time(bake_potato_s1, 10, 10, minute). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
action(bake_potato_s1, wash). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
object_of(bake_potato_s1, potato). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
overlaps(bake_potato_s1, bake_potato_s0). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
step(bake_potato, bake_potato_s3). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
action(bake_potato_s3, prick). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
object_of(bake_potato_s3, potato). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
uses(bake_potato_s3, fork). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
overlaps(bake_potato_s3, bake_potato_s1). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
step(bake_potato, bake_potato_s4). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
action(bake_potato_s4, rub). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
object_of(bake_potato_s4, potato). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
uses(bake_potato_s4, olive_oil). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
uses(bake_potato_s4, salt). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
overlaps(bake_potato_s4, bake_potato_s1). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
step(bake_potato, bake_potato_s5). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
action(bake_potato_s5, bake). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
object_of(bake_potato_s5, potato). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
uses(bake_potato_s5, baking_tray). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
until(bake_potato_s5, soft). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
takes_time(bake_potato_s5, 50, 60, minute). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
before(bake_potato_s4, bake_potato_s5). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
before(bake_potato_s1, bake_potato_s5). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
uses(bake_potato_s1, oven). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
overlaps(bake_potato_s1, bake_potato_s2). @conf 1 @source teacher:curriculum @conv s1794998234696-502418-18
```
