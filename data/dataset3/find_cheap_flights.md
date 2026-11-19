---
id: find_cheap_flights
stage: procedures
task: find_cheap_flights
schema: v3
---
# Find cheap flights

## requests
- I need to fly next month and want the lowest fare. How do I find cheap flights?
- What's the best way to hunt down a bargain flight and book it?
- Can you help me track down cheap plane tickets? I want to get the price down without much fuss.

## situation
- I have a laptop, an internet connection and a credit card.
- I want to book a flight and have about half an hour to spare.

## ground truth
```datalog
task(find_cheap_flights).
domain(find_cheap_flights, travel).
goal(find_cheap_flights, cheap_flight).
needs(find_cheap_flights, laptop).
needs(find_cheap_flights, internet).
needs(find_cheap_flights, credit_card).
takes_time(find_cheap_flights, 20, 30, minute).
step(find_cheap_flights, find_cheap_flights_s1).
action(find_cheap_flights_s1, search).
object_of(find_cheap_flights_s1, flights).
uses(find_cheap_flights_s1, laptop).
role(find_cheap_flights_s1, laptop, hold).
takes_time(find_cheap_flights_s1, 10, 15, minute).
warning(find_cheap_flights_s1, prices_change_hourly).
step(find_cheap_flights, find_cheap_flights_s2).
action(find_cheap_flights_s2, compare).
object_of(find_cheap_flights_s2, fares).
purpose(find_cheap_flights_s2, best_price).
step(find_cheap_flights, find_cheap_flights_s3).
action(find_cheap_flights_s3, check).
object_of(find_cheap_flights_s3, nearby_airports).
optional(find_cheap_flights_s3).
overlaps(find_cheap_flights_s3, find_cheap_flights_s2).
step(find_cheap_flights, find_cheap_flights_s4).
action(find_cheap_flights_s4, book).
object_of(find_cheap_flights_s4, cheapest_fare).
purpose(find_cheap_flights_s4, seat_secured).
check(find_cheap_flights_s4, booking_confirmed).
before(find_cheap_flights_s1, find_cheap_flights_s2).
before(find_cheap_flights_s2, find_cheap_flights_s4).
before(find_cheap_flights_s3, find_cheap_flights_s4).
```

## aliases
```datalog
task_alias(find_cheap_flights, "cheap flights").
task_alias(find_cheap_flights, "cheapest flight").
task_alias(find_cheap_flights, "bargain flight").
alias(laptop, "computer").
alias(credit_card, "card").
alias(flights, "flight options").
alias(fares, "prices").
alias(cheapest_fare, "lowest fare").
alias(nearby_airports, "nearby airports").
alias(best_price, "lowest price").
alias(prices_change_hourly, "prices change hourly").
```

## teach
- To find cheap flights you need a laptop, internet and a credit card.
- Finding cheap flights takes 20 to 30 minutes.
- Step 1 of finding cheap flights: search the flights on the laptop for 10 to 15 minutes, since prices change hourly.
- Step 2 of finding cheap flights: compare the fares to get the lowest price.
- While step 2 runs, step 3 of finding cheap flights: check nearby airports.
- Step 4 of finding cheap flights: book the cheapest fare to secure the seat, then check that the booking is confirmed.

## checklist
- names what is needed, including a laptop, internet and a credit card
- gives the steps in order: search flights, compare fares, book the cheapest fare
- says checking nearby airports can be done while the fares are compared
- gives a total time of about 20 to 30 minutes
- warns that prices change hourly, so the search should not drag on

## variant
- change: the laptop is broken, so the search is done on a phone instead
- request: My laptop has died. Can I still find cheap flights on my phone?
- situation: I have a phone, internet and a credit card, but no laptop.
- expect: The reply runs the same search, compare and book steps on the phone in place of the laptop, with the same time and card booking.

```datalog situation
lacks(user, laptop).
has(user, phone).
has(user, internet).
has(user, credit_card).
```

```datalog difference
swap(find_cheap_flights_s1, laptop, phone).
no_need(laptop).
extra_need(phone).
```

## variant
- change: only ten minutes, so the nearby airports check is dropped and the total time shortened
- request: I've only got ten minutes before I need to book. Is there a quick way to get cheap flights?
- situation: I have a laptop, internet and a credit card, and only 10 minutes.
- expect: The reply skips checking nearby airports and gives the search, compare and book steps in about 10 to 15 minutes.

```datalog situation
has(user, laptop).
has(user, internet).
has(user, credit_card).
time_limit(user, 10, minute).
```

```datalog difference
drop(find_cheap_flights_s3).
change(find_cheap_flights, takes_time, 20_30, 10_15).
```
