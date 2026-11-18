---
id: prepare_for_long_flight
stage: social
task: prepare_for_long_flight
schema: v3
tier: T
---
# Prepare for a long-haul flight

## requests
- I've got a 13-hour flight to Tokyo next week. How do I make it bearable?
- What should I do to prepare for a long-haul flight?
- Any tips for getting ready for an overnight flight so I arrive in decent shape?

## situation
- I have a phone, a neck pillow, a phone charger, compression socks and an empty water bottle.

## ground truth
```datalog
task(prepare_for_long_flight).
domain(prepare_for_long_flight, travel).
goal(prepare_for_long_flight, comfortable_flight).
needs(prepare_for_long_flight, phone).
needs(prepare_for_long_flight, neck_pillow).
needs(prepare_for_long_flight, charger).
needs(prepare_for_long_flight, compression_sock).
needs(prepare_for_long_flight, water_bottle).
takes_time(prepare_for_long_flight, 1, 2, hour).
step(prepare_for_long_flight, prepare_for_long_flight_s1).
action(prepare_for_long_flight_s1, download).
object_of(prepare_for_long_flight_s1, film).
uses(prepare_for_long_flight_s1, phone).
role(prepare_for_long_flight_s1, phone, access).
step(prepare_for_long_flight, prepare_for_long_flight_s2).
action(prepare_for_long_flight_s2, charge).
object_of(prepare_for_long_flight_s2, phone).
uses(prepare_for_long_flight_s2, charger).
role(prepare_for_long_flight_s2, charger, power).
step(prepare_for_long_flight, prepare_for_long_flight_s3).
action(prepare_for_long_flight_s3, pack).
object_of(prepare_for_long_flight_s3, neck_pillow).
step(prepare_for_long_flight, prepare_for_long_flight_s4).
action(prepare_for_long_flight_s4, wear).
object_of(prepare_for_long_flight_s4, compression_sock).
purpose(prepare_for_long_flight_s4, no_swelling).
step(prepare_for_long_flight, prepare_for_long_flight_s5).
action(prepare_for_long_flight_s5, fill).
object_of(prepare_for_long_flight_s5, water_bottle).
condition(prepare_for_long_flight_s5, after_security).
step(prepare_for_long_flight, prepare_for_long_flight_s6).
action(prepare_for_long_flight_s6, walk).
object_of(prepare_for_long_flight_s6, aisle).
condition(prepare_for_long_flight_s6, every_hour).
purpose(prepare_for_long_flight_s6, good_circulation).
before(prepare_for_long_flight_s1, prepare_for_long_flight_s2).
before(prepare_for_long_flight_s2, prepare_for_long_flight_s3).
before(prepare_for_long_flight_s3, prepare_for_long_flight_s4).
before(prepare_for_long_flight_s4, prepare_for_long_flight_s5).
before(prepare_for_long_flight_s5, prepare_for_long_flight_s6).
```

## aliases
```datalog
task_alias(prepare_for_long_flight, "a long-haul flight").
task_alias(prepare_for_long_flight, "a long flight").
task_alias(prepare_for_long_flight, "an overnight flight").
alias(film, "films").
alias(neck_pillow, "travel pillow").
alias(compression_sock, "compression socks").
alias(aisle, "the aisle").
alias(no_swelling, "stop your legs swelling").
alias(every_hour, "once an hour").
```

## teach
- To prepare for a long-haul flight you need a phone, a neck pillow, a charger, compression socks and a water bottle.
- Preparing for a long-haul flight takes 1 to 2 hours.
- Step 1 of preparing for a long-haul flight: download films on your phone.
- Step 2 of preparing for a long-haul flight: charge the phone with the charger.
- Step 3 of preparing for a long-haul flight: pack the neck pillow.
- Step 4 of preparing for a long-haul flight: wear compression socks to stop your legs swelling.
- Step 5 of preparing for a long-haul flight: fill the water bottle after security.
- Step 6 of preparing for a long-haul flight: walk the aisle once an hour for good circulation.

## checklist
- names what is needed, including a neck pillow, compression socks and a water bottle
- gives the steps in order: download films, charge, pack pillow, socks, fill bottle, walk the aisle
- mentions walking once an hour

## variant
- change: no compression socks: walk more often
- request: I didn't get compression socks in time. Does it matter much for my long flight?
- situation: I have a phone, a neck pillow, a charger and a water bottle.
- expect: The reply skips the compression socks and suggests walking and stretching more often.

```datalog situation
has(user, phone).
has(user, neck_pillow).
has(user, charger).
has(user, water_bottle).
lacks(user, compression_sock).
```

```datalog difference
drop(prepare_for_long_flight_s4).
no_need(compression_sock).
```

## variant
- change: wants to sleep: pack an eye mask and earplugs
- request: It's an overnight flight and I really want to sleep. What should I add to my prep?
- situation: I have everything for the flight plus an eye mask and earplugs.
- expect: The reply adds packing the eye mask and earplugs with the neck pillow.

```datalog situation
has(user, phone).
has(user, neck_pillow).
has(user, charger).
has(user, compression_sock).
has(user, water_bottle).
has(user, eye_mask).
has(user, earplug).
wants(user, sleep).
```

```datalog difference
extra_need(eye_mask).
extra_need(earplug).
add_step(prepare_for_long_flight_s9, prepare_for_long_flight_s3).
step(prepare_for_long_flight, prepare_for_long_flight_s9).
action(prepare_for_long_flight_s9, pack).
object_of(prepare_for_long_flight_s9, eye_mask).
before(prepare_for_long_flight_s3, prepare_for_long_flight_s9).
before(prepare_for_long_flight_s9, prepare_for_long_flight_s4).
```
