---
id: lay_turf
stage: household
task: lay_turf
schema: v3
tier: G
---
# Lay turf

## requests
- I want an instant lawn instead of waiting for seed. How do I put down rolls of turf?
- The turf I ordered is arriving tomorrow. What's the right way to lay it?
- How do you lay new grass rolls so the joins don't show?

## situation
- I have twenty rolls of turf, a rake, an old knife, a wooden plank and a garden hose.
- The ground has already been dug over.

## ground truth
```datalog
task(lay_turf).
domain(lay_turf, gardening).
goal(lay_turf, new_lawn).
needs(lay_turf, rake).
needs(lay_turf, turf_roll).
needs(lay_turf, knife).
needs(lay_turf, plank).
needs(lay_turf, hose).
takes_time(lay_turf, 120, 180, minute).
step(lay_turf, lay_turf_s1).
action(lay_turf_s1, rake).
object_of(lay_turf_s1, soil).
uses(lay_turf_s1, rake).
role(lay_turf_s1, rake, smooth).
achieves(lay_turf_s1, level).
takes_time(lay_turf_s1, 20, 30, minute).
step(lay_turf, lay_turf_s2).
action(lay_turf_s2, tread).
object_of(lay_turf_s2, soil).
purpose(lay_turf_s2, firm_base).
step(lay_turf, lay_turf_s3).
action(lay_turf_s3, lay).
object_of(lay_turf_s3, turf_roll).
onto(lay_turf_s3, soil).
check(lay_turf_s3, joins_staggered).
takes_time(lay_turf_s3, 60, 90, minute).
step(lay_turf, lay_turf_s4).
action(lay_turf_s4, cut).
object_of(lay_turf_s4, turf_roll).
with_part(lay_turf_s4, edge).
uses(lay_turf_s4, knife).
role(lay_turf_s4, knife, cut).
step(lay_turf, lay_turf_s5).
action(lay_turf_s5, press).
object_of(lay_turf_s5, turf_roll).
uses(lay_turf_s5, plank).
role(lay_turf_s5, plank, press).
purpose(lay_turf_s5, roots_touch_soil).
step(lay_turf, lay_turf_s6).
action(lay_turf_s6, water).
object_of(lay_turf_s6, turf_roll).
uses(lay_turf_s6, hose).
role(lay_turf_s6, hose, pour).
until(lay_turf_s6, soaked).
before(lay_turf_s1, lay_turf_s2).
before(lay_turf_s2, lay_turf_s3).
before(lay_turf_s3, lay_turf_s4).
before(lay_turf_s4, lay_turf_s5).
before(lay_turf_s5, lay_turf_s6).
```

## aliases
```datalog
task_alias(lay_turf, "laying turf").
task_alias(lay_turf, "put down rolls of turf").
task_alias(lay_turf, "instant lawn").
alias(turf_roll, "rolls of turf").
alias(turf_roll, "turf").
alias(turf_roll, "grass rolls").
alias(knife, "old knife").
alias(plank, "wooden plank").
alias(hose, "garden hose").
alias(joins_staggered, "joins staggered like brickwork").
alias(utility_knife, "craft knife").
alias(utility_knife, "Stanley knife").
alias(soaked, "soaked through").
```

## teach
- To lay turf you need a rake, the turf rolls, an old knife, a plank and a hose.
- Laying twenty rolls of turf takes 2 to 3 hours.
- Step 1 of laying turf: rake the soil smooth until it is level, for 20 to 30 minutes.
- Step 2 of laying turf: tread the soil all over to make a firm base.
- Step 3 of laying turf: lay the turf rolls on the soil with the joins staggered like brickwork, for 60 to 90 minutes.
- Step 4 of laying turf: cut the turf to fit at the edges with the knife.
- Step 5 of laying turf: press the turf down with the plank so the roots touch the soil.
- Step 6 of laying turf: water the turf with the hose until it is soaked through.

## checklist
- names what is needed: a rake, turf, a knife, a plank and a hose
- gives the steps in order: rake level, tread firm, lay staggered, trim edges, press, water
- says to stagger the joins
- gives a total time of about 2 to 3 hours

## variant
- change: forty rolls: the same steps, about twice as long
- request: We've got a bigger garden than I thought - forty rolls of turf turned up. How long is this going to take, and how do I do it?
- situation: I have forty rolls of turf, a rake, an old knife, a plank and a hose.
- expect: The reply gives the same steps and says laying forty rolls takes about twice as long, around 4 to 6 hours.

```datalog situation
has(user, turf_roll).
has(user, rake).
has(user, knife).
has(user, plank).
has(user, hose).
count(user, turf_roll, 40).
```

```datalog difference
change(lay_turf_s3, takes_time, 60_90, 120_180).
change(lay_turf, takes_time, 120_180, 240_360).
```

## variant
- change: no old knife: trim the edges with a craft knife
- request: I don't want to ruin a kitchen knife cutting turf. I have a Stanley knife - is that OK for the edges?
- situation: I have the turf, a rake, a plank, a hose and a craft knife, but no old knife to spare.
- expect: The reply trims the turf edges with the craft knife instead and keeps the other steps.

```datalog situation
lacks(user, knife).
has(user, utility_knife).
has(user, turf_roll).
has(user, rake).
has(user, plank).
has(user, hose).
```

```datalog difference
no_need(knife).
extra_need(utility_knife).
swap(lay_turf_s4, knife, utility_knife).
```
