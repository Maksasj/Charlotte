---
id: grilled_cheese
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Grilled cheese

Fry a buttered cheese sandwich until golden on both sides.

## ground truth
```datalog
task(grilled_cheese).
domain(grilled_cheese, cooking).
goal(grilled_cheese, plan_meal).
needs(grilled_cheese, bread).
needs(grilled_cheese, cheese).
needs(grilled_cheese, butter).
needs(grilled_cheese, pan).
needs(grilled_cheese, knife).
takes_time(grilled_cheese, 8, 12, minute).
step(grilled_cheese, grilled_cheese_s1).
action(grilled_cheese_s1, spread).
object_of(grilled_cheese_s1, butter).
uses(grilled_cheese_s1, knife).
role(grilled_cheese_s1, knife, spread).
uses(grilled_cheese_s1, bread).
role(grilled_cheese_s1, bread, hold).
takes_time(grilled_cheese_s1, 1, 2, minute).
onto(grilled_cheese_s1, bread).
step(grilled_cheese, grilled_cheese_s2).
action(grilled_cheese_s2, add).
object_of(grilled_cheese_s2, cheese).
uses(grilled_cheese_s2, bread).
role(grilled_cheese_s2, bread, hold).
takes_time(grilled_cheese_s2, 1, 1, minute).
onto(grilled_cheese_s2, bread).
step(grilled_cheese, grilled_cheese_s3).
action(grilled_cheese_s3, heat).
object_of(grilled_cheese_s3, pan).
takes_time(grilled_cheese_s3, 1, 2, minute).
until(grilled_cheese_s3, hot).
step(grilled_cheese, grilled_cheese_s4).
action(grilled_cheese_s4, fry).
object_of(grilled_cheese_s4, sandwich).
uses(grilled_cheese_s4, pan).
role(grilled_cheese_s4, pan, heat).
takes_time(grilled_cheese_s4, 2, 3, minute).
until(grilled_cheese_s4, golden).
step(grilled_cheese, grilled_cheese_s5).
action(grilled_cheese_s5, flip).
object_of(grilled_cheese_s5, sandwich).
uses(grilled_cheese_s5, pan).
role(grilled_cheese_s5, pan, heat).
takes_time(grilled_cheese_s5, 2, 3, minute).
until(grilled_cheese_s5, golden).
step(grilled_cheese, grilled_cheese_s6).
action(grilled_cheese_s6, cut).
object_of(grilled_cheese_s6, sandwich).
uses(grilled_cheese_s6, knife).
role(grilled_cheese_s6, knife, cut).
overlaps(grilled_cheese_s3, grilled_cheese_s1).
before(grilled_cheese_s1, grilled_cheese_s2).
before(grilled_cheese_s2, grilled_cheese_s4).
before(grilled_cheese_s3, grilled_cheese_s4).
before(grilled_cheese_s4, grilled_cheese_s5).
before(grilled_cheese_s5, grilled_cheese_s6).
```

## aliases
```datalog
task_alias(grilled_cheese, "grilled cheese sandwich").
task_alias(grilled_cheese, "making a grilled cheese").
task_alias(grilled_cheese, "cheese toastie").
task_alias(grilled_cheese, "make grilled cheese").
alias(pan, "frying pan").
alias(pan, "skillet").
alias(bread, "slice of bread").
alias(bread, "bread slices").
alias(sandwich, "cheese sandwich").
alias(fry, "grill").
alias(golden, "golden brown").
alias(flip, "turn over").
alias(spread, "butter").
```
