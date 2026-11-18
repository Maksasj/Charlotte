---
id: send_wedding_rsvp
stage: social
task: send_wedding_rsvp
schema: v3
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
domain(send_wedding_rsvp, errands).
goal(send_wedding_rsvp, wedding_reply).
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
role(send_wedding_rsvp_s2, pen, record).
takes_time(send_wedding_rsvp_s2, 3, 5, minute).
step(send_wedding_rsvp, send_wedding_rsvp_s3).
action(send_wedding_rsvp_s3, tick).
object_of(send_wedding_rsvp_s3, meal).
step(send_wedding_rsvp, send_wedding_rsvp_s4).
action(send_wedding_rsvp_s4, note).
object_of(send_wedding_rsvp_s4, allergy).
condition(send_wedding_rsvp_s4, has_allergy).
takes_time(send_wedding_rsvp_s4, 1, 2, minute).
step(send_wedding_rsvp, send_wedding_rsvp_s5).
action(send_wedding_rsvp_s5, stamp).
object_of(send_wedding_rsvp_s5, rsvp_card).
uses(send_wedding_rsvp_s5, stamp).
role(send_wedding_rsvp_s5, stamp, pay).
step(send_wedding_rsvp, send_wedding_rsvp_s6).
action(send_wedding_rsvp_s6, post).
object_of(send_wedding_rsvp_s6, rsvp_card).
takes_time(send_wedding_rsvp_s6, 3, 5, minute).
before(send_wedding_rsvp_s1, send_wedding_rsvp_s2).
before(send_wedding_rsvp_s2, send_wedding_rsvp_s3).
before(send_wedding_rsvp_s3, send_wedding_rsvp_s4).
before(send_wedding_rsvp_s4, send_wedding_rsvp_s5).
before(send_wedding_rsvp_s5, send_wedding_rsvp_s6).
```

## aliases
```datalog
task_alias(send_wedding_rsvp, "replying to a wedding invitation").
task_alias(send_wedding_rsvp, "reply to a wedding invitation").
task_alias(send_wedding_rsvp, "RSVP to my cousin's wedding").
task_alias(send_wedding_rsvp, "answer a wedding invite").
alias(rsvp_card, "RSVP card").
alias(rsvp_card, "reply card").
alias(rsvp_card, "card").
alias(meal, "meal choice").
alias(allergy, "allergies").
alias(has_allergy, "any allergies").
alias(decline_box, "decline box").
alias(decline_box, "tick no").
alias(congratulation_note, "note of congratulations").
```

## teach
- To reply to a wedding invitation you need the RSVP card, a pen, a stamp and a calendar.
- Replying to a wedding invitation takes 10 to 15 minutes.
- Step 1 of replying to a wedding invitation: check your calendar for the wedding date for 2 to 3 minutes.
- Step 2 of replying to a wedding invitation: fill in the RSVP card with the pen: your name, yes or no, and the number of guests, for 3 to 5 minutes.
- Step 3 of replying to a wedding invitation: tick your meal choice.
- Step 4 of replying to a wedding invitation: note any allergies for 1 to 2 minutes.
- Step 5 of replying to a wedding invitation: stamp the RSVP card.
- Step 6 of replying to a wedding invitation: post it before the reply date for 3 to 5 minutes.

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

```datalog situation
has(user, rsvp_card).
has(user, pen).
has(user, stamp).
```

```datalog difference
drop(send_wedding_rsvp_s3).
drop(send_wedding_rsvp_s4).
add_step(send_wedding_rsvp_s9, send_wedding_rsvp_s2).
step(send_wedding_rsvp, send_wedding_rsvp_s9).
action(send_wedding_rsvp_s9, tick).
object_of(send_wedding_rsvp_s9, decline_box).
before(send_wedding_rsvp_s2, send_wedding_rsvp_s9).
add_step(send_wedding_rsvp_s10, send_wedding_rsvp_s9).
step(send_wedding_rsvp, send_wedding_rsvp_s10).
action(send_wedding_rsvp_s10, write).
object_of(send_wedding_rsvp_s10, congratulation_note).
uses(send_wedding_rsvp_s10, pen).
role(send_wedding_rsvp_s10, pen, record).
before(send_wedding_rsvp_s9, send_wedding_rsvp_s10).
before(send_wedding_rsvp_s10, send_wedding_rsvp_s5).
```
