---
id: make_sandwich
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Make a sandwich

Build a simple ham and cheese sandwich.

## ground truth
```datalog
task(make_sandwich).
domain(make_sandwich, cooking).
goal(make_sandwich, plan_meal).
needs(make_sandwich, bread).
needs(make_sandwich, butter).
needs(make_sandwich, ham).
needs(make_sandwich, cheese).
needs(make_sandwich, lettuce).
needs(make_sandwich, knife).
takes_time(make_sandwich, 5, 10, minute).
step(make_sandwich, make_sandwich_s1).
action(make_sandwich_s1, spread).
object_of(make_sandwich_s1, butter).
uses(make_sandwich_s1, knife).
role(make_sandwich_s1, knife, spread).
uses(make_sandwich_s1, bread).
role(make_sandwich_s1, bread, hold).
takes_time(make_sandwich_s1, 1, 2, minute).
onto(make_sandwich_s1, bread).
step(make_sandwich, make_sandwich_s2).
action(make_sandwich_s2, add).
object_of(make_sandwich_s2, ham).
takes_time(make_sandwich_s2, 1, 1, minute).
step(make_sandwich, make_sandwich_s3).
action(make_sandwich_s3, add).
object_of(make_sandwich_s3, cheese).
takes_time(make_sandwich_s3, 1, 1, minute).
step(make_sandwich, make_sandwich_s4).
action(make_sandwich_s4, add).
object_of(make_sandwich_s4, lettuce).
takes_time(make_sandwich_s4, 1, 1, minute).
step(make_sandwich, make_sandwich_s5).
action(make_sandwich_s5, close).
object_of(make_sandwich_s5, sandwich).
uses(make_sandwich_s5, bread).
role(make_sandwich_s5, bread, cover).
step(make_sandwich, make_sandwich_s6).
action(make_sandwich_s6, cut).
object_of(make_sandwich_s6, sandwich).
uses(make_sandwich_s6, knife).
role(make_sandwich_s6, knife, cut).
takes_time(make_sandwich_s6, 1, 1, minute).
before(make_sandwich_s1, make_sandwich_s2).
before(make_sandwich_s2, make_sandwich_s3).
before(make_sandwich_s3, make_sandwich_s4).
before(make_sandwich_s4, make_sandwich_s5).
before(make_sandwich_s5, make_sandwich_s6).
```

## aliases
```datalog
task_alias(make_sandwich, "making a sandwich").
task_alias(make_sandwich, "ham and cheese sandwich").
task_alias(make_sandwich, "make a ham sandwich").
alias(bread, "slice of bread").
alias(bread, "slices of bread").
alias(ham, "sliced ham").
alias(cheese, "cheese slice").
alias(lettuce, "lettuce leaf").
alias(close, "top").
alias(spread, "butter").
```
