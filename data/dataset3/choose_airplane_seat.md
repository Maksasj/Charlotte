---
id: choose_airplane_seat
stage: procedures
task: choose_airplane_seat
schema: v3
---
# Choose an airplane seat

## requests
- I'm flying next week and want to pick my seat ahead of time. How do I do that online?
- What's the way to reserve a window seat on a flight before I get to the airport?
- Can you tell me how to select my seat on an airplane?

## situation
- I have my phone and I have an airline booking for a flight next week.
- I'd like to pick my seat before I get to the airport.

## ground truth
```datalog
task(choose_airplane_seat).
domain(choose_airplane_seat, travel).
goal(choose_airplane_seat, seat_booked).
needs(choose_airplane_seat, phone).
takes_time(choose_airplane_seat, 5, 10, minute).
step(choose_airplane_seat, choose_airplane_seat_s1).
action(choose_airplane_seat_s1, open).
object_of(choose_airplane_seat_s1, airline_app).
uses(choose_airplane_seat_s1, phone).
role(choose_airplane_seat_s1, phone, hold).
step(choose_airplane_seat, choose_airplane_seat_s2).
action(choose_airplane_seat_s2, view).
object_of(choose_airplane_seat_s2, seat_map).
step(choose_airplane_seat, choose_airplane_seat_s3).
action(choose_airplane_seat_s3, select).
object_of(choose_airplane_seat_s3, seat).
step(choose_airplane_seat, choose_airplane_seat_s4).
action(choose_airplane_seat_s4, confirm).
object_of(choose_airplane_seat_s4, seat).
before(choose_airplane_seat_s1, choose_airplane_seat_s2).
before(choose_airplane_seat_s2, choose_airplane_seat_s3).
before(choose_airplane_seat_s3, choose_airplane_seat_s4).
```

## aliases
```datalog
task_alias(choose_airplane_seat, "pick a plane seat").
task_alias(choose_airplane_seat, "reserve an airplane seat").
alias(airline_app, "airline app").
alias(seat_map, "seat map").
alias(phone, "mobile phone").
alias(seat_booked, "seat booked").
```

## teach
- To choose an airplane seat you need a phone.
- Choosing an airplane seat takes 5 to 10 minutes.
- Step 1 of choosing an airplane seat: open the airline app on the phone.
- Step 2 of choosing an airplane seat: view the seat map.
- Step 3 of choosing an airplane seat: select a seat.
- Step 4 of choosing an airplane seat: confirm the seat.

## checklist
- names what is needed, including a phone
- gives the steps in order: open the airline app, view the seat map, select a seat, confirm the seat
- gives a total time of about 5 to 10 minutes

## variant
- change: no phone, only a laptop: open the airline site on the laptop instead of the app
- request: I don't have my phone with me, only a laptop. Can I still choose my seat?
- situation: I have a laptop and my booking, but no phone.
- expect: The reply has the user open the airline website on the laptop, view the seat map, select a seat and confirm it.

```datalog situation
has(user, laptop).
lacks(user, phone).
```

```datalog difference
swap(choose_airplane_seat_s1, phone, laptop).
extra_need(laptop).
no_need(phone).
```

## variant
- change: travelling with a child, so add a step after confirming to request seats next to each other
- request: I'm travelling with my young child and we need to sit together. How do I choose our seats?
- situation: I have my phone, and two of us need seats next to each other.
- expect: The reply gives the four steps and adds a fifth step, after confirming, to request two adjacent seats, taking 2 to 3 minutes.

```datalog situation
has(user, phone).
wants(user, adjacent_seat).
count(user, traveller, 2).
```

```datalog difference
add_step(choose_airplane_seat_s9, choose_airplane_seat_s4).
step(choose_airplane_seat, choose_airplane_seat_s9).
action(choose_airplane_seat_s9, request).
object_of(choose_airplane_seat_s9, adjacent_seat).
takes_time(choose_airplane_seat_s9, 2, 3, minute).
before(choose_airplane_seat_s4, choose_airplane_seat_s9).
```
