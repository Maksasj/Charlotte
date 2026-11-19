---
id: carve_wooden_spoon
stage: procedures
task: carve_wooden_spoon
schema: v3
---
# Carve a wooden spoon

## requests
- How do I carve a wooden spoon from a block of wood?
- Could you walk me through whittling a spoon by hand?
- What's the process for making a wooden spoon with a knife?

## situation
- I have a block of birch, a pencil, a carving knife, sandpaper and some food-safe oil.
- I have a free afternoon, about two to three hours.

## ground truth
```datalog
task(carve_wooden_spoon).
domain(carve_wooden_spoon, crafts).
goal(carve_wooden_spoon, wooden_spoon).
needs(carve_wooden_spoon, wood_block).
needs(carve_wooden_spoon, pencil).
needs(carve_wooden_spoon, carving_knife).
needs(carve_wooden_spoon, sandpaper).
needs(carve_wooden_spoon, food_safe_oil).
takes_time(carve_wooden_spoon, 120, 180, minute).
step(carve_wooden_spoon, carve_wooden_spoon_s1).
action(carve_wooden_spoon_s1, mark).
object_of(carve_wooden_spoon_s1, wood_block).
uses(carve_wooden_spoon_s1, pencil).
role(carve_wooden_spoon_s1, pencil, mark).
step(carve_wooden_spoon, carve_wooden_spoon_s2).
action(carve_wooden_spoon_s2, cut).
object_of(carve_wooden_spoon_s2, wood_block).
uses(carve_wooden_spoon_s2, carving_knife).
role(carve_wooden_spoon_s2, carving_knife, cut).
step(carve_wooden_spoon, carve_wooden_spoon_s3).
action(carve_wooden_spoon_s3, carve).
object_of(carve_wooden_spoon_s3, wood_block).
uses(carve_wooden_spoon_s3, carving_knife).
role(carve_wooden_spoon_s3, carving_knife, cut).
step(carve_wooden_spoon, carve_wooden_spoon_s4).
action(carve_wooden_spoon_s4, sand).
object_of(carve_wooden_spoon_s4, wood_block).
uses(carve_wooden_spoon_s4, sandpaper).
role(carve_wooden_spoon_s4, sandpaper, scrub).
step(carve_wooden_spoon, carve_wooden_spoon_s5).
action(carve_wooden_spoon_s5, coat).
object_of(carve_wooden_spoon_s5, wood_block).
uses(carve_wooden_spoon_s5, food_safe_oil).
role(carve_wooden_spoon_s5, food_safe_oil, coat).
optional(carve_wooden_spoon_s5).
before(carve_wooden_spoon_s1, carve_wooden_spoon_s2).
before(carve_wooden_spoon_s2, carve_wooden_spoon_s3).
before(carve_wooden_spoon_s3, carve_wooden_spoon_s4).
before(carve_wooden_spoon_s4, carve_wooden_spoon_s5).
```

## aliases
```datalog
task_alias(carve_wooden_spoon, "carving a wooden spoon").
task_alias(carve_wooden_spoon, "whittling a spoon").
alias(wood_block, "block of wood").
alias(wood_block, "spoon blank").
alias(carving_knife, "knife").
alias(food_safe_oil, "food-safe oil").
alias(sandpaper, "sanding paper").
alias(mark, "draw").
alias(cut, "rough out").
alias(carve, "hollow out").
alias(sand, "smooth").
alias(coat, "oil").
```

## teach
- To carve a wooden spoon you need a block of wood, a pencil, a carving knife, sandpaper and food-safe oil.
- Carving a wooden spoon takes 120 to 180 minutes.
- Step 1 of carving a wooden spoon: mark the spoon outline on the wood block with the pencil.
- Step 2 of carving a wooden spoon: cut the wood block to a rough spoon shape with the carving knife.
- Step 3 of carving a wooden spoon: carve the spoon bowl out of the wood block with the carving knife.
- Step 4 of carving a wooden spoon: sand the spoon smooth with sandpaper.
- Step 5 of carving a wooden spoon: coat the spoon with food-safe oil; this step is optional.

## checklist
- names what is needed, including a block of wood, a carving knife, sandpaper and food-safe oil
- gives the steps in order: mark the outline, rough out the shape, carve the bowl, sand smooth, oil
- says the oiling step is optional
- gives a total time of about 120 to 180 minutes

## variant
- change: no carving knife: use a pocket knife for the rough cut and the bowl carving
- request: I don't own a carving knife, only a pocket knife. Can I still carve a spoon with it?
- situation: I have a block of birch, a pencil, a pocket knife, sandpaper and food-safe oil.
- expect: The reply uses the pocket knife in place of the carving knife for the rough cut and the bowl carving, and keeps the same steps in the same order.

```datalog situation
has(user, wood_block).
has(user, pencil).
has(user, pocket_knife).
has(user, sandpaper).
has(user, food_safe_oil).
lacks(user, carving_knife).
```

```datalog difference
swap(carve_wooden_spoon_s2, carving_knife, pocket_knife).
swap(carve_wooden_spoon_s3, carving_knife, pocket_knife).
no_need(carving_knife).
extra_need(pocket_knife).
```

## variant
- change: no food-safe oil: skip the oiling step
- request: I don't have any food-safe oil. Do I have to oil the spoon?
- situation: I have a block of birch, a pencil, a carving knife and sandpaper, but no oil.
- expect: The reply says the oiling step can be skipped, so the spoon is carved and sanded but left unoiled.

```datalog situation
has(user, wood_block).
has(user, pencil).
has(user, carving_knife).
has(user, sandpaper).
lacks(user, food_safe_oil).
```

```datalog difference
drop(carve_wooden_spoon_s5).
no_need(food_safe_oil).
```
