---
id: make_mulled_wine__alcohol_free
stage: procedures
task: make_mulled_wine
schema: v3
tier: S
base: make_mulled_wine
---
# Make mulled wine

## requests
- Some of the guests at our winter party don't drink. Can I do a booze-free version of the spiced warm wine?

## ground truth
```datalog
task(make_mulled_wine).
domain(make_mulled_wine, cooking).
goal(make_mulled_wine, mulled_wine).
needs(make_mulled_wine, red_wine).
needs(make_mulled_wine, orange).
needs(make_mulled_wine, cinnamon_stick).
needs(make_mulled_wine, clove).
needs(make_mulled_wine, sugar).
needs(make_mulled_wine, knife).
needs(make_mulled_wine, saucepan).
needs(make_mulled_wine, ladle).
needs(make_mulled_wine, mug).
takes_time(make_mulled_wine, 18, 24, minute).
step(make_mulled_wine, make_mulled_wine_s1).
action(make_mulled_wine_s1, slice).
object_of(make_mulled_wine_s1, orange).
uses(make_mulled_wine_s1, knife).
role(make_mulled_wine_s1, knife, cut).
takes_time(make_mulled_wine_s1, 1, 2, minute).
step(make_mulled_wine, make_mulled_wine_s2).
action(make_mulled_wine_s2, pour).
object_of(make_mulled_wine_s2, red_wine).
uses(make_mulled_wine_s2, saucepan).
role(make_mulled_wine_s2, saucepan, contain).
into(make_mulled_wine_s2, saucepan).
step(make_mulled_wine, make_mulled_wine_s3).
action(make_mulled_wine_s3, add).
object_of(make_mulled_wine_s3, cinnamon_stick).
uses(make_mulled_wine_s3, orange).
role(make_mulled_wine_s3, orange, flavour).
uses(make_mulled_wine_s3, clove).
role(make_mulled_wine_s3, clove, flavour).
uses(make_mulled_wine_s3, sugar).
role(make_mulled_wine_s3, sugar, sweeten).
into(make_mulled_wine_s3, saucepan).
step(make_mulled_wine, make_mulled_wine_s4).
action(make_mulled_wine_s4, warm).
object_of(make_mulled_wine_s4, red_wine).
setting(make_mulled_wine_s4, heat, low, level).
until(make_mulled_wine_s4, steaming).
avoid(make_mulled_wine_s4, boil, red_wine).
takes_time(make_mulled_wine_s4, 15, 20, minute).
step(make_mulled_wine, make_mulled_wine_s5).
action(make_mulled_wine_s5, ladle).
object_of(make_mulled_wine_s5, red_wine).
uses(make_mulled_wine_s5, ladle).
role(make_mulled_wine_s5, ladle, pour).
uses(make_mulled_wine_s5, mug).
role(make_mulled_wine_s5, mug, contain).
into(make_mulled_wine_s5, mug).
before(make_mulled_wine_s1, make_mulled_wine_s2).
before(make_mulled_wine_s2, make_mulled_wine_s3).
before(make_mulled_wine_s3, make_mulled_wine_s4).
before(make_mulled_wine_s4, make_mulled_wine_s5).
```

## aliases
```datalog
task_alias(make_mulled_wine, "making mulled wine").
task_alias(make_mulled_wine, "pot of mulled wine").
task_alias(make_mulled_wine, "spice and warm a bottle of red wine").
alias(red_wine, "wine").
alias(red_wine, "bottle of red wine").
alias(orange, "orange slices").
alias(cinnamon_stick, "cinnamon sticks").
alias(clove, "cloves").
alias(mug, "mugs").
alias(low, "gently").
alias(boil, "letting it boil").
```

## teach
- To make mulled wine you need a bottle of red wine, an orange, cinnamon sticks, cloves, sugar, a knife, a saucepan, a ladle and mugs.
- Making mulled wine takes 18 to 24 minutes.
- Step 1 of making mulled wine: slice the orange with the knife for 1 to 2 minutes.
- Step 2 of making mulled wine: pour the red wine into the saucepan.
- Step 3 of making mulled wine: add the orange slices, the cinnamon sticks, the cloves and the sugar.
- Step 4 of making mulled wine: warm the wine gently until steaming, without letting it boil, for 15 to 20 minutes.
- Step 5 of making mulled wine: ladle the wine into the mugs.

## variant
- change: alcohol-free: use red grape juice instead of wine
- request: Some of the guests at our winter party don't drink. Can I do a booze-free version of the spiced warm wine?
- situation: Nobody can have alcohol. I have red grape juice, an orange, cinnamon sticks, cloves, sugar, a knife, a saucepan, a ladle and mugs.
- expect: The reply uses red grape juice in place of the red wine and keeps the spices, gentle warming and ladling.

```datalog situation
has(user, grape_juice).
has(user, orange).
has(user, cinnamon_stick).
has(user, clove).
has(user, sugar).
has(user, knife).
has(user, saucepan).
has(user, ladle).
has(user, mug).
excludes(user, alcohol).
```

```datalog difference
change(make_mulled_wine_s2, object_of, red_wine, grape_juice).
change(make_mulled_wine_s4, object_of, red_wine, grape_juice).
change(make_mulled_wine_s5, object_of, red_wine, grape_juice).
no_need(red_wine).
extra_need(grape_juice).
```
