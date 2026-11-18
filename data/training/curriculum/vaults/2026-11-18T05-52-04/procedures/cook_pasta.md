---
id: cook_pasta
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Cook pasta

Boil dried pasta in salted water until soft and drain it.

```datalog
task(cook_pasta).
solves(cook_pasta, plan_meal).
needs(cook_pasta, pasta).
needs(cook_pasta, water).
needs(cook_pasta, salt).
needs(cook_pasta, pot).
needs(cook_pasta, colander).
takes_time(cook_pasta, 15, 20, minute).
step(cook_pasta, cook_pasta_s1).
action(cook_pasta_s1, boil).
object_of(cook_pasta_s1, water).
uses(cook_pasta_s1, pot).
takes_time(cook_pasta_s1, 5, 8, minute).
until(cook_pasta_s1, boiling).
step(cook_pasta, cook_pasta_s2).
action(cook_pasta_s2, add).
object_of(cook_pasta_s2, salt).
step(cook_pasta, cook_pasta_s3).
action(cook_pasta_s3, add).
object_of(cook_pasta_s3, pasta).
step(cook_pasta, cook_pasta_s4).
action(cook_pasta_s4, cook).
object_of(cook_pasta_s4, pasta).
uses(cook_pasta_s4, pot).
takes_time(cook_pasta_s4, 8, 10, minute).
until(cook_pasta_s4, soft).
step(cook_pasta, cook_pasta_s5).
action(cook_pasta_s5, drain).
object_of(cook_pasta_s5, pasta).
uses(cook_pasta_s5, colander).
takes_time(cook_pasta_s5, 1, 1, minute).
before(cook_pasta_s1, cook_pasta_s2).
before(cook_pasta_s2, cook_pasta_s3).
before(cook_pasta_s3, cook_pasta_s4).
before(cook_pasta_s4, cook_pasta_s5).
```
