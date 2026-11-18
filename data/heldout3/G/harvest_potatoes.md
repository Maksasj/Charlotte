---
id: harvest_potatoes
stage: household
task: harvest_potatoes
schema: v3
tier: G
---
# Harvest potatoes

## requests
- The leaves on my potato plants have gone yellow and flopped. Is it time to dig them up, and how?
- How do I lift my potatoes without stabbing them all?
- What's the right way to harvest and store spuds from the veg bed?

## situation
- I have shears, a garden fork, a bucket and a paper sack.
- It's a dry, sunny day.

## ground truth
```datalog
task(harvest_potatoes).
domain(harvest_potatoes, gardening).
goal(harvest_potatoes, stored_potatoes).
needs(harvest_potatoes, shears).
needs(harvest_potatoes, garden_fork).
needs(harvest_potatoes, bucket).
needs(harvest_potatoes, paper_sack).
takes_time(harvest_potatoes, 150, 270, minute).
step(harvest_potatoes, harvest_potatoes_s1).
action(harvest_potatoes_s1, cut).
object_of(harvest_potatoes_s1, potato_stem).
uses(harvest_potatoes_s1, shears).
role(harvest_potatoes_s1, shears, cut).
step(harvest_potatoes, harvest_potatoes_s2).
action(harvest_potatoes_s2, lift).
object_of(harvest_potatoes_s2, potato_plant).
uses(harvest_potatoes_s2, garden_fork).
role(harvest_potatoes_s2, garden_fork, lift).
avoid(harvest_potatoes_s2, pierce, potato).
takes_time(harvest_potatoes_s2, 15, 20, minute).
step(harvest_potatoes, harvest_potatoes_s3).
action(harvest_potatoes_s3, gather).
object_of(harvest_potatoes_s3, potato).
into(harvest_potatoes_s3, bucket).
uses(harvest_potatoes_s3, bucket).
role(harvest_potatoes_s3, bucket, contain).
step(harvest_potatoes, harvest_potatoes_s4).
action(harvest_potatoes_s4, dry).
object_of(harvest_potatoes_s4, potato).
onto(harvest_potatoes_s4, ground).
takes_time(harvest_potatoes_s4, 2, 4, hour).
purpose(harvest_potatoes_s4, skin_sets).
step(harvest_potatoes, harvest_potatoes_s5).
action(harvest_potatoes_s5, store).
object_of(harvest_potatoes_s5, potato).
into(harvest_potatoes_s5, paper_sack).
uses(harvest_potatoes_s5, paper_sack).
role(harvest_potatoes_s5, paper_sack, contain).
condition(harvest_potatoes_s5, cool_dark_place).
before(harvest_potatoes_s1, harvest_potatoes_s2).
before(harvest_potatoes_s2, harvest_potatoes_s3).
before(harvest_potatoes_s3, harvest_potatoes_s4).
before(harvest_potatoes_s4, harvest_potatoes_s5).
```

## aliases
```datalog
task_alias(harvest_potatoes, "harvesting potatoes").
task_alias(harvest_potatoes, "dig up the potatoes").
task_alias(harvest_potatoes, "lift my spuds").
alias(potato, "potatoes").
alias(potato, "spuds").
alias(potato_stem, "stems").
alias(potato_stem, "haulm").
alias(potato_plant, "potato plants").
alias(garden_fork, "fork").
alias(paper_sack, "paper sack").
alias(skin_sets, "the skins firm up").
alias(cool_dark_place, "somewhere cool and dark").
alias(trowel, "hand trowel").
```

## teach
- To harvest potatoes you need shears, a garden fork, a bucket and a paper sack.
- Harvesting potatoes takes 2 and a half to 4 and a half hours, most of it leaving them to dry.
- Step 1 of harvesting potatoes: cut the potato stems down with the shears.
- Step 2 of harvesting potatoes: lift each potato plant with the garden fork, digging well out from the stem so you don't pierce the potatoes, for 15 to 20 minutes.
- Step 3 of harvesting potatoes: gather the potatoes into the bucket.
- Step 4 of harvesting potatoes: dry the potatoes on the ground for 2 to 4 hours so the skins set.
- Step 5 of harvesting potatoes: store the potatoes in the paper sack somewhere cool and dark.

## checklist
- names what is needed: shears, a garden fork, a bucket and a paper sack
- gives the steps in order: cut stems, lift with fork, gather, dry, store
- says to dig wide so the fork doesn't pierce the potatoes
- says to dry them for a few hours before storing

## variant
- change: only 20 minutes and eating them tonight: skip drying and storing
- request: I just want enough potatoes for tonight's dinner and I've got about 20 minutes. What do I do?
- situation: I have shears, a garden fork and a bucket, and about 20 minutes. I'll cook the potatoes today.
- expect: The reply cuts, lifts and gathers the potatoes and leaves out drying and storing, since they are eaten the same day.

```datalog situation
has(user, shears).
has(user, garden_fork).
has(user, bucket).
time_limit(user, 20, minute).
```

```datalog difference
drop(harvest_potatoes_s4).
drop(harvest_potatoes_s5).
no_need(paper_sack).
change(harvest_potatoes, takes_time, 150_270, 20_25).
```

## variant
- change: garden fork broken: dig the potatoes out with a hand trowel
- request: My garden fork bent in half on a stone. Can I still get the potatoes out? I only have a little trowel.
- situation: My garden fork is broken. I have shears, a hand trowel, a bucket and a paper sack.
- expect: The reply digs the potatoes out with the trowel instead of the fork, taking care not to cut them, and keeps the other steps.

```datalog situation
lacks(user, garden_fork).
has(user, trowel).
has(user, shears).
has(user, bucket).
has(user, paper_sack).
```

```datalog difference
no_need(garden_fork).
extra_need(trowel).
swap(harvest_potatoes_s2, garden_fork, trowel).
change(harvest_potatoes_s2, takes_time, 15_20, 25_35).
```
