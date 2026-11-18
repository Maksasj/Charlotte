---
id: send_wedding_rsvp__reply_by_tomorrow
stage: social
task: send_wedding_rsvp
schema: v3
tier: S
base: send_wedding_rsvp
---
# Reply to a wedding invitation

## requests
- I forgot to answer my cousin's wedding invite and the reply date is tomorrow. A card won't get there in time, will it?

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

## variant
- change: reply due tomorrow: message the couple instead of posting the card
- request: I forgot to answer my cousin's wedding invite and the reply date is tomorrow. A card won't get there in time, will it?
- situation: The reply date is tomorrow. I have the RSVP card, a pen, my calendar and my phone with the couple's number.
- expect: The reply checks the date and fills in the answers, then sends them to the couple by phone message instead of stamping and posting the card.

```datalog situation
has(user, rsvp_card).
has(user, pen).
has(user, calendar).
has(user, phone).
time_limit(user, 1, day).
```

```datalog difference
drop(send_wedding_rsvp_s5).
drop(send_wedding_rsvp_s6).
no_need(stamp).
extra_need(phone).
add_step(send_wedding_rsvp_s9, send_wedding_rsvp_s4).
step(send_wedding_rsvp, send_wedding_rsvp_s9).
action(send_wedding_rsvp_s9, message).
object_of(send_wedding_rsvp_s9, couple).
uses(send_wedding_rsvp_s9, phone).
role(send_wedding_rsvp_s9, phone, inform).
before(send_wedding_rsvp_s4, send_wedding_rsvp_s9).
```
