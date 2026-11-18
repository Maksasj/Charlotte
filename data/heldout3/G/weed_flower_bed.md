---
id: weed_flower_bed
stage: household
task: weed_flower_bed
schema: v3
tier: G
---
# Weed a flower bed

## requests
- Weeds have taken over the border by my patio. How do I get rid of them properly?
- What's the right way to weed a flower bed so they don't just grow back?
- My flower bed is full of dandelions and grass. Can you walk me through clearing it?

## situation
- I have gardening gloves, a hand fork, a bucket and a compost bin.
- I've got about half an hour this afternoon.

## ground truth
```datalog
task(weed_flower_bed).
domain(weed_flower_bed, gardening).
goal(weed_flower_bed, weed_free_bed).
needs(weed_flower_bed, glove).
needs(weed_flower_bed, hand_fork).
needs(weed_flower_bed, bucket).
needs(weed_flower_bed, compost_bin).
takes_time(weed_flower_bed, 30, 40, minute).
step(weed_flower_bed, weed_flower_bed_s1).
action(weed_flower_bed_s1, put_on).
object_of(weed_flower_bed_s1, glove).
uses(weed_flower_bed_s1, glove).
role(weed_flower_bed_s1, glove, protect).
step(weed_flower_bed, weed_flower_bed_s2).
action(weed_flower_bed_s2, loosen).
object_of(weed_flower_bed_s2, soil).
uses(weed_flower_bed_s2, hand_fork).
role(weed_flower_bed_s2, hand_fork, loosen).
step(weed_flower_bed, weed_flower_bed_s3).
action(weed_flower_bed_s3, pull).
object_of(weed_flower_bed_s3, weed).
with_part(weed_flower_bed_s3, root).
purpose(weed_flower_bed_s3, stop_regrowth).
takes_time(weed_flower_bed_s3, 20, 30, minute).
step(weed_flower_bed, weed_flower_bed_s4).
action(weed_flower_bed_s4, put).
object_of(weed_flower_bed_s4, weed).
into(weed_flower_bed_s4, bucket).
uses(weed_flower_bed_s4, bucket).
role(weed_flower_bed_s4, bucket, contain).
overlaps(weed_flower_bed_s4, weed_flower_bed_s3).
step(weed_flower_bed, weed_flower_bed_s5).
action(weed_flower_bed_s5, empty).
object_of(weed_flower_bed_s5, bucket).
into(weed_flower_bed_s5, compost_bin).
uses(weed_flower_bed_s5, compost_bin).
role(weed_flower_bed_s5, compost_bin, contain).
before(weed_flower_bed_s1, weed_flower_bed_s2).
before(weed_flower_bed_s2, weed_flower_bed_s3).
before(weed_flower_bed_s3, weed_flower_bed_s5).
before(weed_flower_bed_s4, weed_flower_bed_s5).
```

## aliases
```datalog
task_alias(weed_flower_bed, "weeding a flower bed").
task_alias(weed_flower_bed, "weed the border").
task_alias(weed_flower_bed, "clear weeds from the bed").
alias(glove, "gardening gloves").
alias(hand_fork, "small garden fork").
alias(weed, "weeds").
alias(weed, "dandelions").
alias(root, "roots").
alias(fork, "old kitchen fork").
alias(fork, "dinner fork").
alias(stop_regrowth, "so they don't grow back").
```

## teach
- To weed a flower bed you need gloves, a hand fork, a bucket and a compost bin.
- Weeding a flower bed takes 30 to 40 minutes.
- Step 1 of weeding a flower bed: put on the gloves to protect your hands.
- Step 2 of weeding a flower bed: loosen the soil around the weeds with the hand fork.
- Step 3 of weeding a flower bed: pull out each weed with its whole root so it does not grow back, for 20 to 30 minutes.
- While step 3 runs, step 4 of weeding a flower bed: put the weeds into the bucket as you go.
- Step 5 of weeding a flower bed: empty the bucket into the compost bin.

## checklist
- names what is needed: gloves, a hand fork, a bucket and a compost bin
- gives the steps in order: gloves, loosen soil, pull weeds with roots, bucket, compost
- says to get the whole root out
- gives a total time of about 30 to 40 minutes

## variant
- change: no hand fork: loosen the soil with an old kitchen fork
- request: I want to weed the border but I can't find my little garden fork. I do have an old dinner fork. Would that do?
- situation: I have gloves, a bucket, a compost bin and an old kitchen fork, but no hand fork.
- expect: The reply loosens the soil with the kitchen fork instead and keeps the other steps.

```datalog situation
lacks(user, hand_fork).
has(user, fork).
has(user, glove).
has(user, bucket).
has(user, compost_bin).
```

```datalog difference
no_need(hand_fork).
extra_need(fork).
swap(weed_flower_bed_s2, hand_fork, fork).
```

## variant
- change: only 10 minutes: a quick weed of the worst patch
- request: I've only got ten minutes before I head out. Is it worth doing anything about the weeds in the bed?
- situation: I have gloves, a hand fork, a bucket and a compost bin, and about 10 minutes.
- expect: The reply keeps the steps but limits the pulling to a few minutes on the worst weeds so the whole job fits in about 10 minutes.

```datalog situation
has(user, glove).
has(user, hand_fork).
has(user, bucket).
has(user, compost_bin).
time_limit(user, 10, minute).
```

```datalog difference
change(weed_flower_bed_s3, takes_time, 20_30, 5_7).
change(weed_flower_bed, takes_time, 30_40, 10_10).
```
