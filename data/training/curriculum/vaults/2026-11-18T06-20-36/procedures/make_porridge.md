---
id: make_porridge
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Make porridge

Cook oats in milk, stirring until thick.

```datalog
task(make_porridge).
solves(make_porridge, plan_meal).
needs(make_porridge, oat).
needs(make_porridge, milk).
needs(make_porridge, salt).
needs(make_porridge, pot).
needs(make_porridge, spoon).
takes_time(make_porridge, 5, 10, minute).
step(make_porridge, make_porridge_s1).
action(make_porridge_s1, pour).
object_of(make_porridge_s1, milk).
uses(make_porridge_s1, pot).
takes_time(make_porridge_s1, 1, 1, minute).
step(make_porridge, make_porridge_s2).
action(make_porridge_s2, add).
object_of(make_porridge_s2, oat).
takes_time(make_porridge_s2, 1, 1, minute).
step(make_porridge, make_porridge_s3).
action(make_porridge_s3, stir).
object_of(make_porridge_s3, porridge).
uses(make_porridge_s3, spoon).
takes_time(make_porridge_s3, 4, 6, minute).
until(make_porridge_s3, thick).
step(make_porridge, make_porridge_s4).
action(make_porridge_s4, add).
object_of(make_porridge_s4, salt).
step(make_porridge, make_porridge_s5).
action(make_porridge_s5, serve).
object_of(make_porridge_s5, porridge).
before(make_porridge_s1, make_porridge_s2).
before(make_porridge_s2, make_porridge_s3).
before(make_porridge_s3, make_porridge_s4).
before(make_porridge_s4, make_porridge_s5).
```
