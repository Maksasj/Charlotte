---
id: plan_meal
status: confirmed
source: seed
uses: 0
successes: 0
---
# Plan a meal

Helping someone decide what to cook: ask what they have and how much time,
then suggest a dish they can make and how.

```datalog
task_slot(plan_meal, ingredients, required).
slot_question(plan_meal, ingredients, "What ingredients do you have").
slot_fills(plan_meal, ingredients, available).
task_slot(plan_meal, time, required).
slot_question(plan_meal, time, "how much time do you want to spend cooking").
slot_fills(plan_meal, time, time_available).
closing(plan_meal, "Enjoy your dinner!").
same(plan_dinner, plan_meal).
same(plan_lunch, plan_meal).
same(cook_dinner, plan_meal).
same(make_dinner, plan_meal).
```
