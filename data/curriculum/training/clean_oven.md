---
id: clean_oven
stage: household
task: clean_oven
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
takes_time(clean_oven_s1, 2, 3, minute).
step(clean_oven, clean_oven_s2).
action(clean_oven_s2, mix).
object_of(clean_oven_s2, baking_soda).
uses(clean_oven_s2, water).
uses(clean_oven_s2, bowl).
takes_time(clean_oven_s2, 2, 3, minute).
step(clean_oven, clean_oven_s3).
action(clean_oven_s3, spread).
object_of(clean_oven_s3, paste).
uses(clean_oven_s3, sponge).
takes_time(clean_oven_s3, 5, 7, minute).
step(clean_oven, clean_oven_s4).
action(clean_oven_s4, leave).
object_of(clean_oven_s4, paste).
takes_time(clean_oven_s4, 30, 40, minute).
step(clean_oven, clean_oven_s5).
action(clean_oven_s5, scrub).
object_of(clean_oven_s5, oven_rack).
uses(clean_oven_s5, dish_soap).
uses(clean_oven_s5, sponge).
takes_time(clean_oven_s5, 10, 15, minute).
overlaps(clean_oven_s5, clean_oven_s4).
step(clean_oven, clean_oven_s6).
action(clean_oven_s6, wipe).
object_of(clean_oven_s6, paste).
uses(clean_oven_s6, vinegar).
uses(clean_oven_s6, spray_bottle).
uses(clean_oven_s6, sponge).
until(clean_oven_s6, clean).
takes_time(clean_oven_s6, 5, 10, minute).
before(clean_oven_s1, clean_oven_s2).
before(clean_oven_s2, clean_oven_s3).
before(clean_oven_s3, clean_oven_s4).
before(clean_oven_s3, clean_oven_s5).
before(clean_oven_s4, clean_oven_s6).
before(clean_oven_s5, clean_oven_s6).
```

## teach
- To clean the oven you need baking soda, water, a bowl, white vinegar, a spray bottle, a sponge and dish soap.
- Cleaning the oven takes 45 to 60 minutes.
- Step 1 of cleaning the oven: take the oven racks out of the oven for 2 to 3 minutes.
- Step 2 of cleaning the oven: mix the baking soda with a little water in the bowl to make a paste for 2 to 3 minutes.
- Step 3 of cleaning the oven: spread the paste over the inside of the oven with the sponge for 5 to 7 minutes.
- Step 4 of cleaning the oven: leave the paste to work on the grease for 30 to 40 minutes.
- While step 4 runs, step 5 of cleaning the oven: scrub the oven racks with the sponge in hot water and dish soap for 10 to 15 minutes.
- Step 6 of cleaning the oven: spray vinegar from the spray bottle on the paste and wipe it away with the sponge until clean, for 5 to 10 minutes.

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
