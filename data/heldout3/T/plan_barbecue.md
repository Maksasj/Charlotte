---
id: plan_barbecue
stage: social
task: plan_barbecue
schema: v3
tier: T
---
# Plan a barbecue

## requests
- I want to have friends round for a barbecue in the garden next month. How should I plan it?
- What do I need to sort out for a summer BBQ?
- Thinking of grilling for about eight mates on Saturday. What's the plan of action?

## situation
- I have a phone, a charcoal grill, a bag of charcoal and a cool box.
- About eight friends would come.

## ground truth
```datalog
task(plan_barbecue).
domain(plan_barbecue, planning).
goal(plan_barbecue, barbecue).
goal(plan_barbecue, social_gathering).
needs(plan_barbecue, phone).
needs(plan_barbecue, food).
needs(plan_barbecue, drink).
needs(plan_barbecue, grill).
needs(plan_barbecue, charcoal).
needs(plan_barbecue, cool_box).
takes_time(plan_barbecue, 1, 2, week).
step(plan_barbecue, plan_barbecue_s1).
action(plan_barbecue_s1, choose).
object_of(plan_barbecue_s1, date).
check(plan_barbecue_s1, good_forecast).
step(plan_barbecue, plan_barbecue_s2).
action(plan_barbecue_s2, invite).
object_of(plan_barbecue_s2, guest).
uses(plan_barbecue_s2, phone).
role(plan_barbecue_s2, phone, inform).
step(plan_barbecue, plan_barbecue_s3).
action(plan_barbecue_s3, buy).
object_of(plan_barbecue_s3, food).
amount(plan_barbecue_s3, food, usual, piece).
uses(plan_barbecue_s3, drink).
role(plan_barbecue_s3, drink, serve).
step(plan_barbecue, plan_barbecue_s4).
action(plan_barbecue_s4, light).
object_of(plan_barbecue_s4, charcoal).
uses(plan_barbecue_s4, grill).
role(plan_barbecue_s4, grill, heat).
takes_time(plan_barbecue_s4, 30, 30, minute).
until(plan_barbecue_s4, coals_white).
step(plan_barbecue, plan_barbecue_s5).
action(plan_barbecue_s5, chill).
object_of(plan_barbecue_s5, drink).
uses(plan_barbecue_s5, cool_box).
role(plan_barbecue_s5, cool_box, contain).
overlaps(plan_barbecue_s5, plan_barbecue_s4).
step(plan_barbecue, plan_barbecue_s6).
action(plan_barbecue_s6, grill).
object_of(plan_barbecue_s6, food).
uses(plan_barbecue_s6, grill).
role(plan_barbecue_s6, grill, heat).
requires_state(plan_barbecue_s6, coals_white).
before(plan_barbecue_s1, plan_barbecue_s2).
before(plan_barbecue_s2, plan_barbecue_s3).
before(plan_barbecue_s3, plan_barbecue_s4).
before(plan_barbecue_s3, plan_barbecue_s5).
before(plan_barbecue_s4, plan_barbecue_s6).
```

## aliases
```datalog
task_alias(plan_barbecue, "planning a barbecue").
task_alias(plan_barbecue, "a bbq").
task_alias(plan_barbecue, "having a cookout").
alias(grill, "barbecue").
alias(grill, "bbq").
alias(cool_box, "cooler").
alias(coals_white, "the coals are grey and white").
alias(good_forecast, "a dry forecast").
```

## teach
- To plan a barbecue you need a phone, food, drinks, a grill, charcoal and a cool box.
- Planning a barbecue takes 1 to 2 weeks.
- Step 1 of planning a barbecue: choose a date and check it has a dry forecast.
- Step 2 of planning a barbecue: invite the guests by phone.
- Step 3 of planning a barbecue: buy the usual amount of food and the drinks to serve.
- Step 4 of planning a barbecue: light the charcoal in the grill about 30 minutes ahead, until the coals are grey and white.
- While step 4 runs, step 5 of planning a barbecue: chill the drinks in the cool box.
- Step 6 of planning a barbecue: grill the food once the coals are white.

## checklist
- names what is needed, including food, drinks, a grill, charcoal and a cool box
- gives the steps in order: date, invite, shop, light charcoal, chill drinks meanwhile, grill
- says the charcoal needs about 30 minutes before cooking

## variant
- change: 20 guests: buy more food and ask people to bring drinks
- request: It's grown to about twenty people coming to my barbecue. What changes?
- situation: I have a phone, a grill, charcoal and a cool box, and 20 guests.
- expect: The reply buys more food for 20 and asks guests to each bring a drink so the cool box is not the only supply.

```datalog situation
has(user, phone).
has(user, grill).
has(user, charcoal).
has(user, cool_box).
count(user, guest, 20).
```

```datalog difference
change(plan_barbecue_s3, amount, usual, more).
add_step(plan_barbecue_s9, plan_barbecue_s2).
step(plan_barbecue, plan_barbecue_s9).
action(plan_barbecue_s9, ask).
object_of(plan_barbecue_s9, guest).
purpose(plan_barbecue_s9, guests_bring_drink).
before(plan_barbecue_s2, plan_barbecue_s9).
before(plan_barbecue_s9, plan_barbecue_s3).
```

## variant
- change: vegetarian guests: grill vegetables first on a clean grill
- request: Some of the people coming to my barbecue are vegetarian. How do I plan the food?
- situation: I have a phone, a grill, charcoal, a cool box and vegetable skewers; three guests do not eat meat.
- expect: The reply adds vegetable skewers to the shopping and grills them first, before any meat touches the grill.

```datalog situation
has(user, phone).
has(user, grill).
has(user, charcoal).
has(user, cool_box).
has(user, vegetable_skewer).
excludes(user, meat).
```

```datalog difference
extra_need(vegetable_skewer).
add_step(plan_barbecue_s9, plan_barbecue_s4).
step(plan_barbecue, plan_barbecue_s9).
action(plan_barbecue_s9, grill).
object_of(plan_barbecue_s9, vegetable_skewer).
uses(plan_barbecue_s9, grill).
role(plan_barbecue_s9, grill, heat).
purpose(plan_barbecue_s9, no_meat_contact).
before(plan_barbecue_s4, plan_barbecue_s9).
before(plan_barbecue_s9, plan_barbecue_s6).
```
