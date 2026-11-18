---
id: plan_coffee_catch_up
stage: social
task: plan_coffee_catch_up
schema: v3
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
domain(plan_coffee_catch_up, planning).
goal(plan_coffee_catch_up, coffee_catch_up).
needs(plan_coffee_catch_up, phone).
needs(plan_coffee_catch_up, calendar).
takes_time(plan_coffee_catch_up, 10, 15, minute).
step(plan_coffee_catch_up, plan_coffee_catch_up_s1).
action(plan_coffee_catch_up_s1, message).
object_of(plan_coffee_catch_up_s1, friend).
uses(plan_coffee_catch_up_s1, phone).
role(plan_coffee_catch_up_s1, phone, inform).
step(plan_coffee_catch_up, plan_coffee_catch_up_s2).
action(plan_coffee_catch_up_s2, suggest).
object_of(plan_coffee_catch_up_s2, coffee).
takes_time(plan_coffee_catch_up_s2, 2, 3, minute).
step(plan_coffee_catch_up, plan_coffee_catch_up_s3).
action(plan_coffee_catch_up_s3, find).
object_of(plan_coffee_catch_up_s3, time).
uses(plan_coffee_catch_up_s3, calendar).
role(plan_coffee_catch_up_s3, calendar, record).
takes_time(plan_coffee_catch_up_s3, 3, 5, minute).
step(plan_coffee_catch_up, plan_coffee_catch_up_s4).
action(plan_coffee_catch_up_s4, choose).
object_of(plan_coffee_catch_up_s4, cafe).
takes_time(plan_coffee_catch_up_s4, 3, 5, minute).
step(plan_coffee_catch_up, plan_coffee_catch_up_s5).
action(plan_coffee_catch_up_s5, add).
object_of(plan_coffee_catch_up_s5, plan).
uses(plan_coffee_catch_up_s5, calendar).
role(plan_coffee_catch_up_s5, calendar, record).
into(plan_coffee_catch_up_s5, calendar).
takes_time(plan_coffee_catch_up_s5, 1, 1, minute).
step(plan_coffee_catch_up, plan_coffee_catch_up_s6).
action(plan_coffee_catch_up_s6, confirm).
object_of(plan_coffee_catch_up_s6, plan).
uses(plan_coffee_catch_up_s6, phone).
role(plan_coffee_catch_up_s6, phone, inform).
takes_time(plan_coffee_catch_up_s6, 1, 1, minute).
before(plan_coffee_catch_up_s1, plan_coffee_catch_up_s2).
before(plan_coffee_catch_up_s2, plan_coffee_catch_up_s3).
before(plan_coffee_catch_up_s3, plan_coffee_catch_up_s4).
before(plan_coffee_catch_up_s4, plan_coffee_catch_up_s5).
before(plan_coffee_catch_up_s5, plan_coffee_catch_up_s6).
```

## aliases
```datalog
task_alias(plan_coffee_catch_up, "planning a coffee catch-up").
task_alias(plan_coffee_catch_up, "set up a coffee to catch up").
task_alias(plan_coffee_catch_up, "meet a friend for coffee").
task_alias(plan_coffee_catch_up, "reconnect over coffee").
alias(friend, "old friend").
alias(friend, "former colleague").
alias(coffee, "meeting for coffee").
alias(cafe, "cafe halfway between you").
alias(time, "a time that works for both of you").
alias(phone, "my phone").
alias(calendar, "my calendar").
alias(call_link, "call link").
alias(call_link, "video call").
```

## teach
- To plan a coffee catch-up you need a phone and a calendar.
- Planning a coffee catch-up takes 10 to 15 minutes.
- Step 1 of planning a coffee catch-up: message your friend on the phone.
- Step 2 of planning a coffee catch-up: suggest meeting for coffee for 2 to 3 minutes.
- Step 3 of planning a coffee catch-up: find a time in your calendar that works for both of you for 3 to 5 minutes.
- Step 4 of planning a coffee catch-up: choose a cafe halfway between you for 3 to 5 minutes.
- Step 5 of planning a coffee catch-up: add the plan to your calendar for 1 minute.
- Step 6 of planning a coffee catch-up: confirm the plan with a message on the phone the day before for 1 minute.

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

```datalog situation
has(user, phone).
has(user, laptop).
has(user, calendar).
```

```datalog difference
drop(plan_coffee_catch_up_s4).
add_step(plan_coffee_catch_up_s9, plan_coffee_catch_up_s3).
step(plan_coffee_catch_up, plan_coffee_catch_up_s9).
action(plan_coffee_catch_up_s9, send).
object_of(plan_coffee_catch_up_s9, call_link).
before(plan_coffee_catch_up_s3, plan_coffee_catch_up_s9).
before(plan_coffee_catch_up_s9, plan_coffee_catch_up_s5).
```
