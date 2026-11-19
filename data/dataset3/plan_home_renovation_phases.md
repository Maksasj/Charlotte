---
id: plan_home_renovation_phases
stage: procedures
task: plan_home_renovation_phases
schema: v3
---
# Plan a home renovation in phases

## requests
- I'm planning to renovate my house in stages. How should I plan the phases?
- What's the right way to split a home renovation into phases so we don't overspend?
- Can you help me put together a phased plan for fixing up our place?

## situation
- I have a tape measure, a notebook and a budget spreadsheet.
- I have a few weekends to plan before the work starts.

## ground truth
```datalog
task(plan_home_renovation_phases).
domain(plan_home_renovation_phases, planning).
goal(plan_home_renovation_phases, renovation_plan).
needs(plan_home_renovation_phases, tape_measure).
needs(plan_home_renovation_phases, notebook).
needs(plan_home_renovation_phases, budget_sheet).
takes_time(plan_home_renovation_phases, 120, 180, minute).
step(plan_home_renovation_phases, plan_home_renovation_phases_s1).
action(plan_home_renovation_phases_s1, measure).
object_of(plan_home_renovation_phases_s1, room).
uses(plan_home_renovation_phases_s1, tape_measure).
role(plan_home_renovation_phases_s1, tape_measure, measure).
takes_time(plan_home_renovation_phases_s1, 30, 45, minute).
step(plan_home_renovation_phases, plan_home_renovation_phases_s2).
action(plan_home_renovation_phases_s2, list).
object_of(plan_home_renovation_phases_s2, job).
uses(plan_home_renovation_phases_s2, notebook).
role(plan_home_renovation_phases_s2, notebook, record).
step(plan_home_renovation_phases, plan_home_renovation_phases_s3).
action(plan_home_renovation_phases_s3, estimate).
object_of(plan_home_renovation_phases_s3, cost).
uses(plan_home_renovation_phases_s3, budget_sheet).
role(plan_home_renovation_phases_s3, budget_sheet, record).
step(plan_home_renovation_phases, plan_home_renovation_phases_s4).
action(plan_home_renovation_phases_s4, order).
object_of(plan_home_renovation_phases_s4, job).
purpose(plan_home_renovation_phases_s4, avoid_rework).
step(plan_home_renovation_phases, plan_home_renovation_phases_s5).
action(plan_home_renovation_phases_s5, share).
object_of(plan_home_renovation_phases_s5, plan).
before(plan_home_renovation_phases_s1, plan_home_renovation_phases_s2).
before(plan_home_renovation_phases_s2, plan_home_renovation_phases_s3).
before(plan_home_renovation_phases_s3, plan_home_renovation_phases_s4).
before(plan_home_renovation_phases_s4, plan_home_renovation_phases_s5).
```

## aliases
```datalog
task_alias(plan_home_renovation_phases, "phased renovation plan").
task_alias(plan_home_renovation_phases, "renovation phases").
alias(tape_measure, "tape measure").
alias(budget_sheet, "budget spreadsheet").
alias(notebook, "notepad").
alias(renovation_plan, "renovation plan").
alias(avoid_rework, "stops redoing work").
alias(job, "work item").
```

## teach
- To plan a home renovation in phases you need a tape measure, a notebook and a budget sheet.
- Planning a home renovation in phases takes 120 to 180 minutes.
- Step 1 of planning a home renovation in phases: measure each room with the tape measure, for 30 to 45 minutes.
- Step 2 of planning a home renovation in phases: list the jobs for each room in the notebook.
- Step 3 of planning a home renovation in phases: estimate the cost of each job on the budget sheet.
- Step 4 of planning a home renovation in phases: order the jobs into phases so later work does not undo earlier work.
- Step 5 of planning a home renovation in phases: share the phased plan with your household.

## checklist
- names what is needed: a tape measure, a notebook and a budget sheet
- gives the steps in order: measure the rooms, list the jobs, estimate the costs, order the jobs into phases, share the plan
- says ordering the jobs into phases is done so later work does not undo earlier work
- gives a total time of about 120 to 180 minutes

## variant
- change: no budget spreadsheet: estimate the costs in the notebook instead
- request: We have no spreadsheet for the budget, so how should we plan the phases of the renovation?
- situation: I have a tape measure and a notebook, but no spreadsheet for the budget.
- expect: The reply keeps the same five steps but estimates the costs in the notebook, since there is no budget spreadsheet.

```datalog situation
has(user, tape_measure).
has(user, notebook).
lacks(user, budget_sheet).
```

```datalog difference
swap(plan_home_renovation_phases_s3, budget_sheet, notebook).
no_need(budget_sheet).
```

## variant
- change: only one evening: skip sharing the plan and aim for 60 to 90 minutes
- request: We only have one evening free. Can you give me a quick phased plan for the house?
- situation: I have a tape measure, a notebook and a budget spreadsheet, and just one evening free.
- expect: The reply gives the four steps of measuring, listing, estimating and ordering the phases, leaves out sharing the plan, and says it takes about 60 to 90 minutes.

```datalog situation
has(user, tape_measure).
has(user, notebook).
has(user, budget_sheet).
time_limit(user, 90, minute).
```

```datalog difference
drop(plan_home_renovation_phases_s5).
change(plan_home_renovation_phases, takes_time, 120_180, 60_90).
```
