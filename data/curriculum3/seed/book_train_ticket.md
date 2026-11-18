---
id: book_train_ticket
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Book a train ticket

Find a train online, pick a seat and pay for the ticket.

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
