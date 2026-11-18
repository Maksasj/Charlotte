---
id: plan_weekly_meals
stage: social
task: plan_weekly_meals
schema: v3
tier: T
---
# Plan a week of meals

## requests
- I keep wasting food and buying takeaways. How do I plan what to eat each week?
- Can you help me set up a weekly dinner plan with a shopping list?
- How should I go about meal planning for the week?

## situation
- I have a notebook, a recipe book and a fridge with some leftovers in it.

## ground truth
```datalog
task(plan_weekly_meals).
domain(plan_weekly_meals, planning).
goal(plan_weekly_meals, meal_plan).
goal(plan_weekly_meals, less_food_waste).
needs(plan_weekly_meals, notebook).
needs(plan_weekly_meals, recipe_book).
takes_time(plan_weekly_meals, 2, 3, hour).
step(plan_weekly_meals, plan_weekly_meals_s1).
action(plan_weekly_meals_s1, check).
object_of(plan_weekly_meals_s1, fridge).
purpose(plan_weekly_meals_s1, use_leftovers).
step(plan_weekly_meals, plan_weekly_meals_s2).
action(plan_weekly_meals_s2, choose).
object_of(plan_weekly_meals_s2, meal).
uses(plan_weekly_meals_s2, recipe_book).
role(plan_weekly_meals_s2, recipe_book, inform).
amount(plan_weekly_meals_s2, meal, 7, piece).
step(plan_weekly_meals, plan_weekly_meals_s3).
action(plan_weekly_meals_s3, write).
object_of(plan_weekly_meals_s3, meal_plan).
uses(plan_weekly_meals_s3, notebook).
role(plan_weekly_meals_s3, notebook, record).
step(plan_weekly_meals, plan_weekly_meals_s4).
action(plan_weekly_meals_s4, write).
object_of(plan_weekly_meals_s4, shopping_list).
uses(plan_weekly_meals_s4, notebook).
role(plan_weekly_meals_s4, notebook, record).
purpose(plan_weekly_meals_s4, buy_only_needed).
step(plan_weekly_meals, plan_weekly_meals_s5).
action(plan_weekly_meals_s5, buy).
object_of(plan_weekly_meals_s5, groceries).
takes_time(plan_weekly_meals_s5, 1, 1, hour).
step(plan_weekly_meals, plan_weekly_meals_s6).
action(plan_weekly_meals_s6, prep).
object_of(plan_weekly_meals_s6, food).
takes_time(plan_weekly_meals_s6, 1, 2, hour).
optional(plan_weekly_meals_s6).
before(plan_weekly_meals_s1, plan_weekly_meals_s2).
before(plan_weekly_meals_s2, plan_weekly_meals_s3).
before(plan_weekly_meals_s3, plan_weekly_meals_s4).
before(plan_weekly_meals_s4, plan_weekly_meals_s5).
before(plan_weekly_meals_s5, plan_weekly_meals_s6).
```

## aliases
```datalog
task_alias(plan_weekly_meals, "meal planning").
task_alias(plan_weekly_meals, "planning the week's dinners").
task_alias(plan_weekly_meals, "a weekly menu").
alias(meal, "dinners").
alias(meal_plan, "a plan for the week").
alias(shopping_list, "shopping list").
alias(use_leftovers, "use up what is there").
alias(prep, "batch cook").
```

## teach
- To plan a week of meals you need a notebook and a recipe book.
- Planning a week of meals takes 2 to 3 hours including shopping.
- Step 1 of planning a week of meals: check the fridge to use up what is there.
- Step 2 of planning a week of meals: choose seven dinners from the recipe book.
- Step 3 of planning a week of meals: write the meal plan in the notebook.
- Step 4 of planning a week of meals: write a shopping list in the notebook so you buy only what you need.
- Step 5 of planning a week of meals: buy the groceries, about 1 hour.
- Step 6 of planning a week of meals: if you like, prep some food ahead for 1 to 2 hours.

## checklist
- names what is needed: a notebook and a recipe book
- gives the steps in order: check fridge, choose meals, write plan, write list, shop, prep
- says prepping ahead is optional

## variant
- change: no notebook: write the plan and list on a sheet of paper
- request: I don't have a notebook. What should I write my meal plan on?
- situation: I have a recipe book, a sheet of paper and a pen.
- expect: The reply writes the meal plan and the shopping list on the sheet of paper instead of a notebook.

```datalog situation
has(user, recipe_book).
has(user, paper).
has(user, pen).
lacks(user, notebook).
```

```datalog difference
swap(plan_weekly_meals_s3, notebook, paper).
swap(plan_weekly_meals_s4, notebook, paper).
extra_need(paper).
no_need(notebook).
```

## variant
- change: gluten-free: check each recipe and the shopping for gluten
- request: My partner was just told to go gluten-free. How do I plan our week's meals now?
- situation: I have a notebook and a recipe book. My partner cannot eat gluten.
- expect: The reply adds checking each chosen recipe for gluten and choosing gluten-free products when writing the shopping list.

```datalog situation
has(user, notebook).
has(user, recipe_book).
excludes(user, gluten).
```

```datalog difference
add_step(plan_weekly_meals_s9, plan_weekly_meals_s2).
step(plan_weekly_meals, plan_weekly_meals_s9).
action(plan_weekly_meals_s9, check).
object_of(plan_weekly_meals_s9, recipe).
purpose(plan_weekly_meals_s9, gluten_free).
before(plan_weekly_meals_s2, plan_weekly_meals_s9).
before(plan_weekly_meals_s9, plan_weekly_meals_s3).
```
