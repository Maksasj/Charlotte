---
id: mop_floor
stage: household
task: mop_floor
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
takes_time(mop_floor_s1, 5, 10, minute).
step(mop_floor, mop_floor_s2).
action(mop_floor_s2, fill).
object_of(mop_floor_s2, bucket).
uses(mop_floor_s2, water).
uses(mop_floor_s2, floor_cleaner).
takes_time(mop_floor_s2, 2, 3, minute).
step(mop_floor, mop_floor_s3).
action(mop_floor_s3, wring).
object_of(mop_floor_s3, mop).
uses(mop_floor_s3, bucket).
until(mop_floor_s3, damp).
takes_time(mop_floor_s3, 1, 1, minute).
step(mop_floor, mop_floor_s4).
action(mop_floor_s4, mop).
object_of(mop_floor_s4, floor).
uses(mop_floor_s4, mop).
takes_time(mop_floor_s4, 10, 15, minute).
step(mop_floor, mop_floor_s5).
action(mop_floor_s5, dry).
object_of(mop_floor_s5, floor).
takes_time(mop_floor_s5, 5, 10, minute).
before(mop_floor_s1, mop_floor_s2).
before(mop_floor_s2, mop_floor_s3).
before(mop_floor_s3, mop_floor_s4).
before(mop_floor_s4, mop_floor_s5).
```

## teach
- To mop the floor you need a broom, a bucket, a mop, floor cleaner and warm water.
- Mopping the floor takes 25 to 35 minutes.
- Step 1 of mopping the floor: sweep the floor with the broom for 5 to 10 minutes.
- Step 2 of mopping the floor: fill the bucket with warm water and a little floor cleaner for 2 to 3 minutes.
- Step 3 of mopping the floor: dip the mop in the bucket and wring it until it is damp, not dripping, for 1 minute.
- Step 4 of mopping the floor: mop the floor from the far corner toward the door for 10 to 15 minutes.
- Step 5 of mopping the floor: let the floor dry before you walk on it for 5 to 10 minutes.

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
