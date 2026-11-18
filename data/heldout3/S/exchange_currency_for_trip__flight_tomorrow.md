---
id: exchange_currency_for_trip__flight_tomorrow
stage: social
task: exchange_currency_for_trip
schema: v3
tier: S
base: exchange_currency_for_trip
---
# Get foreign currency for a trip

## requests
- I fly to Japan tomorrow morning and completely forgot to get yen. What's the fastest way?

## ground truth
```datalog
task(exchange_currency_for_trip).
domain(exchange_currency_for_trip, travel).
goal(exchange_currency_for_trip, foreign_cash).
needs(exchange_currency_for_trip, phone).
needs(exchange_currency_for_trip, payment_card).
needs(exchange_currency_for_trip, passport).
needs(exchange_currency_for_trip, money_belt).
takes_time(exchange_currency_for_trip, 35, 50, minute).
step(exchange_currency_for_trip, exchange_currency_for_trip_s1).
action(exchange_currency_for_trip_s1, compare).
object_of(exchange_currency_for_trip_s1, rate).
uses(exchange_currency_for_trip_s1, phone).
role(exchange_currency_for_trip_s1, phone, inform).
takes_time(exchange_currency_for_trip_s1, 5, 10, minute).
step(exchange_currency_for_trip, exchange_currency_for_trip_s2).
action(exchange_currency_for_trip_s2, decide).
object_of(exchange_currency_for_trip_s2, amount).
purpose(exchange_currency_for_trip_s2, cover_first_days).
takes_time(exchange_currency_for_trip_s2, 5, 5, minute).
step(exchange_currency_for_trip, exchange_currency_for_trip_s3).
action(exchange_currency_for_trip_s3, order).
object_of(exchange_currency_for_trip_s3, currency).
step(exchange_currency_for_trip, exchange_currency_for_trip_s4).
action(exchange_currency_for_trip_s4, pay).
object_of(exchange_currency_for_trip_s4, currency).
uses(exchange_currency_for_trip_s4, payment_card).
role(exchange_currency_for_trip_s4, payment_card, pay).
takes_time(exchange_currency_for_trip_s4, 10, 15, minute).
step(exchange_currency_for_trip, exchange_currency_for_trip_s5).
action(exchange_currency_for_trip_s5, collect).
object_of(exchange_currency_for_trip_s5, currency).
from(exchange_currency_for_trip_s5, counter).
takes_time(exchange_currency_for_trip_s5, 15, 20, minute).
step(exchange_currency_for_trip, exchange_currency_for_trip_s6).
action(exchange_currency_for_trip_s6, show).
object_of(exchange_currency_for_trip_s6, passport).
step(exchange_currency_for_trip, exchange_currency_for_trip_s7).
action(exchange_currency_for_trip_s7, split).
object_of(exchange_currency_for_trip_s7, currency).
uses(exchange_currency_for_trip_s7, money_belt).
role(exchange_currency_for_trip_s7, money_belt, contain).
into(exchange_currency_for_trip_s7, money_belt).
into(exchange_currency_for_trip_s7, wallet).
takes_time(exchange_currency_for_trip_s7, 2, 3, minute).
before(exchange_currency_for_trip_s1, exchange_currency_for_trip_s2).
before(exchange_currency_for_trip_s2, exchange_currency_for_trip_s3).
before(exchange_currency_for_trip_s3, exchange_currency_for_trip_s4).
before(exchange_currency_for_trip_s4, exchange_currency_for_trip_s5).
before(exchange_currency_for_trip_s5, exchange_currency_for_trip_s6).
before(exchange_currency_for_trip_s6, exchange_currency_for_trip_s7).
```

## aliases
```datalog
task_alias(exchange_currency_for_trip, "getting foreign currency for a trip").
task_alias(exchange_currency_for_trip, "get foreign currency").
task_alias(exchange_currency_for_trip, "sort out foreign cash").
task_alias(exchange_currency_for_trip, "change money before travelling").
alias(currency, "foreign cash").
alias(currency, "cash").
alias(currency, "yen").
alias(rate, "exchange rates").
alias(amount, "amount of cash").
alias(payment_card, "card").
alias(cover_first_days, "the first few days").
```

## teach
- To get foreign currency for a trip you need a phone, a payment card, a passport and a money belt.
- Getting foreign currency for a trip takes 35 to 50 minutes.
- Step 1 of getting foreign currency for a trip: compare exchange rates on your phone for 5 to 10 minutes.
- Step 2 of getting foreign currency for a trip: decide the amount of cash you need for the first few days for 5 minutes.
- Step 3 of getting foreign currency for a trip: order the currency online for pickup.
- Step 4 of getting foreign currency for a trip: pay with the payment card for 10 to 15 minutes.
- Step 5 of getting foreign currency for a trip: collect the currency at the counter for 15 to 20 minutes.
- Step 6 of getting foreign currency for a trip: show your passport.
- Step 7 of getting foreign currency for a trip: split the cash between your wallet and the money belt for 2 to 3 minutes.

## variant
- change: flight tomorrow: skip ordering ahead and buy over the counter at a bureau de change today
- request: I fly to Japan tomorrow morning and completely forgot to get yen. What's the fastest way?
- situation: I fly tomorrow. I have my phone, a payment card, my passport and a money belt, and a bureau de change in town.
- expect: The reply compares rates and decides the amount, skips ordering online for pickup, and buys the currency at a bureau de change today, then shows the passport and splits the cash.

```datalog situation
has(user, phone).
has(user, payment_card).
has(user, passport).
has(user, money_belt).
has(user, bureau_de_change).
time_limit(user, 1, day).
```

```datalog difference
drop(exchange_currency_for_trip_s3).
change(exchange_currency_for_trip_s5, from, counter, bureau_de_change).
```
