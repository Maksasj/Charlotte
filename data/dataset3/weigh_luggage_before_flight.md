---
id: weigh_luggage_before_flight
stage: procedures
task: weigh_luggage_before_flight
schema: v3
---
# Weigh luggage before a flight

## requests
- Can you tell me how to check how heavy my suitcase is before I fly, using a luggage scale?
- What's the way to weigh a bag before going to the airport?
- I need to make sure my suitcase is under the airline weight limit. How do I weigh it first?

## situation
- I have a suitcase and a luggage scale at home.
- My flight is tomorrow morning.

## ground truth
```datalog
task(weigh_luggage_before_flight).
domain(weigh_luggage_before_flight, travel).
goal(weigh_luggage_before_flight, luggage_within_limit).
needs(weigh_luggage_before_flight, suitcase).
needs(weigh_luggage_before_flight, luggage_scale).
takes_time(weigh_luggage_before_flight, 5, 10, minute).
step(weigh_luggage_before_flight, weigh_luggage_before_flight_s1).
action(weigh_luggage_before_flight_s1, place).
object_of(weigh_luggage_before_flight_s1, suitcase).
uses(weigh_luggage_before_flight_s1, luggage_scale).
role(weigh_luggage_before_flight_s1, luggage_scale, hold).
step(weigh_luggage_before_flight, weigh_luggage_before_flight_s2).
action(weigh_luggage_before_flight_s2, read).
object_of(weigh_luggage_before_flight_s2, weight).
uses(weigh_luggage_before_flight_s2, luggage_scale).
role(weigh_luggage_before_flight_s2, luggage_scale, measure).
step(weigh_luggage_before_flight, weigh_luggage_before_flight_s3).
action(weigh_luggage_before_flight_s3, compare).
object_of(weigh_luggage_before_flight_s3, weight).
check(weigh_luggage_before_flight_s3, under_limit).
step(weigh_luggage_before_flight, weigh_luggage_before_flight_s4).
action(weigh_luggage_before_flight_s4, move).
object_of(weigh_luggage_before_flight_s4, item).
from(weigh_luggage_before_flight_s4, suitcase).
condition(weigh_luggage_before_flight_s4, over_limit).
before(weigh_luggage_before_flight_s1, weigh_luggage_before_flight_s2).
before(weigh_luggage_before_flight_s2, weigh_luggage_before_flight_s3).
before(weigh_luggage_before_flight_s3, weigh_luggage_before_flight_s4).
```

## aliases
```datalog
task_alias(weigh_luggage_before_flight, "weigh my bag before a flight").
task_alias(weigh_luggage_before_flight, "check luggage weight").
alias(luggage_scale, "bag scale").
alias(suitcase, "bag").
alias(under_limit, "under the weight limit").
alias(over_limit, "over the limit").
alias(luggage_within_limit, "within the limit").
```

## teach
- To weigh luggage before a flight you need a suitcase and a luggage scale.
- Weighing luggage before a flight takes 5 to 10 minutes.
- Step 1 of weighing luggage before a flight: place the suitcase on the luggage scale.
- Step 2 of weighing luggage before a flight: read the weight on the display.
- Step 3 of weighing luggage before a flight: compare the weight with the airline limit.
- Step 4 of weighing luggage before a flight: if it is over the limit, move some items out of the suitcase.

## checklist
- names what is needed, including a suitcase and a luggage scale
- gives the steps in order: place the suitcase on the scale, read the weight, compare it with the limit, move items out if over
- says that moving items out of the suitcase is only needed when it is over the limit
- gives a total time of about 5 to 10 minutes

## variant
- change: no luggage scale at home, so the weighing cannot be done; the answer says so
- request: I don't have a luggage scale at home. Can I still check my suitcase weight before the flight?
- situation: I have a suitcase but no luggage scale, and my flight is tomorrow morning.
- expect: The reply says it cannot be done at home without a luggage scale and suggests weighing the suitcase at the airport check-in desk.

```datalog situation
lacks(user, luggage_scale).
has(user, suitcase).
```

```datalog difference
cannot(no_scale).
```

## variant
- change: two bags to check: the second bag is also weighed on the luggage scale after the first, so the total time is longer
- request: I have two bags to check in tomorrow. How do I weigh both of them before the flight?
- situation: I have a suitcase, a second bag, a luggage scale, and about 15 minutes before I leave.
- expect: The reply weighs the first suitcase as before, then weighs the second bag on the luggage scale, and gives a total of about 10 to 15 minutes.

```datalog situation
has(user, suitcase).
has(user, luggage_scale).
has(user, second_bag).
count(user, bag, 2).
```

```datalog difference
add_step(weigh_luggage_before_flight_s9, weigh_luggage_before_flight_s4).
step(weigh_luggage_before_flight, weigh_luggage_before_flight_s9).
action(weigh_luggage_before_flight_s9, place).
object_of(weigh_luggage_before_flight_s9, second_bag).
uses(weigh_luggage_before_flight_s9, luggage_scale).
role(weigh_luggage_before_flight_s9, luggage_scale, hold).
before(weigh_luggage_before_flight_s4, weigh_luggage_before_flight_s9).
extra_need(second_bag).
change(weigh_luggage_before_flight, takes_time, 5_10, 10_15).
```
