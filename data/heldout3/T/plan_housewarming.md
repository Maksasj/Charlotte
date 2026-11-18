---
id: plan_housewarming
stage: social
task: plan_housewarming
schema: v3
tier: T
---
# Plan a housewarming party

## requests
- I've just moved into a new flat. How do I throw a little party to show it off?
- What's involved in planning a housewarming?
- Can you help me organise a get-together at my new place for friends and family?

## situation
- I have a calendar, a phone and some money for snacks and drinks.
- About 12 people would come.

## ground truth
```datalog
task(plan_housewarming).
domain(plan_housewarming, planning).
goal(plan_housewarming, housewarming).
goal(plan_housewarming, social_gathering).
needs(plan_housewarming, calendar).
needs(plan_housewarming, phone).
needs(plan_housewarming, snack).
needs(plan_housewarming, drink).
takes_time(plan_housewarming, 2, 3, week).
step(plan_housewarming, plan_housewarming_s1).
action(plan_housewarming_s1, pick).
object_of(plan_housewarming_s1, date).
uses(plan_housewarming_s1, calendar).
role(plan_housewarming_s1, calendar, record).
step(plan_housewarming, plan_housewarming_s2).
action(plan_housewarming_s2, invite).
object_of(plan_housewarming_s2, guest).
uses(plan_housewarming_s2, phone).
role(plan_housewarming_s2, phone, inform).
step(plan_housewarming, plan_housewarming_s3).
action(plan_housewarming_s3, tell).
object_of(plan_housewarming_s3, neighbour).
purpose(plan_housewarming_s3, neighbours_warned).
step(plan_housewarming, plan_housewarming_s4).
action(plan_housewarming_s4, buy).
object_of(plan_housewarming_s4, snack).
uses(plan_housewarming_s4, drink).
role(plan_housewarming_s4, drink, serve).
amount(plan_housewarming_s4, snack, usual, piece).
step(plan_housewarming, plan_housewarming_s5).
action(plan_housewarming_s5, clear).
object_of(plan_housewarming_s5, space).
purpose(plan_housewarming_s5, room_to_move).
step(plan_housewarming, plan_housewarming_s6).
action(plan_housewarming_s6, set_out).
object_of(plan_housewarming_s6, snack).
before(plan_housewarming_s1, plan_housewarming_s2).
before(plan_housewarming_s2, plan_housewarming_s3).
before(plan_housewarming_s3, plan_housewarming_s4).
before(plan_housewarming_s4, plan_housewarming_s5).
before(plan_housewarming_s5, plan_housewarming_s6).
```

## aliases
```datalog
task_alias(plan_housewarming, "a housewarming").
task_alias(plan_housewarming, "a housewarming party").
task_alias(plan_housewarming, "a party at the new place").
alias(neighbour, "the neighbours").
alias(snack, "snacks").
alias(set_out, "put out").
alias(neighbours_warned, "they know about the noise").
```

## teach
- To plan a housewarming party you need a calendar, a phone, snacks and drinks.
- Planning a housewarming party takes 2 to 3 weeks.
- Step 1 of planning a housewarming party: pick a date on the calendar.
- Step 2 of planning a housewarming party: invite the guests by phone.
- Step 3 of planning a housewarming party: tell the neighbours so they know about the noise.
- Step 4 of planning a housewarming party: buy the usual amount of snacks and drinks to serve.
- Step 5 of planning a housewarming party: clear space so people have room to move.
- Step 6 of planning a housewarming party: set out the snacks.

## checklist
- names what is needed: a calendar, a phone, snacks and drinks
- gives the steps in order: date, invite, tell neighbours, shop, clear space, set out snacks
- mentions warning the neighbours

## variant
- change: 30 guests: more food and ask people to bring a dish
- request: Turns out about thirty people want to come to my housewarming. How do I cope?
- situation: I have a calendar and a phone; there will be 30 guests in a small flat.
- expect: The reply buys more snacks and drinks and asks guests to each bring something to share.

```datalog situation
has(user, calendar).
has(user, phone).
count(user, guest, 30).
```

```datalog difference
change(plan_housewarming_s4, amount, usual, more).
add_step(plan_housewarming_s9, plan_housewarming_s2).
step(plan_housewarming, plan_housewarming_s9).
action(plan_housewarming_s9, ask).
object_of(plan_housewarming_s9, guest).
purpose(plan_housewarming_s9, guests_bring_dish).
before(plan_housewarming_s2, plan_housewarming_s9).
before(plan_housewarming_s9, plan_housewarming_s4).
```

## variant
- change: this weekend: send a group message and keep it simple
- request: Could I throw a housewarming this Saturday? That's only two days away.
- situation: I have a phone and a bit of money; the party would be in 2 days.
- expect: The reply invites everyone with one group message the same day and skips the calendar planning, so it all fits in 2 days.

```datalog situation
has(user, phone).
time_limit(user, 2, day).
```

```datalog difference
drop(plan_housewarming_s1).
no_need(calendar).
change(plan_housewarming, takes_time, 2_3_week, 2_2_day).
```
