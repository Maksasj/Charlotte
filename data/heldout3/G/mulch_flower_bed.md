---
id: mulch_flower_bed
stage: household
task: mulch_flower_bed
schema: v3
tier: G
---
# Mulch a flower bed

## requests
- I keep reading that I should put bark down on my borders. How do I do it?
- How do I mulch a flower bed so it holds water and keeps weeds down?
- What's the proper way to spread bark chips around my shrubs?

## situation
- I have bags of bark mulch, a wheelbarrow, a rake, a watering can and water.
- I've got most of the afternoon free.

## ground truth
```datalog
task(mulch_flower_bed).
domain(mulch_flower_bed, gardening).
goal(mulch_flower_bed, mulched_bed).
needs(mulch_flower_bed, watering_can).
needs(mulch_flower_bed, water).
needs(mulch_flower_bed, wheelbarrow).
needs(mulch_flower_bed, bark_mulch).
needs(mulch_flower_bed, rake).
takes_time(mulch_flower_bed, 40, 60, minute).
step(mulch_flower_bed, mulch_flower_bed_s1).
action(mulch_flower_bed_s1, pull).
object_of(mulch_flower_bed_s1, weed).
takes_time(mulch_flower_bed_s1, 10, 15, minute).
step(mulch_flower_bed, mulch_flower_bed_s2).
action(mulch_flower_bed_s2, water).
object_of(mulch_flower_bed_s2, flower_bed).
uses(mulch_flower_bed_s2, watering_can).
role(mulch_flower_bed_s2, watering_can, pour).
uses(mulch_flower_bed_s2, water).
role(mulch_flower_bed_s2, water, wet).
purpose(mulch_flower_bed_s2, moisture_locked_in).
takes_time(mulch_flower_bed_s2, 5, 10, minute).
step(mulch_flower_bed, mulch_flower_bed_s3).
action(mulch_flower_bed_s3, carry).
object_of(mulch_flower_bed_s3, bark_mulch).
uses(mulch_flower_bed_s3, wheelbarrow).
role(mulch_flower_bed_s3, wheelbarrow, contain).
step(mulch_flower_bed, mulch_flower_bed_s4).
action(mulch_flower_bed_s4, spread).
object_of(mulch_flower_bed_s4, bark_mulch).
onto(mulch_flower_bed_s4, flower_bed).
uses(mulch_flower_bed_s4, rake).
role(mulch_flower_bed_s4, rake, spread).
setting(mulch_flower_bed_s4, depth, 5, centimetre).
avoid(mulch_flower_bed_s4, pile, stem).
warning(mulch_flower_bed_s4, stem_rot).
takes_time(mulch_flower_bed_s4, 15, 25, minute).
before(mulch_flower_bed_s1, mulch_flower_bed_s2).
before(mulch_flower_bed_s2, mulch_flower_bed_s3).
before(mulch_flower_bed_s3, mulch_flower_bed_s4).
```

## aliases
```datalog
task_alias(mulch_flower_bed, "mulching a flower bed").
task_alias(mulch_flower_bed, "put bark down on the border").
task_alias(mulch_flower_bed, "spread bark chips").
alias(bark_mulch, "bark").
alias(bark_mulch, "bark chips").
alias(flower_bed, "border").
alias(flower_bed, "bed").
alias(weed, "weeds").
alias(stem, "stems").
alias(laundry_basket, "plastic laundry basket").
alias(moisture_locked_in, "locks the moisture in").
```

## teach
- To mulch a flower bed you need a watering can, water, a wheelbarrow, bark mulch and a rake.
- Mulching a flower bed takes 40 to 60 minutes.
- Step 1 of mulching a flower bed: pull out the weeds, for 10 to 15 minutes.
- Step 2 of mulching a flower bed: water the bed well with the watering can so the mulch locks the moisture in, for 5 to 10 minutes.
- Step 3 of mulching a flower bed: carry the bark mulch to the bed in the wheelbarrow.
- Step 4 of mulching a flower bed: spread the mulch over the bed with the rake about 5 centimetres deep, for 15 to 25 minutes, and do not pile it against the stems or they can rot.

## checklist
- names what is needed: bark mulch, a wheelbarrow, a rake and water
- gives the steps in order: weed, water, carry bark, spread
- says about 5 centimetres deep and to keep it off the stems
- gives a total time of about 40 to 60 minutes

## variant
- change: no wheelbarrow: carry the bark in a laundry basket
- request: I want to get bark down on the border but I don't own a wheelbarrow. I've got a big plastic laundry basket though.
- situation: I have bark mulch, a rake, a watering can, water and a plastic laundry basket, but no wheelbarrow.
- expect: The reply carries the bark to the bed in the laundry basket instead of a wheelbarrow and keeps the other steps.

```datalog situation
lacks(user, wheelbarrow).
has(user, laundry_basket).
has(user, bark_mulch).
has(user, rake).
has(user, watering_can).
has(user, water).
```

```datalog difference
no_need(wheelbarrow).
extra_need(laundry_basket).
swap(mulch_flower_bed_s3, wheelbarrow, laundry_basket).
```

## variant
- change: only 25 minutes: skip watering and do a quick weed
- request: I've got about 25 minutes of daylight left. Can I still get the bark spread on the bed?
- situation: I have bark mulch, a wheelbarrow, a rake, a watering can and water, and about 25 minutes. The soil is already moist from rain.
- expect: The reply leaves out the watering step, keeps the weeding short, and spreads the mulch so the job fits in about 25 minutes.

```datalog situation
has(user, bark_mulch).
has(user, wheelbarrow).
has(user, rake).
has(user, watering_can).
has(user, water).
time_limit(user, 25, minute).
```

```datalog difference
drop(mulch_flower_bed_s2).
no_need(watering_can).
no_need(water).
change(mulch_flower_bed_s1, takes_time, 10_15, 5_5).
change(mulch_flower_bed, takes_time, 40_60, 25_25).
```
