---
id: make_hot_chocolate__wants_marshmallow
stage: procedures
task: make_hot_chocolate
schema: v3
tier: S
base: make_hot_chocolate
---
# Make hot chocolate

## requests
- Making cocoa for the kids and they're begging for marshmallows on top. When do they go in?

## ground truth
```datalog
task(make_hot_chocolate).
domain(make_hot_chocolate, cooking).
goal(make_hot_chocolate, hot_chocolate).
needs(make_hot_chocolate, milk).
needs(make_hot_chocolate, cocoa_powder).
needs(make_hot_chocolate, sugar).
needs(make_hot_chocolate, saucepan).
needs(make_hot_chocolate, mug).
takes_time(make_hot_chocolate, 6, 9, minute).
step(make_hot_chocolate, make_hot_chocolate_s1).
action(make_hot_chocolate_s1, heat).
object_of(make_hot_chocolate_s1, milk).
uses(make_hot_chocolate_s1, saucepan).
role(make_hot_chocolate_s1, saucepan, heat).
setting(make_hot_chocolate_s1, heat, medium, level).
takes_time(make_hot_chocolate_s1, 4, 5, minute).
step(make_hot_chocolate, make_hot_chocolate_s2).
action(make_hot_chocolate_s2, mix).
object_of(make_hot_chocolate_s2, cocoa_powder).
uses(make_hot_chocolate_s2, sugar).
role(make_hot_chocolate_s2, sugar, sweeten).
uses(make_hot_chocolate_s2, mug).
role(make_hot_chocolate_s2, mug, contain).
uses(make_hot_chocolate_s2, milk).
role(make_hot_chocolate_s2, milk, mix).
overlaps(make_hot_chocolate_s2, make_hot_chocolate_s1).
takes_time(make_hot_chocolate_s2, 1, 2, minute).
step(make_hot_chocolate, make_hot_chocolate_s3).
action(make_hot_chocolate_s3, pour).
object_of(make_hot_chocolate_s3, milk).
uses(make_hot_chocolate_s3, mug).
role(make_hot_chocolate_s3, mug, contain).
into(make_hot_chocolate_s3, mug).
requires_state(make_hot_chocolate_s3, milk_hot).
step(make_hot_chocolate, make_hot_chocolate_s4).
action(make_hot_chocolate_s4, stir).
object_of(make_hot_chocolate_s4, cocoa_powder).
into(make_hot_chocolate_s4, milk).
until(make_hot_chocolate_s4, smooth).
takes_time(make_hot_chocolate_s4, 1, 2, minute).
before(make_hot_chocolate_s1, make_hot_chocolate_s3).
before(make_hot_chocolate_s2, make_hot_chocolate_s3).
before(make_hot_chocolate_s3, make_hot_chocolate_s4).
```

## aliases
```datalog
task_alias(make_hot_chocolate, "making hot chocolate").
task_alias(make_hot_chocolate, "hot cocoa").
task_alias(make_hot_chocolate, "mug of hot chocolate").
alias(milk, "hot milk").
alias(milk, "cold milk").
alias(milk, "regular milk").
alias(milk, "cow's milk").
alias(cocoa_powder, "cocoa").
alias(oat_milk, "oat milk").
alias(medium, "medium heat").
```

## teach
- To make hot chocolate you need milk, cocoa powder, sugar, a saucepan and a mug.
- Making hot chocolate takes 6 to 9 minutes.
- Step 1 of making hot chocolate: heat the milk in the saucepan over medium heat for 4 to 5 minutes.
- While step 1 runs, step 2 of making hot chocolate: mix the cocoa powder and the sugar in the mug with a splash of cold milk for 1 to 2 minutes.
- Step 3 of making hot chocolate: pour the hot milk into the mug.
- Step 4 of making hot chocolate: stir the cocoa powder into the milk until smooth, for 1 to 2 minutes.

## variant
- change: add marshmallows on top at the end
- request: Making cocoa for the kids and they're begging for marshmallows on top. When do they go in?
- situation: I have milk, cocoa powder, sugar, a saucepan, a mug and a bag of mini marshmallows.
- expect: The reply makes the hot chocolate as usual and drops the marshmallows on top after the final stir.

```datalog situation
has(user, milk).
has(user, cocoa_powder).
has(user, sugar).
has(user, saucepan).
has(user, mug).
has(user, marshmallow).
wants(user, marshmallow).
```

```datalog difference
extra_need(marshmallow).
add_step(make_hot_chocolate_s9, make_hot_chocolate_s4).
step(make_hot_chocolate, make_hot_chocolate_s9).
action(make_hot_chocolate_s9, top).
object_of(make_hot_chocolate_s9, marshmallow).
onto(make_hot_chocolate_s9, milk).
before(make_hot_chocolate_s4, make_hot_chocolate_s9).
```
