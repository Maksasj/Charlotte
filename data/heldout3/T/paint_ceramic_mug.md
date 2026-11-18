---
id: paint_ceramic_mug
stage: tools
task: paint_ceramic_mug
schema: v3
tier: T
---
# Paint a ceramic mug

## requests
- I want to decorate a plain white mug as a present. How do I do it so it lasts?
- How do you paint a design on a ceramic mug with porcelain pens?
- Can you explain how to personalise a mug at home?

## situation
- I have a plain white mug, washing-up liquid, a tea towel, rubbing alcohol, porcelain pens and an oven.

## ground truth
```datalog
task(paint_ceramic_mug).
domain(paint_ceramic_mug, crafts).
goal(paint_ceramic_mug, painted_mug).
goal(paint_ceramic_mug, gift).
needs(paint_ceramic_mug, mug).
needs(paint_ceramic_mug, dish_soap).
needs(paint_ceramic_mug, towel).
needs(paint_ceramic_mug, rubbing_alcohol).
needs(paint_ceramic_mug, porcelain_pen).
needs(paint_ceramic_mug, oven).
takes_time(paint_ceramic_mug, 25, 26, hour).
step(paint_ceramic_mug, paint_ceramic_mug_s1).
action(paint_ceramic_mug_s1, wash).
object_of(paint_ceramic_mug_s1, mug).
uses(paint_ceramic_mug_s1, dish_soap).
role(paint_ceramic_mug_s1, dish_soap, clean).
step(paint_ceramic_mug, paint_ceramic_mug_s2).
action(paint_ceramic_mug_s2, dry).
object_of(paint_ceramic_mug_s2, mug).
uses(paint_ceramic_mug_s2, towel).
role(paint_ceramic_mug_s2, towel, dry).
step(paint_ceramic_mug, paint_ceramic_mug_s3).
action(paint_ceramic_mug_s3, wipe).
object_of(paint_ceramic_mug_s3, mug).
uses(paint_ceramic_mug_s3, rubbing_alcohol).
role(paint_ceramic_mug_s3, rubbing_alcohol, clean).
purpose(paint_ceramic_mug_s3, grease_free).
step(paint_ceramic_mug, paint_ceramic_mug_s4).
action(paint_ceramic_mug_s4, draw).
object_of(paint_ceramic_mug_s4, design).
uses(paint_ceramic_mug_s4, porcelain_pen).
role(paint_ceramic_mug_s4, porcelain_pen, write).
avoid(paint_ceramic_mug_s4, touch, design).
step(paint_ceramic_mug, paint_ceramic_mug_s5).
action(paint_ceramic_mug_s5, leave).
object_of(paint_ceramic_mug_s5, mug).
takes_time(paint_ceramic_mug_s5, 24, 24, hour).
achieves(paint_ceramic_mug_s5, paint_dry).
step(paint_ceramic_mug, paint_ceramic_mug_s6).
action(paint_ceramic_mug_s6, bake).
object_of(paint_ceramic_mug_s6, mug).
uses(paint_ceramic_mug_s6, oven).
role(paint_ceramic_mug_s6, oven, heat).
setting(paint_ceramic_mug_s6, temperature, 150, celsius).
takes_time(paint_ceramic_mug_s6, 35, 35, minute).
requires_state(paint_ceramic_mug_s6, oven_cold_at_start).
before(paint_ceramic_mug_s1, paint_ceramic_mug_s2).
before(paint_ceramic_mug_s2, paint_ceramic_mug_s3).
before(paint_ceramic_mug_s3, paint_ceramic_mug_s4).
before(paint_ceramic_mug_s4, paint_ceramic_mug_s5).
before(paint_ceramic_mug_s5, paint_ceramic_mug_s6).
```

## aliases
```datalog
task_alias(paint_ceramic_mug, "decorating a mug").
task_alias(paint_ceramic_mug, "painting a mug").
task_alias(paint_ceramic_mug, "personalising a mug").
alias(mug, "plain white mug").
alias(porcelain_pen, "porcelain pens").
alias(towel, "tea towel").
alias(design, "the design").
alias(grease_free, "free of grease").
alias(oven_cold_at_start, "put it in a cold oven").
```

## teach
- To paint a ceramic mug you need the mug, washing-up liquid, a towel, rubbing alcohol, porcelain pens and an oven.
- Painting a ceramic mug takes 25 to 26 hours, mostly drying.
- Step 1 of painting a ceramic mug: wash the mug with washing-up liquid.
- Step 2 of painting a ceramic mug: dry the mug with the towel.
- Step 3 of painting a ceramic mug: wipe the mug with rubbing alcohol so it is free of grease.
- Step 4 of painting a ceramic mug: draw the design with the porcelain pens; do not touch the design.
- Step 5 of painting a ceramic mug: leave the mug for 24 hours until the paint is dry.
- Step 6 of painting a ceramic mug: bake the mug at 150 degrees for 35 minutes, putting it in a cold oven.

## checklist
- names what is needed, including the mug, rubbing alcohol, porcelain pens and an oven
- gives the steps in order: wash, dry, wipe with alcohol, draw, leave 24 hours, bake
- says to start the mug in a cold oven

## variant
- change: no rubbing alcohol: wipe with glass cleaner
- request: I don't have rubbing alcohol to degrease the mug. Will anything else do?
- situation: I have the mug, washing-up liquid, a towel, glass cleaner, porcelain pens and an oven.
- expect: The reply wipes the mug with glass cleaner instead of rubbing alcohol before drawing.

```datalog situation
has(user, mug).
has(user, dish_soap).
has(user, towel).
has(user, glass_cleaner).
has(user, porcelain_pen).
has(user, oven).
lacks(user, rubbing_alcohol).
```

```datalog difference
swap(paint_ceramic_mug_s3, rubbing_alcohol, glass_cleaner).
extra_need(glass_cleaner).
no_need(rubbing_alcohol).
```

## variant
- change: gift is tonight: dry 4 hours only and say it is for display
- request: The birthday is this evening, about five hours away. Can I still paint a mug in time?
- situation: I have everything for painting the mug, and 5 hours.
- expect: The reply shortens the drying to about 4 hours before baking and warns the design will be less hard-wearing, so hand-wash only.

```datalog situation
has(user, mug).
has(user, dish_soap).
has(user, towel).
has(user, rubbing_alcohol).
has(user, porcelain_pen).
has(user, oven).
time_limit(user, 5, hour).
```

```datalog difference
change(paint_ceramic_mug_s5, takes_time, 24_24_hour, 4_4_hour).
change(paint_ceramic_mug, takes_time, 25_26_hour, 5_5_hour).
```
