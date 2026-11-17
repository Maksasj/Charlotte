---
id: mow_lawn
stage: household
task: mow_lawn
---
# Mow the lawn

## requests
- The grass is getting long. How do I mow the lawn properly?
- Can you walk me through mowing the back garden?
- I've never mowed a lawn before. What are the steps?

## situation
- I have a lawn mower, garden shears and a rake.
- I have about an hour on Saturday morning.

## ground truth
```datalog
task(mow_lawn).
needs(mow_lawn, lawn_mower).
needs(mow_lawn, shears).
needs(mow_lawn, rake).
takes_time(mow_lawn, 50, 70, minute).
step(mow_lawn, mow_lawn_s1).
action(mow_lawn_s1, clear).
object_of(mow_lawn_s1, lawn).
takes_time(mow_lawn_s1, 5, 10, minute).
step(mow_lawn, mow_lawn_s2).
action(mow_lawn_s2, set).
object_of(mow_lawn_s2, blade).
uses(mow_lawn_s2, lawn_mower).
takes_time(mow_lawn_s2, 1, 2, minute).
step(mow_lawn, mow_lawn_s3).
action(mow_lawn_s3, mow).
object_of(mow_lawn_s3, lawn).
uses(mow_lawn_s3, lawn_mower).
takes_time(mow_lawn_s3, 30, 40, minute).
step(mow_lawn, mow_lawn_s4).
action(mow_lawn_s4, trim).
object_of(mow_lawn_s4, edge).
uses(mow_lawn_s4, shears).
takes_time(mow_lawn_s4, 5, 10, minute).
step(mow_lawn, mow_lawn_s5).
action(mow_lawn_s5, rake).
object_of(mow_lawn_s5, clipping).
uses(mow_lawn_s5, rake).
takes_time(mow_lawn_s5, 5, 10, minute).
before(mow_lawn_s1, mow_lawn_s2).
before(mow_lawn_s2, mow_lawn_s3).
before(mow_lawn_s3, mow_lawn_s4).
before(mow_lawn_s4, mow_lawn_s5).
```

## teach
- To mow the lawn you need a lawn mower, garden shears and a rake.
- Mowing the lawn takes 50 to 70 minutes.
- Step 1 of mowing the lawn: clear the lawn of stones, sticks and toys, for 5 to 10 minutes.
- Step 2 of mowing the lawn: set the blade of the lawn mower to a medium height for 1 to 2 minutes.
- Step 3 of mowing the lawn: mow the lawn with the lawn mower in straight, slightly overlapping rows, for 30 to 40 minutes.
- Step 4 of mowing the lawn: trim the edges with the shears for 5 to 10 minutes.
- Step 5 of mowing the lawn: rake up the loose clippings for 5 to 10 minutes.

## checklist
- names what is needed: a lawn mower, garden shears and a rake
- gives the steps in order: clear lawn, set blade, mow lawn, trim edge, rake clipping
- gives a total time of about 50 to 70 minutes
- closes politely

## variant
- change: very long grass: mow twice, first with the blade on its highest setting
- request: I've been away and the grass is nearly knee high. How do I mow it without wrecking the mower?
- situation: I have a lawn mower, garden shears and a rake, and the grass is very long.
- expect: The reply mows once on the highest blade setting and then again at medium height, so the total grows to about 80 to 100 minutes, and keeps the clear, trim and rake steps.
