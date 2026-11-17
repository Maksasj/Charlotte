---
id: make_apple_crumble
stage: procedures
task: make_apple_crumble
---
# Make apple crumble

## requests
- I'd like to make an apple crumble for pudding. How?
- Can you walk me through a classic apple crumble?
- We've got loads of apples from the garden. How do I bake a crumble with them?

## situation
- I have apples, flour, butter, sugar, a knife, a bowl, a baking dish and an oven.
- We eat in about an hour and a half.

## ground truth
```datalog
task(make_apple_crumble).
needs(make_apple_crumble, apple).
needs(make_apple_crumble, flour).
needs(make_apple_crumble, butter).
needs(make_apple_crumble, sugar).
needs(make_apple_crumble, knife).
needs(make_apple_crumble, bowl).
needs(make_apple_crumble, baking_dish).
needs(make_apple_crumble, oven).
takes_time(make_apple_crumble, 48, 55, minute).
step(make_apple_crumble, make_apple_crumble_s1).
action(make_apple_crumble_s1, preheat).
object_of(make_apple_crumble_s1, oven).
takes_time(make_apple_crumble_s1, 10, 12, minute).
step(make_apple_crumble, make_apple_crumble_s2).
action(make_apple_crumble_s2, slice).
object_of(make_apple_crumble_s2, apple).
uses(make_apple_crumble_s2, knife).
takes_time(make_apple_crumble_s2, 8, 10, minute).
overlaps(make_apple_crumble_s2, make_apple_crumble_s1).
step(make_apple_crumble, make_apple_crumble_s3).
action(make_apple_crumble_s3, rub).
object_of(make_apple_crumble_s3, butter).
uses(make_apple_crumble_s3, flour).
uses(make_apple_crumble_s3, sugar).
uses(make_apple_crumble_s3, bowl).
until(make_apple_crumble_s3, crumbly).
takes_time(make_apple_crumble_s3, 5, 6, minute).
step(make_apple_crumble, make_apple_crumble_s4).
action(make_apple_crumble_s4, spread).
object_of(make_apple_crumble_s4, apple).
uses(make_apple_crumble_s4, baking_dish).
takes_time(make_apple_crumble_s4, 1, 2, minute).
step(make_apple_crumble, make_apple_crumble_s5).
action(make_apple_crumble_s5, bake).
object_of(make_apple_crumble_s5, crumble).
uses(make_apple_crumble_s5, oven).
until(make_apple_crumble_s5, golden).
takes_time(make_apple_crumble_s5, 30, 35, minute).
before(make_apple_crumble_s1, make_apple_crumble_s3).
before(make_apple_crumble_s2, make_apple_crumble_s3).
before(make_apple_crumble_s3, make_apple_crumble_s4).
before(make_apple_crumble_s4, make_apple_crumble_s5).
```

## teach
- To make apple crumble you need apples, flour, butter, sugar, a knife, a bowl, a baking dish and an oven.
- Making apple crumble takes 48 to 55 minutes.
- Step 1 of making apple crumble: preheat the oven to 180 degrees for 10 to 12 minutes.
- While step 1 runs, step 2 of making apple crumble: peel and slice the apples with the knife for 8 to 10 minutes.
- Step 3 of making apple crumble: rub the butter into the flour and sugar in the bowl until it looks like breadcrumbs, for 5 to 6 minutes.
- Step 4 of making apple crumble: spread the apples in the baking dish and scatter the crumble on top for 1 to 2 minutes.
- Step 5 of making apple crumble: bake the crumble in the oven until golden, for 30 to 35 minutes.

## checklist
- names what is needed: apples, flour, butter, sugar, a knife, a bowl, a baking dish and an oven
- gives the steps in order: preheat oven, slice apple (while preheat oven), rub butter, spread apple, bake crumble
- gives a total time of about 48 to 55 minutes
- closes politely

## variant
- change: pears instead of apples: slice the pears a little thicker and keep everything else
- request: I don't have apples, only some ripe pears. Can I make a crumble with those?
- situation: I have ripe pears, flour, butter, sugar, a knife, a bowl, a baking dish and an oven.
- expect: The reply uses pears in place of apples and keeps the preheat, rub, spread and bake steps with the same times.
