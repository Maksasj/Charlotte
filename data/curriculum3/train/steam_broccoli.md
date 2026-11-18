---
id: steam_broccoli
stage: procedures
task: steam_broccoli
schema: v3
---
# Steam broccoli

## requests
- How long do I steam broccoli so it's not mushy?
- I want a quick green side dish. Can you tell me how to steam some broccoli?
- What's the right way to steam a head of broccoli in a pot?

## situation
- I have a head of broccoli, water, salt, a knife, a pot with a lid and a steamer basket.
- The rest of dinner is ready in 15 minutes.

## ground truth
```datalog
task(steam_broccoli).
domain(steam_broccoli, cooking).
goal(steam_broccoli, steamed_broccoli).
goal(steam_broccoli, side_dish).
needs(steam_broccoli, broccoli).
needs(steam_broccoli, water).
needs(steam_broccoli, salt).
needs(steam_broccoli, knife).
needs(steam_broccoli, pot).
needs(steam_broccoli, steamer_basket).
takes_time(steam_broccoli, 9, 12, minute).
step(steam_broccoli, steam_broccoli_s1).
action(steam_broccoli_s1, boil).
object_of(steam_broccoli_s1, water).
uses(steam_broccoli_s1, pot).
role(steam_broccoli_s1, pot, contain).
takes_time(steam_broccoli_s1, 4, 5, minute).
step(steam_broccoli, steam_broccoli_s2).
action(steam_broccoli_s2, cut).
object_of(steam_broccoli_s2, broccoli).
uses(steam_broccoli_s2, knife).
role(steam_broccoli_s2, knife, cut).
achieves(steam_broccoli_s2, floret).
takes_time(steam_broccoli_s2, 2, 3, minute).
overlaps(steam_broccoli_s2, steam_broccoli_s1).
step(steam_broccoli, steam_broccoli_s3).
action(steam_broccoli_s3, steam).
object_of(steam_broccoli_s3, broccoli).
uses(steam_broccoli_s3, steamer_basket).
role(steam_broccoli_s3, steamer_basket, hold).
uses(steam_broccoli_s3, pot).
role(steam_broccoli_s3, pot, contain).
requires_state(steam_broccoli_s3, lid_on).
until(steam_broccoli_s3, tender).
check(steam_broccoli_s3, bright_green).
takes_time(steam_broccoli_s3, 4, 6, minute).
step(steam_broccoli, steam_broccoli_s4).
action(steam_broccoli_s4, season).
object_of(steam_broccoli_s4, broccoli).
uses(steam_broccoli_s4, salt).
role(steam_broccoli_s4, salt, season).
before(steam_broccoli_s1, steam_broccoli_s3).
before(steam_broccoli_s2, steam_broccoli_s3).
before(steam_broccoli_s3, steam_broccoli_s4).
```

## aliases
```datalog
task_alias(steam_broccoli, "steaming broccoli").
task_alias(steam_broccoli, "steam some broccoli").
task_alias(steam_broccoli, "steam a head of broccoli").
alias(broccoli, "head of broccoli").
alias(broccoli, "florets").
alias(floret, "florets").
alias(steamer_basket, "steamer basket").
alias(steamer_basket, "basket").
alias(pot, "pot with a lid").
alias(lid_on, "with the lid on").
alias(bright_green, "bright green").
alias(drain, "drain them").
```

## teach
- To steam broccoli you need broccoli, water, salt, a knife, a pot with a lid and a steamer basket.
- Steaming broccoli takes 9 to 12 minutes.
- Step 1 of steaming broccoli: boil a little water in the pot for 4 to 5 minutes.
- While step 1 runs, step 2 of steaming broccoli: cut the broccoli into florets with the knife for 2 to 3 minutes.
- Step 3 of steaming broccoli: steam the broccoli in the basket over the water in the pot with the lid on until bright green and tender, for 4 to 6 minutes.
- Step 4 of steaming broccoli: season the broccoli with salt.

## checklist
- names what is needed: broccoli, water, salt, a knife, a pot with a lid and a steamer basket
- gives the steps in order: boil water, cut broccoli (while boil water), steam broccoli, season broccoli
- gives a total time of about 9 to 12 minutes
- closes politely

## variant
- change: no steamer basket: boil the florets in a little water for 3 to 4 minutes and drain them
- request: I don't have a steamer basket. How else can I cook broccoli quickly?
- situation: I have broccoli, water, salt, a knife and a pot with a lid, but no steamer basket.
- expect: The reply boils the florets in a little water for about 3 to 4 minutes and drains them instead of steaming, and keeps the cut and season steps.

```datalog situation
has(user, broccoli).
has(user, water).
has(user, salt).
has(user, knife).
has(user, pot).
lacks(user, steamer_basket).
```

```datalog difference
drop(steam_broccoli_s3).
no_need(steamer_basket).
add_step(steam_broccoli_s9, steam_broccoli_s2).
step(steam_broccoli, steam_broccoli_s9).
action(steam_broccoli_s9, boil).
object_of(steam_broccoli_s9, broccoli).
uses(steam_broccoli_s9, water).
role(steam_broccoli_s9, water, heat).
uses(steam_broccoli_s9, pot).
role(steam_broccoli_s9, pot, contain).
takes_time(steam_broccoli_s9, 3, 4, minute).
before(steam_broccoli_s1, steam_broccoli_s9).
before(steam_broccoli_s2, steam_broccoli_s9).
add_step(steam_broccoli_s10, steam_broccoli_s9).
step(steam_broccoli, steam_broccoli_s10).
action(steam_broccoli_s10, drain).
object_of(steam_broccoli_s10, broccoli).
before(steam_broccoli_s9, steam_broccoli_s10).
before(steam_broccoli_s10, steam_broccoli_s4).
```
