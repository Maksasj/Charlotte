---
id: bake_banana_bread__30_minutes
stage: procedures
task: bake_banana_bread
schema: v3
tier: S
base: bake_banana_bread
---
# Bake banana bread

## requests
- Can I bake a banana loaf in the half hour before my guests come?

## ground truth
```datalog
task(bake_banana_bread).
domain(bake_banana_bread, cooking).
goal(bake_banana_bread, banana_bread).
needs(bake_banana_bread, banana).
needs(bake_banana_bread, flour).
needs(bake_banana_bread, sugar).
needs(bake_banana_bread, egg).
needs(bake_banana_bread, butter).
needs(bake_banana_bread, baking_soda).
needs(bake_banana_bread, bowl).
needs(bake_banana_bread, fork).
needs(bake_banana_bread, loaf_tin).
needs(bake_banana_bread, oven).
takes_time(bake_banana_bread, 65, 80, minute).
step(bake_banana_bread, bake_banana_bread_s1).
action(bake_banana_bread_s1, preheat).
object_of(bake_banana_bread_s1, oven).
setting(bake_banana_bread_s1, temperature, 175, celsius).
takes_time(bake_banana_bread_s1, 10, 15, minute).
step(bake_banana_bread, bake_banana_bread_s2).
action(bake_banana_bread_s2, mash).
object_of(bake_banana_bread_s2, banana).
uses(bake_banana_bread_s2, bowl).
role(bake_banana_bread_s2, bowl, contain).
uses(bake_banana_bread_s2, fork).
role(bake_banana_bread_s2, fork, press).
requires_state(bake_banana_bread_s2, banana_ripe).
overlaps(bake_banana_bread_s2, bake_banana_bread_s1).
takes_time(bake_banana_bread_s2, 3, 5, minute).
step(bake_banana_bread, bake_banana_bread_s3).
action(bake_banana_bread_s3, mix).
object_of(bake_banana_bread_s3, flour).
uses(bake_banana_bread_s3, butter).
role(bake_banana_bread_s3, butter, flavour).
uses(bake_banana_bread_s3, sugar).
role(bake_banana_bread_s3, sugar, sweeten).
uses(bake_banana_bread_s3, egg).
role(bake_banana_bread_s3, egg, bind).
uses(bake_banana_bread_s3, baking_soda).
role(bake_banana_bread_s3, baking_soda, raise).
requires_state(bake_banana_bread_s3, butter_melted).
takes_time(bake_banana_bread_s3, 3, 5, minute).
step(bake_banana_bread, bake_banana_bread_s4).
action(bake_banana_bread_s4, pour).
object_of(bake_banana_bread_s4, batter).
uses(bake_banana_bread_s4, loaf_tin).
role(bake_banana_bread_s4, loaf_tin, contain).
into(bake_banana_bread_s4, loaf_tin).
requires_state(bake_banana_bread_s4, tin_greased).
takes_time(bake_banana_bread_s4, 1, 1, minute).
step(bake_banana_bread, bake_banana_bread_s5).
action(bake_banana_bread_s5, bake).
object_of(bake_banana_bread_s5, banana_bread).
uses(bake_banana_bread_s5, oven).
role(bake_banana_bread_s5, oven, heat).
until(bake_banana_bread_s5, clean).
check(bake_banana_bread_s5, skewer_clean).
takes_time(bake_banana_bread_s5, 50, 60, minute).
before(bake_banana_bread_s1, bake_banana_bread_s3).
before(bake_banana_bread_s2, bake_banana_bread_s3).
before(bake_banana_bread_s3, bake_banana_bread_s4).
before(bake_banana_bread_s4, bake_banana_bread_s5).
```

## aliases
```datalog
task_alias(bake_banana_bread, "baking banana bread").
task_alias(bake_banana_bread, "loaf of banana bread").
alias(banana, "bananas").
alias(banana, "ripe bananas").
alias(banana, "brown bananas").
alias(egg, "eggs").
alias(butter, "melted butter").
alias(loaf_tin, "greased loaf tin").
alias(cake_tin, "square cake tin").
alias(preheat, "heat").
alias(mix, "mix in").
alias(clean, "a skewer comes out clean").
alias(skewer_clean, "skewer comes out clean").
```

## teach
- To bake banana bread you need ripe bananas, flour, sugar, eggs, melted butter, baking soda, a bowl, a fork, a loaf tin and an oven.
- Baking banana bread takes 65 to 80 minutes.
- Step 1 of baking banana bread: preheat the oven to 175 degrees for 10 to 15 minutes.
- While step 1 runs, step 2 of baking banana bread: mash the bananas in the bowl with the fork for 3 to 5 minutes.
- Step 3 of baking banana bread: mix in the melted butter, the sugar, the eggs, the flour and the baking soda, for 3 to 5 minutes.
- Step 4 of baking banana bread: pour the batter into the greased loaf tin for 1 minute.
- Step 5 of baking banana bread: bake the banana bread in the oven until a skewer comes out clean, for 50 to 60 minutes.

## variant
- change: only 30 minutes: a loaf cannot be baked in time
- request: Can I bake a banana loaf in the half hour before my guests come?
- situation: I have everything for banana bread and an oven, but only 30 minutes.
- expect: The reply says a loaf needs 50 to 60 minutes in the oven alone, so it cannot be done in 30 minutes.

```datalog situation
has(user, banana).
has(user, flour).
has(user, sugar).
has(user, egg).
has(user, butter).
has(user, baking_soda).
has(user, bowl).
has(user, fork).
has(user, loaf_tin).
has(user, oven).
time_limit(user, 30, minute).
```

```datalog difference
cannot(not_enough_time).
```
