---
id: plan_day_trip
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Plan a day trip

Pick a place, check the weather, plan the route and pack.

## ground truth
```datalog
task(plan_day_trip).
domain(plan_day_trip, planning).
goal(plan_day_trip, day_trip).
needs(plan_day_trip, map).
needs(plan_day_trip, phone).
needs(plan_day_trip, backpack).
takes_time(plan_day_trip, 30, 50, minute).
step(plan_day_trip, plan_day_trip_s1).
action(plan_day_trip_s1, choose).
object_of(plan_day_trip_s1, destination).
uses(plan_day_trip_s1, map).
role(plan_day_trip_s1, map, inform).
takes_time(plan_day_trip_s1, 10, 15, minute).
step(plan_day_trip, plan_day_trip_s2).
action(plan_day_trip_s2, check).
object_of(plan_day_trip_s2, weather).
uses(plan_day_trip_s2, phone).
role(plan_day_trip_s2, phone, access).
takes_time(plan_day_trip_s2, 2, 5, minute).
step(plan_day_trip, plan_day_trip_s3).
action(plan_day_trip_s3, plan).
object_of(plan_day_trip_s3, route).
uses(plan_day_trip_s3, map).
role(plan_day_trip_s3, map, inform).
takes_time(plan_day_trip_s3, 10, 15, minute).
step(plan_day_trip, plan_day_trip_s4).
action(plan_day_trip_s4, pack).
object_of(plan_day_trip_s4, backpack).
takes_time(plan_day_trip_s4, 10, 15, minute).
step(plan_day_trip, plan_day_trip_s5).
action(plan_day_trip_s5, set).
object_of(plan_day_trip_s5, alarm).
uses(plan_day_trip_s5, phone).
role(plan_day_trip_s5, phone, access).
takes_time(plan_day_trip_s5, 1, 1, minute).
before(plan_day_trip_s1, plan_day_trip_s2).
before(plan_day_trip_s1, plan_day_trip_s3).
before(plan_day_trip_s2, plan_day_trip_s4).
before(plan_day_trip_s3, plan_day_trip_s4).
before(plan_day_trip_s4, plan_day_trip_s5).
```

## aliases
```datalog
task_alias(plan_day_trip, "planning a day trip").
task_alias(plan_day_trip, "plan a day out").
task_alias(plan_day_trip, "day trip").
alias(destination, "place").
alias(destination, "where to go").
alias(weather, "weather forecast").
alias(weather, "forecast").
alias(backpack, "bag").
alias(backpack, "rucksack").
alias(choose, "pick").
alias(alarm, "wake-up alarm").
```
