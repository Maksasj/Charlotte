---
id: prepare_car_for_road_trip
stage: social
task: prepare_car_for_road_trip
schema: v3
---
# Prepare a car for a road trip

## requests
- We're driving to the coast next weekend. What should I check on the car before we go?
- Can you help me get my car ready for a long drive?
- How do I prepare for a road trip so we don't break down halfway?

## situation
- I have a tyre pressure gauge, washer fluid, an emergency kit and my phone.
- We leave on Saturday morning.

## ground truth
```datalog
task(prepare_car_for_road_trip).
domain(prepare_car_for_road_trip, travel).
goal(prepare_car_for_road_trip, car_ready).
needs(prepare_car_for_road_trip, pressure_gauge).
needs(prepare_car_for_road_trip, washer_fluid).
needs(prepare_car_for_road_trip, emergency_kit).
needs(prepare_car_for_road_trip, phone).
takes_time(prepare_car_for_road_trip, 40, 55, minute).
step(prepare_car_for_road_trip, prepare_car_for_road_trip_s1).
action(prepare_car_for_road_trip_s1, check).
object_of(prepare_car_for_road_trip_s1, tyre).
uses(prepare_car_for_road_trip_s1, pressure_gauge).
role(prepare_car_for_road_trip_s1, pressure_gauge, measure).
takes_time(prepare_car_for_road_trip_s1, 5, 10, minute).
step(prepare_car_for_road_trip, prepare_car_for_road_trip_s2).
action(prepare_car_for_road_trip_s2, check).
object_of(prepare_car_for_road_trip_s2, oil).
takes_time(prepare_car_for_road_trip_s2, 3, 5, minute).
step(prepare_car_for_road_trip, prepare_car_for_road_trip_s3).
action(prepare_car_for_road_trip_s3, fill).
object_of(prepare_car_for_road_trip_s3, washer_fluid).
takes_time(prepare_car_for_road_trip_s3, 3, 5, minute).
step(prepare_car_for_road_trip, prepare_car_for_road_trip_s4).
action(prepare_car_for_road_trip_s4, pack).
object_of(prepare_car_for_road_trip_s4, emergency_kit).
takes_time(prepare_car_for_road_trip_s4, 10, 15, minute).
step(prepare_car_for_road_trip, prepare_car_for_road_trip_s5).
action(prepare_car_for_road_trip_s5, plan).
object_of(prepare_car_for_road_trip_s5, route).
uses(prepare_car_for_road_trip_s5, phone).
role(prepare_car_for_road_trip_s5, phone, navigate).
step(prepare_car_for_road_trip, prepare_car_for_road_trip_s6).
action(prepare_car_for_road_trip_s6, download).
object_of(prepare_car_for_road_trip_s6, offline_map).
uses(prepare_car_for_road_trip_s6, phone).
role(prepare_car_for_road_trip_s6, phone, navigate).
takes_time(prepare_car_for_road_trip_s6, 15, 20, minute).
before(prepare_car_for_road_trip_s1, prepare_car_for_road_trip_s2).
before(prepare_car_for_road_trip_s2, prepare_car_for_road_trip_s3).
before(prepare_car_for_road_trip_s3, prepare_car_for_road_trip_s4).
before(prepare_car_for_road_trip_s4, prepare_car_for_road_trip_s5).
before(prepare_car_for_road_trip_s5, prepare_car_for_road_trip_s6).
```

## aliases
```datalog
task_alias(prepare_car_for_road_trip, "preparing a car for a road trip").
task_alias(prepare_car_for_road_trip, "prepare a car for a road trip").
task_alias(prepare_car_for_road_trip, "get my car ready for a long drive").
task_alias(prepare_car_for_road_trip, "prepare for a road trip").
alias(tyre, "tyre pressure").
alias(tyre, "spare tyre").
alias(pressure_gauge, "tyre pressure gauge").
alias(pressure_gauge, "gauge").
alias(oil, "oil level").
alias(washer_fluid, "windscreen washer fluid").
alias(fill, "top up").
alias(route, "the route and the stops").
alias(offline_map, "offline maps").
alias(electric_car, "electric car").
alias(charging_stop, "charging stops").
alias(full_charge, "charges the car fully").
```

## teach
- To prepare a car for a road trip you need a tyre pressure gauge, washer fluid, an emergency kit and a phone.
- Preparing a car for a road trip takes 40 to 55 minutes.
- Step 1 of preparing a car for a road trip: check the tyre pressure with the gauge, including the spare tyre, for 5 to 10 minutes.
- Step 2 of preparing a car for a road trip: check the oil level on the dipstick for 3 to 5 minutes.
- Step 3 of preparing a car for a road trip: top up the windscreen washer fluid for 3 to 5 minutes.
- Step 4 of preparing a car for a road trip: pack the emergency kit with a torch, a first aid kit and a warning triangle, for 10 to 15 minutes.
- Step 5 of preparing a car for a road trip: plan the route and the stops on your phone.
- Step 6 of preparing a car for a road trip: download offline maps for 15 to 20 minutes.

## checklist
- names what is needed: a tyre pressure gauge, washer fluid, an emergency kit and a phone
- gives the steps in order: check tyre, check oil, fill washer fluid, pack emergency kit, plan route
- gives a total time of about 40 to 55 minutes
- closes politely

## variant
- change: electric car: check the charge and plan charging stops instead of checking the oil
- request: Our car is electric. Is getting it ready for a road trip any different?
- situation: We have an electric car, a tyre pressure gauge, washer fluid, an emergency kit and a phone.
- expect: The reply drops the oil check, charges the car fully before leaving and plans the route around charging stops, keeping the tyre, washer fluid and emergency kit steps.

```datalog situation
has(user, electric_car).
has(user, pressure_gauge).
has(user, washer_fluid).
has(user, emergency_kit).
has(user, phone).
```

```datalog difference
drop(prepare_car_for_road_trip_s2).
add_step(prepare_car_for_road_trip_s9, prepare_car_for_road_trip_s1).
step(prepare_car_for_road_trip, prepare_car_for_road_trip_s9).
action(prepare_car_for_road_trip_s9, charge).
object_of(prepare_car_for_road_trip_s9, electric_car).
until(prepare_car_for_road_trip_s9, full_charge).
before(prepare_car_for_road_trip_s1, prepare_car_for_road_trip_s9).
before(prepare_car_for_road_trip_s9, prepare_car_for_road_trip_s3).
add_step(prepare_car_for_road_trip_s10, prepare_car_for_road_trip_s5).
step(prepare_car_for_road_trip, prepare_car_for_road_trip_s10).
action(prepare_car_for_road_trip_s10, plan).
object_of(prepare_car_for_road_trip_s10, charging_stop).
uses(prepare_car_for_road_trip_s10, phone).
role(prepare_car_for_road_trip_s10, phone, navigate).
before(prepare_car_for_road_trip_s5, prepare_car_for_road_trip_s10).
before(prepare_car_for_road_trip_s10, prepare_car_for_road_trip_s6).
```
