---
id: sharpen_kitchen_knife
stage: tools
task: sharpen_kitchen_knife
schema: v3
---
# Sharpen a kitchen knife

## requests
- My chef's knife won't even cut a tomato. How do I sharpen it?
- Can you explain how to use a whetstone on a kitchen knife?
- I got a sharpening stone as a gift. How do I use it?

## situation
- I have a dull kitchen knife, a whetstone, water and a towel.
- I have about half an hour.

## ground truth
```datalog
task(sharpen_kitchen_knife).
domain(sharpen_kitchen_knife, repair).
goal(sharpen_kitchen_knife, sharp_knife).
needs(sharpen_kitchen_knife, whetstone).
needs(sharpen_kitchen_knife, water).
needs(sharpen_kitchen_knife, towel).
takes_time(sharpen_kitchen_knife, 20, 30, minute).
step(sharpen_kitchen_knife, sharpen_kitchen_knife_s1).
action(sharpen_kitchen_knife_s1, soak).
object_of(sharpen_kitchen_knife_s1, whetstone).
uses(sharpen_kitchen_knife_s1, water).
role(sharpen_kitchen_knife_s1, water, wet).
takes_time(sharpen_kitchen_knife_s1, 10, 15, minute).
step(sharpen_kitchen_knife, sharpen_kitchen_knife_s2).
action(sharpen_kitchen_knife_s2, place).
object_of(sharpen_kitchen_knife_s2, whetstone).
uses(sharpen_kitchen_knife_s2, towel).
role(sharpen_kitchen_knife_s2, towel, hold).
onto(sharpen_kitchen_knife_s2, towel).
purpose(sharpen_kitchen_knife_s2, stone_not_slide).
takes_time(sharpen_kitchen_knife_s2, 1, 1, minute).
step(sharpen_kitchen_knife, sharpen_kitchen_knife_s3).
action(sharpen_kitchen_knife_s3, sharpen).
object_of(sharpen_kitchen_knife_s3, knife).
uses(sharpen_kitchen_knife_s3, whetstone).
role(sharpen_kitchen_knife_s3, whetstone, sharpen).
with_part(sharpen_kitchen_knife_s3, blade).
setting(sharpen_kitchen_knife_s3, stone_side, coarse, level).
setting(sharpen_kitchen_knife_s3, angle, low, level).
repeat(sharpen_kitchen_knife_s3, 10).
takes_time(sharpen_kitchen_knife_s3, 5, 8, minute).
step(sharpen_kitchen_knife, sharpen_kitchen_knife_s4).
action(sharpen_kitchen_knife_s4, hone).
object_of(sharpen_kitchen_knife_s4, knife).
uses(sharpen_kitchen_knife_s4, whetstone).
role(sharpen_kitchen_knife_s4, whetstone, sharpen).
setting(sharpen_kitchen_knife_s4, stone_side, fine, level).
takes_time(sharpen_kitchen_knife_s4, 3, 5, minute).
step(sharpen_kitchen_knife, sharpen_kitchen_knife_s5).
action(sharpen_kitchen_knife_s5, rinse).
object_of(sharpen_kitchen_knife_s5, knife).
step(sharpen_kitchen_knife, sharpen_kitchen_knife_s6).
action(sharpen_kitchen_knife_s6, dry).
object_of(sharpen_kitchen_knife_s6, knife).
uses(sharpen_kitchen_knife_s6, towel).
role(sharpen_kitchen_knife_s6, towel, dry).
takes_time(sharpen_kitchen_knife_s6, 1, 1, minute).
before(sharpen_kitchen_knife_s1, sharpen_kitchen_knife_s2).
before(sharpen_kitchen_knife_s2, sharpen_kitchen_knife_s3).
before(sharpen_kitchen_knife_s3, sharpen_kitchen_knife_s4).
before(sharpen_kitchen_knife_s4, sharpen_kitchen_knife_s5).
before(sharpen_kitchen_knife_s5, sharpen_kitchen_knife_s6).
```

## aliases
```datalog
task_alias(sharpen_kitchen_knife, "sharpening a kitchen knife").
task_alias(sharpen_kitchen_knife, "sharpen a kitchen knife").
task_alias(sharpen_kitchen_knife, "use a whetstone on a kitchen knife").
alias(knife, "chef's knife").
alias(knife, "dull kitchen knife").
alias(knife, "kitchen knife").
alias(whetstone, "sharpening stone").
alias(whetstone, "stone").
alias(towel, "damp towel").
alias(coarse, "coarse side").
alias(fine, "fine side").
alias(low, "a low angle").
alias(stone_not_slide, "it does not slide").
alias(mug, "ceramic mug").
```

## teach
- To sharpen a kitchen knife you need a whetstone, water and a towel.
- Sharpening a kitchen knife takes 20 to 30 minutes.
- Step 1 of sharpening a kitchen knife: soak the whetstone in water for 10 to 15 minutes.
- Step 2 of sharpening a kitchen knife: place the whetstone on the damp towel so it does not slide for 1 minute.
- Step 3 of sharpening a kitchen knife: sharpen the knife on the coarse side, drawing the blade across the whetstone at a low angle ten times on each side, for 5 to 8 minutes.
- Step 4 of sharpening a kitchen knife: hone the knife on the fine side of the whetstone the same way for 3 to 5 minutes.
- Step 5 of sharpening a kitchen knife: rinse the knife.
- Step 6 of sharpening a kitchen knife: dry it with the towel for 1 minute.

## checklist
- names what is needed: a whetstone, water and a towel
- gives the steps in order: soak whetstone, place whetstone, sharpen knife, hone knife, rinse knife
- gives a total time of about 20 to 30 minutes
- closes politely

## variant
- change: no whetstone: use the rough unglazed ring on the bottom of a ceramic mug
- request: I need a sharper knife tonight but I don't own a sharpening stone. Is there a trick?
- situation: I have a dull kitchen knife, a ceramic mug with an unglazed bottom ring and a towel.
- expect: The reply skips soaking, turns the mug upside down on the towel and draws the blade across the unglazed ring at a low angle on each side, then rinses and dries the knife.

```datalog situation
has(user, knife).
has(user, mug).
has(user, towel).
lacks(user, whetstone).
```

```datalog difference
drop(sharpen_kitchen_knife_s1).
drop(sharpen_kitchen_knife_s4).
no_need(whetstone).
extra_need(mug).
change(sharpen_kitchen_knife_s2, object_of, whetstone, mug).
swap(sharpen_kitchen_knife_s3, whetstone, mug).
```
