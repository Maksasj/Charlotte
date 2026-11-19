---
id: rebook_missed_connection
stage: procedures
task: rebook_missed_connection
schema: v3
---
# Rebook a missed connection

## requests
- My connection got missed. How do I rebook the next flight?
- I missed my connecting flight. What's the process to get on another one?
- Could you walk me through rebooking after a missed connection?

## situation
- I missed my connecting flight and I have my phone, my booking code and a card.
- I'm at the airport and need a seat on a later flight today.

## ground truth
```datalog
task(rebook_missed_connection).
domain(rebook_missed_connection, travel).
goal(rebook_missed_connection, rebooked_flight).
needs(rebook_missed_connection, phone).
needs(rebook_missed_connection, booking_reference).
needs(rebook_missed_connection, card).
takes_time(rebook_missed_connection, 20, 40, minute).
step(rebook_missed_connection, rebook_missed_connection_s1).
action(rebook_missed_connection_s1, look_up).
object_of(rebook_missed_connection_s1, later_flight).
uses(rebook_missed_connection_s1, phone).
role(rebook_missed_connection_s1, phone, hold).
purpose(rebook_missed_connection_s1, seat_available).
step(rebook_missed_connection, rebook_missed_connection_s2).
action(rebook_missed_connection_s2, request).
object_of(rebook_missed_connection_s2, new_flight).
uses(rebook_missed_connection_s2, phone).
role(rebook_missed_connection_s2, phone, hold).
takes_time(rebook_missed_connection_s2, 15, 30, minute).
step(rebook_missed_connection, rebook_missed_connection_s3).
action(rebook_missed_connection_s3, pay).
object_of(rebook_missed_connection_s3, fare_difference).
uses(rebook_missed_connection_s3, card).
role(rebook_missed_connection_s3, card, pay).
condition(rebook_missed_connection_s3, fare_difference_due).
step(rebook_missed_connection, rebook_missed_connection_s4).
action(rebook_missed_connection_s4, save).
object_of(rebook_missed_connection_s4, confirmation).
uses(rebook_missed_connection_s4, phone).
role(rebook_missed_connection_s4, phone, contain).
check(rebook_missed_connection_s4, confirmation_received).
before(rebook_missed_connection_s1, rebook_missed_connection_s2).
before(rebook_missed_connection_s2, rebook_missed_connection_s3).
before(rebook_missed_connection_s3, rebook_missed_connection_s4).
```

## aliases
```datalog
task_alias(rebook_missed_connection, "rebook a missed connection").
task_alias(rebook_missed_connection, "rebook a missed flight").
task_alias(rebook_missed_connection, "missed connection").
alias(later_flight, "next flight").
alias(later_flight, "later flight").
alias(booking_reference, "booking code").
alias(booking_reference, "confirmation code").
alias(card, "credit card").
alias(fare_difference, "price difference").
alias(confirmation, "confirmation email").
alias(confirmation_received, "confirmation arrives").
alias(seat_available, "seat is free").
```

## teach
- To rebook a missed connection you need a phone, your booking reference and a card.
- Rebooking a missed connection takes 20 to 40 minutes.
- Step 1 of rebook a missed connection: look up the later flights on your phone so you can see which seats are free.
- Step 2 of rebook a missed connection: request the new flight from the airline on your phone.
- Step 3 of rebook a missed connection: pay the fare difference with your card, if one is due.
- Step 4 of rebook a missed connection: save the confirmation on your phone, checking that the confirmation arrives.

## checklist
- names what is needed, including a phone, booking code and card
- gives the steps in order: look up later flights, request the new flight, pay any fare difference, save the confirmation
- says the fare payment only applies if a difference is due
- gives a total time of about 20 to 40 minutes

## variant
- change: the phone battery is dead, so do it on a laptop at the airline's website instead
- request: My phone is dead. I missed my connection, can I still rebook from my laptop?
- situation: I missed my connection, my phone battery is dead, I have a laptop, my booking code and a card.
- expect: The reply has you look up later flights and request the new flight on your laptop at the airline's website, pay any fare difference with your card, and save the confirmation on the laptop.

```datalog situation
has(user, laptop).
lacks(user, phone).
has(user, booking_reference).
has(user, card).
```

```datalog difference
swap(rebook_missed_connection_s1, phone, laptop).
swap(rebook_missed_connection_s2, phone, laptop).
swap(rebook_missed_connection_s4, phone, laptop).
extra_need(laptop).
no_need(phone).
```

## variant
- change: add a request for an aisle seat on the new flight after paying the fare difference
- request: I missed my connection and I'd like an aisle seat on the new flight. What do I do?
- situation: I missed my connection and want an aisle seat, and I have my phone, booking code and card.
- expect: The reply gives the usual steps and adds a step after paying the fare difference where you request an aisle seat on the new flight by phone, before saving the confirmation.

```datalog situation
has(user, phone).
has(user, booking_reference).
has(user, card).
wants(user, aisle_seat).
```

```datalog difference
add_step(rebook_missed_connection_s9, rebook_missed_connection_s3).
step(rebook_missed_connection, rebook_missed_connection_s9).
action(rebook_missed_connection_s9, request).
object_of(rebook_missed_connection_s9, aisle_seat).
uses(rebook_missed_connection_s9, phone).
role(rebook_missed_connection_s9, phone, hold).
before(rebook_missed_connection_s3, rebook_missed_connection_s9).
before(rebook_missed_connection_s9, rebook_missed_connection_s4).
```
