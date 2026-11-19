---
id: clean_sofa_upholstery
stage: procedures
task: clean_sofa_upholstery
schema: v3
---
# Clean a sofa

## requests
- How do I clean the fabric on my sofa without ruining it?
- What's the proper way to clean an upholstered couch?
- Can you walk me through deep-cleaning my sofa cushions and frame?

## situation
- I have a fabric sofa, a vacuum cleaner, upholstery cleaner, a soft brush, a cloth and a fan.
- I've got about 45 minutes this afternoon.

## ground truth
```datalog
task(clean_sofa_upholstery).
domain(clean_sofa_upholstery, cleaning).
goal(clean_sofa_upholstery, clean_sofa).
needs(clean_sofa_upholstery, sofa).
needs(clean_sofa_upholstery, vacuum_cleaner).
needs(clean_sofa_upholstery, upholstery_brush).
needs(clean_sofa_upholstery, upholstery_cleaner).
needs(clean_sofa_upholstery, cloth).
needs(clean_sofa_upholstery, fan).
takes_time(clean_sofa_upholstery, 45, 60, minute).
step(clean_sofa_upholstery, clean_sofa_upholstery_s1).
action(clean_sofa_upholstery_s1, vacuum).
object_of(clean_sofa_upholstery_s1, sofa).
uses(clean_sofa_upholstery_s1, vacuum_cleaner).
role(clean_sofa_upholstery_s1, vacuum_cleaner, clean).
takes_time(clean_sofa_upholstery_s1, 10, 15, minute).
step(clean_sofa_upholstery, clean_sofa_upholstery_s2).
action(clean_sofa_upholstery_s2, scrub).
object_of(clean_sofa_upholstery_s2, sofa).
uses(clean_sofa_upholstery_s2, upholstery_brush).
role(clean_sofa_upholstery_s2, upholstery_brush, scrub).
uses(clean_sofa_upholstery_s2, upholstery_cleaner).
role(clean_sofa_upholstery_s2, upholstery_cleaner, clean).
purpose(clean_sofa_upholstery_s2, dirt_lifts).
warning(clean_sofa_upholstery_s2, too_wet).
takes_time(clean_sofa_upholstery_s2, 15, 20, minute).
step(clean_sofa_upholstery, clean_sofa_upholstery_s3).
action(clean_sofa_upholstery_s3, wipe).
object_of(clean_sofa_upholstery_s3, sofa).
uses(clean_sofa_upholstery_s3, cloth).
role(clean_sofa_upholstery_s3, cloth, wipe).
purpose(clean_sofa_upholstery_s3, cleaner_removed).
takes_time(clean_sofa_upholstery_s3, 10, 15, minute).
step(clean_sofa_upholstery, clean_sofa_upholstery_s4).
action(clean_sofa_upholstery_s4, dry).
object_of(clean_sofa_upholstery_s4, sofa).
uses(clean_sofa_upholstery_s4, fan).
role(clean_sofa_upholstery_s4, fan, dry).
overlaps(clean_sofa_upholstery_s4, clean_sofa_upholstery_s3).
before(clean_sofa_upholstery_s1, clean_sofa_upholstery_s2).
before(clean_sofa_upholstery_s2, clean_sofa_upholstery_s3).
```

## aliases
```datalog
task_alias(clean_sofa_upholstery, "cleaning a sofa").
task_alias(clean_sofa_upholstery, "clean a couch").
task_alias(clean_sofa_upholstery, "deep-cleaning a sofa").
alias(sofa, "couch").
alias(vacuum_cleaner, "hoover").
alias(upholstery_brush, "soft brush").
alias(upholstery_cleaner, "fabric cleaner").
alias(fan, "electric fan").
alias(dirt_lifts, "dirt lifts").
alias(too_wet, "too wet").
alias(cleaner_removed, "cleaner is removed").
```

## teach
- To clean a sofa you need a sofa, a vacuum cleaner, an upholstery brush, an upholstery cleaner, a cloth and a fan.
- Cleaning a sofa takes 45 to 60 minutes.
- Step 1 of cleaning a sofa: vacuum the sofa with the vacuum cleaner for 10 to 15 minutes.
- Step 2 of cleaning a sofa: scrub the sofa with the upholstery brush and upholstery cleaner for 15 to 20 minutes so the dirt lifts, but do not let the fabric get too wet.
- Step 3 of cleaning a sofa: wipe the sofa with a cloth for 10 to 15 minutes so the cleaner is removed.
- While step 3 runs, step 4 of cleaning a sofa: dry the sofa with the fan.

## checklist
- names what is needed, including a sofa, a vacuum cleaner, an upholstery brush, upholstery cleaner, a cloth and a fan
- gives the steps in order: vacuum, scrub with brush and cleaner, wipe with cloth, dry with fan
- says the fabric must not get too wet while scrubbing
- says drying with the fan can start while the wiping is still going on
- gives a total time of about 45 to 60 minutes

## variant
- change: no fan: use the hair dryer on a cool setting to dry the sofa instead.
- request: I don't have a fan. Can I use my hair dryer to dry the sofa after cleaning it?
- situation: I have a sofa, a vacuum cleaner, upholstery brush, upholstery cleaner, a cloth and a hair dryer, but no fan.
- expect: The reply keeps every step in order and swaps the fan for the hair dryer on a cool setting in the drying step.

```datalog situation
has(user, sofa).
has(user, vacuum_cleaner).
has(user, upholstery_brush).
has(user, upholstery_cleaner).
has(user, cloth).
has(user, hair_dryer).
lacks(user, fan).
```

```datalog difference
swap(clean_sofa_upholstery_s4, fan, hair_dryer).
extra_need(hair_dryer).
no_need(fan).
```

## variant
- change: only 30 minutes: skip the vacuuming and go straight to scrubbing, wiping and drying the sofa.
- request: I only have half an hour before guests arrive. Can you give me a quicker way to clean the sofa?
- situation: I have a sofa, upholstery brush, upholstery cleaner, a cloth and a fan, and only 30 minutes.
- expect: The reply leaves out the vacuuming and gives the scrub, wipe and dry steps in order, finishing in about 25 to 35 minutes.

```datalog situation
has(user, sofa).
has(user, upholstery_brush).
has(user, upholstery_cleaner).
has(user, cloth).
has(user, fan).
time_limit(user, 30, minute).
```

```datalog difference
drop(clean_sofa_upholstery_s1).
no_need(vacuum_cleaner).
change(clean_sofa_upholstery, takes_time, 45_60, 25_35).
```
