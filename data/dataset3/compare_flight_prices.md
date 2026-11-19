---
id: compare_flight_prices
stage: procedures
task: compare_flight_prices
schema: v3
---
# Compare flight prices

## requests
- I want to find the cheapest flight for a trip next month. How do I compare prices?
- What's the best way to shop around for airfares?
- Can you take me through checking several flights so I get the lowest fare?

## situation
- I have a laptop and a good internet connection.
- I want to fly next month and I'm flexible on the exact dates.

## ground truth
```datalog
task(compare_flight_prices).
domain(compare_flight_prices, travel).
goal(compare_flight_prices, cheapest_flight).
needs(compare_flight_prices, laptop).
needs(compare_flight_prices, internet_connection).
takes_time(compare_flight_prices, 13, 23, minute).
step(compare_flight_prices, compare_flight_prices_s1).
action(compare_flight_prices_s1, enter).
object_of(compare_flight_prices_s1, route_and_dates).
takes_time(compare_flight_prices_s1, 2, 3, minute).
step(compare_flight_prices, compare_flight_prices_s2).
action(compare_flight_prices_s2, search).
object_of(compare_flight_prices_s2, flights).
takes_time(compare_flight_prices_s2, 1, 2, minute).
step(compare_flight_prices, compare_flight_prices_s3).
action(compare_flight_prices_s3, note).
object_of(compare_flight_prices_s3, fares).
purpose(compare_flight_prices_s3, spot_best_price).
warning(compare_flight_prices_s3, prices_change_quickly).
takes_time(compare_flight_prices_s3, 2, 3, minute).
step(compare_flight_prices, compare_flight_prices_s4).
action(compare_flight_prices_s4, review).
object_of(compare_flight_prices_s4, fare_rules).
takes_time(compare_flight_prices_s4, 3, 5, minute).
step(compare_flight_prices, compare_flight_prices_s5).
action(compare_flight_prices_s5, book).
object_of(compare_flight_prices_s5, fare).
check(compare_flight_prices_s5, booking_confirmed).
takes_time(compare_flight_prices_s5, 5, 10, minute).
before(compare_flight_prices_s1, compare_flight_prices_s2).
before(compare_flight_prices_s2, compare_flight_prices_s3).
before(compare_flight_prices_s3, compare_flight_prices_s4).
before(compare_flight_prices_s4, compare_flight_prices_s5).
```

## aliases
```datalog
task_alias(compare_flight_prices, "comparing flight prices").
task_alias(compare_flight_prices, "finding cheap flights").
task_alias(compare_flight_prices, "shopping around for airfares").
alias(route_and_dates, "route and travel dates").
alias(flights, "flight options").
alias(fares, "ticket prices").
alias(fare_rules, "bag and change fees").
alias(fare, "cheapest fare").
alias(internet_connection, "internet access").
alias(booking_confirmed, "booking is confirmed").
alias(spot_best_price, "find the best price").
alias(prices_change_quickly, "prices change fast").
```

## teach
- To compare flight prices you need a laptop and an internet connection.
- Comparing flight prices takes 13 to 23 minutes.
- Step 1 of comparing flight prices: enter the route and travel dates on a flight search site, for 2 to 3 minutes.
- Step 2 of comparing flight prices: search for flights on those dates, for 1 to 2 minutes.
- Step 3 of comparing flight prices: note the ticket prices of the cheapest options so you can find the best price, since prices change quickly, for 2 to 3 minutes.
- Step 4 of comparing flight prices: review the bag and change fees for those fares, for 3 to 5 minutes.
- Step 5 of comparing flight prices: book the cheapest fare that includes your bag, for 5 to 10 minutes, until the booking is confirmed.

## checklist
- names what is needed: a laptop and an internet connection
- gives the steps in order: enter route and dates, search, note fares, review bag and change fees, book
- warns that prices change quickly, so the fares should be noted during the search
- gives a total time of about 13 to 23 minutes

## variant
- change: only 15 minutes: review the fees briefly and book a little faster
- request: I only have about 15 minutes to find a cheap flight. Can you tell me the quick version?
- situation: I have a laptop, an internet connection and 15 minutes.
- expect: The reply gives the same five steps in order, with a brief look at the bag fees and a faster booking, for a total of about 9 to 15 minutes.

```datalog situation
has(user, laptop).
has(user, internet_connection).
time_limit(user, 15, minute).
```

```datalog difference
change(compare_flight_prices_s4, takes_time, 3_5, 1_2).
change(compare_flight_prices_s5, takes_time, 5_10, 3_5).
change(compare_flight_prices, takes_time, 13_23, 9_15).
```

- change: no internet at home, so the comparison cannot be done as written
- request: I don't have internet at home. Can I still compare flight prices?
- situation: I have a laptop but no internet connection.
- expect: The reply says the comparison cannot be done without internet access and suggests using a library or a mobile connection instead.

```datalog situation
has(user, laptop).
lacks(user, internet_connection).
```

```datalog difference
cannot(no_internet_connection).
```
