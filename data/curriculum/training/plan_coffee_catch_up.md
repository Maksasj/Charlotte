---
id: plan_coffee_catch_up
stage: social
task: plan_coffee_catch_up
---
# Plan a coffee catch-up

## requests
- I haven't seen an old friend in ages. How do I set up a coffee to catch up?
- Can you help me arrange to meet a friend for coffee?
- I'd like to reconnect with a former colleague over coffee. How do I go about it?

## situation
- I have my phone and my calendar.
- I'm free most afternoons next week.

## ground truth
```datalog
task(plan_coffee_catch_up).
needs(plan_coffee_catch_up, phone).
needs(plan_coffee_catch_up, calendar).
takes_time(plan_coffee_catch_up, 10, 15, minute).
step(plan_coffee_catch_up, plan_coffee_catch_up_s1).
action(plan_coffee_catch_up_s1, message).
object_of(plan_coffee_catch_up_s1, friend).
uses(plan_coffee_catch_up_s1, phone).
takes_time(plan_coffee_catch_up_s1, 2, 3, minute).
step(plan_coffee_catch_up, plan_coffee_catch_up_s2).
action(plan_coffee_catch_up_s2, find).
object_of(plan_coffee_catch_up_s2, time).
uses(plan_coffee_catch_up_s2, calendar).
takes_time(plan_coffee_catch_up_s2, 3, 5, minute).
step(plan_coffee_catch_up, plan_coffee_catch_up_s3).
action(plan_coffee_catch_up_s3, choose).
object_of(plan_coffee_catch_up_s3, cafe).
takes_time(plan_coffee_catch_up_s3, 3, 5, minute).
step(plan_coffee_catch_up, plan_coffee_catch_up_s4).
action(plan_coffee_catch_up_s4, add).
object_of(plan_coffee_catch_up_s4, plan).
uses(plan_coffee_catch_up_s4, calendar).
takes_time(plan_coffee_catch_up_s4, 1, 1, minute).
step(plan_coffee_catch_up, plan_coffee_catch_up_s5).
action(plan_coffee_catch_up_s5, confirm).
object_of(plan_coffee_catch_up_s5, plan).
uses(plan_coffee_catch_up_s5, phone).
takes_time(plan_coffee_catch_up_s5, 1, 1, minute).
before(plan_coffee_catch_up_s1, plan_coffee_catch_up_s2).
before(plan_coffee_catch_up_s2, plan_coffee_catch_up_s3).
before(plan_coffee_catch_up_s3, plan_coffee_catch_up_s4).
before(plan_coffee_catch_up_s4, plan_coffee_catch_up_s5).
```

## teach
- To plan a coffee catch-up you need a phone and a calendar.
- Planning a coffee catch-up takes 10 to 15 minutes.
- Step 1 of planning a coffee catch-up: message your friend on the phone and suggest meeting for coffee for 2 to 3 minutes.
- Step 2 of planning a coffee catch-up: find a time in your calendar that works for both of you for 3 to 5 minutes.
- Step 3 of planning a coffee catch-up: choose a cafe halfway between you for 3 to 5 minutes.
- Step 4 of planning a coffee catch-up: add the plan to your calendar for 1 minute.
- Step 5 of planning a coffee catch-up: confirm the plan with a message on the phone the day before for 1 minute.

## checklist
- names what is needed: a phone and a calendar
- gives the steps in order: message friend, find time, choose cafe, add plan, confirm plan
- gives a total time of about 10 to 15 minutes
- closes politely

## variant
- change: friend lives far away: set up a video call with a coffee each instead of a cafe
- request: The friend I want to catch up with lives five hours away. Can we still do a coffee?
- situation: My friend lives in another city. I have my phone, my laptop and my calendar.
- expect: The reply suggests a video call where each person has a coffee at home, replacing the cafe step with sending a call link, and keeps the message, time and confirm steps.
