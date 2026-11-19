---
id: clean_baseboards
stage: procedures
task: clean_baseboards
schema: v3
---
# Clean baseboards

## requests
- How do I clean the skirting boards in my hallway properly?
- What's the best way to get dust and grime off the baseboards along my walls?
- Can you give me the steps for washing baseboards with soapy water?

## situation
- I have a vacuum cleaner, a bucket, dish soap, a sponge and a dry cloth.
- I have about half an hour free.

## ground truth
```datalog
task(clean_baseboards).
domain(clean_baseboards, cleaning).
goal(clean_baseboards, clean_room).
needs(clean_baseboards, vacuum_cleaner).
needs(clean_baseboards, bucket).
needs(clean_baseboards, dish_soap).
needs(clean_baseboards, sponge).
needs(clean_baseboards, dry_cloth).
takes_time(clean_baseboards, 20, 30, minute).
step(clean_baseboards, clean_baseboards_s1).
action(clean_baseboards_s1, vacuum).
object_of(clean_baseboards_s1, baseboard).
uses(clean_baseboards_s1, vacuum_cleaner).
role(clean_baseboards_s1, vacuum_cleaner, clean).
purpose(clean_baseboards_s1, dust_removed).
takes_time(clean_baseboards_s1, 5, 10, minute).
step(clean_baseboards, clean_baseboards_s2).
action(clean_baseboards_s2, mix).
object_of(clean_baseboards_s2, dish_soap).
into(clean_baseboards_s2, bucket).
uses(clean_baseboards_s2, bucket).
role(clean_baseboards_s2, bucket, contain).
overlaps(clean_baseboards_s2, clean_baseboards_s1).
step(clean_baseboards, clean_baseboards_s3).
action(clean_baseboards_s3, scrub).
object_of(clean_baseboards_s3, baseboard).
uses(clean_baseboards_s3, sponge).
role(clean_baseboards_s3, sponge, scrub).
purpose(clean_baseboards_s3, lift_dirt).
takes_time(clean_baseboards_s3, 10, 15, minute).
step(clean_baseboards, clean_baseboards_s4).
action(clean_baseboards_s4, wipe).
object_of(clean_baseboards_s4, baseboard).
uses(clean_baseboards_s4, dry_cloth).
role(clean_baseboards_s4, dry_cloth, wipe).
achieves(clean_baseboards_s4, dry).
takes_time(clean_baseboards_s4, 3, 5, minute).
before(clean_baseboards_s1, clean_baseboards_s3).
before(clean_baseboards_s2, clean_baseboards_s3).
before(clean_baseboards_s3, clean_baseboards_s4).
```

## aliases
```datalog
task_alias(clean_baseboards, "clean baseboards").
task_alias(clean_baseboards, "skirting boards").
task_alias(clean_baseboards, "wash baseboards").
alias(baseboard, "skirting board").
alias(baseboard, "skirting").
alias(vacuum_cleaner, "hoover").
alias(dish_soap, "washing up liquid").
alias(dust_removed, "dust removed").
alias(dry, "dry").
```

## teach
- To clean baseboards you need a vacuum cleaner, a bucket, dish soap, a sponge and a dry cloth.
- Cleaning baseboards takes 20 to 30 minutes.
- Step 1 of cleaning baseboards: vacuum the baseboards with the vacuum cleaner to remove dust.
- While step 1 runs, step 2 of cleaning baseboards: mix dish soap into the bucket of warm water.
- Step 3 of cleaning baseboards: scrub the baseboards with the sponge to lift dirt.
- Step 4 of cleaning baseboards: wipe the baseboards with the dry cloth so they are dry.

## checklist
- names what is needed, including a vacuum cleaner, a bucket, dish soap, a sponge and a dry cloth
- gives the steps in order: vacuum, mix soapy water while vacuuming, scrub, wipe dry
- says the mixing of the soapy water can be done while the vacuuming runs
- gives a total time of about 20 to 30 minutes

## variant
- change: no vacuum cleaner: dust the baseboards with the dry cloth instead
- request: My vacuum cleaner has broken. Can I still clean my skirting boards?
- situation: I have a bucket, dish soap, a sponge and a dry cloth, but no vacuum cleaner.
- expect: The reply dusts the baseboards with the dry cloth in place of the vacuum, then scrubs and dries them as usual.

```datalog situation
lacks(user, vacuum_cleaner).
has(user, bucket).
has(user, dish_soap).
has(user, sponge).
has(user, dry_cloth).
```

```datalog difference
swap(clean_baseboards_s1, vacuum_cleaner, dry_cloth).
no_need(vacuum_cleaner).
```

## variant
- change: only 15 minutes: skip the vacuuming and just wash and dry the baseboards
- request: I've only got a quarter of an hour. Can I still clean the baseboards quickly?
- situation: I have all the usual items and fifteen minutes.
- expect: The reply skips vacuuming, mixes the soapy water, scrubs and dries the baseboards in about 10 to 15 minutes.

```datalog situation
has(user, vacuum_cleaner).
has(user, bucket).
has(user, dish_soap).
has(user, sponge).
has(user, dry_cloth).
time_limit(user, 15, minute).
```

```datalog difference
drop(clean_baseboards_s1).
no_need(vacuum_cleaner).
change(clean_baseboards, takes_time, 20_30, 10_15).
```
