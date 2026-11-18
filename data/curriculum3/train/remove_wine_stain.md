---
id: remove_wine_stain
stage: household
task: remove_wine_stain
schema: v3
---
# Remove a wine stain

## requests
- I just spilled red wine on my white tablecloth! How do I get it out?
- What's the best way to get a fresh red wine stain out of fabric?
- Help, there's wine on my shirt. What do I do right now?

## situation
- I have paper towels, salt, cold water and dish soap.
- The spill happened a minute ago.

## ground truth
```datalog
task(remove_wine_stain).
domain(remove_wine_stain, cleaning).
goal(remove_wine_stain, clean_fabric).
needs(remove_wine_stain, paper_towel).
needs(remove_wine_stain, salt).
needs(remove_wine_stain, water).
needs(remove_wine_stain, dish_soap).
takes_time(remove_wine_stain, 13, 20, minute).
step(remove_wine_stain, remove_wine_stain_s1).
action(remove_wine_stain_s1, blot).
object_of(remove_wine_stain_s1, stain).
uses(remove_wine_stain_s1, paper_towel).
role(remove_wine_stain_s1, paper_towel, absorb).
avoid(remove_wine_stain_s1, rub, stain).
takes_time(remove_wine_stain_s1, 1, 2, minute).
step(remove_wine_stain, remove_wine_stain_s2).
action(remove_wine_stain_s2, cover).
object_of(remove_wine_stain_s2, stain).
uses(remove_wine_stain_s2, salt).
role(remove_wine_stain_s2, salt, absorb).
purpose(remove_wine_stain_s2, wine_absorbed).
takes_time(remove_wine_stain_s2, 5, 10, minute).
step(remove_wine_stain, remove_wine_stain_s3).
action(remove_wine_stain_s3, rinse).
object_of(remove_wine_stain_s3, stain).
uses(remove_wine_stain_s3, water).
role(remove_wine_stain_s3, water, clean).
with_part(remove_wine_stain_s3, back).
takes_time(remove_wine_stain_s3, 2, 3, minute).
step(remove_wine_stain, remove_wine_stain_s4).
action(remove_wine_stain_s4, rub).
object_of(remove_wine_stain_s4, stain).
uses(remove_wine_stain_s4, dish_soap).
role(remove_wine_stain_s4, dish_soap, clean).
amount(remove_wine_stain_s4, dish_soap, 1, drop).
step(remove_wine_stain, remove_wine_stain_s5).
action(remove_wine_stain_s5, leave).
object_of(remove_wine_stain_s5, stain).
takes_time(remove_wine_stain_s5, 5, 5, minute).
before(remove_wine_stain_s1, remove_wine_stain_s2).
before(remove_wine_stain_s2, remove_wine_stain_s3).
before(remove_wine_stain_s3, remove_wine_stain_s4).
before(remove_wine_stain_s4, remove_wine_stain_s5).
```

## aliases
```datalog
task_alias(remove_wine_stain, "removing a wine stain").
task_alias(remove_wine_stain, "remove a wine stain").
task_alias(remove_wine_stain, "get a red wine stain out").
alias(stain, "wine stain").
alias(stain, "red wine stain").
alias(paper_towel, "paper towels").
alias(water, "cold water").
alias(dish_soap, "a drop of dish soap").
alias(back, "from the back").
alias(wine_absorbed, "soak up the wine").
alias(cloth, "clean cloth").
alias(dry, "blot it dry").
```

## teach
- To remove a wine stain you need paper towels, salt, cold water and dish soap.
- Removing a wine stain takes 13 to 20 minutes.
- Step 1 of removing a wine stain: blot the stain with the paper towel without rubbing for 1 to 2 minutes.
- Step 2 of removing a wine stain: cover the stain with salt to soak up the wine for 5 to 10 minutes.
- Step 3 of removing a wine stain: rinse the stain from the back with cold water for 2 to 3 minutes.
- Step 4 of removing a wine stain: rub a drop of dish soap into the stain.
- Step 5 of removing a wine stain: leave it for 5 minutes.

## checklist
- names what is needed: paper towels, salt, cold water and dish soap
- gives the steps in order: blot stain, cover stain, rinse stain, rub stain
- gives a total time of about 13 to 20 minutes
- closes politely

## variant
- change: stain on a carpet: blot, salt, then dab with soapy water and blot dry instead of rinsing from the back
- request: I knocked a glass of red wine onto the carpet. How do I deal with it?
- situation: I have paper towels, salt, cold water, dish soap and a clean cloth. The carpet can't be taken up.
- expect: The reply keeps the blot and salt steps but dabs the stain with cold soapy water on the cloth and blots it dry instead of rinsing from the back.

```datalog situation
has(user, paper_towel).
has(user, salt).
has(user, water).
has(user, dish_soap).
has(user, cloth).
```

```datalog difference
drop(remove_wine_stain_s3).
drop(remove_wine_stain_s4).
drop(remove_wine_stain_s5).
extra_need(cloth).
add_step(remove_wine_stain_s9, remove_wine_stain_s2).
step(remove_wine_stain, remove_wine_stain_s9).
action(remove_wine_stain_s9, dab).
object_of(remove_wine_stain_s9, stain).
uses(remove_wine_stain_s9, water).
role(remove_wine_stain_s9, water, clean).
uses(remove_wine_stain_s9, dish_soap).
role(remove_wine_stain_s9, dish_soap, clean).
uses(remove_wine_stain_s9, cloth).
role(remove_wine_stain_s9, cloth, wipe).
before(remove_wine_stain_s2, remove_wine_stain_s9).
add_step(remove_wine_stain_s10, remove_wine_stain_s9).
step(remove_wine_stain, remove_wine_stain_s10).
action(remove_wine_stain_s10, blot).
object_of(remove_wine_stain_s10, stain).
achieves(remove_wine_stain_s10, dry).
before(remove_wine_stain_s9, remove_wine_stain_s10).
```
