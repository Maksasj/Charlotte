---
id: make_pesto
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make pesto

Learned in conversation.

```datalog
task(make_pesto). @conf 1 @source ""
needs(make_pesto, basil). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
needs(make_pesto, pine_nut). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
needs(make_pesto, parmesan). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
needs(make_pesto, garlic). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
needs(make_pesto, olive_oil). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
needs(make_pesto, frying_pan). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
needs(make_pesto, grater). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
needs(make_pesto, food_processor). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
takes_time(make_pesto, 5, 8, minute). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
step(make_pesto, make_pesto_s1). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
action(make_pesto_s1, toast). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
object_of(make_pesto_s1, pine_nut). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
uses(make_pesto_s1, frying_pan). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
until(make_pesto_s1, golden). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
takes_time(make_pesto_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
step(make_pesto, make_pesto_s2). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
action(make_pesto_s2, grate). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
object_of(make_pesto_s2, parmesan). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
uses(make_pesto_s2, grater). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
takes_time(make_pesto_s2, 2, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
overlaps(make_pesto_s2, make_pesto_s1). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
step(make_pesto, make_pesto_s3). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
action(make_pesto_s3, blend). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
object_of(make_pesto_s3, basil). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
uses(make_pesto_s3, food_processor). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
takes_time(make_pesto_s3, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
object_of(make_pesto_s3, pine_nut). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
object_of(make_pesto_s3, parmesan). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
object_of(make_pesto_s3, garlic). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
before(make_pesto_s2, make_pesto_s3). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
before(make_pesto_s1, make_pesto_s3). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
step(make_pesto, make_pesto_s4). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
action(make_pesto_s4, pour). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
object_of(make_pesto_s4, olive_oil). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
uses(make_pesto_s4, food_processor). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
until(make_pesto_s4, smooth). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
takes_time(make_pesto_s4, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
before(make_pesto_s3, make_pesto_s4). @conf 1 @source teacher:curriculum @conv s1794946314769-131028-59
step(make_pesto, make_pesto_s0). @conf 1 @source teacher:curriculum @conv s1794946613835-131028-270
before(make_pesto_s0, make_pesto_s1). @conf 1 @source teacher:curriculum @conv s1794946613835-131028-270
```
