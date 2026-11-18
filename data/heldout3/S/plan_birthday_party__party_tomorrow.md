---
id: plan_birthday_party__party_tomorrow
stage: social
task: plan_birthday_party
schema: v3
tier: S
base: plan_birthday_party
---
# Plan a birthday party

## requests
- My housemate's birthday is tomorrow and we only just decided to throw her something. Can we pull it off?

## ground truth
```datalog
task(plan_birthday_party).
domain(plan_birthday_party, planning).
goal(plan_birthday_party, birthday_party).
needs(plan_birthday_party, invitation).
needs(plan_birthday_party, cake).
needs(plan_birthday_party, balloon).
takes_time(plan_birthday_party, 120, 180, minute).
step(plan_birthday_party, plan_birthday_party_s1).
action(plan_birthday_party_s1, choose).
object_of(plan_birthday_party_s1, date).
check(plan_birthday_party_s1, suits_birthday_person).
takes_time(plan_birthday_party_s1, 10, 10, minute).
step(plan_birthday_party, plan_birthday_party_s2).
action(plan_birthday_party_s2, write).
object_of(plan_birthday_party_s2, guest_list).
takes_time(plan_birthday_party_s2, 15, 20, minute).
step(plan_birthday_party, plan_birthday_party_s3).
action(plan_birthday_party_s3, send).
object_of(plan_birthday_party_s3, invitation).
setting(plan_birthday_party_s3, lead_time, 2, week).
takes_time(plan_birthday_party_s3, 20, 30, minute).
step(plan_birthday_party, plan_birthday_party_s4).
action(plan_birthday_party_s4, order).
object_of(plan_birthday_party_s4, cake).
from(plan_birthday_party_s4, bakery).
takes_time(plan_birthday_party_s4, 15, 20, minute).
step(plan_birthday_party, plan_birthday_party_s5).
action(plan_birthday_party_s5, buy).
object_of(plan_birthday_party_s5, balloon).
takes_time(plan_birthday_party_s5, 30, 45, minute).
step(plan_birthday_party, plan_birthday_party_s6).
action(plan_birthday_party_s6, decorate).
object_of(plan_birthday_party_s6, room).
uses(plan_birthday_party_s6, balloon).
role(plan_birthday_party_s6, balloon, decorate).
condition(plan_birthday_party_s6, party_day).
takes_time(plan_birthday_party_s6, 30, 55, minute).
before(plan_birthday_party_s1, plan_birthday_party_s2).
before(plan_birthday_party_s2, plan_birthday_party_s3).
before(plan_birthday_party_s3, plan_birthday_party_s4).
before(plan_birthday_party_s4, plan_birthday_party_s5).
before(plan_birthday_party_s5, plan_birthday_party_s6).
```

## aliases
```datalog
task_alias(plan_birthday_party, "planning a birthday party").
task_alias(plan_birthday_party, "throw a party").
task_alias(plan_birthday_party, "organise a birthday party").
task_alias(plan_birthday_party, "surprise party").
alias(invitation, "invitations").
alias(balloon, "balloons").
alias(balloon, "decorations").
alias(guest_list, "guest list").
alias(guest, "guests").
alias(party_day, "on the day").
alias(suits_birthday_person, "suits the birthday person").
alias(lead_time, "ahead").
alias(lead_time, "well ahead").
alias(secret_kept, "keep it secret").
alias(birthday_person_away, "when he is away").
alias(arrival, "get him home").
alias(guests_arrive_first, "just after everyone arrives").
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

## variant
- change: party tomorrow: message invitations today and buy a cake from the supermarket
- request: My housemate's birthday is tomorrow and we only just decided to throw her something. Can we pull it off?
- situation: The party is tomorrow at home. There is no time to order from a bakery. A supermarket is close by.
- expect: The reply sends the invitations by message today instead of two weeks ahead, buys a ready-made cake from the supermarket instead of ordering from a bakery, and keeps the balloons and decorating.

```datalog situation
has(user, supermarket).
time_limit(user, 1, day).
```

```datalog difference
change(plan_birthday_party_s3, setting, 2_week, same_day).
change(plan_birthday_party_s4, from, bakery, supermarket).
```
