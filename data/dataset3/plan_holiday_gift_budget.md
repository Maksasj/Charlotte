---
id: plan_holiday_gift_budget
stage: procedures
task: plan_holiday_gift_budget
schema: v3
---
# Plan a holiday gift budget

## requests
- I need to work out how much to spend on holiday gifts this year. How should I go about it?
- What's the best way to plan a gift budget for the holidays?
- Can you take me through setting a budget for presents for everyone on my list?

## situation
- I have a notebook and a calculator.
- I have about 30 to 45 minutes this evening.

## ground truth
```datalog
task(plan_holiday_gift_budget).
domain(plan_holiday_gift_budget, planning).
goal(plan_holiday_gift_budget, holiday_gift_budget).
needs(plan_holiday_gift_budget, notebook).
needs(plan_holiday_gift_budget, calculator).
takes_time(plan_holiday_gift_budget, 30, 45, minute).
step(plan_holiday_gift_budget, plan_holiday_gift_budget_s1).
action(plan_holiday_gift_budget_s1, list).
object_of(plan_holiday_gift_budget_s1, recipient).
uses(plan_holiday_gift_budget_s1, notebook).
role(plan_holiday_gift_budget_s1, notebook, hold).
step(plan_holiday_gift_budget, plan_holiday_gift_budget_s2).
action(plan_holiday_gift_budget_s2, divide).
object_of(plan_holiday_gift_budget_s2, budget).
uses(plan_holiday_gift_budget_s2, calculator).
role(plan_holiday_gift_budget_s2, calculator, measure).
step(plan_holiday_gift_budget, plan_holiday_gift_budget_s3).
action(plan_holiday_gift_budget_s3, write).
object_of(plan_holiday_gift_budget_s3, amount).
uses(plan_holiday_gift_budget_s3, notebook).
role(plan_holiday_gift_budget_s3, notebook, hold).
step(plan_holiday_gift_budget, plan_holiday_gift_budget_s4).
action(plan_holiday_gift_budget_s4, review).
object_of(plan_holiday_gift_budget_s4, budget_plan).
uses(plan_holiday_gift_budget_s4, calculator).
role(plan_holiday_gift_budget_s4, calculator, measure).
check(plan_holiday_gift_budget_s4, within_budget).
before(plan_holiday_gift_budget_s1, plan_holiday_gift_budget_s2).
before(plan_holiday_gift_budget_s2, plan_holiday_gift_budget_s3).
before(plan_holiday_gift_budget_s3, plan_holiday_gift_budget_s4).
```

## aliases
```datalog
task_alias(plan_holiday_gift_budget, "holiday gift budget").
task_alias(plan_holiday_gift_budget, "budget for holiday gifts").
alias(recipient, "person on my list").
alias(divide, "split").
alias(budget_plan, "budget plan").
alias(within_budget, "within budget").
alias(amount, "amount per person").
```

## teach
- To plan a holiday gift budget you need a notebook and a calculator.
- Planning a holiday gift budget takes 30 to 45 minutes.
- Step 1 of planning a holiday gift budget: list each person on your gift list in the notebook.
- Step 2 of planning a holiday gift budget: split your total budget between the people on the list, using the calculator.
- Step 3 of planning a holiday gift budget: write the amount for each person in the notebook.
- Step 4 of planning a holiday gift budget: review the budget plan with the calculator to check the total is within budget.

## checklist
- names the notebook and the calculator as what is needed
- gives the steps in order: list recipients, divide the budget, write the amounts, review the plan
- says the review checks the total stays within budget
- gives a total time of about 30 to 45 minutes

## variant
- change: no calculator, only a phone, and about 20 minutes
- request: I want to plan my holiday gift budget but my calculator is broken and I only have 20 minutes. Can I use my phone?
- situation: I have a notebook and my phone, and about 20 minutes.
- expect: The reply uses the phone in place of the calculator for the divide and review steps and fits the plan into about 15 to 20 minutes.

```datalog situation
has(user, notebook).
has(user, phone).
lacks(user, calculator).
time_limit(user, 20, minute).
```

```datalog difference
extra_need(phone).
no_need(calculator).
swap(plan_holiday_gift_budget_s2, calculator, phone).
swap(plan_holiday_gift_budget_s4, calculator, phone).
change(plan_holiday_gift_budget, takes_time, 30_45, 15_20).
```

## variant
- change: add a step to estimate the wrapping paper, using the calculator after writing the amounts
- request: I want to plan my gift budget for five people and also set aside money for wrapping paper. Can you add that?
- situation: I have a notebook and a calculator, I want wrapping paper, and I have five people to buy for.
- expect: The reply adds a step after writing the amounts that uses the calculator to estimate the wrapping paper cost, before the final review, and notes the plan is for five people.

```datalog situation
has(user, notebook).
has(user, calculator).
wants(user, wrapping_paper).
count(user, recipient, 5).
```

```datalog difference
add_step(plan_holiday_gift_budget_s9, plan_holiday_gift_budget_s3).
step(plan_holiday_gift_budget, plan_holiday_gift_budget_s9).
action(plan_holiday_gift_budget_s9, estimate).
object_of(plan_holiday_gift_budget_s9, wrapping_paper).
uses(plan_holiday_gift_budget_s9, calculator).
role(plan_holiday_gift_budget_s9, calculator, measure).
takes_time(plan_holiday_gift_budget_s9, 5, 10, minute).
before(plan_holiday_gift_budget_s3, plan_holiday_gift_budget_s9).
before(plan_holiday_gift_budget_s9, plan_holiday_gift_budget_s4).
extra_need(wrapping_paper).
```
