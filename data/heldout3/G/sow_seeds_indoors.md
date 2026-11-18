---
id: sow_seeds_indoors
stage: household
task: sow_seeds_indoors
schema: v3
tier: G
---
# Sow seeds indoors

## requests
- I want to get my seeds started on the windowsill before spring. How do I go about it?
- What's the right way to start seeds off in a tray indoors?
- I've got a packet of basil seeds and a tray. Where do I begin?

## situation
- I have a seed tray, seed compost, a packet of seeds, a watering can, water and a clear plastic lid.

## ground truth
```datalog
task(sow_seeds_indoors).
domain(sow_seeds_indoors, gardening).
goal(sow_seeds_indoors, sown_seeds).
needs(sow_seeds_indoors, seed_tray).
needs(sow_seeds_indoors, seed_compost).
needs(sow_seeds_indoors, seed).
needs(sow_seeds_indoors, watering_can).
needs(sow_seeds_indoors, water).
optional_need(sow_seeds_indoors, clear_lid).
takes_time(sow_seeds_indoors, 15, 20, minute).
step(sow_seeds_indoors, sow_seeds_indoors_s1).
action(sow_seeds_indoors_s1, put).
object_of(sow_seeds_indoors_s1, seed_compost).
into(sow_seeds_indoors_s1, seed_tray).
uses(sow_seeds_indoors_s1, seed_tray).
role(sow_seeds_indoors_s1, seed_tray, contain).
takes_time(sow_seeds_indoors_s1, 3, 5, minute).
step(sow_seeds_indoors, sow_seeds_indoors_s2).
action(sow_seeds_indoors_s2, press).
object_of(sow_seeds_indoors_s2, seed_compost).
purpose(sow_seeds_indoors_s2, level_surface).
step(sow_seeds_indoors, sow_seeds_indoors_s3).
action(sow_seeds_indoors_s3, sow).
object_of(sow_seeds_indoors_s3, seed).
onto(sow_seeds_indoors_s3, seed_compost).
amount(sow_seeds_indoors_s3, seed, 2, piece).
step(sow_seeds_indoors, sow_seeds_indoors_s4).
action(sow_seeds_indoors_s4, cover).
object_of(sow_seeds_indoors_s4, seed).
uses(sow_seeds_indoors_s4, seed_compost).
role(sow_seeds_indoors_s4, seed_compost, cover).
setting(sow_seeds_indoors_s4, depth, 1, centimetre).
step(sow_seeds_indoors, sow_seeds_indoors_s5).
action(sow_seeds_indoors_s5, water).
object_of(sow_seeds_indoors_s5, seed).
uses(sow_seeds_indoors_s5, watering_can).
role(sow_seeds_indoors_s5, watering_can, pour).
uses(sow_seeds_indoors_s5, water).
role(sow_seeds_indoors_s5, water, wet).
avoid(sow_seeds_indoors_s5, flood, seed_compost).
step(sow_seeds_indoors, sow_seeds_indoors_s6).
action(sow_seeds_indoors_s6, cover).
object_of(sow_seeds_indoors_s6, seed_tray).
uses(sow_seeds_indoors_s6, clear_lid).
role(sow_seeds_indoors_s6, clear_lid, cover).
purpose(sow_seeds_indoors_s6, keeps_moist).
optional(sow_seeds_indoors_s6).
before(sow_seeds_indoors_s1, sow_seeds_indoors_s2).
before(sow_seeds_indoors_s2, sow_seeds_indoors_s3).
before(sow_seeds_indoors_s3, sow_seeds_indoors_s4).
before(sow_seeds_indoors_s4, sow_seeds_indoors_s5).
before(sow_seeds_indoors_s5, sow_seeds_indoors_s6).
```

## aliases
```datalog
task_alias(sow_seeds_indoors, "sowing seeds indoors").
task_alias(sow_seeds_indoors, "starting seeds indoors").
task_alias(sow_seeds_indoors, "start seeds off in a tray").
alias(seed, "seeds").
alias(seed, "basil seeds").
alias(seed_tray, "tray").
alias(seed_compost, "compost").
alias(clear_lid, "clear plastic lid").
alias(clear_lid, "propagator lid").
alias(keeps_moist, "keeps the moisture in").
alias(egg_carton, "egg box").
alias(egg_carton, "cardboard egg carton").
```

## teach
- To sow seeds indoors you need a seed tray, seed compost, seeds, a watering can and water, and a clear lid helps but is not required.
- Sowing seeds indoors takes 15 to 20 minutes.
- Step 1 of sowing seeds indoors: put the seed compost into the seed tray, for 3 to 5 minutes.
- Step 2 of sowing seeds indoors: press the compost down gently so the surface is level.
- Step 3 of sowing seeds indoors: sow two seeds onto the compost in each spot.
- Step 4 of sowing seeds indoors: cover the seeds with about 1 centimetre of compost.
- Step 5 of sowing seeds indoors: water the seeds gently with the watering can and do not flood the compost.
- Step 6 of sowing seeds indoors: if you have one, cover the tray with a clear lid to keep the moisture in.

## checklist
- names what is needed: a seed tray, seed compost, seeds, a watering can and water
- gives the steps in order: fill tray, press compost, sow, cover with compost, water, cover tray
- says the lid is optional and keeps moisture in
- gives a total time of about 15 to 20 minutes

## variant
- change: no seed tray: use an egg carton as the container
- request: I don't have a proper seed tray, but I've got an empty egg box. Can I start my seeds in that?
- situation: I have seed compost, seeds, a watering can, water, a clear lid and an empty egg carton, but no seed tray.
- expect: The reply fills the egg carton cups with compost instead of a seed tray and otherwise keeps the steps.

```datalog situation
lacks(user, seed_tray).
has(user, egg_carton).
has(user, seed_compost).
has(user, seed).
has(user, watering_can).
has(user, water).
has(user, clear_lid).
```

```datalog difference
no_need(seed_tray).
extra_need(egg_carton).
swap(sow_seeds_indoors_s1, seed_tray, egg_carton).
change(sow_seeds_indoors_s6, object_of, seed_tray, egg_carton).
```

## variant
- change: no clear lid: skip covering the tray
- request: How do I sow seeds indoors if I've got nothing to put over the tray?
- situation: I have a seed tray, seed compost, seeds, a watering can and water, but no lid or cover.
- expect: The reply leaves out the covering step and keeps the rest of the steps.

```datalog situation
has(user, seed_tray).
has(user, seed_compost).
has(user, seed).
has(user, watering_can).
has(user, water).
lacks(user, clear_lid).
```

```datalog difference
no_need(clear_lid).
drop(sow_seeds_indoors_s6).
```
