---
id: plan_picnic
stage: social
task: plan_picnic
---
# Plan a picnic

## requests
- I'd like to take a few friends on a picnic this Saturday. How should I plan it?
- Can you help me organise a simple picnic in the park?
- The weather's getting nice and I want to have a picnic with friends. Where do I start?

## situation
- I have a picnic basket, a blanket, bread and fillings for sandwiches, some fruit and drinks.
- I can spend about an hour getting it ready.

## ground truth
```datalog
task(plan_picnic).
needs(plan_picnic, basket).
needs(plan_picnic, blanket).
needs(plan_picnic, bread).
needs(plan_picnic, fruit).
needs(plan_picnic, drink).
takes_time(plan_picnic, 45, 60, minute).
step(plan_picnic, plan_picnic_s1).
action(plan_picnic_s1, check).
object_of(plan_picnic_s1, weather).
takes_time(plan_picnic_s1, 5, 5, minute).
step(plan_picnic, plan_picnic_s2).
action(plan_picnic_s2, choose).
object_of(plan_picnic_s2, park).
takes_time(plan_picnic_s2, 5, 10, minute).
step(plan_picnic, plan_picnic_s3).
action(plan_picnic_s3, invite).
object_of(plan_picnic_s3, friend).
takes_time(plan_picnic_s3, 10, 10, minute).
step(plan_picnic, plan_picnic_s4).
action(plan_picnic_s4, make).
object_of(plan_picnic_s4, sandwich).
uses(plan_picnic_s4, bread).
takes_time(plan_picnic_s4, 15, 20, minute).
step(plan_picnic, plan_picnic_s5).
action(plan_picnic_s5, pack).
object_of(plan_picnic_s5, basket).
uses(plan_picnic_s5, blanket).
uses(plan_picnic_s5, fruit).
uses(plan_picnic_s5, drink).
takes_time(plan_picnic_s5, 10, 15, minute).
before(plan_picnic_s1, plan_picnic_s2).
before(plan_picnic_s2, plan_picnic_s3).
before(plan_picnic_s3, plan_picnic_s4).
before(plan_picnic_s4, plan_picnic_s5).
```

## teach
- To plan a picnic you need a basket, a blanket, bread, fruit and drinks.
- Planning a picnic takes 45 to 60 minutes.
- Step 1 of planning a picnic: check the weather forecast for the day for 5 minutes.
- Step 2 of planning a picnic: choose a park with some shade for 5 to 10 minutes.
- Step 3 of planning a picnic: invite your friends and tell them the time and place, for 10 minutes.
- Step 4 of planning a picnic: make the sandwiches with the bread for 15 to 20 minutes.
- Step 5 of planning a picnic: pack the basket with the sandwiches, the fruit, the drinks and the blanket for 10 to 15 minutes.

## checklist
- suggests checking the weather and choosing a place
- gives the steps in order: check weather, choose park, invite friends, make sandwiches, pack basket
- mentions bringing a blanket, food and drinks
- closes politely

## variant
- change: rain forecast: hold an indoor picnic at home instead of in the park
- request: I was planning a picnic for Saturday but now they say it's going to rain all day. Can I still make it work?
- situation: The forecast says heavy rain on Saturday. I have a blanket, a basket, bread, fruit and drinks, and a big living room.
- expect: The reply moves the picnic indoors, spreading the blanket on the living room floor instead of choosing a park, and keeps the food steps.
