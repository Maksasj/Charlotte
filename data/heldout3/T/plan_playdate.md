---
id: plan_playdate
stage: social
task: plan_playdate
schema: v3
tier: T
---
# Plan a children's playdate

## requests
- My daughter wants her friend from school round to play. How do I set that up?
- What should I sort out before having another kid over for a playdate?
- How do I arrange a playdate with a classmate's parents?

## situation
- I have a phone, some snacks and a box of toys.
- Two children will play.

## ground truth
```datalog
task(plan_playdate).
domain(plan_playdate, planning).
goal(plan_playdate, playdate).
needs(plan_playdate, phone).
needs(plan_playdate, snack).
needs(plan_playdate, toy).
takes_time(plan_playdate, 2, 4, day).
step(plan_playdate, plan_playdate_s1).
action(plan_playdate_s1, message).
object_of(plan_playdate_s1, parent).
uses(plan_playdate_s1, phone).
role(plan_playdate_s1, phone, inform).
step(plan_playdate, plan_playdate_s2).
action(plan_playdate_s2, agree).
object_of(plan_playdate_s2, time).
step(plan_playdate, plan_playdate_s3).
action(plan_playdate_s3, ask).
object_of(plan_playdate_s3, allergy).
purpose(plan_playdate_s3, safe_snack).
step(plan_playdate, plan_playdate_s4).
action(plan_playdate_s4, plan).
object_of(plan_playdate_s4, activity).
uses(plan_playdate_s4, toy).
role(plan_playdate_s4, toy, play).
step(plan_playdate, plan_playdate_s5).
action(plan_playdate_s5, prepare).
object_of(plan_playdate_s5, snack).
amount(plan_playdate_s5, snack, 2, piece).
step(plan_playdate, plan_playdate_s6).
action(plan_playdate_s6, tidy).
object_of(plan_playdate_s6, room).
purpose(plan_playdate_s6, room_to_play).
before(plan_playdate_s1, plan_playdate_s2).
before(plan_playdate_s2, plan_playdate_s3).
before(plan_playdate_s3, plan_playdate_s4).
before(plan_playdate_s4, plan_playdate_s5).
before(plan_playdate_s5, plan_playdate_s6).
```

## aliases
```datalog
task_alias(plan_playdate, "a playdate").
task_alias(plan_playdate, "having a child round to play").
task_alias(plan_playdate, "a children's play date").
alias(parent, "the other parent").
alias(toy, "toys").
alias(snack, "snacks").
alias(activity, "something to do").
```

## teach
- To plan a children's playdate you need a phone, snacks and toys.
- Planning a playdate takes 2 to 4 days.
- Step 1 of planning a playdate: message the other parent by phone.
- Step 2 of planning a playdate: agree a time.
- Step 3 of planning a playdate: ask about allergies so the snack is safe.
- Step 4 of planning a playdate: plan an activity with the toys.
- Step 5 of planning a playdate: prepare two snacks for each child.
- Step 6 of planning a playdate: tidy the room so there is room to play.

## checklist
- names what is needed: a phone, snacks and toys
- gives the steps in order: message parent, agree time, ask allergies, plan activity, snacks, tidy
- mentions asking about allergies

## variant
- change: four children: more snacks and a group game
- request: My son wants three friends over at once. How do I plan for four kids?
- situation: I have a phone, snacks and toys; four children will come.
- expect: The reply messages each parent, prepares more snacks and plans a group game everyone can join.

```datalog situation
has(user, phone).
has(user, snack).
has(user, toy).
count(user, child, 4).
```

```datalog difference
change(plan_playdate_s5, amount, snack_2_piece, snack_8_piece).
change(plan_playdate_s4, object_of, activity, group_game).
```

## variant
- change: wants to go to the park: pack a bag and plan the walk
- request: It's sunny, so I'd like to take the kids to the park on the playdate. What extra do I plan?
- situation: I have a phone, snacks, toys, a backpack and water bottles.
- expect: The reply adds packing a bag with snacks, water and a ball before walking to the park, and tells the other parent.

```datalog situation
has(user, phone).
has(user, snack).
has(user, toy).
has(user, backpack).
has(user, water_bottle).
wants(user, park).
```

```datalog difference
extra_need(backpack).
extra_need(water_bottle).
add_step(plan_playdate_s9, plan_playdate_s5).
step(plan_playdate, plan_playdate_s9).
action(plan_playdate_s9, pack).
object_of(plan_playdate_s9, backpack).
uses(plan_playdate_s9, water_bottle).
role(plan_playdate_s9, water_bottle, contain).
before(plan_playdate_s5, plan_playdate_s9).
```
