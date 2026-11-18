---
id: make_pesto__no_food_processor
stage: procedures
task: make_pesto
schema: v3
tier: S
base: make_pesto
---
# Make pesto

## requests
- I want to whizz up a fresh basil sauce for pasta tonight, but I only have a blender, no food processor. Will that do?

## ground truth
```datalog
task(make_pesto).
domain(make_pesto, cooking).
goal(make_pesto, pesto).
needs(make_pesto, basil).
needs(make_pesto, pine_nut).
needs(make_pesto, parmesan).
needs(make_pesto, garlic).
needs(make_pesto, olive_oil).
needs(make_pesto, frying_pan).
needs(make_pesto, grater).
needs(make_pesto, food_processor).
takes_time(make_pesto, 5, 8, minute).
step(make_pesto, make_pesto_s1).
action(make_pesto_s1, toast).
object_of(make_pesto_s1, pine_nut).
uses(make_pesto_s1, frying_pan).
role(make_pesto_s1, frying_pan, heat).
requires_state(make_pesto_s1, pan_dry).
until(make_pesto_s1, golden).
takes_time(make_pesto_s1, 2, 3, minute).
step(make_pesto, make_pesto_s2).
action(make_pesto_s2, grate).
object_of(make_pesto_s2, parmesan).
uses(make_pesto_s2, grater).
role(make_pesto_s2, grater, cut).
overlaps(make_pesto_s2, make_pesto_s1).
takes_time(make_pesto_s2, 2, 2, minute).
step(make_pesto, make_pesto_s3).
action(make_pesto_s3, blend).
object_of(make_pesto_s3, basil).
uses(make_pesto_s3, food_processor).
role(make_pesto_s3, food_processor, blend).
uses(make_pesto_s3, pine_nut).
role(make_pesto_s3, pine_nut, mix).
uses(make_pesto_s3, parmesan).
role(make_pesto_s3, parmesan, flavour).
uses(make_pesto_s3, garlic).
role(make_pesto_s3, garlic, flavour).
takes_time(make_pesto_s3, 1, 2, minute).
step(make_pesto, make_pesto_s4).
action(make_pesto_s4, pour).
object_of(make_pesto_s4, olive_oil).
uses(make_pesto_s4, food_processor).
role(make_pesto_s4, food_processor, blend).
into(make_pesto_s4, food_processor).
setting(make_pesto_s4, speed, slow, level).
until(make_pesto_s4, smooth).
takes_time(make_pesto_s4, 1, 2, minute).
before(make_pesto_s1, make_pesto_s3).
before(make_pesto_s2, make_pesto_s3).
before(make_pesto_s3, make_pesto_s4).
```

## aliases
```datalog
task_alias(make_pesto, "making pesto").
task_alias(make_pesto, "basil pesto").
task_alias(make_pesto, "homemade pesto").
alias(pine_nut, "pine nuts").
alias(frying_pan, "small frying pan").
alias(frying_pan, "dry frying pan").
alias(food_processor, "processor").
alias(basil, "bunch of basil").
alias(olive_oil, "oil").
alias(slow, "slowly").
alias(pan_dry, "dry frying pan").
```

## teach
- To make pesto you need basil, pine nuts, parmesan, garlic, olive oil, a small frying pan, a grater and a food processor.
- Making pesto takes 5 to 8 minutes.
- Step 1 of making pesto: toast the pine nuts in the dry frying pan until golden, for 2 to 3 minutes.
- While step 1 runs, step 2 of making pesto: grate the parmesan with the grater for 2 minutes.
- Step 3 of making pesto: blend the basil with the pine nuts, the parmesan and the garlic in the food processor, for 1 to 2 minutes.
- Step 4 of making pesto: pour the olive oil slowly into the food processor while blending until smooth, for 1 to 2 minutes.

## variant
- change: no food processor: blend the pesto in a blender
- request: I want to whizz up a fresh basil sauce for pasta tonight, but I only have a blender, no food processor. Will that do?
- situation: I have basil, pine nuts, parmesan, garlic, olive oil, a small frying pan, a grater and a blender, but no food processor.
- expect: The reply blends the basil, nuts, cheese and garlic in the blender and streams the oil into the blender, keeping the toast and grate steps.

```datalog situation
has(user, basil).
has(user, pine_nut).
has(user, parmesan).
has(user, garlic).
has(user, olive_oil).
has(user, frying_pan).
has(user, grater).
has(user, blender).
lacks(user, food_processor).
```

```datalog difference
swap(make_pesto_s3, food_processor, blender).
swap(make_pesto_s4, food_processor, blender).
change(make_pesto_s4, into, food_processor, blender).
no_need(food_processor).
extra_need(blender).
```
