---
id: bleed_radiator
stage: tools
task: bleed_radiator
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
needs(bleed_radiator, radiator_key).
needs(bleed_radiator, cloth).
takes_time(bleed_radiator, 10, 15, minute).
step(bleed_radiator, bleed_radiator_s1).
action(bleed_radiator_s1, switch).
object_of(bleed_radiator_s1, heating).
takes_time(bleed_radiator_s1, 5, 10, minute).
step(bleed_radiator, bleed_radiator_s2).
action(bleed_radiator_s2, hold).
object_of(bleed_radiator_s2, cloth).
step(bleed_radiator, bleed_radiator_s3).
action(bleed_radiator_s3, open).
object_of(bleed_radiator_s3, valve).
uses(bleed_radiator_s3, radiator_key).
until(bleed_radiator_s3, dripping).
takes_time(bleed_radiator_s3, 1, 2, minute).
step(bleed_radiator, bleed_radiator_s4).
action(bleed_radiator_s4, close).
object_of(bleed_radiator_s4, valve).
uses(bleed_radiator_s4, radiator_key).
takes_time(bleed_radiator_s4, 1, 1, minute).
step(bleed_radiator, bleed_radiator_s5).
action(bleed_radiator_s5, check).
object_of(bleed_radiator_s5, boiler_pressure).
takes_time(bleed_radiator_s5, 1, 2, minute).
before(bleed_radiator_s1, bleed_radiator_s2).
before(bleed_radiator_s2, bleed_radiator_s3).
before(bleed_radiator_s3, bleed_radiator_s4).
before(bleed_radiator_s4, bleed_radiator_s5).
```

## teach
- To bleed a radiator you need a radiator key and a cloth.
- Bleeding a radiator takes 10 to 15 minutes.
- Step 1 of bleeding a radiator: switch off the heating and let the radiator cool for 5 to 10 minutes.
- Step 2 of bleeding a radiator: hold the cloth under the bleed valve.
- Step 3 of bleeding a radiator: open the bleed valve a quarter turn with the radiator key until water starts to drip, for 1 to 2 minutes.
- Step 4 of bleeding a radiator: close the valve again with the radiator key for 1 minute.
- Step 5 of bleeding a radiator: check the boiler pressure and switch the heating back on for 1 to 2 minutes.

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
