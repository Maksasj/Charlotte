---
id: trim_hedge
stage: household
task: trim_hedge
schema: v3
tier: G
---
# Trim a hedge

## requests
- The privet out front is getting shaggy. How do I give it a tidy?
- What's the best order to cut a hedge so it looks neat?
- Could you explain how to trim a garden hedge with an electric trimmer?

## situation
- I have an electric hedge trimmer, a tarp, a rake and a bin bag.
- The hedge is about waist high and five metres long.

## ground truth
```datalog
task(trim_hedge).
domain(trim_hedge, gardening).
goal(trim_hedge, tidy_hedge).
needs(trim_hedge, tarp).
needs(trim_hedge, hedge_trimmer).
needs(trim_hedge, rake).
needs(trim_hedge, bin_bag).
takes_time(trim_hedge, 30, 45, minute).
step(trim_hedge, trim_hedge_s1).
action(trim_hedge_s1, spread).
object_of(trim_hedge_s1, tarp).
onto(trim_hedge_s1, ground).
purpose(trim_hedge_s1, catch_clippings).
step(trim_hedge, trim_hedge_s2).
action(trim_hedge_s2, trim).
object_of(trim_hedge_s2, hedge).
with_part(trim_hedge_s2, side).
uses(trim_hedge_s2, hedge_trimmer).
role(trim_hedge_s2, hedge_trimmer, cut).
takes_time(trim_hedge_s2, 15, 20, minute).
check(trim_hedge_s2, narrower_at_top).
step(trim_hedge, trim_hedge_s3).
action(trim_hedge_s3, trim).
object_of(trim_hedge_s3, hedge).
with_part(trim_hedge_s3, top).
uses(trim_hedge_s3, hedge_trimmer).
role(trim_hedge_s3, hedge_trimmer, cut).
takes_time(trim_hedge_s3, 10, 15, minute).
step(trim_hedge, trim_hedge_s4).
action(trim_hedge_s4, gather).
object_of(trim_hedge_s4, clipping).
uses(trim_hedge_s4, rake).
role(trim_hedge_s4, rake, gather).
step(trim_hedge, trim_hedge_s5).
action(trim_hedge_s5, put).
object_of(trim_hedge_s5, clipping).
into(trim_hedge_s5, bin_bag).
uses(trim_hedge_s5, bin_bag).
role(trim_hedge_s5, bin_bag, contain).
before(trim_hedge_s1, trim_hedge_s2).
before(trim_hedge_s2, trim_hedge_s3).
before(trim_hedge_s3, trim_hedge_s4).
before(trim_hedge_s4, trim_hedge_s5).
```

## aliases
```datalog
task_alias(trim_hedge, "trimming a hedge").
task_alias(trim_hedge, "cut a hedge").
task_alias(trim_hedge, "give the hedge a tidy").
alias(hedge, "privet").
alias(hedge_trimmer, "electric hedge trimmer").
alias(hedge_trimmer, "trimmer").
alias(tarp, "sheet").
alias(clipping, "clippings").
alias(side, "sides").
alias(narrower_at_top, "slightly narrower at the top than the bottom").
alias(shears, "hand shears").
alias(shears, "garden shears").
alias(string, "string line").
alias(cane, "canes").
alias(straight_top, "a dead straight top").
```

## teach
- To trim a hedge you need a tarp, a hedge trimmer, a rake and a bin bag.
- Trimming a hedge takes 30 to 45 minutes.
- Step 1 of trimming a hedge: spread the tarp on the ground along the hedge to catch the clippings.
- Step 2 of trimming a hedge: trim the sides of the hedge with the hedge trimmer, for 15 to 20 minutes, so it ends up slightly narrower at the top than the bottom.
- Step 3 of trimming a hedge: trim the top of the hedge with the trimmer, for 10 to 15 minutes.
- Step 4 of trimming a hedge: gather the clippings with the rake.
- Step 5 of trimming a hedge: put the clippings into the bin bag.

## checklist
- names what is needed: a tarp, a hedge trimmer, a rake and a bin bag
- gives the steps in order: tarp down, sides, top, rake clippings, bag them
- says to keep the hedge narrower at the top
- gives a total time of about 30 to 45 minutes

## variant
- change: trimmer broken: cut by hand with garden shears, which takes longer
- request: My hedge trimmer just died on me halfway through. Can I finish the hedge with hand shears?
- situation: My electric hedge trimmer is broken. I have garden shears, a tarp, a rake and a bin bag.
- expect: The reply cuts the sides and top with the garden shears instead and says it will take longer, about an hour to an hour and a half.

```datalog situation
lacks(user, hedge_trimmer).
has(user, shears).
has(user, tarp).
has(user, rake).
has(user, bin_bag).
```

```datalog difference
no_need(hedge_trimmer).
extra_need(shears).
swap(trim_hedge_s2, hedge_trimmer, shears).
swap(trim_hedge_s3, hedge_trimmer, shears).
change(trim_hedge, takes_time, 30_45, 60_90).
```

## variant
- change: wants a perfectly straight top: run a string line between two canes as a guide
- request: Last time I cut the hedge the top came out all wavy. How do I trim it and get a really straight top this time?
- situation: I have a hedge trimmer, a tarp, a rake, a bin bag, two canes and some string, and I want a straight top.
- expect: The reply adds pushing in two canes at each end and tying a string line between them at the height wanted before trimming the top, and follows the line.

```datalog situation
has(user, hedge_trimmer).
has(user, tarp).
has(user, rake).
has(user, bin_bag).
has(user, cane).
has(user, string).
wants(user, straight_top).
```

```datalog difference
extra_need(cane).
extra_need(string).
add_step(trim_hedge_s9, trim_hedge_s2).
step(trim_hedge, trim_hedge_s9).
action(trim_hedge_s9, stretch).
object_of(trim_hedge_s9, string).
uses(trim_hedge_s9, cane).
role(trim_hedge_s9, cane, hold).
uses(trim_hedge_s9, string).
role(trim_hedge_s9, string, mark).
purpose(trim_hedge_s9, straight_top).
before(trim_hedge_s2, trim_hedge_s9).
before(trim_hedge_s9, trim_hedge_s3).
```
