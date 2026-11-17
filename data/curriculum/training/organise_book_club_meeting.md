---
id: organise_book_club_meeting
stage: social
task: organise_book_club_meeting
---
# Organise a book club meeting

## requests
- It's my turn to host book club. How do I organise the meeting?
- Can you help me plan the next book club evening?
- I'm starting a book club with friends. How do I run the first meeting?

## situation
- I have a copy of the book, a notebook and room for six people in my flat.
- The meeting is in three weeks.

## ground truth
```datalog
task(organise_book_club_meeting).
needs(organise_book_club_meeting, book).
needs(organise_book_club_meeting, notebook).
needs(organise_book_club_meeting, snack).
needs(organise_book_club_meeting, drink).
takes_time(organise_book_club_meeting, 35, 50, minute).
step(organise_book_club_meeting, organise_book_club_meeting_s1).
action(organise_book_club_meeting_s1, choose).
object_of(organise_book_club_meeting_s1, book).
takes_time(organise_book_club_meeting_s1, 5, 10, minute).
step(organise_book_club_meeting, organise_book_club_meeting_s2).
action(organise_book_club_meeting_s2, set).
object_of(organise_book_club_meeting_s2, date).
takes_time(organise_book_club_meeting_s2, 5, 10, minute).
step(organise_book_club_meeting, organise_book_club_meeting_s3).
action(organise_book_club_meeting_s3, send).
object_of(organise_book_club_meeting_s3, reminder).
takes_time(organise_book_club_meeting_s3, 5, 5, minute).
step(organise_book_club_meeting, organise_book_club_meeting_s4).
action(organise_book_club_meeting_s4, write).
object_of(organise_book_club_meeting_s4, question).
uses(organise_book_club_meeting_s4, notebook).
takes_time(organise_book_club_meeting_s4, 10, 15, minute).
step(organise_book_club_meeting, organise_book_club_meeting_s5).
action(organise_book_club_meeting_s5, prepare).
object_of(organise_book_club_meeting_s5, snack).
uses(organise_book_club_meeting_s5, drink).
takes_time(organise_book_club_meeting_s5, 10, 15, minute).
before(organise_book_club_meeting_s1, organise_book_club_meeting_s2).
before(organise_book_club_meeting_s2, organise_book_club_meeting_s3).
before(organise_book_club_meeting_s3, organise_book_club_meeting_s4).
before(organise_book_club_meeting_s4, organise_book_club_meeting_s5).
```

## teach
- To organise a book club meeting you need the book, a notebook, snacks and drinks.
- Organising a book club meeting takes 35 to 50 minutes.
- Step 1 of organising a book club meeting: choose the book with the group for 5 to 10 minutes.
- Step 2 of organising a book club meeting: set a date and a place that suits everyone for 5 to 10 minutes.
- Step 3 of organising a book club meeting: send a reminder a week before for 5 minutes.
- Step 4 of organising a book club meeting: write five discussion questions about the book in the notebook for 10 to 15 minutes.
- Step 5 of organising a book club meeting: prepare some snacks and drinks for 10 to 15 minutes.

## checklist
- names what is needed: the book, a notebook, snacks and drinks
- gives the steps in order: choose book, set date, send reminder, write question, prepare snack
- gives a total time of about 35 to 50 minutes
- closes politely

## variant
- change: online meeting: send a video call link instead of a place and skip the snacks
- request: Half of our book club has moved away. How do I organise the meeting online?
- situation: The members live in different cities. I have the book, a notebook and a laptop.
- expect: The reply sets a date and sends a video call link instead of a place, keeps the reminder and the discussion questions and leaves out the snacks.
