---
id: grilled_cheese
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Grilled cheese

Fry a buttered cheese sandwich until golden on both sides.

```datalog
task(grilled_cheese).
solves(grilled_cheese, plan_meal).
needs(grilled_cheese, bread).
needs(grilled_cheese, cheese).
needs(grilled_cheese, butter).
needs(grilled_cheese, pan).
needs(grilled_cheese, knife).
takes_time(grilled_cheese, 8, 12, minute).
step(grilled_cheese, grilled_cheese_s1).
action(grilled_cheese_s1, spread).
object_of(grilled_cheese_s1, butter).
uses(grilled_cheese_s1, knife).
uses(grilled_cheese_s1, bread).
takes_time(grilled_cheese_s1, 1, 2, minute).
step(grilled_cheese, grilled_cheese_s2).
action(grilled_cheese_s2, add).
object_of(grilled_cheese_s2, cheese).
uses(grilled_cheese_s2, bread).
takes_time(grilled_cheese_s2, 1, 1, minute).
step(grilled_cheese, grilled_cheese_s3).
action(grilled_cheese_s3, heat).
object_of(grilled_cheese_s3, pan).
takes_time(grilled_cheese_s3, 1, 2, minute).
until(grilled_cheese_s3, hot).
step(grilled_cheese, grilled_cheese_s4).
action(grilled_cheese_s4, fry).
object_of(grilled_cheese_s4, sandwich).
uses(grilled_cheese_s4, pan).
takes_time(grilled_cheese_s4, 2, 3, minute).
until(grilled_cheese_s4, golden).
step(grilled_cheese, grilled_cheese_s5).
action(grilled_cheese_s5, flip).
object_of(grilled_cheese_s5, sandwich).
uses(grilled_cheese_s5, pan).
takes_time(grilled_cheese_s5, 2, 3, minute).
until(grilled_cheese_s5, golden).
step(grilled_cheese, grilled_cheese_s6).
action(grilled_cheese_s6, cut).
object_of(grilled_cheese_s6, sandwich).
uses(grilled_cheese_s6, knife).
overlaps(grilled_cheese_s3, grilled_cheese_s1).
before(grilled_cheese_s1, grilled_cheese_s2).
before(grilled_cheese_s2, grilled_cheese_s4).
before(grilled_cheese_s3, grilled_cheese_s4).
before(grilled_cheese_s4, grilled_cheese_s5).
before(grilled_cheese_s5, grilled_cheese_s6).
```
