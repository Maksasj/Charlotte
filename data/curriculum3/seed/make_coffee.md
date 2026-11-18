---
id: make_coffee
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Make coffee

Brew a mug of coffee in a French press.

## ground truth
```datalog
task(make_coffee).
domain(make_coffee, cooking).
goal(make_coffee, coffee).
needs(make_coffee, ground_coffee).
needs(make_coffee, water).
needs(make_coffee, kettle).
needs(make_coffee, french_press).
needs(make_coffee, mug).
takes_time(make_coffee, 8, 10, minute).
step(make_coffee, make_coffee_s1).
action(make_coffee_s1, boil).
object_of(make_coffee_s1, water).
uses(make_coffee_s1, kettle).
role(make_coffee_s1, kettle, heat).
takes_time(make_coffee_s1, 3, 4, minute).
step(make_coffee, make_coffee_s2).
action(make_coffee_s2, put).
object_of(make_coffee_s2, ground_coffee).
uses(make_coffee_s2, french_press).
role(make_coffee_s2, french_press, contain).
takes_time(make_coffee_s2, 1, 1, minute).
into(make_coffee_s2, french_press).
step(make_coffee, make_coffee_s3).
action(make_coffee_s3, pour).
object_of(make_coffee_s3, water).
uses(make_coffee_s3, french_press).
role(make_coffee_s3, french_press, contain).
takes_time(make_coffee_s3, 1, 1, minute).
into(make_coffee_s3, french_press).
step(make_coffee, make_coffee_s4).
action(make_coffee_s4, steep).
object_of(make_coffee_s4, coffee).
takes_time(make_coffee_s4, 4, 4, minute).
step(make_coffee, make_coffee_s5).
action(make_coffee_s5, pour).
object_of(make_coffee_s5, coffee).
uses(make_coffee_s5, mug).
role(make_coffee_s5, mug, contain).
takes_time(make_coffee_s5, 1, 1, minute).
into(make_coffee_s5, mug).
overlaps(make_coffee_s2, make_coffee_s1).
before(make_coffee_s1, make_coffee_s3).
before(make_coffee_s2, make_coffee_s3).
before(make_coffee_s3, make_coffee_s4).
before(make_coffee_s4, make_coffee_s5).
```

## aliases
```datalog
task_alias(make_coffee, "making coffee").
task_alias(make_coffee, "brew coffee").
task_alias(make_coffee, "French press coffee").
task_alias(make_coffee, "a mug of coffee").
alias(french_press, "French press").
alias(french_press, "cafetiere").
alias(french_press, "coffee press").
alias(ground_coffee, "ground coffee").
alias(ground_coffee, "coffee grounds").
alias(mug, "cup").
alias(steep, "brew").
alias(water, "hot water").
```
