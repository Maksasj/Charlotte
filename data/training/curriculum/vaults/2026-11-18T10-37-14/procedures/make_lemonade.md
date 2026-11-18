---
id: make_lemonade
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Make lemonade

Mix fresh lemon juice, sugar and cold water over ice.

```datalog
task(make_lemonade).
needs(make_lemonade, lemon).
needs(make_lemonade, sugar).
needs(make_lemonade, water).
needs(make_lemonade, ice).
needs(make_lemonade, jug).
takes_time(make_lemonade, 10, 15, minute).
step(make_lemonade, make_lemonade_s1).
action(make_lemonade_s1, squeeze).
object_of(make_lemonade_s1, lemon).
takes_time(make_lemonade_s1, 3, 5, minute).
step(make_lemonade, make_lemonade_s2).
action(make_lemonade_s2, dissolve).
object_of(make_lemonade_s2, sugar).
uses(make_lemonade_s2, water).
takes_time(make_lemonade_s2, 2, 3, minute).
until(make_lemonade_s2, dissolved).
step(make_lemonade, make_lemonade_s3).
action(make_lemonade_s3, mix).
object_of(make_lemonade_s3, lemon_juice).
uses(make_lemonade_s3, jug).
uses(make_lemonade_s3, water).
takes_time(make_lemonade_s3, 1, 2, minute).
step(make_lemonade, make_lemonade_s4).
action(make_lemonade_s4, add).
object_of(make_lemonade_s4, ice).
uses(make_lemonade_s4, jug).
takes_time(make_lemonade_s4, 1, 1, minute).
before(make_lemonade_s1, make_lemonade_s2).
before(make_lemonade_s2, make_lemonade_s3).
before(make_lemonade_s3, make_lemonade_s4).
```
