---
id: mop_floor
stage: household
task: mop_floor
schema: v3
---
# Mop the floor

## requests
- The kitchen floor is sticky. How should I mop it properly?
- Can you explain the right way to mop a tiled floor?
- I never know how wet the mop should be. How do I mop the floor?

## situation
- I have a broom, a bucket, a mop, floor cleaner and warm water.
- I have about 40 minutes.

## ground truth
```datalog
task(mop_floor).
domain(mop_floor, cleaning).
goal(mop_floor, clean_floor).
needs(mop_floor, broom).
needs(mop_floor, bucket).
needs(mop_floor, mop).
needs(mop_floor, floor_cleaner).
needs(mop_floor, water).
takes_time(mop_floor, 25, 35, minute).
step(mop_floor, mop_floor_s1).
action(mop_floor_s1, sweep).
object_of(mop_floor_s1, floor).
uses(mop_floor_s1, broom).
role(mop_floor_s1, broom, clean).
takes_time(mop_floor_s1, 5, 10, minute).
step(mop_floor, mop_floor_s2).
action(mop_floor_s2, fill).
object_of(mop_floor_s2, bucket).
uses(mop_floor_s2, water).
role(mop_floor_s2, water, clean).
uses(mop_floor_s2, floor_cleaner).
role(mop_floor_s2, floor_cleaner, clean).
takes_time(mop_floor_s2, 2, 3, minute).
step(mop_floor, mop_floor_s3).
action(mop_floor_s3, dip).
object_of(mop_floor_s3, mop).
uses(mop_floor_s3, bucket).
role(mop_floor_s3, bucket, contain).
into(mop_floor_s3, bucket).
step(mop_floor, mop_floor_s4).
action(mop_floor_s4, wring).
object_of(mop_floor_s4, mop).
until(mop_floor_s4, damp).
check(mop_floor_s4, not_dripping).
takes_time(mop_floor_s4, 1, 1, minute).
step(mop_floor, mop_floor_s5).
action(mop_floor_s5, mop).
object_of(mop_floor_s5, floor).
uses(mop_floor_s5, mop).
role(mop_floor_s5, mop, clean).
from(mop_floor_s5, far_corner).
takes_time(mop_floor_s5, 10, 15, minute).
step(mop_floor, mop_floor_s6).
action(mop_floor_s6, dry).
object_of(mop_floor_s6, floor).
avoid(mop_floor_s6, walk, floor).
takes_time(mop_floor_s6, 5, 10, minute).
before(mop_floor_s1, mop_floor_s2).
before(mop_floor_s2, mop_floor_s3).
before(mop_floor_s3, mop_floor_s4).
before(mop_floor_s4, mop_floor_s5).
before(mop_floor_s5, mop_floor_s6).
```

## aliases
```datalog
task_alias(mop_floor, "mopping the floor").
task_alias(mop_floor, "mop a tiled floor").
task_alias(mop_floor, "mop it").
alias(floor, "kitchen floor").
alias(floor, "tiled floor").
alias(floor, "wooden floor").
alias(water, "warm water").
alias(floor_cleaner, "cleaner").
alias(wood_floor_cleaner, "wood floor cleaner").
alias(towel, "old towel").
alias(barely_damp, "barely damp").
alias(not_dripping, "not dripping").
alias(far_corner, "the far corner").
alias(section, "small sections").
alias(dry, "let the floor dry").
```

## teach
- To mop the floor you need a broom, a bucket, a mop, floor cleaner and warm water.
- Mopping the floor takes 25 to 35 minutes.
- Step 1 of mopping the floor: sweep the floor with the broom for 5 to 10 minutes.
- Step 2 of mopping the floor: fill the bucket with warm water and a little floor cleaner for 2 to 3 minutes.
- Step 3 of mopping the floor: dip the mop in the bucket.
- Step 4 of mopping the floor: wring the mop until it is damp, not dripping, for 1 minute.
- Step 5 of mopping the floor: mop the floor from the far corner toward the door for 10 to 15 minutes.
- Step 6 of mopping the floor: let the floor dry before you walk on it for 5 to 10 minutes.

## checklist
- names what is needed: a broom, a bucket, a mop, floor cleaner and warm water
- gives the steps in order: sweep floor, fill bucket, wring mop, mop floor, dry floor
- gives a total time of about 25 to 35 minutes
- closes politely

## variant
- change: wooden floor: use a barely damp mop and dry each section straight away with a towel
- request: My living room has a wooden floor. Is mopping it any different?
- situation: I have a broom, a bucket, a mop, wood floor cleaner, warm water and an old towel.
- expect: The reply wrings the mop until barely damp, mops in small sections and dries each one at once with the towel, keeping the sweep and fill steps.

```datalog situation
has(user, broom).
has(user, bucket).
has(user, mop).
has(user, wood_floor_cleaner).
has(user, water).
has(user, towel).
```

```datalog difference
swap(mop_floor_s2, floor_cleaner, wood_floor_cleaner).
no_need(floor_cleaner).
extra_need(wood_floor_cleaner).
change(mop_floor_s4, until, damp, barely_damp).
change(mop_floor_s5, object_of, floor, section).
extra_need(towel).
add_step(mop_floor_s9, mop_floor_s5).
step(mop_floor, mop_floor_s9).
action(mop_floor_s9, dry).
object_of(mop_floor_s9, section).
uses(mop_floor_s9, towel).
role(mop_floor_s9, towel, dry).
overlaps(mop_floor_s9, mop_floor_s5).
```
