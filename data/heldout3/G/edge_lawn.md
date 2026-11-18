---
id: edge_lawn
stage: household
task: edge_lawn
schema: v3
tier: G
---
# Edge a lawn

## requests
- The grass keeps creeping into my flower beds. How do I get a crisp edge back?
- What's the trick to cutting a neat lawn edge along the border?
- How do I edge a lawn with a half-moon edger?

## situation
- I have a half-moon edger, a long wooden plank, edging shears and a bucket.

## ground truth
```datalog
task(edge_lawn).
domain(edge_lawn, gardening).
goal(edge_lawn, neat_lawn_edge).
needs(edge_lawn, plank).
needs(edge_lawn, half_moon_edger).
needs(edge_lawn, bucket).
needs(edge_lawn, edging_shears).
takes_time(edge_lawn, 20, 30, minute).
step(edge_lawn, edge_lawn_s1).
action(edge_lawn_s1, lay).
object_of(edge_lawn_s1, plank).
onto(edge_lawn_s1, lawn).
with_part(edge_lawn_s1, edge).
step(edge_lawn, edge_lawn_s2).
action(edge_lawn_s2, cut).
object_of(edge_lawn_s2, lawn).
with_part(edge_lawn_s2, edge).
uses(edge_lawn_s2, half_moon_edger).
role(edge_lawn_s2, half_moon_edger, cut).
uses(edge_lawn_s2, plank).
role(edge_lawn_s2, plank, guide).
purpose(edge_lawn_s2, straight_line).
takes_time(edge_lawn_s2, 10, 15, minute).
step(edge_lawn, edge_lawn_s3).
action(edge_lawn_s3, lift).
object_of(edge_lawn_s3, turf_strip).
into(edge_lawn_s3, bucket).
uses(edge_lawn_s3, bucket).
role(edge_lawn_s3, bucket, contain).
step(edge_lawn, edge_lawn_s4).
action(edge_lawn_s4, trim).
object_of(edge_lawn_s4, grass).
with_part(edge_lawn_s4, edge).
uses(edge_lawn_s4, edging_shears).
role(edge_lawn_s4, edging_shears, cut).
takes_time(edge_lawn_s4, 5, 10, minute).
before(edge_lawn_s1, edge_lawn_s2).
before(edge_lawn_s2, edge_lawn_s3).
before(edge_lawn_s3, edge_lawn_s4).
```

## aliases
```datalog
task_alias(edge_lawn, "edging a lawn").
task_alias(edge_lawn, "cut a neat lawn edge").
task_alias(edge_lawn, "crisp edge on the lawn").
alias(half_moon_edger, "edger").
alias(half_moon_edger, "half-moon edging iron").
alias(plank, "wooden plank").
alias(plank, "board").
alias(turf_strip, "strips of turf").
alias(edging_shears, "long-handled edging shears").
alias(grass, "overhanging grass").
alias(hose, "garden hose").
alias(scissors, "kitchen scissors").
alias(edge, "lawn edge").
```

## teach
- To edge a lawn you need a long plank, a half-moon edger, a bucket and edging shears.
- Edging a lawn takes 20 to 30 minutes.
- Step 1 of edging a lawn: lay the plank along the edge of the lawn.
- Step 2 of edging a lawn: cut the edge with the half-moon edger, using the plank as a guide for a straight line, for 10 to 15 minutes.
- Step 3 of edging a lawn: lift out the cut strips of turf into the bucket.
- Step 4 of edging a lawn: trim the grass along the edge with the edging shears, for 5 to 10 minutes.

## checklist
- names what is needed: a plank, a half-moon edger, a bucket and edging shears
- gives the steps in order: lay plank, cut edge, lift strips, trim grass
- says the plank guides a straight line
- gives a total time of about 20 to 30 minutes

## variant
- change: no plank: lay a garden hose along the edge as the guide
- request: I've got a curvy border and no plank to cut against. Can I use the garden hose as a guide when I do the lawn edge?
- situation: I have a half-moon edger, a bucket, edging shears and a garden hose, but no plank.
- expect: The reply lays the hose along the edge and cuts against it instead of the plank, which also suits a curved edge.

```datalog situation
lacks(user, plank).
has(user, hose).
has(user, half_moon_edger).
has(user, bucket).
has(user, edging_shears).
```

```datalog difference
no_need(plank).
extra_need(hose).
change(edge_lawn_s1, object_of, plank, hose).
swap(edge_lawn_s2, plank, hose).
```

## variant
- change: no edging shears: trim the overhanging grass with scissors
- request: After cutting the edge I'm meant to trim the overhanging grass, but I don't have edging shears. Will scissors do?
- situation: I have a half-moon edger, a plank, a bucket and a pair of scissors, but no edging shears.
- expect: The reply trims the edge grass with scissors instead of edging shears and keeps the other steps.

```datalog situation
lacks(user, edging_shears).
has(user, scissors).
has(user, half_moon_edger).
has(user, plank).
has(user, bucket).
```

```datalog difference
no_need(edging_shears).
extra_need(scissors).
swap(edge_lawn_s4, edging_shears, scissors).
```
