---
id: plan_birthday_party
stage: social
task: plan_birthday_party
---
# Plan a birthday party

## requests
- My daughter turns eight next month and I want to throw her a party at home. How do I plan it?
- Can you help me organise a small birthday party for my best friend?
- I've never planned a birthday party before. What do I need to do and in what order?

## situation
- I want to have it at home with about ten guests. I can order a cake from the bakery and buy balloons.
- I can spend a few hours in total on getting it ready.

## ground truth
```datalog
task(plan_birthday_party).
needs(plan_birthday_party, invitation).
needs(plan_birthday_party, cake).
needs(plan_birthday_party, balloon).
takes_time(plan_birthday_party, 120, 180, minute).
step(plan_birthday_party, plan_birthday_party_s1).
action(plan_birthday_party_s1, choose).
object_of(plan_birthday_party_s1, date).
takes_time(plan_birthday_party_s1, 10, 10, minute).
step(plan_birthday_party, plan_birthday_party_s2).
action(plan_birthday_party_s2, write).
object_of(plan_birthday_party_s2, guest_list).
takes_time(plan_birthday_party_s2, 15, 20, minute).
step(plan_birthday_party, plan_birthday_party_s3).
action(plan_birthday_party_s3, send).
object_of(plan_birthday_party_s3, invitation).
takes_time(plan_birthday_party_s3, 20, 30, minute).
step(plan_birthday_party, plan_birthday_party_s4).
action(plan_birthday_party_s4, order).
object_of(plan_birthday_party_s4, cake).
takes_time(plan_birthday_party_s4, 15, 20, minute).
step(plan_birthday_party, plan_birthday_party_s5).
action(plan_birthday_party_s5, buy).
object_of(plan_birthday_party_s5, balloon).
takes_time(plan_birthday_party_s5, 30, 45, minute).
step(plan_birthday_party, plan_birthday_party_s6).
action(plan_birthday_party_s6, decorate).
object_of(plan_birthday_party_s6, room).
uses(plan_birthday_party_s6, balloon).
takes_time(plan_birthday_party_s6, 30, 55, minute).
before(plan_birthday_party_s1, plan_birthday_party_s2).
before(plan_birthday_party_s2, plan_birthday_party_s3).
before(plan_birthday_party_s3, plan_birthday_party_s4).
before(plan_birthday_party_s4, plan_birthday_party_s5).
before(plan_birthday_party_s5, plan_birthday_party_s6).
```

## teach
- To plan a birthday party you need invitations, a cake and balloons.
- Planning a birthday party takes 120 to 180 minutes.
- Step 1 of planning a birthday party: choose a date that suits the birthday person for 10 minutes.
- Step 2 of planning a birthday party: write the guest list for 15 to 20 minutes.
- Step 3 of planning a birthday party: send the invitations at least two weeks ahead, for 20 to 30 minutes.
- Step 4 of planning a birthday party: order the cake from the bakery for 15 to 20 minutes.
- Step 5 of planning a birthday party: buy the balloons and other decorations for 30 to 45 minutes.
- Step 6 of planning a birthday party: decorate the room with the balloons on the day for 30 to 55 minutes.

## checklist
- gives the steps in order: choose date, guest list, invitations, cake, balloons, decorate
- says to send the invitations well ahead of the day
- mentions a cake and decorations
- closes politely

## variant
- change: a surprise party: the birthday person must not find out
- request: I want to throw my husband a surprise party for his fortieth. How do I plan it without him finding out?
- situation: It's a surprise for my husband. About fifteen guests at our place, and he works late on Fridays.
- expect: The reply asks guests to keep it secret, picks a time when he is away, and plans a way to get him home just after everyone arrives.
