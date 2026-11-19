---
id: book_airport_transfer
stage: procedures
task: book_airport_transfer
schema: v3
---
# Book an airport transfer

## requests
- I land at the airport tomorrow and need a taxi to pick me up. How do I book an airport transfer?
- Can you walk me through reserving a shuttle from the airport for my arrival?
- What's the way to arrange a pickup from the airport so a driver is waiting for me?

## situation
- I have a laptop, a credit card and my phone.
- My flight lands tomorrow afternoon and I have a few free evenings to organise this.

## ground truth
```datalog
task(book_airport_transfer).
domain(book_airport_transfer, travel).
goal(book_airport_transfer, arrive_by_transfer).
needs(book_airport_transfer, laptop).
needs(book_airport_transfer, credit_card).
needs(book_airport_transfer, phone).
takes_time(book_airport_transfer, 10, 20, minute).
step(book_airport_transfer, book_airport_transfer_s1).
action(book_airport_transfer_s1, look_up).
object_of(book_airport_transfer_s1, flight_details).
step(book_airport_transfer, book_airport_transfer_s2).
action(book_airport_transfer_s2, compare).
object_of(book_airport_transfer_s2, transfer_prices).
uses(book_airport_transfer_s2, laptop).
role(book_airport_transfer_s2, laptop, hold).
step(book_airport_transfer, book_airport_transfer_s3).
action(book_airport_transfer_s3, book).
object_of(book_airport_transfer_s3, transfer).
uses(book_airport_transfer_s3, laptop).
role(book_airport_transfer_s3, laptop, hold).
uses(book_airport_transfer_s3, credit_card).
role(book_airport_transfer_s3, credit_card, pay).
step(book_airport_transfer, book_airport_transfer_s4).
action(book_airport_transfer_s4, save).
object_of(book_airport_transfer_s4, confirmation).
uses(book_airport_transfer_s4, phone).
role(book_airport_transfer_s4, phone, hold).
before(book_airport_transfer_s1, book_airport_transfer_s2).
before(book_airport_transfer_s2, book_airport_transfer_s3).
before(book_airport_transfer_s3, book_airport_transfer_s4).
```

## aliases
```datalog
task_alias(book_airport_transfer, "airport transfer").
task_alias(book_airport_transfer, "airport shuttle").
alias(laptop, "computer").
alias(credit_card, "card").
alias(phone, "mobile").
alias(flight_details, "flight number").
alias(transfer_prices, "prices").
alias(confirmation, "booking confirmation").
alias(look_up, "check").
alias(book, "reserve").
```

## teach
- To book an airport transfer you need a laptop, a credit card and a phone.
- Booking an airport transfer takes 10 to 20 minutes.
- Step 1 of booking an airport transfer: look up your flight's landing time.
- Step 2 of booking an airport transfer: compare the transfer prices on the laptop.
- Step 3 of booking an airport transfer: book the transfer on the laptop, paying with the credit card.
- Step 4 of booking an airport transfer: save the booking confirmation on your phone.

## checklist
- names what is needed: a laptop, a credit card and a phone
- gives the steps in order: look up the flight, compare prices, book and pay, save the confirmation
- gives a total time of about 10 to 20 minutes

## variant
- change: no laptop: compare prices and book on the phone instead
- request: I don't have my laptop with me. Can I still book the airport transfer?
- situation: I have my phone and credit card but no laptop.
- expect: The reply uses the phone in place of the laptop for comparing prices and booking, keeping the same order and the step that saves the confirmation.

```datalog situation
has(user, phone).
has(user, credit_card).
lacks(user, laptop).
```

```datalog difference
no_need(laptop).
swap(book_airport_transfer_s2, laptop, phone).
swap(book_airport_transfer_s3, laptop, phone).
```

## variant
- change: only booking, no saved confirmation, and about 15 minutes: leave out the confirmation step
- request: I only have 15 minutes and don't need a confirmation saved. What are the quick steps to book an airport transfer?
- situation: I have a laptop and a credit card, and about 15 minutes. I don't need the confirmation saved.
- expect: The reply gives the look-up, compare and book steps, leaves out saving the confirmation, and says it fits in about 8 to 12 minutes.

```datalog situation
has(user, laptop).
has(user, credit_card).
time_limit(user, 15, minute).
```

```datalog difference
drop(book_airport_transfer_s4).
no_need(phone).
change(book_airport_transfer, takes_time, 10_20, 8_12).
```
