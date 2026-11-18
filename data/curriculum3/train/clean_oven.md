---
id: clean_oven
stage: household
task: clean_oven
schema: v3
---
# Clean the oven

## requests
- My oven is caked in grease. How do I clean it without harsh chemicals?
- Can you tell me how to clean an oven with baking soda?
- The oven smokes every time I use it. What's the best way to give it a deep clean?

## situation
- I have baking soda, water, a bowl, white vinegar, a spray bottle, a sponge and dish soap.
- I have an afternoon free.

## ground truth
```datalog
task(clean_oven).
domain(clean_oven, cleaning).
goal(clean_oven, grease_free_oven).
needs(clean_oven, baking_soda).
needs(clean_oven, water).
needs(clean_oven, bowl).
needs(clean_oven, vinegar).
needs(clean_oven, spray_bottle).
needs(clean_oven, sponge).
needs(clean_oven, dish_soap).
takes_time(clean_oven, 45, 60, minute).
step(clean_oven, clean_oven_s1).
action(clean_oven_s1, remove).
object_of(clean_oven_s1, oven_rack).
from(clean_oven_s1, oven).
takes_time(clean_oven_s1, 2, 3, minute).
step(clean_oven, clean_oven_s2).
action(clean_oven_s2, mix).
object_of(clean_oven_s2, baking_soda).
uses(clean_oven_s2, water).
role(clean_oven_s2, water, mix).
uses(clean_oven_s2, bowl).
role(clean_oven_s2, bowl, contain).
purpose(clean_oven_s2, paste_forms).
takes_time(clean_oven_s2, 2, 3, minute).
step(clean_oven, clean_oven_s3).
action(clean_oven_s3, spread).
object_of(clean_oven_s3, paste).
uses(clean_oven_s3, sponge).
role(clean_oven_s3, sponge, spread).
onto(clean_oven_s3, oven).
takes_time(clean_oven_s3, 5, 7, minute).
step(clean_oven, clean_oven_s4).
action(clean_oven_s4, leave).
object_of(clean_oven_s4, paste).
purpose(clean_oven_s4, grease_breaks_down).
takes_time(clean_oven_s4, 30, 40, minute).
step(clean_oven, clean_oven_s5).
action(clean_oven_s5, scrub).
object_of(clean_oven_s5, oven_rack).
uses(clean_oven_s5, sponge).
role(clean_oven_s5, sponge, scrub).
uses(clean_oven_s5, water).
role(clean_oven_s5, water, clean).
uses(clean_oven_s5, dish_soap).
role(clean_oven_s5, dish_soap, clean).
overlaps(clean_oven_s5, clean_oven_s4).
takes_time(clean_oven_s5, 10, 15, minute).
step(clean_oven, clean_oven_s6).
action(clean_oven_s6, spray).
object_of(clean_oven_s6, vinegar).
uses(clean_oven_s6, spray_bottle).
role(clean_oven_s6, spray_bottle, spray).
onto(clean_oven_s6, paste).
step(clean_oven, clean_oven_s7).
action(clean_oven_s7, wipe).
object_of(clean_oven_s7, paste).
uses(clean_oven_s7, sponge).
role(clean_oven_s7, sponge, wipe).
until(clean_oven_s7, clean).
takes_time(clean_oven_s7, 5, 10, minute).
before(clean_oven_s1, clean_oven_s2).
before(clean_oven_s2, clean_oven_s3).
before(clean_oven_s3, clean_oven_s4).
before(clean_oven_s3, clean_oven_s5).
before(clean_oven_s4, clean_oven_s6).
before(clean_oven_s5, clean_oven_s6).
before(clean_oven_s6, clean_oven_s7).
```

## aliases
```datalog
task_alias(clean_oven, "cleaning the oven").
task_alias(clean_oven, "deep clean").
alias(oven_rack, "oven racks").
alias(oven_rack, "racks").
alias(vinegar, "white vinegar").
alias(water, "hot water").
alias(paste, "baking soda paste").
alias(remove, "take out").
alias(wipe, "wipe away").
alias(paste_forms, "make a paste").
alias(grease_breaks_down, "work on the grease").
```

## teach
- To clean the oven you need baking soda, water, a bowl, white vinegar, a spray bottle, a sponge and dish soap.
- Cleaning the oven takes 45 to 60 minutes.
- Step 1 of cleaning the oven: take the oven racks out of the oven for 2 to 3 minutes.
- Step 2 of cleaning the oven: mix the baking soda with a little water in the bowl to make a paste for 2 to 3 minutes.
- Step 3 of cleaning the oven: spread the paste over the inside of the oven with the sponge for 5 to 7 minutes.
- Step 4 of cleaning the oven: leave the paste to work on the grease for 30 to 40 minutes.
- While step 4 runs, step 5 of cleaning the oven: scrub the oven racks with the sponge in hot water and dish soap for 10 to 15 minutes.
- Step 6 of cleaning the oven: spray vinegar from the spray bottle on the paste.
- Step 7 of cleaning the oven: wipe the paste away with the sponge until clean, for 5 to 10 minutes.

## checklist
- names what is needed: baking soda, water, a bowl, white vinegar, a spray bottle, a sponge and dish soap
- gives the steps in order: remove oven rack, mix baking soda, spread paste, leave paste, scrub oven rack (while leave paste), wipe paste
- gives a total time of about 45 to 60 minutes
- closes politely

## variant
- change: no vinegar: wipe the paste away with a damp sponge and plain water, rinsing it often
- request: I want to clean the oven with baking soda but I've got no vinegar. Is that a problem?
- situation: I have baking soda, water, a bowl, a sponge and dish soap, but no vinegar.
- expect: The reply wipes the paste off with a damp sponge and plain water, rinsing the sponge often, and keeps the racks, paste and wait steps.

```datalog situation
has(user, baking_soda).
has(user, water).
has(user, bowl).
has(user, sponge).
has(user, dish_soap).
lacks(user, vinegar).
```

```datalog difference
drop(clean_oven_s6).
add_step(clean_oven_s9, clean_oven_s4).
step(clean_oven, clean_oven_s9).
action(clean_oven_s9, dampen).
object_of(clean_oven_s9, sponge).
uses(clean_oven_s9, water).
role(clean_oven_s9, water, clean).
before(clean_oven_s9, clean_oven_s7).
add_step(clean_oven_s10, clean_oven_s7).
step(clean_oven, clean_oven_s10).
action(clean_oven_s10, rinse).
object_of(clean_oven_s10, sponge).
uses(clean_oven_s10, water).
role(clean_oven_s10, water, clean).
overlaps(clean_oven_s10, clean_oven_s7).
no_need(vinegar).
no_need(spray_bottle).
```
