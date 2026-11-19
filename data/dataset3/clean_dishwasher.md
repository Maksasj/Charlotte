---
id: clean_dishwasher
stage: procedures
task: clean_dishwasher
schema: v3
---
# Clean a dishwasher

## requests
- How do I clean the inside of my dishwasher properly?
- What's the routine for deep-cleaning a dishwasher? I want to get rid of the smell.
- Could you tell me the steps to clean out my dishwasher's filter and door seal?

## situation
- I have a dishwasher, a sponge, a microfibre cloth and dishwasher tablets.
- I have about an hour and a half free.

## ground truth
```datalog
task(clean_dishwasher).
domain(clean_dishwasher, cleaning).
goal(clean_dishwasher, fresh_dishes).
needs(clean_dishwasher, dishwasher).
needs(clean_dishwasher, dishwasher_tablet).
needs(clean_dishwasher, sponge).
needs(clean_dishwasher, cloth).
takes_time(clean_dishwasher, 85, 115, minute).
step(clean_dishwasher, clean_dishwasher_s1).
action(clean_dishwasher_s1, scrub).
object_of(clean_dishwasher_s1, dishwasher_filter).
uses(clean_dishwasher_s1, sponge).
role(clean_dishwasher_s1, sponge, scrub).
purpose(clean_dishwasher_s1, spray_unblocked).
takes_time(clean_dishwasher_s1, 5, 10, minute).
step(clean_dishwasher, clean_dishwasher_s2).
action(clean_dishwasher_s2, wipe).
object_of(clean_dishwasher_s2, dishwasher_seal).
uses(clean_dishwasher_s2, cloth).
role(clean_dishwasher_s2, cloth, wipe).
overlaps(clean_dishwasher_s2, clean_dishwasher_s1).
takes_time(clean_dishwasher_s2, 5, 10, minute).
step(clean_dishwasher, clean_dishwasher_s3).
action(clean_dishwasher_s3, place).
object_of(clean_dishwasher_s3, dishwasher_tablet).
into(clean_dishwasher_s3, dishwasher).
takes_time(clean_dishwasher_s3, 1, 2, minute).
step(clean_dishwasher, clean_dishwasher_s4).
action(clean_dishwasher_s4, run).
object_of(clean_dishwasher_s4, dishwasher).
uses(clean_dishwasher_s4, dishwasher).
role(clean_dishwasher_s4, dishwasher, clean).
setting(clean_dishwasher_s4, cycle, normal, level).
takes_time(clean_dishwasher_s4, 80, 100, minute).
before(clean_dishwasher_s1, clean_dishwasher_s3).
before(clean_dishwasher_s2, clean_dishwasher_s3).
before(clean_dishwasher_s3, clean_dishwasher_s4).
```

## aliases
```datalog
task_alias(clean_dishwasher, "deep clean dishwasher").
task_alias(clean_dishwasher, "wash the dishwasher").
alias(dishwasher_filter, "filter").
alias(dishwasher_seal, "door seal").
alias(dishwasher_tablet, "dishwasher detergent tablet").
alias(sponge, "scouring pad").
alias(cloth, "microfibre cloth").
alias(spray_unblocked, "keeps the spray arm clear").
```

## teach
- To clean a dishwasher you need a dishwasher, a dishwasher tablet, a sponge and a cloth.
- Cleaning a dishwasher takes 85 to 115 minutes.
- Step 1 of cleaning a dishwasher: scrub the dishwasher filter with the sponge so the spray arm stays clear, which takes 5 to 10 minutes.
- While step 1 runs, step 2 of cleaning a dishwasher: wipe the door seal with the cloth, which takes 5 to 10 minutes.
- Step 3 of cleaning a dishwasher: put one dishwasher tablet into the dishwasher, which takes 1 to 2 minutes.
- Step 4 of cleaning a dishwasher: run the dishwasher on an empty normal cycle, which takes 80 to 100 minutes.

## checklist
- names a dishwasher, a dishwasher tablet, a sponge and a cloth
- gives the steps in order: scrub the filter, wipe the door seal, put in the tablet, run a normal cycle
- says the filter scrubbing and seal wiping can be done at the same time
- says the tablet goes in before the cycle starts
- gives a total time of about 85 to 115 minutes

## variant
- change: no cloth: wipe the door seal with the sponge instead
- request: I don't have a cloth. Can I clean the dishwasher with just my sponge?
- situation: I have a dishwasher, a sponge and dishwasher tablets, but no cloth.
- expect: The reply uses the sponge to wipe the door seal instead of the cloth and keeps the other steps the same.

```datalog situation
has(user, dishwasher).
has(user, dishwasher_tablet).
has(user, sponge).
lacks(user, cloth).
```

```datalog difference
swap(clean_dishwasher_s2, cloth, sponge).
```

## variant
- change: only 30 minutes: keep the same steps but use a quick cycle
- request: I only have half an hour before guests arrive. Can I still clean the dishwasher?
- situation: I have all the usual items and about 30 minutes.
- expect: The reply keeps the same four steps but switches to a quick cycle of about 20 to 25 minutes, bringing the whole job close to the 30 minutes available.

```datalog situation
has(user, dishwasher).
has(user, dishwasher_tablet).
has(user, sponge).
has(user, cloth).
time_limit(user, 30, minute).
```

```datalog difference
change(clean_dishwasher, takes_time, 85_115, 30_35).
change(clean_dishwasher_s4, takes_time, 80_100, 20_25).
change(clean_dishwasher_s4, setting, normal, quick).
```
