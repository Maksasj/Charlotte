---
id: plant_hanging_basket
stage: household
task: plant_hanging_basket
schema: v3
tier: G
---
# Plant a hanging basket

## requests
- I'd love a hanging basket full of flowers by the front door. How do I make one up?
- What's the right way to plant up a hanging basket with trailing plants?
- I've got a wire basket and some petunias. How do I put it together so it doesn't dry out?

## situation
- I have a wire hanging basket, a liner, potting soil, water-retaining gel, five bedding plants, a watering can, water and a bracket on the wall.

## ground truth
```datalog
task(plant_hanging_basket).
domain(plant_hanging_basket, gardening).
goal(plant_hanging_basket, hanging_basket).
needs(plant_hanging_basket, hanging_basket).
needs(plant_hanging_basket, basket_liner).
needs(plant_hanging_basket, water_retaining_gel).
needs(plant_hanging_basket, potting_soil).
needs(plant_hanging_basket, bedding_plant).
needs(plant_hanging_basket, watering_can).
needs(plant_hanging_basket, water).
needs(plant_hanging_basket, bracket).
takes_time(plant_hanging_basket, 20, 30, minute).
step(plant_hanging_basket, plant_hanging_basket_s1).
action(plant_hanging_basket_s1, put).
object_of(plant_hanging_basket_s1, basket_liner).
into(plant_hanging_basket_s1, hanging_basket).
step(plant_hanging_basket, plant_hanging_basket_s2).
action(plant_hanging_basket_s2, mix).
object_of(plant_hanging_basket_s2, water_retaining_gel).
into(plant_hanging_basket_s2, potting_soil).
amount(plant_hanging_basket_s2, water_retaining_gel, 1, tablespoon).
purpose(plant_hanging_basket_s2, holds_water).
step(plant_hanging_basket, plant_hanging_basket_s3).
action(plant_hanging_basket_s3, fill).
object_of(plant_hanging_basket_s3, hanging_basket).
uses(plant_hanging_basket_s3, potting_soil).
role(plant_hanging_basket_s3, potting_soil, fill).
step(plant_hanging_basket, plant_hanging_basket_s4).
action(plant_hanging_basket_s4, set).
object_of(plant_hanging_basket_s4, bedding_plant).
into(plant_hanging_basket_s4, hanging_basket).
amount(plant_hanging_basket_s4, bedding_plant, 5, piece).
takes_time(plant_hanging_basket_s4, 10, 15, minute).
step(plant_hanging_basket, plant_hanging_basket_s5).
action(plant_hanging_basket_s5, water).
object_of(plant_hanging_basket_s5, bedding_plant).
uses(plant_hanging_basket_s5, watering_can).
role(plant_hanging_basket_s5, watering_can, pour).
uses(plant_hanging_basket_s5, water).
role(plant_hanging_basket_s5, water, wet).
until(plant_hanging_basket_s5, drained).
step(plant_hanging_basket, plant_hanging_basket_s6).
action(plant_hanging_basket_s6, hang).
object_of(plant_hanging_basket_s6, hanging_basket).
onto(plant_hanging_basket_s6, bracket).
before(plant_hanging_basket_s1, plant_hanging_basket_s2).
before(plant_hanging_basket_s2, plant_hanging_basket_s3).
before(plant_hanging_basket_s3, plant_hanging_basket_s4).
before(plant_hanging_basket_s4, plant_hanging_basket_s5).
before(plant_hanging_basket_s5, plant_hanging_basket_s6).
```

## aliases
```datalog
task_alias(plant_hanging_basket, "planting a hanging basket").
task_alias(plant_hanging_basket, "make up a hanging basket").
task_alias(plant_hanging_basket, "plant up a basket").
alias(hanging_basket, "wire basket").
alias(hanging_basket, "basket").
alias(basket_liner, "liner").
alias(water_retaining_gel, "water-retaining gel").
alias(water_retaining_gel, "water crystals").
alias(bedding_plant, "bedding plants").
alias(bedding_plant, "petunias").
alias(bracket, "wall bracket").
alias(drained, "water runs out of the bottom").
alias(peat_free_compost, "peat-free compost").
alias(peat, "peat").
alias(holds_water, "so the soil holds water longer").
```

## teach
- To plant a hanging basket you need the basket, a liner, water-retaining gel, potting soil, five bedding plants, a watering can, water and a wall bracket.
- Planting a hanging basket takes 20 to 30 minutes.
- Step 1 of planting a hanging basket: put the liner into the basket.
- Step 2 of planting a hanging basket: mix a tablespoon of water-retaining gel into the potting soil so it holds water longer.
- Step 3 of planting a hanging basket: fill the basket with the potting soil.
- Step 4 of planting a hanging basket: set the five plants into the basket, for 10 to 15 minutes.
- Step 5 of planting a hanging basket: water the plants with the watering can until water runs out of the bottom.
- Step 6 of planting a hanging basket: hang the basket on the bracket.

## checklist
- names what is needed: a basket, a liner, gel, potting soil, plants, water and a bracket
- gives the steps in order: liner, mix gel, fill, plant, water, hang
- gives the amount of gel and the number of plants
- gives a total time of about 20 to 30 minutes

## variant
- change: no synthetic gel wanted: leave out the gel step
- request: I'd rather not put those water crystals in my basket - they're plastic, aren't they? How do I plant it without them?
- situation: I have the basket, a liner, potting soil, five plants, a watering can, water and a bracket, and I don't want to use water-retaining gel.
- expect: The reply leaves out mixing in the gel and keeps the other steps, perhaps saying to water more often.

```datalog situation
excludes(user, water_retaining_gel).
has(user, hanging_basket).
has(user, basket_liner).
has(user, potting_soil).
has(user, bedding_plant).
has(user, watering_can).
has(user, water).
has(user, bracket).
```

```datalog difference
no_need(water_retaining_gel).
drop(plant_hanging_basket_s2).
```

## variant
- change: no peat: use peat-free compost in place of the potting soil
- request: I only buy peat-free these days. Does that change how I make up my hanging basket?
- situation: I have the basket, a liner, gel, peat-free compost, five plants, a watering can, water and a bracket, and I avoid peat.
- expect: The reply mixes the gel into peat-free compost and fills the basket with it instead of ordinary potting soil, keeping the other steps.

```datalog situation
excludes(user, peat).
has(user, peat_free_compost).
has(user, hanging_basket).
has(user, basket_liner).
has(user, water_retaining_gel).
has(user, bedding_plant).
has(user, watering_can).
has(user, water).
has(user, bracket).
```

```datalog difference
no_need(potting_soil).
extra_need(peat_free_compost).
change(plant_hanging_basket_s2, into, potting_soil, peat_free_compost).
swap(plant_hanging_basket_s3, potting_soil, peat_free_compost).
```
