---
id: water_plant
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# water plant

Learned in conversation.

```datalog
task(water_plant). @conf 1 @source ""
needs(water_plant, watering_can). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
needs(water_plant, water). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
takes_time(water_plant, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
step(water_plant, water_plant_s1). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
action(water_plant_s1, check). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
object_of(water_plant_s1, soil). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
uses(water_plant_s1, finger). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
step(water_plant, water_plant_s3). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
before(water_plant_s1, water_plant_s3). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
step(water_plant, water_plant_s4). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
action(water_plant_s4, empty). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
object_of(water_plant_s4, saucer). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
before(water_plant_s3, water_plant_s4). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
step(water_plant, water_plant_s2). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
action(water_plant_s2, fill). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
object_of(water_plant_s2, watering_can). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
uses(water_plant_s2, water). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
takes_time(water_plant_s2, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
before(water_plant_s1, water_plant_s2). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
action(water_plant_s3, pour). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
object_of(water_plant_s3, water). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
uses(water_plant_s3, watering_can). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
takes_time(water_plant_s3, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
until(water_plant_s3, drain). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
before(water_plant_s2, water_plant_s3). @conf 1 @source teacher:curriculum @conv s1795002265246-760026-57
```

## Taught wording

- water_plant_s2: fill the watering can with water
- task: water a plant
- water_plant_s1: check the soil with your finger to see if it is dry
- water_plant_s3: pour the water slowly onto the soil with the watering can until it drains out of the bottom
- water_plant_s4: empty the saucer under the pot
