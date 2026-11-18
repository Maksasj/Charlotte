---
id: remove_grease_stain
stage: household
task: remove_grease_stain
schema: v3
tier: T
---
# Remove a grease stain from clothes

## requests
- I dripped salad dressing on my shirt. How do I get the oil out?
- What's the way to get a grease mark out of a T-shirt?
- There's a greasy spot on my jeans from cooking. Can it be saved?

## situation
- I have paper towels, baking soda, washing-up liquid and a washing machine.

## ground truth
```datalog
task(remove_grease_stain).
domain(remove_grease_stain, cleaning).
goal(remove_grease_stain, stain_gone).
needs(remove_grease_stain, paper_towel).
needs(remove_grease_stain, baking_soda).
needs(remove_grease_stain, dish_soap).
needs(remove_grease_stain, washing_machine).
takes_time(remove_grease_stain, 75, 90, minute).
step(remove_grease_stain, remove_grease_stain_s1).
action(remove_grease_stain_s1, blot).
object_of(remove_grease_stain_s1, stain).
uses(remove_grease_stain_s1, paper_towel).
role(remove_grease_stain_s1, paper_towel, absorb).
avoid(remove_grease_stain_s1, rub, stain).
step(remove_grease_stain, remove_grease_stain_s2).
action(remove_grease_stain_s2, cover).
object_of(remove_grease_stain_s2, stain).
uses(remove_grease_stain_s2, baking_soda).
role(remove_grease_stain_s2, baking_soda, absorb).
takes_time(remove_grease_stain_s2, 15, 15, minute).
step(remove_grease_stain, remove_grease_stain_s3).
action(remove_grease_stain_s3, shake).
object_of(remove_grease_stain_s3, garment).
purpose(remove_grease_stain_s3, powder_removed).
step(remove_grease_stain, remove_grease_stain_s4).
action(remove_grease_stain_s4, rub).
object_of(remove_grease_stain_s4, stain).
uses(remove_grease_stain_s4, dish_soap).
role(remove_grease_stain_s4, dish_soap, clean).
takes_time(remove_grease_stain_s4, 1, 2, minute).
step(remove_grease_stain, remove_grease_stain_s5).
action(remove_grease_stain_s5, wash).
object_of(remove_grease_stain_s5, garment).
uses(remove_grease_stain_s5, washing_machine).
role(remove_grease_stain_s5, washing_machine, contain).
setting(remove_grease_stain_s5, temperature, 40, celsius).
takes_time(remove_grease_stain_s5, 50, 60, minute).
step(remove_grease_stain, remove_grease_stain_s6).
action(remove_grease_stain_s6, check).
object_of(remove_grease_stain_s6, stain).
check(remove_grease_stain_s6, stain_gone).
warning(remove_grease_stain_s6, heat_sets_stain).
before(remove_grease_stain_s1, remove_grease_stain_s2).
before(remove_grease_stain_s2, remove_grease_stain_s3).
before(remove_grease_stain_s3, remove_grease_stain_s4).
before(remove_grease_stain_s4, remove_grease_stain_s5).
before(remove_grease_stain_s5, remove_grease_stain_s6).
```

## aliases
```datalog
task_alias(remove_grease_stain, "getting grease out of clothes").
task_alias(remove_grease_stain, "removing an oil stain").
task_alias(remove_grease_stain, "grease stain").
alias(garment, "shirt").
alias(garment, "clothes").
alias(dish_soap, "washing-up liquid").
alias(heat_sets_stain, "a dryer can set the stain").
alias(stain_gone, "the stain is gone").
```

## teach
- To remove a grease stain from clothes you need paper towels, baking soda, washing-up liquid and a washing machine.
- Removing a grease stain takes 75 to 90 minutes, including the wash.
- Step 1 of removing a grease stain: blot the stain with paper towel; do not rub it.
- Step 2 of removing a grease stain: cover the stain with baking soda for 15 minutes to soak up the oil.
- Step 3 of removing a grease stain: shake the garment to remove the powder.
- Step 4 of removing a grease stain: rub washing-up liquid into the stain for 1 to 2 minutes.
- Step 5 of removing a grease stain: wash the garment in the washing machine at 40 degrees for 50 to 60 minutes.
- Step 6 of removing a grease stain: check the stain is gone before drying it, because a dryer's heat can set the stain.

## checklist
- names what is needed, including baking soda and washing-up liquid
- gives the steps in order: blot, cover with soda, shake off, rub in soap, wash, check
- says not to rub when blotting and to check before drying

## variant
- change: no baking soda: cover the stain with salt
- request: I want to get an oil stain out but there's no baking soda in the house. Anything else?
- situation: I have paper towels, table salt, washing-up liquid and a washing machine.
- expect: The reply covers the stain with salt instead of baking soda to soak up the grease, then carries on as normal.

```datalog situation
has(user, paper_towel).
has(user, salt).
has(user, dish_soap).
has(user, washing_machine).
lacks(user, baking_soda).
```

```datalog difference
swap(remove_grease_stain_s2, baking_soda, salt).
extra_need(salt).
no_need(baking_soda).
```

## variant
- change: washing machine broken: wash it by hand in a bucket
- request: My washing machine is broken until next week. How do I finish getting this grease out of my top?
- situation: I have paper towels, baking soda, washing-up liquid, a bucket and warm water; the washing machine is broken.
- expect: The reply hand-washes the garment in a bucket of warm soapy water instead of the machine, then checks the stain.

```datalog situation
has(user, paper_towel).
has(user, baking_soda).
has(user, dish_soap).
has(user, bucket).
has(user, warm_water).
lacks(user, washing_machine).
```

```datalog difference
swap(remove_grease_stain_s5, washing_machine, bucket).
change(remove_grease_stain_s5, takes_time, 50_60, 10_15).
change(remove_grease_stain, takes_time, 75_90, 35_45).
extra_need(bucket).
extra_need(warm_water).
no_need(washing_machine).
```
