---
id: prepare_car_for_road_trip
stage: social
task: prepare_car_for_road_trip
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
needs(prepare_car_for_road_trip, pressure_gauge).
needs(prepare_car_for_road_trip, washer_fluid).
needs(prepare_car_for_road_trip, emergency_kit).
needs(prepare_car_for_road_trip, phone).
takes_time(prepare_car_for_road_trip, 40, 55, minute).
step(prepare_car_for_road_trip, prepare_car_for_road_trip_s1).
action(prepare_car_for_road_trip_s1, check).
object_of(prepare_car_for_road_trip_s1, tyre).
uses(prepare_car_for_road_trip_s1, pressure_gauge).
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
takes_time(prepare_car_for_road_trip_s5, 15, 20, minute).
before(prepare_car_for_road_trip_s1, prepare_car_for_road_trip_s2).
before(prepare_car_for_road_trip_s2, prepare_car_for_road_trip_s3).
before(prepare_car_for_road_trip_s3, prepare_car_for_road_trip_s4).
before(prepare_car_for_road_trip_s4, prepare_car_for_road_trip_s5).
```

## teach
- To prepare a car for a road trip you need a tyre pressure gauge, washer fluid, an emergency kit and a phone.
- Preparing a car for a road trip takes 40 to 55 minutes.
- Step 1 of preparing a car for a road trip: check the tyre pressure with the gauge, including the spare tyre, for 5 to 10 minutes.
- Step 2 of preparing a car for a road trip: check the oil level on the dipstick for 3 to 5 minutes.
- Step 3 of preparing a car for a road trip: top up the windscreen washer fluid for 3 to 5 minutes.
- Step 4 of preparing a car for a road trip: pack the emergency kit with a torch, a first aid kit and a warning triangle, for 10 to 15 minutes.
- Step 5 of preparing a car for a road trip: plan the route and the stops on your phone and download offline maps for 15 to 20 minutes.

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
