---
id: book_airport_parking
stage: procedures
task: book_airport_parking
schema: v3
---
# Book airport parking

## requests
- Can you help me reserve parking at the airport for a trip next week?
- What's the easiest way to pre-book long-term parking near the terminal?
- I need to pay for airport parking ahead of time. How do I do that?

## situation
- I have a laptop, a credit card and internet at home.
- I'm flying out next week and need my car parked for the whole trip.

## ground truth
```datalog
task(book_airport_parking).
domain(book_airport_parking, travel).
goal(book_airport_parking, parking_booked).
needs(book_airport_parking, laptop).
needs(book_airport_parking, credit_card).
needs(book_airport_parking, internet).
takes_time(book_airport_parking, 15, 25, minute).
step(book_airport_parking, book_airport_parking_s1).
action(book_airport_parking_s1, search).
object_of(book_airport_parking_s1, parking_options).
uses(book_airport_parking_s1, laptop).
role(book_airport_parking_s1, laptop, hold).
takes_time(book_airport_parking_s1, 5, 10, minute).
step(book_airport_parking, book_airport_parking_s2).
action(book_airport_parking_s2, choose).
object_of(book_airport_parking_s2, parking_lot).
takes_time(book_airport_parking_s2, 3, 5, minute).
step(book_airport_parking, book_airport_parking_s3).
action(book_airport_parking_s3, pay).
object_of(book_airport_parking_s3, booking).
uses(book_airport_parking_s3, credit_card).
role(book_airport_parking_s3, credit_card, pay).
takes_time(book_airport_parking_s3, 3, 5, minute).
step(book_airport_parking, book_airport_parking_s4).
action(book_airport_parking_s4, save).
object_of(book_airport_parking_s4, confirmation).
uses(book_airport_parking_s4, laptop).
role(book_airport_parking_s4, laptop, hold).
purpose(book_airport_parking_s4, proof_of_booking).
before(book_airport_parking_s1, book_airport_parking_s2).
before(book_airport_parking_s2, book_airport_parking_s3).
before(book_airport_parking_s3, book_airport_parking_s4).
```

## aliases
```datalog
task_alias(book_airport_parking, "airport parking booking").
task_alias(book_airport_parking, "reserve airport parking").
alias(laptop, "computer").
alias(credit_card, "card").
alias(parking_options, "parking deals").
alias(parking_lot, "car park").
alias(confirmation, "confirmation email").
alias(proof_of_booking, "proof of booking").
alias(phone, "mobile phone").
```

## teach
- To book airport parking you need a laptop, a credit card and internet.
- Booking airport parking takes 15 to 25 minutes.
- Step 1 of booking airport parking: search the parking options for your flight dates on the laptop, which takes 5 to 10 minutes.
- Step 2 of booking airport parking: choose a parking lot with a shuttle, which takes 3 to 5 minutes.
- Step 3 of booking airport parking: pay for the booking with your credit card, which takes 3 to 5 minutes.
- Step 4 of booking airport parking: save the confirmation email on the laptop as proof of booking.

## checklist
- names what is needed, including a laptop, a credit card and internet
- gives the steps in order: search options, choose a lot, pay, save the confirmation
- gives a total time of about 15 to 25 minutes
- says to keep the confirmation as proof of the booking

## variant
- change: no laptop, so book on the phone instead
- request: I need to book airport parking, but I don't have my laptop, only my phone.
- situation: I have a mobile phone and a credit card, but no laptop.
- expect: The reply uses the phone in place of the laptop for the search and for saving the confirmation, and keeps the same lot choice and card payment.

```datalog situation
lacks(user, laptop).
has(user, phone).
has(user, credit_card).
has(user, internet).
```

```datalog difference
swap(book_airport_parking_s1, laptop, phone).
swap(book_airport_parking_s4, laptop, phone).
no_need(laptop).
extra_need(phone).
```

## variant
- change: only 10 minutes, so skip comparing lots and take the first search result
- request: I'm short on time. What's the fastest way to book airport parking?
- situation: I have a laptop, a credit card and internet, and 10 minutes.
- expect: The reply drops the lot comparison, books the first parking option from the search in about 8 to 10 minutes, pays by card and saves the confirmation.

```datalog situation
has(user, laptop).
has(user, credit_card).
has(user, internet).
time_limit(user, 10, minute).
```

```datalog difference
drop(book_airport_parking_s2).
change(book_airport_parking, takes_time, 15_25, 8_10).
```
