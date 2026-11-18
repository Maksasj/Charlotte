---
id: prune_rose_bush
stage: household
task: prune_rose_bush
schema: v3
tier: G
---
# Prune a rose bush

## requests
- My rose bush has gone all leggy and tangled. How should I cut it back?
- When winter ends, what's the proper way to prune roses?
- Can you talk me through pruning a rose bush without hurting it?

## situation
- I have secateurs, thick gardening gloves and a bin bag.
- There's one rose bush by the front door.

## ground truth
```datalog
task(prune_rose_bush).
domain(prune_rose_bush, gardening).
goal(prune_rose_bush, pruned_rose).
needs(prune_rose_bush, glove).
needs(prune_rose_bush, secateurs).
needs(prune_rose_bush, bin_bag).
takes_time(prune_rose_bush, 15, 25, minute).
step(prune_rose_bush, prune_rose_bush_s1).
action(prune_rose_bush_s1, put_on).
object_of(prune_rose_bush_s1, glove).
uses(prune_rose_bush_s1, glove).
role(prune_rose_bush_s1, glove, protect).
purpose(prune_rose_bush_s1, avoid_thorns).
step(prune_rose_bush, prune_rose_bush_s2).
action(prune_rose_bush_s2, cut).
object_of(prune_rose_bush_s2, dead_stem).
uses(prune_rose_bush_s2, secateurs).
role(prune_rose_bush_s2, secateurs, cut).
takes_time(prune_rose_bush_s2, 5, 8, minute).
step(prune_rose_bush, prune_rose_bush_s3).
action(prune_rose_bush_s3, cut).
object_of(prune_rose_bush_s3, crossing_stem).
uses(prune_rose_bush_s3, secateurs).
role(prune_rose_bush_s3, secateurs, cut).
purpose(prune_rose_bush_s3, open_centre).
step(prune_rose_bush, prune_rose_bush_s4).
action(prune_rose_bush_s4, shorten).
object_of(prune_rose_bush_s4, healthy_stem).
uses(prune_rose_bush_s4, secateurs).
role(prune_rose_bush_s4, secateurs, cut).
with_part(prune_rose_bush_s4, outward_bud).
purpose(prune_rose_bush_s4, new_growth).
takes_time(prune_rose_bush_s4, 5, 10, minute).
step(prune_rose_bush, prune_rose_bush_s5).
action(prune_rose_bush_s5, put).
object_of(prune_rose_bush_s5, cutting).
into(prune_rose_bush_s5, bin_bag).
uses(prune_rose_bush_s5, bin_bag).
role(prune_rose_bush_s5, bin_bag, contain).
before(prune_rose_bush_s1, prune_rose_bush_s2).
before(prune_rose_bush_s2, prune_rose_bush_s3).
before(prune_rose_bush_s3, prune_rose_bush_s4).
before(prune_rose_bush_s4, prune_rose_bush_s5).
```

## aliases
```datalog
task_alias(prune_rose_bush, "pruning a rose bush").
task_alias(prune_rose_bush, "prune roses").
task_alias(prune_rose_bush, "cut back a rose bush").
alias(glove, "gardening gloves").
alias(glove, "thick gloves").
alias(secateurs, "pruners").
alias(dead_stem, "dead wood").
alias(crossing_stem, "crossing branches").
alias(healthy_stem, "healthy canes").
alias(outward_bud, "outward-facing bud").
alias(cutting, "clippings").
alias(shears, "garden shears").
alias(rose_bush, "roses").
```

## teach
- To prune a rose bush you need thick gloves, secateurs and a bin bag.
- Pruning a rose bush takes 15 to 25 minutes.
- Step 1 of pruning a rose bush: put on the gloves to protect your hands from the thorns.
- Step 2 of pruning a rose bush: cut out the dead stems with the secateurs, for 5 to 8 minutes.
- Step 3 of pruning a rose bush: cut out stems that cross through the middle so the centre stays open.
- Step 4 of pruning a rose bush: shorten the healthy stems with the secateurs just above an outward-facing bud for new growth, for 5 to 10 minutes.
- Step 5 of pruning a rose bush: put the cuttings into the bin bag.

## checklist
- names what is needed: gloves, secateurs and a bin bag
- gives the steps in order: gloves on, cut dead stems, cut crossing stems, shorten healthy stems, bag the cuttings
- says to cut above an outward-facing bud
- gives a total time of about 15 to 25 minutes

## variant
- change: no secateurs: use garden shears for the cuts
- request: My pruners have gone missing. Can I still tidy up the roses with the big garden shears?
- situation: I have gloves, garden shears and a bin bag, but no secateurs.
- expect: The reply makes the same cuts with the garden shears instead of secateurs and keeps all the steps.

```datalog situation
lacks(user, secateurs).
has(user, shears).
has(user, glove).
has(user, bin_bag).
```

```datalog difference
no_need(secateurs).
extra_need(shears).
swap(prune_rose_bush_s2, secateurs, shears).
swap(prune_rose_bush_s3, secateurs, shears).
swap(prune_rose_bush_s4, secateurs, shears).
```

## variant
- change: four rose bushes: the same steps for each, about four times as long
- request: There are four rose bushes along my fence that all need doing. How long will I be out there and what do I do?
- situation: I have gloves, secateurs and a bin bag, and four rose bushes.
- expect: The reply gives the same steps for each bush and a total of about an hour to an hour and a half.

```datalog situation
has(user, glove).
has(user, secateurs).
has(user, bin_bag).
count(user, rose_bush, 4).
```

```datalog difference
change(prune_rose_bush, takes_time, 15_25, 60_100).
```
