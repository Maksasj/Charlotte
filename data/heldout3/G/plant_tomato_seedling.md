---
id: plant_tomato_seedling
stage: household
task: plant_tomato_seedling
schema: v3
tier: G
---
# Plant a tomato seedling

## requests
- My tomato seedlings are ready to move on. How do I pot one up into its final pot?
- I want to grow a tomato in a big pot on the balcony. What do I do with the little plant from the shop?
- Can you show me how to plant out a tomato seedling and stake it?

## situation
- I have a tomato seedling, a large pot, multipurpose compost, a bamboo cane, garden twine, a watering can and water.

## ground truth
```datalog
task(plant_tomato_seedling).
domain(plant_tomato_seedling, gardening).
goal(plant_tomato_seedling, planted_tomato).
needs(plant_tomato_seedling, large_pot).
needs(plant_tomato_seedling, multipurpose_compost).
needs(plant_tomato_seedling, tomato_seedling).
needs(plant_tomato_seedling, cane).
needs(plant_tomato_seedling, garden_twine).
needs(plant_tomato_seedling, watering_can).
needs(plant_tomato_seedling, water).
takes_time(plant_tomato_seedling, 10, 15, minute).
step(plant_tomato_seedling, plant_tomato_seedling_s1).
action(plant_tomato_seedling_s1, fill).
object_of(plant_tomato_seedling_s1, large_pot).
uses(plant_tomato_seedling_s1, multipurpose_compost).
role(plant_tomato_seedling_s1, multipurpose_compost, fill).
step(plant_tomato_seedling, plant_tomato_seedling_s2).
action(plant_tomato_seedling_s2, set).
object_of(plant_tomato_seedling_s2, tomato_seedling).
into(plant_tomato_seedling_s2, large_pot).
purpose(plant_tomato_seedling_s2, extra_roots).
check(plant_tomato_seedling_s2, buried_to_first_leaves).
step(plant_tomato_seedling, plant_tomato_seedling_s3).
action(plant_tomato_seedling_s3, push).
object_of(plant_tomato_seedling_s3, cane).
into(plant_tomato_seedling_s3, large_pot).
avoid(plant_tomato_seedling_s3, pierce, root).
step(plant_tomato_seedling, plant_tomato_seedling_s4).
action(plant_tomato_seedling_s4, tie).
object_of(plant_tomato_seedling_s4, tomato_seedling).
onto(plant_tomato_seedling_s4, cane).
uses(plant_tomato_seedling_s4, garden_twine).
role(plant_tomato_seedling_s4, garden_twine, fasten).
step(plant_tomato_seedling, plant_tomato_seedling_s5).
action(plant_tomato_seedling_s5, water).
object_of(plant_tomato_seedling_s5, tomato_seedling).
uses(plant_tomato_seedling_s5, watering_can).
role(plant_tomato_seedling_s5, watering_can, pour).
uses(plant_tomato_seedling_s5, water).
role(plant_tomato_seedling_s5, water, wet).
before(plant_tomato_seedling_s1, plant_tomato_seedling_s2).
before(plant_tomato_seedling_s2, plant_tomato_seedling_s3).
before(plant_tomato_seedling_s3, plant_tomato_seedling_s4).
before(plant_tomato_seedling_s4, plant_tomato_seedling_s5).
```

## aliases
```datalog
task_alias(plant_tomato_seedling, "planting a tomato seedling").
task_alias(plant_tomato_seedling, "pot up a tomato").
task_alias(plant_tomato_seedling, "plant out a tomato").
alias(tomato_seedling, "tomato plant").
alias(tomato_seedling, "seedling").
alias(large_pot, "big pot").
alias(multipurpose_compost, "compost").
alias(cane, "bamboo cane").
alias(garden_twine, "twine").
alias(buried_to_first_leaves, "buried up to its first leaves").
alias(thread, "wool").
alias(thread, "thick thread").
alias(peat_free_compost, "peat-free compost").
alias(peat, "peat").
```

## teach
- To plant a tomato seedling you need a large pot, multipurpose compost, the seedling, a cane, garden twine, a watering can and water.
- Planting a tomato seedling takes 10 to 15 minutes.
- Step 1 of planting a tomato seedling: fill the large pot with compost.
- Step 2 of planting a tomato seedling: set the seedling deep in the pot, buried up to its first leaves, so the stem grows extra roots.
- Step 3 of planting a tomato seedling: push the cane into the pot beside the stem without piercing the roots.
- Step 4 of planting a tomato seedling: tie the seedling loosely to the cane with garden twine.
- Step 5 of planting a tomato seedling: water the seedling with the watering can.

## checklist
- names what is needed: a large pot, compost, the seedling, a cane, twine and water
- gives the steps in order: fill pot, plant deep, cane, tie, water
- says to plant deep up to the first leaves
- gives a total time of about 10 to 15 minutes

## variant
- change: no twine: tie the stem with soft wool instead
- request: I'm planting out my tomato but I've run out of garden twine. I have a ball of wool, can I tie it up with that?
- situation: I have the seedling, a large pot, compost, a cane, a watering can, water and some wool, but no twine.
- expect: The reply ties the stem loosely to the cane with the wool instead of twine and keeps the other steps.

```datalog situation
lacks(user, garden_twine).
has(user, thread).
has(user, tomato_seedling).
has(user, large_pot).
has(user, multipurpose_compost).
has(user, cane).
has(user, watering_can).
has(user, water).
```

```datalog difference
no_need(garden_twine).
extra_need(thread).
swap(plant_tomato_seedling_s4, garden_twine, thread).
```

## variant
- change: no peat: fill the pot with peat-free compost
- request: I don't want to use anything with peat in it for my tomatoes. How should I plant the seedling?
- situation: I have the seedling, a large pot, peat-free compost, a cane, twine, a watering can and water, and I avoid peat.
- expect: The reply fills the pot with peat-free compost instead of ordinary multipurpose compost and keeps the other steps.

```datalog situation
excludes(user, peat).
has(user, peat_free_compost).
has(user, tomato_seedling).
has(user, large_pot).
has(user, cane).
has(user, garden_twine).
has(user, watering_can).
has(user, water).
```

```datalog difference
no_need(multipurpose_compost).
extra_need(peat_free_compost).
swap(plant_tomato_seedling_s1, multipurpose_compost, peat_free_compost).
```
