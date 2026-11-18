---
id: put_up_bean_canes
stage: household
task: put_up_bean_canes
schema: v3
tier: G
---
# Put up bean canes

## requests
- My runner beans are ready to go outside. How do I build them something to climb?
- How do you make a wigwam of canes for climbing beans?
- I've got eight bean plants and a bundle of bamboo. What's the best way to plant them?

## situation
- I have eight bamboo canes, garden twine, a trowel, eight runner bean seedlings, a watering can and water.

## ground truth
```datalog
task(put_up_bean_canes).
domain(put_up_bean_canes, gardening).
goal(put_up_bean_canes, supported_beans).
needs(put_up_bean_canes, cane).
needs(put_up_bean_canes, garden_twine).
needs(put_up_bean_canes, trowel).
needs(put_up_bean_canes, bean_seedling).
needs(put_up_bean_canes, watering_can).
needs(put_up_bean_canes, water).
takes_time(put_up_bean_canes, 25, 35, minute).
step(put_up_bean_canes, put_up_bean_canes_s1).
action(put_up_bean_canes_s1, push).
object_of(put_up_bean_canes_s1, cane).
into(put_up_bean_canes_s1, soil).
amount(put_up_bean_canes_s1, cane, 8, piece).
setting(put_up_bean_canes_s1, spacing, 30, centimetre).
step(put_up_bean_canes, put_up_bean_canes_s2).
action(put_up_bean_canes_s2, tie).
object_of(put_up_bean_canes_s2, cane).
with_part(put_up_bean_canes_s2, top).
uses(put_up_bean_canes_s2, garden_twine).
role(put_up_bean_canes_s2, garden_twine, fasten).
purpose(put_up_bean_canes_s2, stable_frame).
step(put_up_bean_canes, put_up_bean_canes_s3).
action(put_up_bean_canes_s3, dig).
object_of(put_up_bean_canes_s3, hole).
uses(put_up_bean_canes_s3, trowel).
role(put_up_bean_canes_s3, trowel, lift).
step(put_up_bean_canes, put_up_bean_canes_s4).
action(put_up_bean_canes_s4, plant).
object_of(put_up_bean_canes_s4, bean_seedling).
into(put_up_bean_canes_s4, hole).
takes_time(put_up_bean_canes_s4, 10, 15, minute).
step(put_up_bean_canes, put_up_bean_canes_s5).
action(put_up_bean_canes_s5, water).
object_of(put_up_bean_canes_s5, bean_seedling).
uses(put_up_bean_canes_s5, watering_can).
role(put_up_bean_canes_s5, watering_can, pour).
uses(put_up_bean_canes_s5, water).
role(put_up_bean_canes_s5, water, wet).
before(put_up_bean_canes_s1, put_up_bean_canes_s2).
before(put_up_bean_canes_s2, put_up_bean_canes_s3).
before(put_up_bean_canes_s3, put_up_bean_canes_s4).
before(put_up_bean_canes_s4, put_up_bean_canes_s5).
```

## aliases
```datalog
task_alias(put_up_bean_canes, "putting up bean canes").
task_alias(put_up_bean_canes, "make a bean wigwam").
task_alias(put_up_bean_canes, "give the beans something to climb").
alias(cane, "bamboo canes").
alias(cane, "bamboo").
alias(bean_seedling, "runner bean seedlings").
alias(bean_seedling, "bean plants").
alias(garden_twine, "twine").
alias(top, "tops").
alias(tape, "electrical tape").
alias(tape, "duct tape").
alias(stable_frame, "a steady wigwam").
```

## teach
- To put up bean canes you need eight canes, garden twine, a trowel, the bean seedlings, a watering can and water.
- Putting up bean canes takes 25 to 35 minutes.
- Step 1 of putting up bean canes: push the eight canes into the soil in a circle, about 30 centimetres apart, leaning in.
- Step 2 of putting up bean canes: tie the canes together at the top with twine so the frame is stable.
- Step 3 of putting up bean canes: dig a hole beside each cane with the trowel.
- Step 4 of putting up bean canes: plant a bean seedling in each hole, for 10 to 15 minutes.
- Step 5 of putting up bean canes: water the seedlings with the watering can.

## checklist
- names what is needed: canes, twine, a trowel, seedlings and water
- gives the steps in order: push canes in, tie tops, dig holes, plant, water
- says to space the canes about 30 centimetres apart
- gives a total time of about 25 to 35 minutes

## variant
- change: no twine: bind the cane tops together with tape
- request: Building a bean wigwam today and there's no string in the house. Could I hold the tops together with duct tape?
- situation: I have eight canes, a roll of tape, a trowel, the bean seedlings, a watering can and water, but no twine.
- expect: The reply binds the tops of the canes with tape instead of twine and keeps the other steps.

```datalog situation
lacks(user, garden_twine).
has(user, tape).
has(user, cane).
has(user, trowel).
has(user, bean_seedling).
has(user, watering_can).
has(user, water).
```

```datalog difference
no_need(garden_twine).
extra_need(tape).
swap(put_up_bean_canes_s2, garden_twine, tape).
```

## variant
- change: sixteen seedlings: sixteen canes, about twice as long
- request: I ended up with sixteen runner bean plants. How do I set up canes for all of them?
- situation: I have sixteen canes, twine, a trowel, sixteen bean seedlings, a watering can and water.
- expect: The reply uses sixteen canes, as a bigger wigwam or a double row, plants one seedling per cane, and says it takes about twice as long.

```datalog situation
has(user, cane).
has(user, garden_twine).
has(user, trowel).
has(user, bean_seedling).
has(user, watering_can).
has(user, water).
count(user, bean_seedling, 16).
```

```datalog difference
change(put_up_bean_canes_s1, amount, 8, 16).
change(put_up_bean_canes_s4, takes_time, 10_15, 20_30).
change(put_up_bean_canes, takes_time, 25_35, 45_65).
```
