---
id: plan_anniversary_dinner
stage: social
task: plan_anniversary_dinner
schema: v3
tier: T
---
# Plan an anniversary dinner

## requests
- It's our tenth anniversary next month. How do I plan a nice dinner out?
- Help me organise a romantic meal for our wedding anniversary.
- I don't want to forget anything for the anniversary date night. What should I do and when?

## situation
- I have a phone and some money for a card and flowers.

## ground truth
```datalog
task(plan_anniversary_dinner).
domain(plan_anniversary_dinner, planning).
goal(plan_anniversary_dinner, anniversary_dinner).
goal(plan_anniversary_dinner, celebrate_partner).
needs(plan_anniversary_dinner, phone).
needs(plan_anniversary_dinner, card).
needs(plan_anniversary_dinner, flower).
takes_time(plan_anniversary_dinner, 1, 2, week).
step(plan_anniversary_dinner, plan_anniversary_dinner_s1).
action(plan_anniversary_dinner_s1, choose).
object_of(plan_anniversary_dinner_s1, restaurant).
check(plan_anniversary_dinner_s1, partner_likes).
step(plan_anniversary_dinner, plan_anniversary_dinner_s2).
action(plan_anniversary_dinner_s2, book).
object_of(plan_anniversary_dinner_s2, table).
uses(plan_anniversary_dinner_s2, phone).
role(plan_anniversary_dinner_s2, phone, access).
purpose(plan_anniversary_dinner_s2, table_secured).
step(plan_anniversary_dinner, plan_anniversary_dinner_s3).
action(plan_anniversary_dinner_s3, buy).
object_of(plan_anniversary_dinner_s3, card).
step(plan_anniversary_dinner, plan_anniversary_dinner_s4).
action(plan_anniversary_dinner_s4, order).
object_of(plan_anniversary_dinner_s4, flower).
uses(plan_anniversary_dinner_s4, phone).
role(plan_anniversary_dinner_s4, phone, access).
step(plan_anniversary_dinner, plan_anniversary_dinner_s5).
action(plan_anniversary_dinner_s5, confirm).
object_of(plan_anniversary_dinner_s5, booking).
uses(plan_anniversary_dinner_s5, phone).
role(plan_anniversary_dinner_s5, phone, access).
takes_time(plan_anniversary_dinner_s5, 1, 1, day).
before(plan_anniversary_dinner_s1, plan_anniversary_dinner_s2).
before(plan_anniversary_dinner_s2, plan_anniversary_dinner_s3).
before(plan_anniversary_dinner_s3, plan_anniversary_dinner_s4).
before(plan_anniversary_dinner_s4, plan_anniversary_dinner_s5).
```

## aliases
```datalog
task_alias(plan_anniversary_dinner, "an anniversary dinner").
task_alias(plan_anniversary_dinner, "a date night for our anniversary").
task_alias(plan_anniversary_dinner, "a romantic meal out").
alias(flower, "flowers").
alias(table, "a table").
alias(partner_likes, "a place your partner will love").
alias(confirm, "check").
```

## teach
- To plan an anniversary dinner you need a phone, a card and flowers.
- Planning an anniversary dinner takes 1 to 2 weeks.
- Step 1 of planning an anniversary dinner: choose a restaurant your partner will love.
- Step 2 of planning an anniversary dinner: book a table by phone to secure it.
- Step 3 of planning an anniversary dinner: buy a card.
- Step 4 of planning an anniversary dinner: order flowers by phone.
- Step 5 of planning an anniversary dinner: confirm the booking by phone 1 day before.

## checklist
- names what is needed: a phone, a card and flowers
- gives the steps in order: choose restaurant, book, card, flowers, confirm
- says to confirm the booking a day before

## variant
- change: anniversary is tomorrow: cook a special dinner at home instead
- request: Oh no, our anniversary is tomorrow and I've planned nothing. What can I still pull off?
- situation: I have a phone, a kitchen and a bit of money; the anniversary is in 1 day.
- expect: The reply drops booking a restaurant, which will be full, and cooks a special dinner at home instead, still getting a card and flowers.

```datalog situation
has(user, phone).
has(user, kitchen).
time_limit(user, 1, day).
```

```datalog difference
drop(plan_anniversary_dinner_s1).
drop(plan_anniversary_dinner_s2).
drop(plan_anniversary_dinner_s5).
add_step(plan_anniversary_dinner_s9, plan_anniversary_dinner_s4).
step(plan_anniversary_dinner, plan_anniversary_dinner_s9).
action(plan_anniversary_dinner_s9, cook).
object_of(plan_anniversary_dinner_s9, dinner).
uses(plan_anniversary_dinner_s9, kitchen).
role(plan_anniversary_dinner_s9, kitchen, heat).
before(plan_anniversary_dinner_s4, plan_anniversary_dinner_s9).
extra_need(kitchen).
change(plan_anniversary_dinner, takes_time, 1_2_week, 1_1_day).
```

## variant
- change: partner allergic to shellfish: tell the restaurant when booking
- request: My wife has a shellfish allergy. How do I make sure the anniversary dinner is safe?
- situation: I have a phone and money for a card and flowers. My wife cannot eat shellfish.
- expect: The reply tells the restaurant about the allergy when booking and checks again when confirming.

```datalog situation
has(user, phone).
excludes(user, shellfish).
```

```datalog difference
add_step(plan_anniversary_dinner_s9, plan_anniversary_dinner_s2).
step(plan_anniversary_dinner, plan_anniversary_dinner_s9).
action(plan_anniversary_dinner_s9, tell).
object_of(plan_anniversary_dinner_s9, restaurant).
uses(plan_anniversary_dinner_s9, phone).
role(plan_anniversary_dinner_s9, phone, inform).
purpose(plan_anniversary_dinner_s9, safe_meal).
before(plan_anniversary_dinner_s2, plan_anniversary_dinner_s9).
before(plan_anniversary_dinner_s9, plan_anniversary_dinner_s3).
```
