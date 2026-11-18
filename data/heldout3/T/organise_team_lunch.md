---
id: organise_team_lunch
stage: social
task: organise_team_lunch
schema: v3
tier: T
---
# Organise a team lunch

## requests
- My manager asked me to sort a lunch out for our team. Where do I start?
- How do I organise a work lunch for about ten colleagues?
- Can you help me arrange a team meal to celebrate finishing our project?

## situation
- I have a shared calendar, a laptop and a phone.
- There are 10 people on the team.

## ground truth
```datalog
task(organise_team_lunch).
domain(organise_team_lunch, planning).
goal(organise_team_lunch, team_lunch).
goal(organise_team_lunch, team_bonding).
needs(organise_team_lunch, calendar).
needs(organise_team_lunch, laptop).
needs(organise_team_lunch, phone).
takes_time(organise_team_lunch, 1, 2, week).
step(organise_team_lunch, organise_team_lunch_s1).
action(organise_team_lunch_s1, pick).
object_of(organise_team_lunch_s1, date).
uses(organise_team_lunch_s1, calendar).
role(organise_team_lunch_s1, calendar, record).
check(organise_team_lunch_s1, most_free).
step(organise_team_lunch, organise_team_lunch_s2).
action(organise_team_lunch_s2, ask).
object_of(organise_team_lunch_s2, diet).
uses(organise_team_lunch_s2, laptop).
role(organise_team_lunch_s2, laptop, inform).
step(organise_team_lunch, organise_team_lunch_s3).
action(organise_team_lunch_s3, book).
object_of(organise_team_lunch_s3, restaurant).
uses(organise_team_lunch_s3, phone).
role(organise_team_lunch_s3, phone, access).
step(organise_team_lunch, organise_team_lunch_s4).
action(organise_team_lunch_s4, send).
object_of(organise_team_lunch_s4, invite).
uses(organise_team_lunch_s4, laptop).
role(organise_team_lunch_s4, laptop, inform).
step(organise_team_lunch, organise_team_lunch_s5).
action(organise_team_lunch_s5, confirm).
object_of(organise_team_lunch_s5, number).
uses(organise_team_lunch_s5, phone).
role(organise_team_lunch_s5, phone, access).
takes_time(organise_team_lunch_s5, 1, 2, day).
before(organise_team_lunch_s1, organise_team_lunch_s2).
before(organise_team_lunch_s2, organise_team_lunch_s3).
before(organise_team_lunch_s3, organise_team_lunch_s4).
before(organise_team_lunch_s4, organise_team_lunch_s5).
```

## aliases
```datalog
task_alias(organise_team_lunch, "a team lunch").
task_alias(organise_team_lunch, "a work lunch").
task_alias(organise_team_lunch, "a lunch for colleagues").
alias(diet, "dietary needs").
alias(invite, "calendar invite").
alias(most_free, "most people are free").
alias(number, "final numbers").
```

## teach
- To organise a team lunch you need a shared calendar, a laptop and a phone.
- Organising a team lunch takes 1 to 2 weeks.
- Step 1 of organising a team lunch: pick a date on the shared calendar when most people are free.
- Step 2 of organising a team lunch: ask everyone about dietary needs by email from the laptop.
- Step 3 of organising a team lunch: book a restaurant by phone.
- Step 4 of organising a team lunch: send a calendar invite from the laptop.
- Step 5 of organising a team lunch: confirm the final numbers with the restaurant by phone 1 to 2 days before.

## checklist
- names what is needed: a calendar, a laptop and a phone
- gives the steps in order: date, dietary needs, book, invite, confirm numbers

## variant
- change: 25 colleagues: book a private room and a set menu
- request: Actually it's the whole department, about 25 people. Does that change how I plan the lunch?
- situation: I have a calendar, a laptop and a phone; 25 people will come.
- expect: The reply books a private room with a set menu for 25 and collects menu choices in advance.

```datalog situation
has(user, calendar).
has(user, laptop).
has(user, phone).
count(user, colleague, 25).
```

```datalog difference
change(organise_team_lunch_s3, object_of, restaurant, private_room).
add_step(organise_team_lunch_s9, organise_team_lunch_s4).
step(organise_team_lunch, organise_team_lunch_s9).
action(organise_team_lunch_s9, collect).
object_of(organise_team_lunch_s9, menu_choice).
uses(organise_team_lunch_s9, laptop).
role(organise_team_lunch_s9, laptop, inform).
before(organise_team_lunch_s4, organise_team_lunch_s9).
before(organise_team_lunch_s9, organise_team_lunch_s5).
```

## variant
- change: laptop in for repair: do the emails and invite from the phone
- request: My work laptop's away being fixed. Can I still organise the team lunch?
- situation: I have a shared calendar and a phone, but no laptop this week.
- expect: The reply sends the dietary question and the invite from the phone instead of the laptop.

```datalog situation
has(user, calendar).
has(user, phone).
lacks(user, laptop).
```

```datalog difference
swap(organise_team_lunch_s2, laptop, phone).
swap(organise_team_lunch_s4, laptop, phone).
no_need(laptop).
```
