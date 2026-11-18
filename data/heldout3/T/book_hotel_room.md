---
id: book_hotel_room
stage: social
task: book_hotel_room
schema: v3
tier: T
---
# Book a hotel room

## requests
- We're going to Lisbon in May. How should I go about finding and booking a hotel?
- What's the sensible way to book a hotel room online?
- I need somewhere to sleep for three nights on my work trip. How do I sort it out?

## situation
- I have a calendar, a laptop, my bank card and my phone.

## ground truth
```datalog
task(book_hotel_room).
domain(book_hotel_room, travel).
goal(book_hotel_room, place_to_stay).
needs(book_hotel_room, calendar).
needs(book_hotel_room, laptop).
needs(book_hotel_room, payment_card).
needs(book_hotel_room, phone).
takes_time(book_hotel_room, 30, 60, minute).
step(book_hotel_room, book_hotel_room_s1).
action(book_hotel_room_s1, choose).
object_of(book_hotel_room_s1, dates).
uses(book_hotel_room_s1, calendar).
role(book_hotel_room_s1, calendar, record).
step(book_hotel_room, book_hotel_room_s2).
action(book_hotel_room_s2, compare).
object_of(book_hotel_room_s2, hotel).
uses(book_hotel_room_s2, laptop).
role(book_hotel_room_s2, laptop, inform).
check(book_hotel_room_s2, near_plans).
step(book_hotel_room, book_hotel_room_s3).
action(book_hotel_room_s3, read).
object_of(book_hotel_room_s3, review).
purpose(book_hotel_room_s3, avoid_bad_hotel).
step(book_hotel_room, book_hotel_room_s4).
action(book_hotel_room_s4, book).
object_of(book_hotel_room_s4, room).
uses(book_hotel_room_s4, payment_card).
role(book_hotel_room_s4, payment_card, pay).
check(book_hotel_room_s4, free_cancellation).
step(book_hotel_room, book_hotel_room_s5).
action(book_hotel_room_s5, save).
object_of(book_hotel_room_s5, confirmation).
uses(book_hotel_room_s5, phone).
role(book_hotel_room_s5, phone, record).
step(book_hotel_room, book_hotel_room_s6).
action(book_hotel_room_s6, note).
object_of(book_hotel_room_s6, check_in_time).
before(book_hotel_room_s1, book_hotel_room_s2).
before(book_hotel_room_s2, book_hotel_room_s3).
before(book_hotel_room_s3, book_hotel_room_s4).
before(book_hotel_room_s4, book_hotel_room_s5).
before(book_hotel_room_s5, book_hotel_room_s6).
```

## aliases
```datalog
task_alias(book_hotel_room, "booking a hotel").
task_alias(book_hotel_room, "finding a hotel").
task_alias(book_hotel_room, "reserving a room").
alias(hotel, "hotels").
alias(room, "a room").
alias(confirmation, "the booking confirmation").
alias(check_in_time, "the check-in time").
alias(near_plans, "close to where you will be").
alias(free_cancellation, "free cancellation").
```

## teach
- To book a hotel room you need a calendar, a laptop, a payment card and a phone.
- Booking a hotel room takes 30 to 60 minutes.
- Step 1 of booking a hotel room: choose your dates on the calendar.
- Step 2 of booking a hotel room: compare hotels on the laptop and check they are close to where you will be.
- Step 3 of booking a hotel room: read the reviews to avoid a bad hotel.
- Step 4 of booking a hotel room: book the room with your card, checking it has free cancellation.
- Step 5 of booking a hotel room: save the confirmation on your phone.
- Step 6 of booking a hotel room: note the check-in time.

## checklist
- names what is needed: a calendar, a laptop, a card and a phone
- gives the steps in order: dates, compare, reviews, book, save confirmation, note check-in time
- mentions free cancellation

## variant
- change: family of five: book a family room or two connecting rooms
- request: There are five of us, me, my wife and three kids. How do I book a hotel for all of us?
- situation: I have a calendar, a laptop, my card and my phone; we are five people.
- expect: The reply searches for a family room that sleeps five, or two connecting rooms, instead of a standard room.

```datalog situation
has(user, calendar).
has(user, laptop).
has(user, payment_card).
has(user, phone).
count(user, guest, 5).
```

```datalog difference
change(book_hotel_room_s4, object_of, room, family_room).
```

## variant
- change: tonight: book on the phone app and skip the long comparison
- request: My train's been cancelled and I need a hotel for tonight. Quickest way?
- situation: I have my phone and my bank card, and it is already evening.
- expect: The reply skips choosing dates and long comparing, finds a nearby hotel on the phone with good reviews and books it in a few minutes.

```datalog situation
has(user, phone).
has(user, payment_card).
time_limit(user, 15, minute).
```

```datalog difference
drop(book_hotel_room_s1).
swap(book_hotel_room_s2, laptop, phone).
no_need(laptop).
no_need(calendar).
change(book_hotel_room, takes_time, 30_60, 10_15).
```
