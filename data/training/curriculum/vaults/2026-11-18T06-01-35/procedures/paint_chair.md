---
id: paint_chair
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# paint chair

Learned in conversation.

```datalog
task(paint_chair). @conf 1 @source ""
needs(paint_chair, sandpaper). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
needs(paint_chair, damp_cloth). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
needs(paint_chair, primer). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
needs(paint_chair, paint). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
needs(paint_chair, paintbrush). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
needs(paint_chair, newspaper). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
takes_time(paint_chair, 120, 180, minute). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
step(paint_chair, paint_chair_s1). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
action(paint_chair_s1, sand). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
object_of(paint_chair_s1, chair). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
uses(paint_chair_s1, sandpaper). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
takes_time(paint_chair_s1, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
step(paint_chair, paint_chair_s2). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
action(paint_chair_s2, wipe). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
object_of(paint_chair_s2, chair). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
uses(paint_chair_s2, damp_cloth). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
takes_time(paint_chair_s2, 5, 5, minute). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
before(paint_chair_s1, paint_chair_s2). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
step(paint_chair, paint_chair_s3). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
action(paint_chair_s3, set). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
object_of(paint_chair_s3, chair). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
uses(paint_chair_s3, newspaper). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
step(paint_chair, paint_chair_s4). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
action(paint_chair_s4, brush). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
object_of(paint_chair_s4, primer). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
takes_time(paint_chair_s4, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
before(paint_chair_s3, paint_chair_s4). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
before(paint_chair_s2, paint_chair_s3). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
action(paint_chair_s4, dry). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
takes_time(paint_chair_s4, 60, 90, minute). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
step(paint_chair, paint_chair_s5). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
action(paint_chair_s5, paint). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
object_of(paint_chair_s5, chair). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
uses(paint_chair_s5, paint). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
takes_time(paint_chair_s5, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
before(paint_chair_s4, paint_chair_s5). @conf 1 @source teacher:curriculum @conv s1794981719188-278172-74
```
