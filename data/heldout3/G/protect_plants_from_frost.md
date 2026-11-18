---
id: protect_plants_from_frost
stage: household
task: protect_plants_from_frost
schema: v3
tier: G
---
# Protect plants from frost

## requests
- They're forecasting a hard frost tonight and my new plants are out. What should I do?
- How do I protect tender plants from frost with fleece?
- Is there a way to stop a late frost killing my potted geraniums?

## situation
- I have garden fleece, a few bricks, a watering can and water, and my plants are in pots on the patio.

## ground truth
```datalog
task(protect_plants_from_frost).
domain(protect_plants_from_frost, gardening).
goal(protect_plants_from_frost, frost_free_plant).
needs(protect_plants_from_frost, watering_can).
needs(protect_plants_from_frost, water).
needs(protect_plants_from_frost, fleece).
needs(protect_plants_from_frost, brick).
takes_time(protect_plants_from_frost, 15, 20, minute).
step(protect_plants_from_frost, protect_plants_from_frost_s1).
action(protect_plants_from_frost_s1, water).
object_of(protect_plants_from_frost_s1, plant).
uses(protect_plants_from_frost_s1, watering_can).
role(protect_plants_from_frost_s1, watering_can, pour).
uses(protect_plants_from_frost_s1, water).
role(protect_plants_from_frost_s1, water, wet).
condition(protect_plants_from_frost_s1, morning).
purpose(protect_plants_from_frost_s1, soil_holds_heat).
step(protect_plants_from_frost, protect_plants_from_frost_s2).
action(protect_plants_from_frost_s2, move).
object_of(protect_plants_from_frost_s2, pot).
onto(protect_plants_from_frost_s2, house_wall).
purpose(protect_plants_from_frost_s2, sheltered).
step(protect_plants_from_frost, protect_plants_from_frost_s3).
action(protect_plants_from_frost_s3, cover).
object_of(protect_plants_from_frost_s3, plant).
uses(protect_plants_from_frost_s3, fleece).
role(protect_plants_from_frost_s3, fleece, cover).
condition(protect_plants_from_frost_s3, before_dusk).
step(protect_plants_from_frost, protect_plants_from_frost_s4).
action(protect_plants_from_frost_s4, weigh_down).
object_of(protect_plants_from_frost_s4, fleece).
with_part(protect_plants_from_frost_s4, edge).
uses(protect_plants_from_frost_s4, brick).
role(protect_plants_from_frost_s4, brick, hold).
step(protect_plants_from_frost, protect_plants_from_frost_s5).
action(protect_plants_from_frost_s5, remove).
object_of(protect_plants_from_frost_s5, fleece).
condition(protect_plants_from_frost_s5, frost_thawed).
before(protect_plants_from_frost_s1, protect_plants_from_frost_s2).
before(protect_plants_from_frost_s2, protect_plants_from_frost_s3).
before(protect_plants_from_frost_s3, protect_plants_from_frost_s4).
before(protect_plants_from_frost_s4, protect_plants_from_frost_s5).
```

## aliases
```datalog
task_alias(protect_plants_from_frost, "protecting plants from frost").
task_alias(protect_plants_from_frost, "frost-proof my plants").
task_alias(protect_plants_from_frost, "stop the frost killing my plants").
alias(fleece, "garden fleece").
alias(brick, "bricks").
alias(pot, "pots").
alias(plant, "plants").
alias(plant, "geraniums").
alias(house_wall, "against the house wall").
alias(bubble_wrap, "bubble wrap").
alias(warm_roots, "keep the roots in the pots from freezing").
alias(frost_thawed, "once the frost has thawed").
```

## teach
- To protect plants from frost you need a watering can, water, garden fleece and a few bricks.
- Protecting plants from frost takes 15 to 20 minutes.
- Step 1 of protecting plants from frost: in the morning, water the plants with the watering can, because moist soil holds heat.
- Step 2 of protecting plants from frost: move the pots against the house wall where they are sheltered.
- Step 3 of protecting plants from frost: before dusk, cover the plants with the fleece.
- Step 4 of protecting plants from frost: weigh down the edges of the fleece with the bricks.
- Step 5 of protecting plants from frost: remove the fleece the next day once the frost has thawed.

## checklist
- names what is needed: water, fleece and bricks
- gives the steps in order: water in the morning, move pots, cover, weigh down, uncover
- says to take the fleece off once it has thawed
- gives a total time of about 15 to 20 minutes

## variant
- change: only 5 minutes before dark: skip watering and moving, just cover and weigh down
- request: I've just seen the frost warning and it's nearly dark - I've got five minutes at most. What's the bare minimum to save my plants?
- situation: I have fleece, bricks, a watering can and water, and about 5 minutes before dark.
- expect: The reply leaves out watering and moving the pots and just covers the plants with fleece and weighs it down, then uncovers them in the morning.

```datalog situation
has(user, fleece).
has(user, brick).
has(user, watering_can).
has(user, water).
time_limit(user, 5, minute).
```

```datalog difference
drop(protect_plants_from_frost_s1).
drop(protect_plants_from_frost_s2).
no_need(watering_can).
no_need(water).
change(protect_plants_from_frost, takes_time, 15_20, 5_5).
```

## variant
- change: wants the pot roots protected too: wrap the pots in bubble wrap
- request: Last winter the roots in my pots froze solid even with fleece over the top. How do I protect them properly this time?
- situation: I have fleece, bricks, a watering can, water and a roll of bubble wrap.
- expect: The reply adds wrapping each pot in bubble wrap after moving them, then covers with fleece as before.

```datalog situation
has(user, fleece).
has(user, brick).
has(user, watering_can).
has(user, water).
has(user, bubble_wrap).
wants(user, warm_roots).
```

```datalog difference
extra_need(bubble_wrap).
add_step(protect_plants_from_frost_s9, protect_plants_from_frost_s2).
step(protect_plants_from_frost, protect_plants_from_frost_s9).
action(protect_plants_from_frost_s9, wrap).
object_of(protect_plants_from_frost_s9, pot).
uses(protect_plants_from_frost_s9, bubble_wrap).
role(protect_plants_from_frost_s9, bubble_wrap, cover).
purpose(protect_plants_from_frost_s9, warm_roots).
before(protect_plants_from_frost_s2, protect_plants_from_frost_s9).
before(protect_plants_from_frost_s9, protect_plants_from_frost_s3).
```
