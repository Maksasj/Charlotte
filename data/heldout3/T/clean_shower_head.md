---
id: clean_shower_head
stage: household
task: clean_shower_head
schema: v3
tier: T
---
# Clean a shower head

## requests
- Half the holes in my shower head are blocked. How do I clear them?
- How do you clean limescale off a shower head without taking it down?
- The shower just dribbles now instead of spraying. What can I do about it?

## situation
- I have a plastic bag, white vinegar, a rubber band and an old toothbrush.

## ground truth
```datalog
task(clean_shower_head).
domain(clean_shower_head, cleaning).
goal(clean_shower_head, shower_head_clean).
goal(clean_shower_head, strong_spray).
needs(clean_shower_head, plastic_bag).
needs(clean_shower_head, white_vinegar).
needs(clean_shower_head, rubber_band).
needs(clean_shower_head, old_toothbrush).
takes_time(clean_shower_head, 70, 130, minute).
step(clean_shower_head, clean_shower_head_s1).
action(clean_shower_head_s1, fill).
object_of(clean_shower_head_s1, plastic_bag).
uses(clean_shower_head_s1, white_vinegar).
role(clean_shower_head_s1, white_vinegar, clean).
step(clean_shower_head, clean_shower_head_s2).
action(clean_shower_head_s2, tie).
object_of(clean_shower_head_s2, plastic_bag).
onto(clean_shower_head_s2, shower_head).
uses(clean_shower_head_s2, rubber_band).
role(clean_shower_head_s2, rubber_band, fasten).
purpose(clean_shower_head_s2, head_under_vinegar).
step(clean_shower_head, clean_shower_head_s3).
action(clean_shower_head_s3, soak).
object_of(clean_shower_head_s3, shower_head).
takes_time(clean_shower_head_s3, 1, 2, hour).
step(clean_shower_head, clean_shower_head_s4).
action(clean_shower_head_s4, remove).
object_of(clean_shower_head_s4, plastic_bag).
step(clean_shower_head, clean_shower_head_s5).
action(clean_shower_head_s5, scrub).
object_of(clean_shower_head_s5, shower_head).
with_part(clean_shower_head_s5, nozzle).
uses(clean_shower_head_s5, old_toothbrush).
role(clean_shower_head_s5, old_toothbrush, scrub).
step(clean_shower_head, clean_shower_head_s6).
action(clean_shower_head_s6, run).
object_of(clean_shower_head_s6, water).
purpose(clean_shower_head_s6, limescale_flushed).
takes_time(clean_shower_head_s6, 1, 1, minute).
check(clean_shower_head_s6, strong_spray).
before(clean_shower_head_s1, clean_shower_head_s2).
before(clean_shower_head_s2, clean_shower_head_s3).
before(clean_shower_head_s3, clean_shower_head_s4).
before(clean_shower_head_s4, clean_shower_head_s5).
before(clean_shower_head_s5, clean_shower_head_s6).
```

## aliases
```datalog
task_alias(clean_shower_head, "cleaning a shower head").
task_alias(clean_shower_head, "unblocking a shower head").
task_alias(clean_shower_head, "descaling the shower head").
alias(plastic_bag, "bag").
alias(rubber_band, "elastic band").
alias(old_toothbrush, "toothbrush").
alias(nozzle, "the holes").
alias(head_under_vinegar, "the head sits in the vinegar").
alias(strong_spray, "the spray is strong again").
```

## teach
- To clean a shower head you need a plastic bag, white vinegar, a rubber band and an old toothbrush.
- Cleaning a shower head takes 70 to 130 minutes, mostly soaking.
- Step 1 of cleaning a shower head: fill the plastic bag with white vinegar.
- Step 2 of cleaning a shower head: tie the bag over the shower head with the rubber band so the head sits in the vinegar.
- Step 3 of cleaning a shower head: let the shower head soak for 1 to 2 hours.
- Step 4 of cleaning a shower head: remove the bag.
- Step 5 of cleaning a shower head: scrub the nozzles with the old toothbrush.
- Step 6 of cleaning a shower head: run the water for 1 minute to flush out the limescale and check the spray is strong again.

## checklist
- names what is needed: a plastic bag, vinegar, a rubber band and a toothbrush
- gives the steps in order: fill bag, tie on, soak, remove, scrub nozzles, run water
- gives a soak time of about 1 to 2 hours

## variant
- change: no toothbrush: skip scrubbing and run the water longer
- request: I've got no spare toothbrush for cleaning. Is the vinegar soak alone enough for my shower head?
- situation: I have a plastic bag, white vinegar and a rubber band.
- expect: The reply soaks the head as usual, skips the scrubbing step and runs the water a bit longer to flush the loosened scale.

```datalog situation
has(user, plastic_bag).
has(user, white_vinegar).
has(user, rubber_band).
lacks(user, old_toothbrush).
```

```datalog difference
drop(clean_shower_head_s5).
no_need(old_toothbrush).
change(clean_shower_head_s6, takes_time, 1_1, 2_3).
```

## variant
- change: no rubber band: hold the bag on with a hair tie
- request: I can't find a rubber band to hold the vinegar bag on the shower head. What could I use?
- situation: I have a plastic bag, white vinegar, a hair tie and an old toothbrush.
- expect: The reply fastens the bag around the shower head with the hair tie instead of a rubber band.

```datalog situation
has(user, plastic_bag).
has(user, white_vinegar).
has(user, hair_tie).
has(user, old_toothbrush).
lacks(user, rubber_band).
```

```datalog difference
swap(clean_shower_head_s2, rubber_band, hair_tie).
extra_need(hair_tie).
no_need(rubber_band).
```
