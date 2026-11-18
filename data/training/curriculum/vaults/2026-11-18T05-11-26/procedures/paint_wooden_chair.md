---
id: paint_wooden_chair
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# paint wooden chair

Learned in conversation.

```datalog
task(paint_wooden_chair). @conf 1 @source ""
needs(paint_wooden_chair, sandpaper). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
needs(paint_wooden_chair, damp_cloth). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
needs(paint_wooden_chair, primer). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
needs(paint_wooden_chair, paint). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
needs(paint_wooden_chair, paintbrush). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
needs(paint_wooden_chair, newspaper). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
takes_time(paint_wooden_chair, 120, 180, minute). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
step(paint_wooden_chair, paint_wooden_chair_s1). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
action(paint_wooden_chair_s1, sand). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
object_of(paint_wooden_chair_s1, chair). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
uses(paint_wooden_chair_s1, sandpaper). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
takes_time(paint_wooden_chair_s1, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
step(paint_wooden_chair, paint_wooden_chair_s2). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
action(paint_wooden_chair_s2, wipe). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
object_of(paint_wooden_chair_s2, dust). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
uses(paint_wooden_chair_s2, damp_cloth). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
takes_time(paint_wooden_chair_s2, 5, 5, minute). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
before(paint_wooden_chair_s1, paint_wooden_chair_s2). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
step(paint_wooden_chair, paint_wooden_chair_s3). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
action(paint_wooden_chair_s3, brush). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
object_of(paint_wooden_chair_s3, primer). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
uses(paint_wooden_chair_s3, newspaper). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
takes_time(paint_wooden_chair_s3, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
before(paint_wooden_chair_s2, paint_wooden_chair_s3). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
step(paint_wooden_chair, paint_wooden_chair_s4). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
action(paint_wooden_chair_s4, dry). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
object_of(paint_wooden_chair_s4, primer). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
takes_time(paint_wooden_chair_s4, 60, 90, minute). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
before(paint_wooden_chair_s3, paint_wooden_chair_s4). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
step(paint_wooden_chair, paint_wooden_chair_s5). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
action(paint_wooden_chair_s5, paint). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
object_of(paint_wooden_chair_s5, chair). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
uses(paint_wooden_chair_s5, paint). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
takes_time(paint_wooden_chair_s5, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
before(paint_wooden_chair_s4, paint_wooden_chair_s5). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
action(paint_wooden_chair_s3, set). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
object_of(paint_wooden_chair_s3, chair). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
action(paint_wooden_chair_s4, brush). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
takes_time(paint_wooden_chair_s4, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794978795247-85704-101
```
