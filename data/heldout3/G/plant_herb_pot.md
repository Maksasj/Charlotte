---
id: plant_herb_pot
stage: household
task: plant_herb_pot
schema: v3
tier: G
---
# Plant a herb pot

## requests
- I'd love fresh herbs by the back door. How do I plant up a pot of them?
- I've bought basil, thyme and parsley plants. How do I put them together in one container?
- What's the way to plant herbs in a pot for the patio?

## situation
- I have a big pot, some broken pot pieces, potting soil, a trowel, three herb plants, a watering can and water.

## ground truth
```datalog
task(plant_herb_pot).
domain(plant_herb_pot, gardening).
goal(plant_herb_pot, herb_pot).
needs(plant_herb_pot, pot).
needs(plant_herb_pot, crock).
needs(plant_herb_pot, potting_soil).
needs(plant_herb_pot, trowel).
needs(plant_herb_pot, herb_plant).
needs(plant_herb_pot, watering_can).
needs(plant_herb_pot, water).
takes_time(plant_herb_pot, 15, 20, minute).
step(plant_herb_pot, plant_herb_pot_s1).
action(plant_herb_pot_s1, put).
object_of(plant_herb_pot_s1, crock).
into(plant_herb_pot_s1, pot).
purpose(plant_herb_pot_s1, drainage).
step(plant_herb_pot, plant_herb_pot_s2).
action(plant_herb_pot_s2, fill).
object_of(plant_herb_pot_s2, pot).
uses(plant_herb_pot_s2, potting_soil).
role(plant_herb_pot_s2, potting_soil, fill).
uses(plant_herb_pot_s2, trowel).
role(plant_herb_pot_s2, trowel, lift).
step(plant_herb_pot, plant_herb_pot_s3).
action(plant_herb_pot_s3, set).
object_of(plant_herb_pot_s3, herb_plant).
into(plant_herb_pot_s3, pot).
amount(plant_herb_pot_s3, herb_plant, 3, piece).
takes_time(plant_herb_pot_s3, 5, 10, minute).
step(plant_herb_pot, plant_herb_pot_s4).
action(plant_herb_pot_s4, press).
object_of(plant_herb_pot_s4, potting_soil).
purpose(plant_herb_pot_s4, no_air_pockets).
step(plant_herb_pot, plant_herb_pot_s5).
action(plant_herb_pot_s5, water).
object_of(plant_herb_pot_s5, herb_plant).
uses(plant_herb_pot_s5, watering_can).
role(plant_herb_pot_s5, watering_can, pour).
uses(plant_herb_pot_s5, water).
role(plant_herb_pot_s5, water, wet).
before(plant_herb_pot_s1, plant_herb_pot_s2).
before(plant_herb_pot_s2, plant_herb_pot_s3).
before(plant_herb_pot_s3, plant_herb_pot_s4).
before(plant_herb_pot_s4, plant_herb_pot_s5).
```

## aliases
```datalog
task_alias(plant_herb_pot, "planting a herb pot").
task_alias(plant_herb_pot, "plant up a pot of herbs").
task_alias(plant_herb_pot, "plant herbs in a container").
alias(pot, "big pot").
alias(pot, "container").
alias(crock, "broken pot pieces").
alias(crock, "crocks").
alias(herb_plant, "herb plants").
alias(herb_plant, "basil, thyme and parsley").
alias(hose, "garden hose").
alias(no_air_pockets, "gets rid of air pockets").
```

## teach
- To plant a herb pot you need a pot, some crocks, potting soil, a trowel, three herb plants, a watering can and water.
- Planting a herb pot takes 15 to 20 minutes.
- Step 1 of planting a herb pot: put crocks into the bottom of the pot for drainage.
- Step 2 of planting a herb pot: fill the pot with potting soil using the trowel.
- Step 3 of planting a herb pot: set the three herb plants into the pot, for 5 to 10 minutes.
- Step 4 of planting a herb pot: press the soil down around them to get rid of air pockets.
- Step 5 of planting a herb pot: water the herbs with the watering can.

## checklist
- names what is needed: a pot, crocks, potting soil, a trowel, herb plants and water
- gives the steps in order: crocks, fill, set plants, firm, water
- says the crocks are for drainage
- gives a total time of about 15 to 20 minutes

## variant
- change: six herb plants: plant six instead of three, a bit longer
- request: I got carried away at the market and came home with six herbs. Can they all go in my big tub, and how?
- situation: I have a big pot, crocks, potting soil, a trowel, six herb plants, a watering can and water.
- expect: The reply sets all six plants in the pot and gives a total of about 20 to 30 minutes.

```datalog situation
has(user, pot).
has(user, crock).
has(user, potting_soil).
has(user, trowel).
has(user, herb_plant).
has(user, watering_can).
has(user, water).
count(user, herb_plant, 6).
```

```datalog difference
change(plant_herb_pot_s3, amount, 3, 6).
change(plant_herb_pot_s3, takes_time, 5_10, 10_20).
change(plant_herb_pot, takes_time, 15_20, 20_30).
```

## variant
- change: no watering can: water gently with the garden hose
- request: All set to pot up my herbs, only I don't have a watering can. The hose reaches the patio, though.
- situation: I have a pot, crocks, potting soil, a trowel, three herb plants and a garden hose, but no watering can.
- expect: The reply waters the herbs gently with the hose instead of a watering can and keeps the other steps.

```datalog situation
lacks(user, watering_can).
has(user, hose).
has(user, pot).
has(user, crock).
has(user, potting_soil).
has(user, trowel).
has(user, herb_plant).
```

```datalog difference
no_need(watering_can).
no_need(water).
extra_need(hose).
swap(plant_herb_pot_s5, watering_can, hose).
```
