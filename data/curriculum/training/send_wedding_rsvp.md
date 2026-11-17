---
id: send_wedding_rsvp
stage: social
task: send_wedding_rsvp
---
# Reply to a wedding invitation

## requests
- I got a wedding invitation with a little reply card. What do I do with it?
- Can you tell me how to RSVP to my cousin's wedding?
- How do I properly answer a wedding invite by post?

## situation
- I have the RSVP card, a pen, a stamp and my calendar.
- The reply date is in two weeks.

## ground truth
```datalog
task(send_wedding_rsvp).
needs(send_wedding_rsvp, rsvp_card).
needs(send_wedding_rsvp, pen).
needs(send_wedding_rsvp, stamp).
needs(send_wedding_rsvp, calendar).
takes_time(send_wedding_rsvp, 10, 15, minute).
step(send_wedding_rsvp, send_wedding_rsvp_s1).
action(send_wedding_rsvp_s1, check).
object_of(send_wedding_rsvp_s1, calendar).
takes_time(send_wedding_rsvp_s1, 2, 3, minute).
step(send_wedding_rsvp, send_wedding_rsvp_s2).
action(send_wedding_rsvp_s2, fill).
object_of(send_wedding_rsvp_s2, rsvp_card).
uses(send_wedding_rsvp_s2, pen).
takes_time(send_wedding_rsvp_s2, 3, 5, minute).
step(send_wedding_rsvp, send_wedding_rsvp_s3).
action(send_wedding_rsvp_s3, tick).
object_of(send_wedding_rsvp_s3, meal).
takes_time(send_wedding_rsvp_s3, 1, 2, minute).
step(send_wedding_rsvp, send_wedding_rsvp_s4).
action(send_wedding_rsvp_s4, post).
object_of(send_wedding_rsvp_s4, rsvp_card).
uses(send_wedding_rsvp_s4, stamp).
takes_time(send_wedding_rsvp_s4, 3, 5, minute).
before(send_wedding_rsvp_s1, send_wedding_rsvp_s2).
before(send_wedding_rsvp_s2, send_wedding_rsvp_s3).
before(send_wedding_rsvp_s3, send_wedding_rsvp_s4).
```

## teach
- To reply to a wedding invitation you need the RSVP card, a pen, a stamp and a calendar.
- Replying to a wedding invitation takes 10 to 15 minutes.
- Step 1 of replying to a wedding invitation: check your calendar for the wedding date for 2 to 3 minutes.
- Step 2 of replying to a wedding invitation: fill in the RSVP card with the pen: your name, yes or no, and the number of guests, for 3 to 5 minutes.
- Step 3 of replying to a wedding invitation: tick your meal choice and note any allergies for 1 to 2 minutes.
- Step 4 of replying to a wedding invitation: stamp the RSVP card and post it before the reply date for 3 to 5 minutes.

## checklist
- names what is needed: the RSVP card, a pen, a stamp and a calendar
- gives the steps in order: check calendar, fill rsvp card, tick meal, post rsvp card
- gives a total time of about 10 to 15 minutes
- closes politely

## variant
- change: cannot attend: tick no, add a short note of congratulations and think about sending a gift
- request: I can't make it to the wedding because I'll be away. How do I say no nicely on the card?
- situation: I have the RSVP card, a pen and a stamp. I'm abroad on the wedding date.
- expect: The reply ticks the decline box, skips the meal choice, adds a short warm note of congratulations and still posts the card before the reply date.
