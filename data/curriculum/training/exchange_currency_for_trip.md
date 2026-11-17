---
id: exchange_currency_for_trip
stage: social
task: exchange_currency_for_trip
---
# Get foreign currency for a trip

## requests
- I'm going to Japan next month. How do I get some yen before I leave?
- Can you tell me how to sort out foreign cash for a holiday?
- What's the best way to change money before travelling abroad?

## situation
- I have my phone, a payment card, my passport and a money belt.
- I leave in ten days.

## ground truth
```datalog
task(exchange_currency_for_trip).
needs(exchange_currency_for_trip, phone).
needs(exchange_currency_for_trip, payment_card).
needs(exchange_currency_for_trip, passport).
needs(exchange_currency_for_trip, money_belt).
takes_time(exchange_currency_for_trip, 35, 50, minute).
step(exchange_currency_for_trip, exchange_currency_for_trip_s1).
action(exchange_currency_for_trip_s1, compare).
object_of(exchange_currency_for_trip_s1, rate).
uses(exchange_currency_for_trip_s1, phone).
takes_time(exchange_currency_for_trip_s1, 5, 10, minute).
step(exchange_currency_for_trip, exchange_currency_for_trip_s2).
action(exchange_currency_for_trip_s2, decide).
object_of(exchange_currency_for_trip_s2, amount).
takes_time(exchange_currency_for_trip_s2, 5, 5, minute).
step(exchange_currency_for_trip, exchange_currency_for_trip_s3).
action(exchange_currency_for_trip_s3, order).
object_of(exchange_currency_for_trip_s3, currency).
uses(exchange_currency_for_trip_s3, payment_card).
takes_time(exchange_currency_for_trip_s3, 10, 15, minute).
step(exchange_currency_for_trip, exchange_currency_for_trip_s4).
action(exchange_currency_for_trip_s4, collect).
object_of(exchange_currency_for_trip_s4, currency).
uses(exchange_currency_for_trip_s4, passport).
takes_time(exchange_currency_for_trip_s4, 15, 20, minute).
step(exchange_currency_for_trip, exchange_currency_for_trip_s5).
action(exchange_currency_for_trip_s5, split).
object_of(exchange_currency_for_trip_s5, currency).
uses(exchange_currency_for_trip_s5, money_belt).
takes_time(exchange_currency_for_trip_s5, 2, 3, minute).
before(exchange_currency_for_trip_s1, exchange_currency_for_trip_s2).
before(exchange_currency_for_trip_s2, exchange_currency_for_trip_s3).
before(exchange_currency_for_trip_s3, exchange_currency_for_trip_s4).
before(exchange_currency_for_trip_s4, exchange_currency_for_trip_s5).
```

## teach
- To get foreign currency for a trip you need a phone, a payment card, a passport and a money belt.
- Getting foreign currency for a trip takes 35 to 50 minutes.
- Step 1 of getting foreign currency for a trip: compare exchange rates on your phone for 5 to 10 minutes.
- Step 2 of getting foreign currency for a trip: decide the amount of cash you need for the first few days for 5 minutes.
- Step 3 of getting foreign currency for a trip: order the currency online for pickup and pay with the payment card for 10 to 15 minutes.
- Step 4 of getting foreign currency for a trip: collect the currency at the counter and show your passport for 15 to 20 minutes.
- Step 5 of getting foreign currency for a trip: split the cash between your wallet and the money belt for 2 to 3 minutes.

## checklist
- names what is needed: a phone, a payment card, a passport and a money belt
- gives the steps in order: compare rate, decide amount, order currency, collect currency, split currency
- gives a total time of about 35 to 50 minutes
- closes politely
