---
id: deadhead_flowers
stage: household
task: deadhead_flowers
schema: v3
tier: G
---
# Deadhead flowers

## requests
- My petunias are covered in dead brown flowers. Should I take them off, and how?
- How do I deadhead the flowers so they keep blooming all summer?
- What's the right way to snip off faded blooms?

## situation
- I have secateurs, a bucket and a compost bin.

## ground truth
```datalog
task(deadhead_flowers).
domain(deadhead_flowers, gardening).
goal(deadhead_flowers, more_flowers).
needs(deadhead_flowers, secateurs).
needs(deadhead_flowers, bucket).
needs(deadhead_flowers, compost_bin).
takes_time(deadhead_flowers, 10, 15, minute).
step(deadhead_flowers, deadhead_flowers_s1).
action(deadhead_flowers_s1, cut).
object_of(deadhead_flowers_s1, faded_flower).
with_part(deadhead_flowers_s1, stem).
uses(deadhead_flowers_s1, secateurs).
role(deadhead_flowers_s1, secateurs, cut).
purpose(deadhead_flowers_s1, more_flowers).
check(deadhead_flowers_s1, cut_above_leaf).
takes_time(deadhead_flowers_s1, 8, 12, minute).
step(deadhead_flowers, deadhead_flowers_s2).
action(deadhead_flowers_s2, put).
object_of(deadhead_flowers_s2, faded_flower).
into(deadhead_flowers_s2, bucket).
uses(deadhead_flowers_s2, bucket).
role(deadhead_flowers_s2, bucket, contain).
overlaps(deadhead_flowers_s2, deadhead_flowers_s1).
step(deadhead_flowers, deadhead_flowers_s3).
action(deadhead_flowers_s3, empty).
object_of(deadhead_flowers_s3, bucket).
into(deadhead_flowers_s3, compost_bin).
uses(deadhead_flowers_s3, compost_bin).
role(deadhead_flowers_s3, compost_bin, contain).
before(deadhead_flowers_s1, deadhead_flowers_s3).
before(deadhead_flowers_s2, deadhead_flowers_s3).
```

## aliases
```datalog
task_alias(deadhead_flowers, "deadheading flowers").
task_alias(deadhead_flowers, "snip off faded blooms").
task_alias(deadhead_flowers, "take off the dead flowers").
alias(faded_flower, "dead flowers").
alias(faded_flower, "faded blooms").
alias(secateurs, "pruners").
alias(cut_above_leaf, "just above a leaf").
alias(scissors, "kitchen scissors").
alias(more_flowers, "keep blooming").
```

## teach
- To deadhead flowers you need secateurs, a bucket and a compost bin.
- Deadheading flowers takes 10 to 15 minutes.
- Step 1 of deadheading flowers: cut each faded flower off at its stem just above a leaf with the secateurs, so the plant makes more flowers, for 8 to 12 minutes.
- While step 1 runs, step 2 of deadheading flowers: put the faded flowers into the bucket as you go.
- Step 3 of deadheading flowers: empty the bucket into the compost bin.

## checklist
- names what is needed: secateurs, a bucket and a compost bin
- gives the steps in order: cut faded flowers, collect in bucket, compost
- says to cut just above a leaf
- gives a total time of about 10 to 15 minutes

## variant
- change: secateurs broken: snip with kitchen scissors
- request: The spring on my pruners broke. Can I do the dead petunia heads with kitchen scissors?
- situation: My secateurs are broken. I have kitchen scissors, a bucket and a compost bin.
- expect: The reply cuts the faded flowers with the scissors instead of the secateurs and keeps the other steps.

```datalog situation
lacks(user, secateurs).
has(user, scissors).
has(user, bucket).
has(user, compost_bin).
```

```datalog difference
no_need(secateurs).
extra_need(scissors).
swap(deadhead_flowers_s1, secateurs, scissors).
```

## variant
- change: no compost bin: leave out the composting step
- request: I don't compost at all. How do I deadhead my flowers then?
- situation: I have secateurs and a bucket, but no compost bin.
- expect: The reply cuts the faded flowers into the bucket and leaves out emptying into a compost bin.

```datalog situation
has(user, secateurs).
has(user, bucket).
lacks(user, compost_bin).
```

```datalog difference
no_need(compost_bin).
drop(deadhead_flowers_s3).
```
