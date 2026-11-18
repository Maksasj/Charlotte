---
id: make_sandwich
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Make a sandwich

Build a simple ham and cheese sandwich.

```datalog
task(make_sandwich).
solves(make_sandwich, plan_meal).
needs(make_sandwich, bread).
needs(make_sandwich, butter).
needs(make_sandwich, ham).
needs(make_sandwich, cheese).
needs(make_sandwich, lettuce).
needs(make_sandwich, knife).
takes_time(make_sandwich, 5, 10, minute).
step(make_sandwich, make_sandwich_s1).
action(make_sandwich_s1, spread).
object_of(make_sandwich_s1, butter).
uses(make_sandwich_s1, knife).
uses(make_sandwich_s1, bread).
takes_time(make_sandwich_s1, 1, 2, minute).
step(make_sandwich, make_sandwich_s2).
action(make_sandwich_s2, add).
object_of(make_sandwich_s2, ham).
takes_time(make_sandwich_s2, 1, 1, minute).
step(make_sandwich, make_sandwich_s3).
action(make_sandwich_s3, add).
object_of(make_sandwich_s3, cheese).
takes_time(make_sandwich_s3, 1, 1, minute).
step(make_sandwich, make_sandwich_s4).
action(make_sandwich_s4, add).
object_of(make_sandwich_s4, lettuce).
takes_time(make_sandwich_s4, 1, 1, minute).
step(make_sandwich, make_sandwich_s5).
action(make_sandwich_s5, close).
object_of(make_sandwich_s5, sandwich).
uses(make_sandwich_s5, bread).
step(make_sandwich, make_sandwich_s6).
action(make_sandwich_s6, cut).
object_of(make_sandwich_s6, sandwich).
uses(make_sandwich_s6, knife).
takes_time(make_sandwich_s6, 1, 1, minute).
before(make_sandwich_s1, make_sandwich_s2).
before(make_sandwich_s2, make_sandwich_s3).
before(make_sandwich_s3, make_sandwich_s4).
before(make_sandwich_s4, make_sandwich_s5).
before(make_sandwich_s5, make_sandwich_s6).
```
