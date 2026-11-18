---
id: plan_coffee_catch_up__three_friends
stage: social
task: plan_coffee_catch_up
schema: v3
tier: S
base: plan_coffee_catch_up
---
# Plan a coffee catch-up

## requests
- I'd like to get three old uni friends together for a coffee. How do I organise it so everyone can make it?

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

## variant
- change: three friends: message them as a group and find a time that suits all
- request: I'd like to get three old uni friends together for a coffee. How do I organise it so everyone can make it?
- situation: I want to meet three friends. I have my phone and my calendar.
- expect: The reply messages the friends as a group, finds a time that suits all of them, then books the cafe, adds it to the calendar and confirms with the group.

```datalog situation
has(user, phone).
has(user, calendar).
count(user, friend, 3).
```

```datalog difference
change(plan_coffee_catch_up_s1, object_of, friend, friend_group).
change(plan_coffee_catch_up_s6, repeat, 1, 3).
change(plan_coffee_catch_up, takes_time, 10_15, 20_30).
```
