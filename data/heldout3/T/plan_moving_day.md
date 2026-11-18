---
id: plan_moving_day
stage: social
task: plan_moving_day
schema: v3
tier: T
---
# Plan moving day

## requests
- I'm moving flats next month. How do I plan the move so it isn't chaos?
- What do I need to organise for moving house?
- Give me a plan for the day I move all my stuff to the new place.

## situation
- I have a phone, a stack of cardboard boxes, parcel tape and a marker.

## ground truth
```datalog
task(plan_moving_day).
domain(plan_moving_day, planning).
goal(plan_moving_day, moved_house).
needs(plan_moving_day, phone).
needs(plan_moving_day, box).
needs(plan_moving_day, tape).
needs(plan_moving_day, marker).
needs(plan_moving_day, van).
takes_time(plan_moving_day, 2, 4, week).
step(plan_moving_day, plan_moving_day_s1).
action(plan_moving_day_s1, book).
object_of(plan_moving_day_s1, van).
uses(plan_moving_day_s1, phone).
role(plan_moving_day_s1, phone, access).
step(plan_moving_day, plan_moving_day_s2).
action(plan_moving_day_s2, pack).
object_of(plan_moving_day_s2, box).
uses(plan_moving_day_s2, tape).
role(plan_moving_day_s2, tape, fasten).
step(plan_moving_day, plan_moving_day_s3).
action(plan_moving_day_s3, label).
object_of(plan_moving_day_s3, box).
uses(plan_moving_day_s3, marker).
role(plan_moving_day_s3, marker, write).
purpose(plan_moving_day_s3, right_room).
step(plan_moving_day, plan_moving_day_s4).
action(plan_moving_day_s4, ask).
object_of(plan_moving_day_s4, friend).
uses(plan_moving_day_s4, phone).
role(plan_moving_day_s4, phone, inform).
purpose(plan_moving_day_s4, help_lifting).
step(plan_moving_day, plan_moving_day_s5).
action(plan_moving_day_s5, load).
object_of(plan_moving_day_s5, van).
warning(plan_moving_day_s5, heavy_lift).
step(plan_moving_day, plan_moving_day_s6).
action(plan_moving_day_s6, unpack).
object_of(plan_moving_day_s6, box).
condition(plan_moving_day_s6, box_labelled_kitchen).
before(plan_moving_day_s1, plan_moving_day_s2).
before(plan_moving_day_s2, plan_moving_day_s3).
before(plan_moving_day_s3, plan_moving_day_s4).
before(plan_moving_day_s4, plan_moving_day_s5).
before(plan_moving_day_s5, plan_moving_day_s6).
```

## aliases
```datalog
task_alias(plan_moving_day, "moving house").
task_alias(plan_moving_day, "moving flats").
task_alias(plan_moving_day, "planning a move").
alias(van, "removal van").
alias(box, "boxes").
alias(tape, "parcel tape").
alias(heavy_lift, "lift with your legs").
alias(right_room, "it goes in the right room").
```

## teach
- To plan moving day you need a phone, boxes, parcel tape, a marker and a van.
- Planning moving day takes 2 to 4 weeks.
- Step 1 of planning moving day: book a van by phone.
- Step 2 of planning moving day: pack the boxes and seal them with tape.
- Step 3 of planning moving day: label each box with the marker so it goes in the right room.
- Step 4 of planning moving day: ask friends by phone to help with the lifting.
- Step 5 of planning moving day: load the van; lift heavy things with your legs.
- Step 6 of planning moving day: unpack the boxes, starting with the one labelled kitchen.

## checklist
- names what is needed, including boxes, tape, a marker and a van
- gives the steps in order: book van, pack, label, ask friends, load, unpack
- warns about lifting safely

## variant
- change: has a cat: book a pet sitter for the day
- request: I've got a nervous cat. How do I fit her into the moving-day plan?
- situation: I have a phone, boxes, tape and a marker, and a cat who hates noise.
- expect: The reply adds booking someone to look after the cat away from the move, before moving day.

```datalog situation
has(user, phone).
has(user, box).
has(user, tape).
has(user, marker).
wants(user, pet_safe).
```

```datalog difference
add_step(plan_moving_day_s9, plan_moving_day_s4).
step(plan_moving_day, plan_moving_day_s9).
action(plan_moving_day_s9, book).
object_of(plan_moving_day_s9, pet_sitter).
uses(plan_moving_day_s9, phone).
role(plan_moving_day_s9, phone, access).
before(plan_moving_day_s4, plan_moving_day_s9).
before(plan_moving_day_s9, plan_moving_day_s5).
```

## variant
- change: moving in 3 days: friends' cars instead of a van that is booked out
- request: I have to move out in three days and every van is booked. What can I do?
- situation: I have a phone, boxes, tape and a marker; friends have two cars; the move is in 3 days.
- expect: The reply drops booking a van, asks friends to bring their cars and loads those instead, all within 3 days.

```datalog situation
has(user, phone).
has(user, box).
has(user, tape).
has(user, marker).
lacks(user, van).
time_limit(user, 3, day).
```

```datalog difference
drop(plan_moving_day_s1).
change(plan_moving_day_s5, object_of, van, friend_car).
no_need(van).
change(plan_moving_day, takes_time, 2_4_week, 3_3_day).
```
