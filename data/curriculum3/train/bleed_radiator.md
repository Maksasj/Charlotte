---
id: bleed_radiator
stage: tools
task: bleed_radiator
schema: v3
---
# Bleed a radiator

## requests
- My radiator is cold at the top and warm at the bottom. How do I bleed it?
- Can you tell me how to let the air out of a radiator?
- The heating's on but one radiator isn't warming up. What do I do?

## situation
- I have a radiator key and a cloth.
- I have about 20 minutes.

## ground truth
```datalog
task(bleed_radiator).
domain(bleed_radiator, repair).
goal(bleed_radiator, warm_radiator).
needs(bleed_radiator, radiator_key).
needs(bleed_radiator, cloth).
takes_time(bleed_radiator, 10, 15, minute).
step(bleed_radiator, bleed_radiator_s1).
action(bleed_radiator_s1, switch).
object_of(bleed_radiator_s1, heating).
achieves(bleed_radiator_s1, off).
step(bleed_radiator, bleed_radiator_s2).
action(bleed_radiator_s2, cool).
object_of(bleed_radiator_s2, radiator).
takes_time(bleed_radiator_s2, 5, 10, minute).
step(bleed_radiator, bleed_radiator_s3).
action(bleed_radiator_s3, hold).
object_of(bleed_radiator_s3, cloth).
purpose(bleed_radiator_s3, catch_drips).
step(bleed_radiator, bleed_radiator_s4).
action(bleed_radiator_s4, open).
object_of(bleed_radiator_s4, valve).
uses(bleed_radiator_s4, radiator_key).
role(bleed_radiator_s4, radiator_key, loosen).
setting(bleed_radiator_s4, rotation, 0.25, turn).
until(bleed_radiator_s4, dripping).
takes_time(bleed_radiator_s4, 1, 2, minute).
step(bleed_radiator, bleed_radiator_s5).
action(bleed_radiator_s5, close).
object_of(bleed_radiator_s5, valve).
uses(bleed_radiator_s5, radiator_key).
role(bleed_radiator_s5, radiator_key, fasten).
takes_time(bleed_radiator_s5, 1, 1, minute).
step(bleed_radiator, bleed_radiator_s6).
action(bleed_radiator_s6, check).
object_of(bleed_radiator_s6, boiler_pressure).
takes_time(bleed_radiator_s6, 1, 2, minute).
step(bleed_radiator, bleed_radiator_s7).
action(bleed_radiator_s7, switch).
object_of(bleed_radiator_s7, heating).
achieves(bleed_radiator_s7, on).
before(bleed_radiator_s1, bleed_radiator_s2).
before(bleed_radiator_s2, bleed_radiator_s3).
before(bleed_radiator_s3, bleed_radiator_s4).
before(bleed_radiator_s4, bleed_radiator_s5).
before(bleed_radiator_s5, bleed_radiator_s6).
before(bleed_radiator_s6, bleed_radiator_s7).
```

## aliases
```datalog
task_alias(bleed_radiator, "bleeding a radiator").
task_alias(bleed_radiator, "let the air out of a radiator").
alias(valve, "bleed valve").
alias(radiator_key, "key").
alias(radiator, "radiators").
alias(switch, "switch off").
alias(switch, "switch back on").
alias(boiler_pressure, "pressure").
alias(dripping, "water starts to drip").
alias(catch_drips, "catch the drips").
alias(farthest_radiator, "the one farthest from the boiler downstairs").
```

## teach
- To bleed a radiator you need a radiator key and a cloth.
- Bleeding a radiator takes 10 to 15 minutes.
- Step 1 of bleeding a radiator: switch off the heating.
- Step 2 of bleeding a radiator: let the radiator cool for 5 to 10 minutes.
- Step 3 of bleeding a radiator: hold the cloth under the bleed valve to catch the drips.
- Step 4 of bleeding a radiator: open the bleed valve a quarter turn with the radiator key until water starts to drip, for 1 to 2 minutes.
- Step 5 of bleeding a radiator: close the valve again with the radiator key for 1 minute.
- Step 6 of bleeding a radiator: check the boiler pressure for 1 to 2 minutes.
- Step 7 of bleeding a radiator: switch the heating back on.

## checklist
- names what is needed: a radiator key and a cloth
- gives the steps in order: switch heating, hold cloth, open valve, close valve, check boiler pressure
- gives a total time of about 10 to 15 minutes
- closes politely

## variant
- change: several cold radiators: bleed them one by one, starting with the one farthest from the boiler downstairs
- request: Actually three of my radiators are cold at the top. Do I bleed them all the same way?
- situation: I have a radiator key and a cloth, and three radiators need bleeding.
- expect: The reply bleeds each radiator with the same steps, starting downstairs with the one farthest from the boiler, and checks the boiler pressure once at the end.

```datalog situation
has(user, radiator_key).
has(user, cloth).
count(user, radiator, 3).
```

```datalog difference
add_step(bleed_radiator_s9, bleed_radiator_s2).
step(bleed_radiator, bleed_radiator_s9).
action(bleed_radiator_s9, start).
object_of(bleed_radiator_s9, farthest_radiator).
before(bleed_radiator_s9, bleed_radiator_s3).
change(bleed_radiator_s3, repeat, 1, 3).
change(bleed_radiator_s4, repeat, 1, 3).
change(bleed_radiator_s5, repeat, 1, 3).
```
