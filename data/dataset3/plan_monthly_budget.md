---
id: plan_monthly_budget
stage: procedures
task: plan_monthly_budget
schema: v3
---
# Plan a monthly budget

## requests
- How do I plan my budget for the month without overspending?
- Can you walk me through making a monthly budget?
- What's the way to work out what's left over each month from my bank statement?

## situation
- I have my latest bank statement, a notebook and a calculator.
- I have about 45 minutes this evening.

## ground truth
```datalog
task(plan_monthly_budget).
domain(plan_monthly_budget, planning).
goal(plan_monthly_budget, monthly_budget).
needs(plan_monthly_budget, bank_statement).
needs(plan_monthly_budget, notebook).
needs(plan_monthly_budget, calculator).
takes_time(plan_monthly_budget, 45, 60, minute).
step(plan_monthly_budget, plan_monthly_budget_s1).
action(plan_monthly_budget_s1, gather).
object_of(plan_monthly_budget_s1, bank_statement).
takes_time(plan_monthly_budget_s1, 10, 15, minute).
step(plan_monthly_budget, plan_monthly_budget_s2).
action(plan_monthly_budget_s2, list).
object_of(plan_monthly_budget_s2, fixed_bills).
uses(plan_monthly_budget_s2, notebook).
role(plan_monthly_budget_s2, notebook, contain).
takes_time(plan_monthly_budget_s2, 10, 15, minute).
step(plan_monthly_budget, plan_monthly_budget_s3).
action(plan_monthly_budget_s3, total).
object_of(plan_monthly_budget_s3, expenses).
uses(plan_monthly_budget_s3, calculator).
role(plan_monthly_budget_s3, calculator, measure).
takes_time(plan_monthly_budget_s3, 10, 15, minute).
step(plan_monthly_budget, plan_monthly_budget_s4).
action(plan_monthly_budget_s4, subtract).
object_of(plan_monthly_budget_s4, expenses).
uses(plan_monthly_budget_s4, calculator).
role(plan_monthly_budget_s4, calculator, measure).
purpose(plan_monthly_budget_s4, know_surplus).
takes_time(plan_monthly_budget_s4, 5, 10, minute).
step(plan_monthly_budget, plan_monthly_budget_s5).
action(plan_monthly_budget_s5, set).
object_of(plan_monthly_budget_s5, spending_limit).
uses(plan_monthly_budget_s5, notebook).
role(plan_monthly_budget_s5, notebook, contain).
purpose(plan_monthly_budget_s5, avoid_overspending).
takes_time(plan_monthly_budget_s5, 5, 10, minute).
before(plan_monthly_budget_s1, plan_monthly_budget_s2).
before(plan_monthly_budget_s2, plan_monthly_budget_s3).
before(plan_monthly_budget_s3, plan_monthly_budget_s4).
before(plan_monthly_budget_s4, plan_monthly_budget_s5).
```

## aliases
```datalog
task_alias(plan_monthly_budget, "monthly budget").
task_alias(plan_monthly_budget, "budget for the month").
alias(bank_statement, "bank statement").
alias(fixed_bills, "fixed bills").
alias(spending_limit, "spending limit").
alias(expenses, "expenses").
alias(know_surplus, "what is left over").
alias(avoid_overspending, "avoid overspending").
alias(subtract, "take away").
```

## teach
- To plan a monthly budget you need a bank statement, a notebook and a calculator.
- Planning a monthly budget takes 45 to 60 minutes.
- Step 1 of planning a monthly budget: gather the bank statement for the month.
- Step 2 of planning a monthly budget: list your fixed bills in the notebook.
- Step 3 of planning a monthly budget: total the expenses on the calculator.
- Step 4 of planning a monthly budget: subtract the expenses from your income on the calculator to see what is left over.
- Step 5 of planning a monthly budget: set a spending limit in the notebook to avoid overspending.

## checklist
- names what is needed, including a bank statement, a notebook and a calculator
- gives the steps in order: gather the statement, list fixed bills, total expenses, subtract from income, set a spending limit
- says the total time is about 45 to 60 minutes
- explains that subtracting shows what is left over and that the spending limit helps avoid overspending

## variant
- change: no calculator: use the phone's calculator in place of the calculator for totalling and subtracting
- request: I don't have a calculator at home, but I have my phone. Can I still plan my month?
- situation: I have my bank statement, a notebook and my phone, but no calculator.
- expect: The reply keeps every step the same but uses the phone in place of the calculator for the totals and the subtraction.

```datalog situation
lacks(user, calculator).
has(user, phone).
has(user, bank_statement).
has(user, notebook).
```

```datalog difference
swap(plan_monthly_budget_s3, calculator, phone).
swap(plan_monthly_budget_s4, calculator, phone).
no_need(calculator).
extra_need(phone).
```

- change: only 20 minutes: skip setting the spending limit and keep the other steps
- request: I only have 20 minutes, so can you give me a quick version of the budget?
- situation: I have a bank statement, a notebook and a calculator, and 20 minutes.
- expect: The reply gives the gathering, listing, totalling and subtracting steps in about 20 to 25 minutes and leaves out the spending limit.

```datalog situation
has(user, bank_statement).
has(user, notebook).
has(user, calculator).
time_limit(user, 20, minute).
```

```datalog difference
change(plan_monthly_budget, takes_time, 45_60, 20_25).
drop(plan_monthly_budget_s5).
```
