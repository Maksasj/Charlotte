---
id: water_garden
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Water the garden

Check the soil, then water the beds with the hose and the pots with a can.

## ground truth
```datalog
task(water_garden).
domain(water_garden, gardening).
goal(water_garden, watered_garden).
needs(water_garden, hose).
needs(water_garden, watering_can).
needs(water_garden, water).
takes_time(water_garden, 15, 35, minute).
step(water_garden, water_garden_s1).
action(water_garden_s1, check).
object_of(water_garden_s1, soil).
takes_time(water_garden_s1, 2, 3, minute).
step(water_garden, water_garden_s2).
action(water_garden_s2, connect).
object_of(water_garden_s2, hose).
takes_time(water_garden_s2, 1, 2, minute).
step(water_garden, water_garden_s3).
action(water_garden_s3, water).
object_of(water_garden_s3, flower_bed).
uses(water_garden_s3, hose).
role(water_garden_s3, hose, pour).
takes_time(water_garden_s3, 10, 20, minute).
until(water_garden_s3, wet).
step(water_garden, water_garden_s4).
action(water_garden_s4, fill).
object_of(water_garden_s4, watering_can).
uses(water_garden_s4, water).
role(water_garden_s4, water, fill).
takes_time(water_garden_s4, 1, 1, minute).
step(water_garden, water_garden_s5).
action(water_garden_s5, water).
object_of(water_garden_s5, pot_plant).
uses(water_garden_s5, watering_can).
role(water_garden_s5, watering_can, pour).
takes_time(water_garden_s5, 2, 5, minute).
until(water_garden_s5, wet).
before(water_garden_s1, water_garden_s2).
before(water_garden_s2, water_garden_s3).
before(water_garden_s3, water_garden_s4).
before(water_garden_s4, water_garden_s5).
```

## aliases
```datalog
task_alias(water_garden, "watering the garden").
task_alias(water_garden, "water the plants").
task_alias(water_garden, "water the beds").
alias(flower_bed, "beds").
alias(flower_bed, "flower beds").
alias(pot_plant, "pots").
alias(pot_plant, "potted plants").
alias(watering_can, "watering can").
alias(watering_can, "can").
alias(hose, "garden hose").
alias(wet, "moist").
```
