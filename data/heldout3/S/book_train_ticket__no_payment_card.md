---
id: book_train_ticket__no_payment_card
task: book_train_ticket
schema: v3
tier: S
base: book_train_ticket
---
# Book a train ticket

## requests
- I need to get a seat on tomorrow's train to Leeds through the app, but my bank card was cancelled this morning and I have no other way to pay online.

## ground truth
```datalog
task(book_train_ticket).
domain(book_train_ticket, travel).
goal(book_train_ticket, ticket).
needs(book_train_ticket, phone).
needs(book_train_ticket, payment_card).
needs(book_train_ticket, email_address).
takes_time(book_train_ticket, 10, 20, minute).
step(book_train_ticket, book_train_ticket_s1).
action(book_train_ticket_s1, open).
object_of(book_train_ticket_s1, booking_app).
uses(book_train_ticket_s1, phone).
role(book_train_ticket_s1, phone, access).
takes_time(book_train_ticket_s1, 1, 1, minute).
step(book_train_ticket, book_train_ticket_s2).
action(book_train_ticket_s2, enter).
object_of(book_train_ticket_s2, route).
uses(book_train_ticket_s2, phone).
role(book_train_ticket_s2, phone, access).
takes_time(book_train_ticket_s2, 1, 2, minute).
step(book_train_ticket, book_train_ticket_s3).
action(book_train_ticket_s3, choose).
object_of(book_train_ticket_s3, train).
takes_time(book_train_ticket_s3, 3, 5, minute).
step(book_train_ticket, book_train_ticket_s4).
action(book_train_ticket_s4, choose).
object_of(book_train_ticket_s4, seat).
takes_time(book_train_ticket_s4, 1, 3, minute).
step(book_train_ticket, book_train_ticket_s5).
action(book_train_ticket_s5, pay).
object_of(book_train_ticket_s5, fare).
uses(book_train_ticket_s5, payment_card).
role(book_train_ticket_s5, payment_card, pay).
takes_time(book_train_ticket_s5, 2, 3, minute).
step(book_train_ticket, book_train_ticket_s6).
action(book_train_ticket_s6, check).
object_of(book_train_ticket_s6, ticket).
uses(book_train_ticket_s6, email_address).
role(book_train_ticket_s6, email_address, receive).
takes_time(book_train_ticket_s6, 1, 2, minute).
before(book_train_ticket_s1, book_train_ticket_s2).
before(book_train_ticket_s2, book_train_ticket_s3).
before(book_train_ticket_s3, book_train_ticket_s4).
before(book_train_ticket_s4, book_train_ticket_s5).
before(book_train_ticket_s5, book_train_ticket_s6).
```

## aliases
```datalog
task_alias(book_train_ticket, "booking a train ticket").
task_alias(book_train_ticket, "buy a train ticket").
task_alias(book_train_ticket, "train ticket").
alias(booking_app, "booking app").
alias(booking_app, "train app").
alias(payment_card, "bank card").
alias(payment_card, "card").
alias(email_address, "email").
alias(choose, "pick").
alias(fare, "ticket price").
alias(route, "journey").
```

## teach
- To book a train ticket you need a phone, a payment card and an email address.
- Booking a train ticket takes 10 to 20 minutes.
- Step 1 of booking a train ticket: open the booking app on your phone for 1 minute.
- Step 2 of booking a train ticket: enter the route on your phone for 1 to 2 minutes.
- Step 3 of booking a train ticket: choose a train for 3 to 5 minutes.
- Step 4 of booking a train ticket: choose a seat for 1 to 3 minutes.
- Step 5 of booking a train ticket: pay the fare with the payment card for 2 to 3 minutes.
- Step 6 of booking a train ticket: check the ticket that arrives at your email address for 1 to 2 minutes.

## variant
- change: no payment card: the online booking cannot be paid for
- request: I need to get a seat on tomorrow's train to Leeds through the app, but my bank card was cancelled this morning and I have no other way to pay online.
- situation: I have my phone and email, but no payment card or other online payment.
- expect: The reply says the ticket cannot be booked online without a way to pay.

```datalog situation
has(user, phone).
has(user, email_address).
lacks(user, payment_card).
```

```datalog difference
cannot(no_payment_method).
```
