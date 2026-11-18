---
id: make_lemonade
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Make lemonade

Mix fresh lemon juice, sugar and cold water over ice.

## ground truth
```datalog
task(make_lemonade).
domain(make_lemonade, cooking).
goal(make_lemonade, lemonade).
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
role(make_lemonade_s2, water, mix).
takes_time(make_lemonade_s2, 2, 3, minute).
until(make_lemonade_s2, dissolved).
step(make_lemonade, make_lemonade_s3).
action(make_lemonade_s3, mix).
object_of(make_lemonade_s3, lemon_juice).
uses(make_lemonade_s3, jug).
role(make_lemonade_s3, jug, contain).
uses(make_lemonade_s3, water).
role(make_lemonade_s3, water, mix).
takes_time(make_lemonade_s3, 1, 2, minute).
into(make_lemonade_s3, jug).
step(make_lemonade, make_lemonade_s4).
action(make_lemonade_s4, add).
object_of(make_lemonade_s4, ice).
uses(make_lemonade_s4, jug).
role(make_lemonade_s4, jug, contain).
takes_time(make_lemonade_s4, 1, 1, minute).
into(make_lemonade_s4, jug).
before(make_lemonade_s1, make_lemonade_s2).
before(make_lemonade_s2, make_lemonade_s3).
before(make_lemonade_s3, make_lemonade_s4).
```

## aliases
```datalog
task_alias(make_lemonade, "making lemonade").
task_alias(make_lemonade, "homemade lemonade").
task_alias(make_lemonade, "fresh lemonade").
alias(lemon, "lemons").
alias(lemon, "fresh lemons").
alias(lemon_juice, "lemon juice").
alias(lemon_juice, "fresh lemon juice").
alias(water, "cold water").
alias(ice, "ice cubes").
alias(jug, "pitcher").
alias(squeeze, "juice").
```
