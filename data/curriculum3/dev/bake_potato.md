---
id: bake_potato
stage: procedures
task: bake_potato
schema: v3
---
# Bake a potato

## requests
- I'd like a baked potato with crispy skin for dinner. How do I make one in the oven?
- How do you do a jacket potato properly?
- Could you walk me through baking a big potato? I want it soft inside.

## situation
- I have a large potato, olive oil, salt, a fork, a baking tray and an oven.
- I've got a bit over an hour before dinner.

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

## checklist
- names what is needed, including a potato, olive oil, salt and an oven
- gives the steps in order: heat oven, wash, prick, rub with oil and salt, bake until soft
- says the washing, pricking and oiling can be done while the oven heats
- gives a total time of about 60 to 70 minutes

## variant
- change: only 25 minutes: cook the potato in the microwave first, then crisp it in the oven
- request: I want a baked potato but I only have about 25 minutes. Is that possible?
- situation: I have a potato, olive oil, salt, a fork, a microwave and an oven, and about 25 minutes.
- expect: The reply microwaves the pricked potato for about 8 to 10 minutes and then crisps it briefly in a hot oven so it fits in 25 minutes.

```datalog situation
has(user, potato).
has(user, olive_oil).
has(user, salt).
has(user, fork).
has(user, microwave).
has(user, oven).
time_limit(user, 25, minute).
```

```datalog difference
extra_need(microwave).
add_step(bake_potato_s9, bake_potato_s4).
step(bake_potato, bake_potato_s9).
action(bake_potato_s9, microwave).
object_of(bake_potato_s9, potato).
uses(bake_potato_s9, microwave).
role(bake_potato_s9, microwave, heat).
takes_time(bake_potato_s9, 8, 10, minute).
before(bake_potato_s9, bake_potato_s5).
change(bake_potato_s5, takes_time, 50_60, 5_10).
```
