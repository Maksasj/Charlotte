---
id: plan_road_trip_route
stage: procedures
task: plan_road_trip_route
schema: v3
---
# Plan a road trip route

## requests
- I'm going on a road trip next week. How should I work out the route and get ready?
- What's the best way to plan a road trip route before we set off?
- Can you go through planning our road trip, including the route and checking the car?

## situation
- I have a paper map, a phone, a tyre gauge and a cooler.
- We leave in a few hours.

## ground truth
```datalog
task(plan_road_trip_route).
domain(plan_road_trip_route, planning).
goal(plan_road_trip_route, planned_route).
needs(plan_road_trip_route, map).
needs(plan_road_trip_route, phone).
needs(plan_road_trip_route, tyre_gauge).
needs(plan_road_trip_route, cooler).
takes_time(plan_road_trip_route, 30, 40, minute).
step(plan_road_trip_route, plan_road_trip_route_s1).
action(plan_road_trip_route_s1, plan).
object_of(plan_road_trip_route_s1, route).
uses(plan_road_trip_route_s1, map).
role(plan_road_trip_route_s1, map, measure).
takes_time(plan_road_trip_route_s1, 15, 20, minute).
step(plan_road_trip_route, plan_road_trip_route_s2).
action(plan_road_trip_route_s2, check).
object_of(plan_road_trip_route_s2, tyre).
uses(plan_road_trip_route_s2, tyre_gauge).
role(plan_road_trip_route_s2, tyre_gauge, measure).
takes_time(plan_road_trip_route_s2, 10, 10, minute).
overlaps(plan_road_trip_route_s2, plan_road_trip_route_s1).
step(plan_road_trip_route, plan_road_trip_route_s3).
action(plan_road_trip_route_s3, pack).
object_of(plan_road_trip_route_s3, cooler).
uses(plan_road_trip_route_s3, cooler).
role(plan_road_trip_route_s3, cooler, contain).
takes_time(plan_road_trip_route_s3, 15, 20, minute).
overlaps(plan_road_trip_route_s3, plan_road_trip_route_s1).
step(plan_road_trip_route, plan_road_trip_route_s4).
action(plan_road_trip_route_s4, save).
object_of(plan_road_trip_route_s4, route).
uses(plan_road_trip_route_s4, phone).
role(plan_road_trip_route_s4, phone, hold).
takes_time(plan_road_trip_route_s4, 2, 3, minute).
before(plan_road_trip_route_s1, plan_road_trip_route_s4).
```

## aliases
```datalog
task_alias(plan_road_trip_route, "road trip route").
task_alias(plan_road_trip_route, "planning our road trip").
alias(map, "paper map").
alias(tyre_gauge, "tyre pressure gauge").
alias(cooler, "cool box").
alias(route, "driving route").
alias(tyre, "tyres").
alias(planned_route, "planned route").
```

## teach
- To plan a road trip route you need a map, a phone, a tyre gauge and a cooler.
- Planning a road trip route takes 30 to 40 minutes.
- Step 1 of planning a road trip route: plan the route on the map for 15 to 20 minutes.
- While step 1 runs, step 2 of planning a road trip route: check the tyres with the tyre gauge for 10 minutes.
- While step 1 runs, step 3 of planning a road trip route: pack the cooler for 15 to 20 minutes.
- Step 4 of planning a road trip route: save the route to the phone for 2 to 3 minutes.

## checklist
- names what is needed: a map, a phone, a tyre gauge and a cooler
- gives the steps in order: plan the route, check the tyres, pack the cooler, save the route to the phone
- says checking the tyres and packing the cooler can be done while the route is planned
- gives a total time of about 30 to 40 minutes

## variant
- change: no phone: plan the route on the paper map only and skip saving it to a phone
- request: We're going on a road trip, but I don't have a phone. How do I plan the route?
- situation: I have a paper map, a tyre gauge and a cooler, but no phone.
- expect: The reply plans the route on the paper map, checks the tyres, packs the cooler, and leaves out saving the route to a phone because the user has none.

```datalog situation
has(user, map).
lacks(user, phone).
has(user, tyre_gauge).
has(user, cooler).
```

```datalog difference
drop(plan_road_trip_route_s4).
no_need(phone).
```

## variant
- change: only 20 minutes: skip the tyre check because the tyre gauge is missing, and finish planning in about 15 to 20 minutes
- request: I've only got 20 minutes and I don't have a tyre gauge. What do I need to do to plan the trip?
- situation: I have a paper map, a phone and a cooler, no tyre gauge, and 20 minutes.
- expect: The reply plans the route on the map, packs the cooler, saves the route to the phone, and drops the tyre check, all within 20 minutes.

```datalog situation
has(user, map).
has(user, phone).
lacks(user, tyre_gauge).
has(user, cooler).
time_limit(user, 20, minute).
```

```datalog difference
drop(plan_road_trip_route_s2).
no_need(tyre_gauge).
change(plan_road_trip_route, takes_time, 30_40, 15_20).
```
