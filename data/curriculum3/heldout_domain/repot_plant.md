---
id: repot_plant
stage: household
task: repot_plant
schema: v3
---
# Repot a plant

## requests
- My houseplant has roots coming out of the bottom. How do I repot it?
- Can you walk me through moving a plant into a bigger pot?
- I bought a new pot for my monstera. What's the right way to repot it?

## situation
- I have the plant, a bigger pot, potting soil, a trowel, water and some newspaper.
- I have about 20 minutes.

## ground truth
```datalog
task(repot_plant).
domain(repot_plant, gardening).
goal(repot_plant, repotted_plant).
needs(repot_plant, new_pot).
needs(repot_plant, potting_soil).
needs(repot_plant, trowel).
needs(repot_plant, water).
needs(repot_plant, newspaper).
takes_time(repot_plant, 12, 18, minute).
step(repot_plant, repot_plant_s1).
action(repot_plant_s1, spread).
object_of(repot_plant_s1, newspaper).
onto(repot_plant_s1, table).
takes_time(repot_plant_s1, 1, 1, minute).
step(repot_plant, repot_plant_s2).
action(repot_plant_s2, remove).
object_of(repot_plant_s2, plant).
from(repot_plant_s2, old_pot).
takes_time(repot_plant_s2, 2, 3, minute).
step(repot_plant, repot_plant_s3).
action(repot_plant_s3, loosen).
object_of(repot_plant_s3, root).
takes_time(repot_plant_s3, 2, 3, minute).
step(repot_plant, repot_plant_s4).
action(repot_plant_s4, put).
object_of(repot_plant_s4, potting_soil).
into(repot_plant_s4, new_pot).
step(repot_plant, repot_plant_s5).
action(repot_plant_s5, set).
object_of(repot_plant_s5, plant).
into(repot_plant_s5, new_pot).
step(repot_plant, repot_plant_s6).
action(repot_plant_s6, fill).
object_of(repot_plant_s6, new_pot).
uses(repot_plant_s6, potting_soil).
role(repot_plant_s6, potting_soil, fill).
uses(repot_plant_s6, trowel).
role(repot_plant_s6, trowel, lift).
takes_time(repot_plant_s6, 4, 5, minute).
step(repot_plant, repot_plant_s7).
action(repot_plant_s7, water).
object_of(repot_plant_s7, plant).
uses(repot_plant_s7, water).
role(repot_plant_s7, water, wet).
until(repot_plant_s7, drained).
takes_time(repot_plant_s7, 2, 3, minute).
before(repot_plant_s1, repot_plant_s2).
before(repot_plant_s2, repot_plant_s3).
before(repot_plant_s3, repot_plant_s4).
before(repot_plant_s4, repot_plant_s5).
before(repot_plant_s5, repot_plant_s6).
before(repot_plant_s6, repot_plant_s7).
```

## aliases
```datalog
task_alias(repot_plant, "repotting a plant").
task_alias(repot_plant, "repot a plant").
task_alias(repot_plant, "moving a plant into a bigger pot").
task_alias(repot_plant, "repot it").
alias(new_pot, "bigger pot").
alias(new_pot, "new pot").
alias(old_pot, "old pot").
alias(old_pot, "same pot").
alias(plant, "houseplant").
alias(plant, "monstera").
alias(root, "roots").
alias(root, "outer roots").
alias(drained, "water drains from the bottom").
alias(scissors, "clean scissors").
alias(potting_soil, "fresh potting soil").
```

## teach
- To repot a plant you need a bigger pot, potting soil, a trowel, water and newspaper.
- Repotting a plant takes 12 to 18 minutes.
- Step 1 of repotting a plant: spread the newspaper on the table for 1 minute.
- Step 2 of repotting a plant: remove the plant from its old pot by tipping it and easing it out for 2 to 3 minutes.
- Step 3 of repotting a plant: loosen the roots gently with your fingers for 2 to 3 minutes.
- Step 4 of repotting a plant: put a layer of potting soil in the new pot.
- Step 5 of repotting a plant: set the plant in the new pot.
- Step 6 of repotting a plant: fill around it with the trowel, for 4 to 5 minutes.
- Step 7 of repotting a plant: water the plant until water drains from the bottom, for 2 to 3 minutes.

## checklist
- names what is needed: a bigger pot, potting soil, a trowel, water and newspaper
- gives the steps in order: spread newspaper, remove plant, loosen root, fill new pot, water plant
- gives a total time of about 12 to 18 minutes
- closes politely

## variant
- change: no bigger pot: trim the outer roots and repot in the same pot with fresh soil
- request: My plant is root-bound but I don't have a bigger pot. What can I do?
- situation: I have the plant in its old pot, potting soil, a trowel, clean scissors, water and newspaper.
- expect: The reply trims away some of the outer roots with the scissors and puts the plant back in the same pot with fresh potting soil, keeping the remove, loosen and water steps.

```datalog situation
has(user, plant).
has(user, old_pot).
has(user, potting_soil).
has(user, trowel).
has(user, scissors).
has(user, water).
has(user, newspaper).
lacks(user, new_pot).
```

```datalog difference
no_need(new_pot).
extra_need(scissors).
add_step(repot_plant_s9, repot_plant_s3).
step(repot_plant, repot_plant_s9).
action(repot_plant_s9, trim).
object_of(repot_plant_s9, root).
uses(repot_plant_s9, scissors).
role(repot_plant_s9, scissors, cut).
before(repot_plant_s3, repot_plant_s9).
before(repot_plant_s9, repot_plant_s4).
change(repot_plant_s4, into, new_pot, old_pot).
change(repot_plant_s5, into, new_pot, old_pot).
change(repot_plant_s6, object_of, new_pot, old_pot).
```
