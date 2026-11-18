---
id: make_tea
stage: procedures
task: make_tea
schema: v3
---
# Make tea

## requests
- I could really use a cup of tea right now. How do I make one properly?
- Can you walk me through brewing a mug of tea with a tea bag?
- I want to make myself a hot cup of tea. What do I do?

## situation
- I have tea bags, a kettle, water and a mug.
- I've got about 10 minutes before I need to go.

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

## checklist
- names what is needed: a tea bag, water, a kettle and a mug
- gives the steps in order: boil, pour, steep, remove
- gives a total time of about 5 to 7 minutes
- closes politely

## variant
- change: no kettle: heat the water in a pot on the stove
- request: I want a cup of tea but my kettle just broke. Can I still make one?
- situation: I have tea bags, a mug, water and a small pot on the stove, but no kettle.
- expect: The reply boils the water in a pot on the stove instead of a kettle and keeps the pour, steep and remove steps.

```datalog situation
has(user, tea_bag).
has(user, mug).
has(user, water).
has(user, pot).
has(user, stove).
lacks(user, kettle).
```

```datalog difference
swap(make_tea_s1, kettle, pot).
no_need(kettle).
extra_need(pot).
extra_need(stove).
```
