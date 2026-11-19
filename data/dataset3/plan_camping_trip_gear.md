---
id: plan_camping_trip_gear
stage: procedures
task: plan_camping_trip_gear
schema: v3
---
# Plan camping trip gear

## requests
- What's the best way to get my gear ready for a camping trip?
- Can you give me the steps to pack for a weekend camping trip?
- I'm heading out camping on Saturday. What should I do to get everything packed and ready?

## situation
- I have a backpack, a sleeping bag, a lantern, a cooler, food, a sponge, a cooking pot and a car.
- I have about 45 minutes before we leave.

## ground truth
```datalog
task(plan_camping_trip_gear).
domain(plan_camping_trip_gear, planning).
goal(plan_camping_trip_gear, camping_ready).
needs(plan_camping_trip_gear, backpack).
needs(plan_camping_trip_gear, sleeping_bag).
needs(plan_camping_trip_gear, lantern).
needs(plan_camping_trip_gear, cooler).
needs(plan_camping_trip_gear, food).
needs(plan_camping_trip_gear, sponge).
needs(plan_camping_trip_gear, cooking_pot).
takes_time(plan_camping_trip_gear, 30, 45, minute).
step(plan_camping_trip_gear, plan_camping_trip_gear_s1).
action(plan_camping_trip_gear_s1, check).
object_of(plan_camping_trip_gear_s1, weather_forecast).
purpose(plan_camping_trip_gear_s1, pack_for_weather).
step(plan_camping_trip_gear, plan_camping_trip_gear_s2).
action(plan_camping_trip_gear_s2, pack).
object_of(plan_camping_trip_gear_s2, sleeping_bag).
uses(plan_camping_trip_gear_s2, backpack).
role(plan_camping_trip_gear_s2, backpack, contain).
into(plan_camping_trip_gear_s2, backpack).
step(plan_camping_trip_gear, plan_camping_trip_gear_s3).
action(plan_camping_trip_gear_s3, charge).
object_of(plan_camping_trip_gear_s3, lantern).
step(plan_camping_trip_gear, plan_camping_trip_gear_s4).
action(plan_camping_trip_gear_s4, pack).
object_of(plan_camping_trip_gear_s4, food).
uses(plan_camping_trip_gear_s4, cooler).
role(plan_camping_trip_gear_s4, cooler, contain).
into(plan_camping_trip_gear_s4, cooler).
step(plan_camping_trip_gear, plan_camping_trip_gear_s5).
action(plan_camping_trip_gear_s5, clean).
object_of(plan_camping_trip_gear_s5, cooking_pot).
uses(plan_camping_trip_gear_s5, sponge).
role(plan_camping_trip_gear_s5, sponge, scrub).
step(plan_camping_trip_gear, plan_camping_trip_gear_s6).
action(plan_camping_trip_gear_s6, load).
object_of(plan_camping_trip_gear_s6, gear).
into(plan_camping_trip_gear_s6, car).
check(plan_camping_trip_gear_s6, gear_in_car).
overlaps(plan_camping_trip_gear_s3, plan_camping_trip_gear_s1).
before(plan_camping_trip_gear_s1, plan_camping_trip_gear_s2).
before(plan_camping_trip_gear_s2, plan_camping_trip_gear_s4).
before(plan_camping_trip_gear_s4, plan_camping_trip_gear_s6).
before(plan_camping_trip_gear_s5, plan_camping_trip_gear_s6).
```

## aliases
```datalog
task_alias(plan_camping_trip_gear, "camping gear list").
task_alias(plan_camping_trip_gear, "packing for camping").
alias(weather_forecast, "forecast").
alias(cooking_pot, "pot").
alias(sponge, "scrubbing sponge").
alias(gear, "camping gear").
alias(camping_ready, "ready for camping").
alias(scrub, "scrub").
alias(pack_for_weather, "pack for the weather").
```

## teach
- To plan camping trip gear you need a backpack, a sleeping bag, a lantern, a cooler, food, a sponge and a cooking pot.
- Planning camping trip gear takes 30 to 45 minutes.
- Step 1 of plan camping trip gear: check the weather forecast so you know what to pack.
- Step 2 of plan camping trip gear: pack the sleeping bag into the backpack.
- While step 1 runs, step 3 of plan camping trip gear: charge the lantern.
- Step 4 of plan camping trip gear: pack the food into the cooler.
- Step 5 of plan camping trip gear: clean the cooking pot with the sponge.
- Step 6 of plan camping trip gear: load the gear into the car and check that everything is in it.

## checklist
- names what is needed, including a backpack, a sleeping bag, a lantern, a cooler, food, a sponge and a cooking pot
- gives the steps in order: check the forecast, pack the sleeping bag, pack the food, clean the pot, load the car
- says charging the lantern can be done while the forecast is checked
- gives a total time of about 30 to 45 minutes

## variant
- change: the lantern is already charged, so skip charging it, and there are only 15 minutes
- request: I'm leaving in 15 minutes and the lantern is already charged. What do I need to do?
- situation: I have everything, the lantern is already charged, and I have 15 minutes.
- expect: The reply skips charging the lantern and gives the packing steps for the sleeping bag, food, pot and car, finishing within about 10 to 15 minutes.

```datalog situation
has(user, backpack).
has(user, sleeping_bag).
has(user, lantern).
has(user, charged_lantern).
has(user, cooler).
has(user, food).
has(user, sponge).
has(user, cooking_pot).
time_limit(user, 15, minute).
```

```datalog difference
drop(plan_camping_trip_gear_s3).
change(plan_camping_trip_gear, takes_time, 30_45, 10_15).
```

- change: the cooler is missing, so pack the food into a tote bag instead
- request: I don't own a cooler but I have a tote bag. How should I pack the food for the trip?
- situation: I have everything except a cooler, and I have a tote bag.
- expect: The reply gives the same steps as the base plan, packing the food into the tote bag in place of the cooler.

```datalog situation
has(user, backpack).
has(user, sleeping_bag).
has(user, lantern).
lacks(user, cooler).
has(user, tote_bag).
has(user, food).
has(user, sponge).
has(user, cooking_pot).
time_limit(user, 45, minute).
```

```datalog difference
swap(plan_camping_trip_gear_s4, cooler, tote_bag).
no_need(cooler).
extra_need(tote_bag).
```
