---
id: book_airport_lounge_access
stage: procedures
task: book_airport_lounge_access
schema: v3
---
# Book airport lounge access

## requests
- I want to get into the airport lounge before my flight. How do I book access?
- Can you tell me how to reserve a lounge pass online at the airport?
- What are the steps to buy lounge access for my trip?

## situation
- I have my phone, a credit card and my passport.
- I'm flying in a couple of hours and want to book before I leave.

## ground truth
```datalog
task(book_airport_lounge_access).
domain(book_airport_lounge_access, travel).
goal(book_airport_lounge_access, lounge_access).
needs(book_airport_lounge_access, phone).
needs(book_airport_lounge_access, credit_card).
needs(book_airport_lounge_access, passport).
takes_time(book_airport_lounge_access, 10, 15, minute).
step(book_airport_lounge_access, book_airport_lounge_access_s1).
action(book_airport_lounge_access_s1, open).
object_of(book_airport_lounge_access_s1, lounge_website).
uses(book_airport_lounge_access_s1, phone).
role(book_airport_lounge_access_s1, phone, hold).
step(book_airport_lounge_access, book_airport_lounge_access_s2).
action(book_airport_lounge_access_s2, select).
object_of(book_airport_lounge_access_s2, lounge_access).
step(book_airport_lounge_access, book_airport_lounge_access_s3).
action(book_airport_lounge_access_s3, enter).
object_of(book_airport_lounge_access_s3, passenger_details).
uses(book_airport_lounge_access_s3, passport).
role(book_airport_lounge_access_s3, passport, hold).
takes_time(book_airport_lounge_access_s3, 3, 5, minute).
step(book_airport_lounge_access, book_airport_lounge_access_s4).
action(book_airport_lounge_access_s4, pay).
object_of(book_airport_lounge_access_s4, booking).
uses(book_airport_lounge_access_s4, credit_card).
role(book_airport_lounge_access_s4, credit_card, pay).
step(book_airport_lounge_access, book_airport_lounge_access_s5).
action(book_airport_lounge_access_s5, save).
object_of(book_airport_lounge_access_s5, confirmation).
uses(book_airport_lounge_access_s5, phone).
role(book_airport_lounge_access_s5, phone, hold).
before(book_airport_lounge_access_s1, book_airport_lounge_access_s2).
before(book_airport_lounge_access_s2, book_airport_lounge_access_s3).
before(book_airport_lounge_access_s3, book_airport_lounge_access_s4).
before(book_airport_lounge_access_s4, book_airport_lounge_access_s5).
```

## aliases
```datalog
task_alias(book_airport_lounge_access, "lounge access").
task_alias(book_airport_lounge_access, "airport lounge pass").
task_alias(book_airport_lounge_access, "booking a lounge").
alias(lounge_website, "lounge website").
alias(lounge_access, "lounge pass").
alias(passenger_details, "passenger details").
alias(credit_card, "card").
alias(confirmation, "booking confirmation").
alias(phone, "mobile").
```

## teach
- To book airport lounge access you need a phone, a credit card and a passport.
- Booking airport lounge access takes 10 to 15 minutes.
- Step 1 of book airport lounge access: open the lounge website on your phone.
- Step 2 of book airport lounge access: select the lounge access you want.
- Step 3 of book airport lounge access: enter the passenger details from your passport, which takes 3 to 5 minutes.
- Step 4 of book airport lounge access: pay for the booking with your credit card.
- Step 5 of book airport lounge access: save the booking confirmation on your phone.

## checklist
- names what is needed: a phone, a credit card and a passport
- gives the steps in order: open the lounge website, select the lounge access, enter passenger details, pay, save the confirmation
- says the passport is used for the passenger details and the card for payment
- gives a total time of about 10 to 15 minutes

## variant
- change: no phone, so use a laptop instead for the website and the confirmation
- request: I don't have my phone with me, only a laptop. Can I still book the lounge?
- situation: I have a laptop, a credit card and my passport.
- expect: The reply uses the laptop in place of the phone for opening the lounge website and saving the confirmation, and otherwise keeps the same steps and order.

```datalog situation
lacks(user, phone).
has(user, laptop).
has(user, credit_card).
has(user, passport).
```

```datalog difference
swap(book_airport_lounge_access_s1, phone, laptop).
swap(book_airport_lounge_access_s5, phone, laptop).
no_need(phone).
extra_need(laptop).
```

- change: a guest is coming too, so enter two sets of passenger details and allow more time for that step
- request: I'm booking for me and a friend. Can you adjust the steps for two people?
- situation: I have a phone, a credit card and my passport, and I'm booking for two people.
- expect: The reply keeps the same steps and order but says to enter details for both people, which takes 6 to 8 minutes, and warns that the total time is a bit longer.

```datalog situation
has(user, phone).
has(user, credit_card).
has(user, passport).
count(user, guest, 2).
```

```datalog difference
change(book_airport_lounge_access_s3, takes_time, 3_5, 6_8).
```
