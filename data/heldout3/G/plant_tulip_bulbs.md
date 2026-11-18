---
id: plant_tulip_bulbs
stage: household
task: plant_tulip_bulbs
schema: v3
tier: G
---
# Plant tulip bulbs

## requests
- I bought a bag of tulip bulbs at the garden centre. How deep do they go and what do I do?
- What's the right way to get tulips in the ground in autumn?
- I want tulips coming up in my front bed next spring. How do I plant them?

## situation
- I have ten tulip bulbs, a trowel, a bag of compost, a watering can and water.

## ground truth
```datalog
task(plant_tulip_bulbs).
domain(plant_tulip_bulbs, gardening).
goal(plant_tulip_bulbs, planted_bulbs).
needs(plant_tulip_bulbs, tulip_bulb).
needs(plant_tulip_bulbs, trowel).
needs(plant_tulip_bulbs, compost).
needs(plant_tulip_bulbs, watering_can).
needs(plant_tulip_bulbs, water).
takes_time(plant_tulip_bulbs, 15, 20, minute).
step(plant_tulip_bulbs, plant_tulip_bulbs_s1).
action(plant_tulip_bulbs_s1, dig).
object_of(plant_tulip_bulbs_s1, hole).
uses(plant_tulip_bulbs_s1, trowel).
role(plant_tulip_bulbs_s1, trowel, lift).
setting(plant_tulip_bulbs_s1, depth, 15, centimetre).
step(plant_tulip_bulbs, plant_tulip_bulbs_s2).
action(plant_tulip_bulbs_s2, put).
object_of(plant_tulip_bulbs_s2, compost).
into(plant_tulip_bulbs_s2, hole).
uses(plant_tulip_bulbs_s2, compost).
role(plant_tulip_bulbs_s2, compost, fill).
step(plant_tulip_bulbs, plant_tulip_bulbs_s3).
action(plant_tulip_bulbs_s3, set).
object_of(plant_tulip_bulbs_s3, tulip_bulb).
into(plant_tulip_bulbs_s3, hole).
check(plant_tulip_bulbs_s3, pointy_end_up).
step(plant_tulip_bulbs, plant_tulip_bulbs_s4).
action(plant_tulip_bulbs_s4, fill).
object_of(plant_tulip_bulbs_s4, hole).
uses(plant_tulip_bulbs_s4, trowel).
role(plant_tulip_bulbs_s4, trowel, lift).
step(plant_tulip_bulbs, plant_tulip_bulbs_s5).
action(plant_tulip_bulbs_s5, water).
object_of(plant_tulip_bulbs_s5, tulip_bulb).
uses(plant_tulip_bulbs_s5, watering_can).
role(plant_tulip_bulbs_s5, watering_can, pour).
uses(plant_tulip_bulbs_s5, water).
role(plant_tulip_bulbs_s5, water, wet).
before(plant_tulip_bulbs_s1, plant_tulip_bulbs_s2).
before(plant_tulip_bulbs_s2, plant_tulip_bulbs_s3).
before(plant_tulip_bulbs_s3, plant_tulip_bulbs_s4).
before(plant_tulip_bulbs_s4, plant_tulip_bulbs_s5).
```

## aliases
```datalog
task_alias(plant_tulip_bulbs, "planting tulip bulbs").
task_alias(plant_tulip_bulbs, "get tulips in the ground").
task_alias(plant_tulip_bulbs, "plant tulips").
alias(tulip_bulb, "tulip bulbs").
alias(tulip_bulb, "bulbs").
alias(pointy_end_up, "pointy end facing up").
alias(spoon, "big kitchen spoon").
alias(spoon, "serving spoon").
alias(hole, "holes").
```

## teach
- To plant tulip bulbs you need the bulbs, a trowel, compost, a watering can and water.
- Planting ten tulip bulbs takes 15 to 20 minutes.
- Step 1 of planting tulip bulbs: dig a hole about 15 centimetres deep with the trowel.
- Step 2 of planting tulip bulbs: put a handful of compost into the hole.
- Step 3 of planting tulip bulbs: set the bulb in the hole with the pointy end up.
- Step 4 of planting tulip bulbs: fill the hole back in with the trowel.
- Step 5 of planting tulip bulbs: water the bulbs with the watering can.

## checklist
- names what is needed: tulip bulbs, a trowel, compost and water
- gives the steps in order: dig hole, compost, set bulb, fill in, water
- says about 15 centimetres deep and pointy end up
- gives a total time of about 15 to 20 minutes

## variant
- change: thirty bulbs: same steps, about three times as long
- request: I went a bit overboard and bought thirty tulip bulbs. How do I get them all planted and how long will it take?
- situation: I have thirty tulip bulbs, a trowel, compost, a watering can and water.
- expect: The reply repeats the same steps for each bulb and gives a total of about 45 minutes to an hour.

```datalog situation
has(user, tulip_bulb).
has(user, trowel).
has(user, compost).
has(user, watering_can).
has(user, water).
count(user, tulip_bulb, 30).
```

```datalog difference
change(plant_tulip_bulbs, takes_time, 15_20, 45_60).
```

## variant
- change: no trowel: dig and fill the holes with a big kitchen spoon
- request: Want to put some tulips in but I don't own a trowel. Is a big serving spoon any good for digging the holes?
- situation: I have tulip bulbs, compost, a watering can, water and a large kitchen spoon, but no trowel.
- expect: The reply digs and fills the holes with the spoon instead of a trowel and keeps the depth and other steps.

```datalog situation
lacks(user, trowel).
has(user, spoon).
has(user, tulip_bulb).
has(user, compost).
has(user, watering_can).
has(user, water).
```

```datalog difference
no_need(trowel).
extra_need(spoon).
swap(plant_tulip_bulbs_s1, trowel, spoon).
swap(plant_tulip_bulbs_s4, trowel, spoon).
```
