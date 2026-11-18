---
id: water_plant
stage: household
task: water_plant
schema: v3
---
# Water a plant

## requests
- I just got my first house plant and I'm scared of killing it. How do I water it?
- How should I water a potted plant so I don't drown it?
- My plant looks a bit droopy. What's the right way to give it water?

## situation
- I have a potted plant on the windowsill, a watering can and tap water.
- It only needs to take a few minutes.

## ground truth
```datalog
task(water_plant).
domain(water_plant, gardening).
goal(water_plant, watered_plant).
needs(water_plant, watering_can).
needs(water_plant, water).
takes_time(water_plant, 3, 5, minute).
step(water_plant, water_plant_s1).
action(water_plant_s1, check).
object_of(water_plant_s1, soil).
purpose(water_plant_s1, dryness_known).
step(water_plant, water_plant_s2).
action(water_plant_s2, fill).
object_of(water_plant_s2, watering_can).
uses(water_plant_s2, water).
role(water_plant_s2, water, fill).
takes_time(water_plant_s2, 1, 1, minute).
step(water_plant, water_plant_s3).
action(water_plant_s3, pour).
object_of(water_plant_s3, water).
onto(water_plant_s3, soil).
uses(water_plant_s3, watering_can).
role(water_plant_s3, watering_can, pour).
setting(water_plant_s3, speed, slow, level).
until(water_plant_s3, drain).
takes_time(water_plant_s3, 1, 2, minute).
step(water_plant, water_plant_s4).
action(water_plant_s4, empty).
object_of(water_plant_s4, saucer).
purpose(water_plant_s4, no_standing_water).
before(water_plant_s1, water_plant_s2).
before(water_plant_s2, water_plant_s3).
before(water_plant_s3, water_plant_s4).
```

## aliases
```datalog
task_alias(water_plant, "watering a plant").
task_alias(water_plant, "water a potted plant").
task_alias(water_plant, "give it water").
alias(plant, "house plant").
alias(plant, "potted plant").
alias(water, "tap water").
alias(drain, "drains out of the bottom").
alias(slow, "slowly").
alias(saucer, "saucer under the pot").
alias(dryness_known, "see if it is dry").
alias(no_standing_water, "standing in water").
alias(jug, "kitchen jug").
```

## teach
- To water a plant you need a watering can and water.
- Watering a plant takes 3 to 5 minutes.
- Step 1 of watering a plant: check the soil with your finger to see if it is dry.
- Step 2 of watering a plant: fill the watering can with water for 1 minute.
- Step 3 of watering a plant: pour the water slowly onto the soil with the watering can until it drains out of the bottom, for 1 to 2 minutes.
- Step 4 of watering a plant: empty the saucer under the pot so the plant is not left standing in water.

## checklist
- says to check the soil is dry before watering
- gives the steps in order: check soil, fill can, pour until it drains, empty saucer
- warns against leaving the plant standing in water
- closes politely

## variant
- change: no watering can: use a jug or a large glass
- request: I want to water my plant but I don't own a watering can. Can I still do it?
- situation: I don't have a watering can, but I have a kitchen jug and tap water.
- expect: The reply fills a jug instead of a watering can and pours slowly so the water does not splash out.

```datalog situation
lacks(user, watering_can).
has(user, jug).
has(user, water).
```

```datalog difference
change(water_plant_s2, object_of, watering_can, jug).
swap(water_plant_s3, watering_can, jug).
no_need(watering_can).
extra_need(jug).
```
