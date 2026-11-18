---
id: get_to_airport
stage: social
task: get_to_airport
schema: v3
tier: T
---
# Get to the airport on time

## requests
- My flight's at 7am. How do I make sure I get to the airport in time?
- What's the best way to plan the trip to the airport for an international flight?
- I always end up rushing for flights. Give me a plan for getting there calmly.

## situation
- I have my phone, a printer, my passport and an online check-in done.

## ground truth
```datalog
task(get_to_airport).
domain(get_to_airport, travel).
goal(get_to_airport, reach_airport).
goal(get_to_airport, catch_flight).
needs(get_to_airport, phone).
needs(get_to_airport, printer).
needs(get_to_airport, boarding_pass).
needs(get_to_airport, passport).
takes_time(get_to_airport, 3, 4, hour).
step(get_to_airport, get_to_airport_s1).
action(get_to_airport_s1, check).
object_of(get_to_airport_s1, flight_time).
uses(get_to_airport_s1, phone).
role(get_to_airport_s1, phone, inform).
step(get_to_airport, get_to_airport_s2).
action(get_to_airport_s2, plan).
object_of(get_to_airport_s2, route).
uses(get_to_airport_s2, phone).
role(get_to_airport_s2, phone, navigate).
step(get_to_airport, get_to_airport_s3).
action(get_to_airport_s3, book).
object_of(get_to_airport_s3, taxi).
uses(get_to_airport_s3, phone).
role(get_to_airport_s3, phone, access).
step(get_to_airport, get_to_airport_s4).
action(get_to_airport_s4, print).
object_of(get_to_airport_s4, boarding_pass).
uses(get_to_airport_s4, printer).
role(get_to_airport_s4, printer, write).
step(get_to_airport, get_to_airport_s5).
action(get_to_airport_s5, pack).
object_of(get_to_airport_s5, passport).
check(get_to_airport_s5, passport_valid).
step(get_to_airport, get_to_airport_s6).
action(get_to_airport_s6, leave).
object_of(get_to_airport_s6, home).
setting(get_to_airport_s6, lead_time, 3, hour).
purpose(get_to_airport_s6, arrive_early).
before(get_to_airport_s1, get_to_airport_s2).
before(get_to_airport_s2, get_to_airport_s3).
before(get_to_airport_s3, get_to_airport_s4).
before(get_to_airport_s4, get_to_airport_s5).
before(get_to_airport_s5, get_to_airport_s6).
```

## aliases
```datalog
task_alias(get_to_airport, "getting to the airport").
task_alias(get_to_airport, "making my flight").
task_alias(get_to_airport, "the airport journey").
alias(flight_time, "the flight time").
alias(boarding_pass, "boarding pass").
alias(route, "the route").
alias(leave, "set off").
alias(arrive_early, "arrive with time for security").
```

## teach
- To get to the airport on time you need a phone, a printer, the boarding pass and your passport.
- Getting to the airport on time takes 3 to 4 hours including the journey.
- Step 1 of getting to the airport: check the flight time on your phone.
- Step 2 of getting to the airport: plan the route with your phone.
- Step 3 of getting to the airport: book a taxi on your phone.
- Step 4 of getting to the airport: print the boarding pass.
- Step 5 of getting to the airport: pack your passport and check it is valid.
- Step 6 of getting to the airport: leave home 3 hours before the flight so you arrive with time for security.

## checklist
- names what is needed: a phone, a boarding pass and a passport
- gives the steps in order: check flight time, plan route, book taxi, print boarding pass, pack passport, leave
- says to leave about 3 hours before an international flight

## variant
- change: flight in 90 minutes, carry-on only: skip printing and leave now
- request: I've just realised my flight is in an hour and a half! How do I still make it?
- situation: I have my phone and my passport, hand luggage only, and 90 minutes.
- expect: The reply skips printing, uses the phone boarding pass, books a taxi straight away and goes directly to security.

```datalog situation
has(user, phone).
has(user, passport).
time_limit(user, 90, minute).
```

```datalog difference
drop(get_to_airport_s2).
drop(get_to_airport_s4).
no_need(printer).
change(get_to_airport_s6, setting, lead_time_3_hour, lead_time_0_hour).
change(get_to_airport, takes_time, 3_4_hour, 60_90_minute).
```

## variant
- change: printer broken: download the boarding pass to the phone
- request: My printer just died and I need my boarding pass for tomorrow. What do I do?
- situation: I have my phone, a broken printer and my passport.
- expect: The reply downloads the boarding pass to the phone's wallet instead of printing it.

```datalog situation
has(user, phone).
has(user, passport).
lacks(user, printer).
```

```datalog difference
drop(get_to_airport_s4).
no_need(printer).
add_step(get_to_airport_s9, get_to_airport_s3).
step(get_to_airport, get_to_airport_s9).
action(get_to_airport_s9, download).
object_of(get_to_airport_s9, boarding_pass).
uses(get_to_airport_s9, phone).
role(get_to_airport_s9, phone, access).
before(get_to_airport_s3, get_to_airport_s9).
before(get_to_airport_s9, get_to_airport_s5).
```
