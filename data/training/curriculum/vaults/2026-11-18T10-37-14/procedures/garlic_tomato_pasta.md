---
id: garlic_tomato_pasta
status: confirmed
source: seed
uses: 0
successes: 0
---
# Garlic tomato pasta

The design's example recipe (Design 2.0, "Procedures").

```datalog
task(garlic_tomato_pasta).
solves(garlic_tomato_pasta, plan_meal).
needs(garlic_tomato_pasta, pasta).
needs(garlic_tomato_pasta, garlic).
needs(garlic_tomato_pasta, olive_oil).
needs(garlic_tomato_pasta, cherry_tomato).
takes_time(garlic_tomato_pasta, 15, 20, minute).
step(garlic_tomato_pasta, gtp_s1).
action(gtp_s1, boil).
object_of(gtp_s1, pasta).
takes_time(gtp_s1, 10, 12, minute).
step(garlic_tomato_pasta, gtp_s2).
action(gtp_s2, fry).
object_of(gtp_s2, garlic).
uses(gtp_s2, olive_oil).
takes_time(gtp_s2, 1, 1, minute).
overlaps(gtp_s2, gtp_s1).
step(garlic_tomato_pasta, gtp_s3).
action(gtp_s3, add).
object_of(gtp_s3, cherry_tomato).
until(gtp_s3, soft).
takes_time(gtp_s3, 5, 7, minute).
step(garlic_tomato_pasta, gtp_s4).
action(gtp_s4, mix).
object_of(gtp_s4, pasta).
before(gtp_s2, gtp_s3).
before(gtp_s1, gtp_s4).
before(gtp_s3, gtp_s4).
```
