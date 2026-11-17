---
id: mop_floor
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# mop floor

Learned in conversation.

```datalog
task(mop_floor). @conf 1 @source ""
needs(mop_floor, broom). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
needs(mop_floor, bucket). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
needs(mop_floor, mop). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
needs(mop_floor, floor_cleaner). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
needs(mop_floor, water). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
takes_time(mop_floor, 25, 35, minute). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
step(mop_floor, mop_floor_s2). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
action(mop_floor_s2, fill). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
object_of(mop_floor_s2, bucket). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
uses(mop_floor_s2, warm_water). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
uses(mop_floor_s2, floor_cleaner). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
takes_time(mop_floor_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
step(mop_floor, mop_floor_s3). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
action(mop_floor_s3, dip). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
object_of(mop_floor_s3, mop). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
uses(mop_floor_s3, bucket). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
until(mop_floor_s3, damp). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
takes_time(mop_floor_s3, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
before(mop_floor_s2, mop_floor_s3). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
step(mop_floor, mop_floor_s4). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
action(mop_floor_s4, mop). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
object_of(mop_floor_s4, floor). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
takes_time(mop_floor_s4, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
before(mop_floor_s3, mop_floor_s4). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
step(mop_floor, mop_floor_s5). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
action(mop_floor_s5, dry). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
object_of(mop_floor_s5, floor). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
takes_time(mop_floor_s5, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
before(mop_floor_s4, mop_floor_s5). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
step(mop_floor, mop_floor_s1). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
action(mop_floor_s1, sweep). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
object_of(mop_floor_s1, floor). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
uses(mop_floor_s1, broom). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
takes_time(mop_floor_s1, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946354656-131028-80
action(mop_floor_s3, wring). @conf 1 @source teacher:curriculum @conv s1794946649413-131028-308
takes_time(mop_floor_s4, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794946649413-131028-308
uses(mop_floor_s4, towel). @conf 1 @source teacher:curriculum @conv s1794946649413-131028-308
before(mop_floor_s1, mop_floor_s2). @conf 1 @source teacher:curriculum @conv s1794946700320-131028-365
```
