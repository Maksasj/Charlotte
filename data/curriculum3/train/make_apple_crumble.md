---
id: make_apple_crumble
stage: procedures
task: make_apple_crumble
schema: v3
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
domain(make_apple_crumble, cooking).
goal(make_apple_crumble, apple_crumble).
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
setting(make_apple_crumble_s1, temperature, 180, celsius).
takes_time(make_apple_crumble_s1, 10, 12, minute).
step(make_apple_crumble, make_apple_crumble_s2).
action(make_apple_crumble_s2, peel).
object_of(make_apple_crumble_s2, apple).
uses(make_apple_crumble_s2, knife).
role(make_apple_crumble_s2, knife, cut).
overlaps(make_apple_crumble_s2, make_apple_crumble_s1).
step(make_apple_crumble, make_apple_crumble_s3).
action(make_apple_crumble_s3, slice).
object_of(make_apple_crumble_s3, apple).
uses(make_apple_crumble_s3, knife).
role(make_apple_crumble_s3, knife, cut).
overlaps(make_apple_crumble_s3, make_apple_crumble_s1).
takes_time(make_apple_crumble_s3, 8, 10, minute).
step(make_apple_crumble, make_apple_crumble_s4).
action(make_apple_crumble_s4, rub).
object_of(make_apple_crumble_s4, butter).
uses(make_apple_crumble_s4, flour).
role(make_apple_crumble_s4, flour, mix).
uses(make_apple_crumble_s4, sugar).
role(make_apple_crumble_s4, sugar, sweeten).
uses(make_apple_crumble_s4, bowl).
role(make_apple_crumble_s4, bowl, contain).
until(make_apple_crumble_s4, crumbly).
takes_time(make_apple_crumble_s4, 5, 6, minute).
step(make_apple_crumble, make_apple_crumble_s5).
action(make_apple_crumble_s5, spread).
object_of(make_apple_crumble_s5, apple).
uses(make_apple_crumble_s5, baking_dish).
role(make_apple_crumble_s5, baking_dish, contain).
into(make_apple_crumble_s5, baking_dish).
step(make_apple_crumble, make_apple_crumble_s6).
action(make_apple_crumble_s6, scatter).
object_of(make_apple_crumble_s6, crumble).
onto(make_apple_crumble_s6, apple).
takes_time(make_apple_crumble_s6, 1, 2, minute).
step(make_apple_crumble, make_apple_crumble_s7).
action(make_apple_crumble_s7, bake).
object_of(make_apple_crumble_s7, crumble).
uses(make_apple_crumble_s7, oven).
role(make_apple_crumble_s7, oven, heat).
until(make_apple_crumble_s7, golden).
takes_time(make_apple_crumble_s7, 30, 35, minute).
before(make_apple_crumble_s2, make_apple_crumble_s3).
before(make_apple_crumble_s1, make_apple_crumble_s4).
before(make_apple_crumble_s3, make_apple_crumble_s4).
before(make_apple_crumble_s4, make_apple_crumble_s5).
before(make_apple_crumble_s5, make_apple_crumble_s6).
before(make_apple_crumble_s6, make_apple_crumble_s7).
```

## aliases
```datalog
task_alias(make_apple_crumble, "making apple crumble").
task_alias(make_apple_crumble, "classic apple crumble").
task_alias(make_apple_crumble, "bake a crumble").
alias(apple, "apples").
alias(pear, "pears").
alias(pear, "ripe pears").
alias(preheat, "heat").
alias(crumbly, "looks like breadcrumbs").
alias(crumble, "crumble topping").
alias(scatter, "scatter on top").
```

## teach
- To make apple crumble you need apples, flour, butter, sugar, a knife, a bowl, a baking dish and an oven.
- Making apple crumble takes 48 to 55 minutes.
- Step 1 of making apple crumble: preheat the oven to 180 degrees for 10 to 12 minutes.
- While step 1 runs, step 2 of making apple crumble: peel the apples with the knife.
- While step 1 runs, step 3 of making apple crumble: slice the apples with the knife for 8 to 10 minutes.
- Step 4 of making apple crumble: rub the butter into the flour and sugar in the bowl until it looks like breadcrumbs, for 5 to 6 minutes.
- Step 5 of making apple crumble: spread the apples in the baking dish.
- Step 6 of making apple crumble: scatter the crumble on top for 1 to 2 minutes.
- Step 7 of making apple crumble: bake the crumble in the oven until golden, for 30 to 35 minutes.

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

```datalog situation
has(user, pear).
has(user, flour).
has(user, butter).
has(user, sugar).
has(user, knife).
has(user, bowl).
has(user, baking_dish).
has(user, oven).
lacks(user, apple).
```

```datalog difference
change(make_apple_crumble_s2, object_of, apple, pear).
change(make_apple_crumble_s3, object_of, apple, pear).
change(make_apple_crumble_s5, object_of, apple, pear).
change(make_apple_crumble_s6, onto, apple, pear).
extra_need(pear).
no_need(apple).
```
