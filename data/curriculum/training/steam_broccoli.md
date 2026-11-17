---
id: steam_broccoli
stage: procedures
task: steam_broccoli
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
takes_time(steam_broccoli_s1, 4, 5, minute).
step(steam_broccoli, steam_broccoli_s2).
action(steam_broccoli_s2, cut).
object_of(steam_broccoli_s2, broccoli).
uses(steam_broccoli_s2, knife).
takes_time(steam_broccoli_s2, 2, 3, minute).
overlaps(steam_broccoli_s2, steam_broccoli_s1).
step(steam_broccoli, steam_broccoli_s3).
action(steam_broccoli_s3, steam).
object_of(steam_broccoli_s3, broccoli).
uses(steam_broccoli_s3, steamer_basket).
uses(steam_broccoli_s3, pot).
until(steam_broccoli_s3, tender).
takes_time(steam_broccoli_s3, 4, 6, minute).
step(steam_broccoli, steam_broccoli_s4).
action(steam_broccoli_s4, season).
object_of(steam_broccoli_s4, broccoli).
uses(steam_broccoli_s4, salt).
before(steam_broccoli_s1, steam_broccoli_s3).
before(steam_broccoli_s2, steam_broccoli_s3).
before(steam_broccoli_s3, steam_broccoli_s4).
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
