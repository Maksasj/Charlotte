---
id: make_tea__wants_milk
stage: procedures
task: make_tea
schema: v3
tier: S
base: make_tea
---
# Make tea

## requests
- How do I make a proper builder's brew? I like it with a splash of milk.

## ground truth
```datalog
task(make_tea).
domain(make_tea, cooking).
goal(make_tea, warm_drink).
needs(make_tea, tea_bag).
needs(make_tea, water).
needs(make_tea, kettle).
needs(make_tea, mug).
takes_time(make_tea, 5, 7, minute).
step(make_tea, make_tea_s1).
action(make_tea_s1, boil).
object_of(make_tea_s1, water).
uses(make_tea_s1, kettle).
role(make_tea_s1, kettle, heat).
takes_time(make_tea_s1, 3, 4, minute).
step(make_tea, make_tea_s2).
action(make_tea_s2, pour).
object_of(make_tea_s2, water).
uses(make_tea_s2, mug).
role(make_tea_s2, mug, contain).
uses(make_tea_s2, tea_bag).
role(make_tea_s2, tea_bag, flavour).
step(make_tea, make_tea_s3).
action(make_tea_s3, steep).
object_of(make_tea_s3, tea_bag).
takes_time(make_tea_s3, 2, 3, minute).
step(make_tea, make_tea_s4).
action(make_tea_s4, remove).
object_of(make_tea_s4, tea_bag).
from(make_tea_s4, mug).
before(make_tea_s1, make_tea_s2).
before(make_tea_s2, make_tea_s3).
before(make_tea_s3, make_tea_s4).
```

## aliases
```datalog
task_alias(make_tea, "making tea").
task_alias(make_tea, "cup of tea").
task_alias(make_tea, "brewing a mug of tea").
task_alias(make_tea, "hot cup of tea").
alias(tea_bag, "tea bags").
alias(tea_bag, "teabag").
alias(steep, "brew").
alias(pot, "small pot").
alias(pot, "pot on the stove").
```

## teach
- To make tea you need a tea bag, water, a kettle and a mug.
- Making tea takes 5 to 7 minutes.
- Step 1 of making tea: boil the water in the kettle for 3 to 4 minutes.
- Step 2 of making tea: pour the water into the mug over the tea bag.
- Step 3 of making tea: steep the tea bag for 2 to 3 minutes.
- Step 4 of making tea: remove the tea bag.

## variant
- change: add a splash of milk after the tea bag comes out
- request: How do I make a proper builder's brew? I like it with a splash of milk.
- situation: I have tea bags, a kettle, water, a mug and milk.
- expect: The reply brews the tea as usual and adds a splash of milk after taking the tea bag out.

```datalog situation
has(user, tea_bag).
has(user, kettle).
has(user, water).
has(user, mug).
has(user, milk).
wants(user, milk).
```

```datalog difference
extra_need(milk).
add_step(make_tea_s9, make_tea_s4).
step(make_tea, make_tea_s9).
action(make_tea_s9, add).
object_of(make_tea_s9, milk).
into(make_tea_s9, mug).
before(make_tea_s4, make_tea_s9).
```
