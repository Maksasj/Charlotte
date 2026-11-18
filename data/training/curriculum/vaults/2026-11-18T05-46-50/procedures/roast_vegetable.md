---
id: roast_vegetable
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# roast vegetable

Learned in conversation.

```datalog
task(roast_vegetable). @conf 1 @source ""
needs(roast_vegetable, vegetable). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
needs(roast_vegetable, olive_oil). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
needs(roast_vegetable, salt). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
needs(roast_vegetable, knife). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
needs(roast_vegetable, baking_tray). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
needs(roast_vegetable, oven). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
takes_time(roast_vegetable, 40, 50, minute). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
step(roast_vegetable, roast_vegetable_s1). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
action(roast_vegetable_s1, preheat). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
object_of(roast_vegetable_s1, oven). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
takes_time(roast_vegetable_s1, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
step(roast_vegetable, roast_vegetable_s2). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
action(roast_vegetable_s2, chop). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
object_of(roast_vegetable_s2, vegetable). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
uses(roast_vegetable_s2, knife). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
takes_time(roast_vegetable_s2, 8, 10, minute). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
overlaps(roast_vegetable_s2, roast_vegetable_s1). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
step(roast_vegetable, roast_vegetable_s3). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
action(roast_vegetable_s3, toss). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
object_of(roast_vegetable_s3, vegetable). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
uses(roast_vegetable_s3, olive_oil). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
uses(roast_vegetable_s3, salt). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
uses(roast_vegetable_s3, baking_tray). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
takes_time(roast_vegetable_s3, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
before(roast_vegetable_s2, roast_vegetable_s3). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
before(roast_vegetable_s1, roast_vegetable_s3). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
step(roast_vegetable, roast_vegetable_s4). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
action(roast_vegetable_s4, roast). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
object_of(roast_vegetable_s4, vegetable). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
uses(roast_vegetable_s4, oven). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
until(roast_vegetable_s4, golden). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
takes_time(roast_vegetable_s4, 25, 30, minute). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
before(roast_vegetable_s3, roast_vegetable_s4). @conf 1 @source teacher:curriculum @conv s1794978862660-85704-161
```
