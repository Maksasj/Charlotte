---
id: bake_potato__no_fork
stage: procedures
task: bake_potato
schema: v3
tier: S
base: bake_potato
---
# Bake a potato

## requests
- Doing jacket potatoes but I can't find a single fork. There's a metal skewer in the drawer.

## ground truth
```datalog
task(bake_potato).
domain(bake_potato, cooking).
goal(bake_potato, baked_potato).
needs(bake_potato, potato).
needs(bake_potato, olive_oil).
needs(bake_potato, salt).
needs(bake_potato, fork).
needs(bake_potato, baking_tray).
needs(bake_potato, oven).
takes_time(bake_potato, 60, 70, minute).
step(bake_potato, bake_potato_s1).
action(bake_potato_s1, heat).
object_of(bake_potato_s1, oven).
setting(bake_potato_s1, temperature, 200, celsius).
takes_time(bake_potato_s1, 10, 10, minute).
step(bake_potato, bake_potato_s2).
action(bake_potato_s2, wash).
object_of(bake_potato_s2, potato).
overlaps(bake_potato_s2, bake_potato_s1).
step(bake_potato, bake_potato_s3).
action(bake_potato_s3, prick).
object_of(bake_potato_s3, potato).
uses(bake_potato_s3, fork).
role(bake_potato_s3, fork, pierce).
purpose(bake_potato_s3, steam_escapes).
overlaps(bake_potato_s3, bake_potato_s1).
step(bake_potato, bake_potato_s4).
action(bake_potato_s4, rub).
object_of(bake_potato_s4, potato).
uses(bake_potato_s4, olive_oil).
role(bake_potato_s4, olive_oil, coat).
uses(bake_potato_s4, salt).
role(bake_potato_s4, salt, season).
purpose(bake_potato_s4, crispy_skin).
overlaps(bake_potato_s4, bake_potato_s1).
step(bake_potato, bake_potato_s5).
action(bake_potato_s5, bake).
object_of(bake_potato_s5, potato).
uses(bake_potato_s5, baking_tray).
role(bake_potato_s5, baking_tray, hold).
uses(bake_potato_s5, oven).
role(bake_potato_s5, oven, heat).
until(bake_potato_s5, soft).
check(bake_potato_s5, soft_inside).
takes_time(bake_potato_s5, 50, 60, minute).
before(bake_potato_s2, bake_potato_s3).
before(bake_potato_s3, bake_potato_s4).
before(bake_potato_s1, bake_potato_s5).
before(bake_potato_s4, bake_potato_s5).
```

## aliases
```datalog
task_alias(bake_potato, "baked potato").
task_alias(bake_potato, "jacket potato").
task_alias(bake_potato, "baking a potato").
alias(potato, "large potato").
alias(potato, "big potato").
alias(baking_tray, "baking sheet").
alias(olive_oil, "oil").
alias(heat, "preheat").
alias(soft, "soft inside").
alias(steam_escapes, "lets steam out").
```

## teach
- To bake a potato you need a potato, olive oil, salt, a fork, a baking tray and an oven.
- Baking a potato takes 60 to 70 minutes.
- Step 1 of baking a potato: heat the oven to 200 degrees for 10 minutes.
- While step 1 runs, step 2 of baking a potato: wash the potato.
- While step 1 runs, step 3 of baking a potato: prick the potato all over with the fork so steam can escape.
- While step 1 runs, step 4 of baking a potato: rub the potato with olive oil and salt for a crispy skin.
- Step 5 of baking a potato: bake the potato on the baking tray in the oven until it is soft inside, for 50 to 60 minutes.

## variant
- change: no fork: prick the potato with a metal skewer
- request: Doing jacket potatoes but I can't find a single fork. There's a metal skewer in the drawer.
- situation: I have a large potato, olive oil, salt, a metal skewer, a baking tray and an oven, but no fork.
- expect: The reply pricks the potato with the skewer instead of a fork and keeps the other steps and times.

```datalog situation
has(user, potato).
has(user, olive_oil).
has(user, salt).
has(user, skewer).
has(user, baking_tray).
has(user, oven).
lacks(user, fork).
```

```datalog difference
swap(bake_potato_s3, fork, skewer).
no_need(fork).
extra_need(skewer).
```
