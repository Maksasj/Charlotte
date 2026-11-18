---
id: make_tacos__ten_people
stage: procedures
task: make_tacos
schema: v3
tier: S
base: make_tacos
---
# Make beef tacos

## requests
- Taco night for ten! My frying pan only fits one packet of mince at a time. How do I manage?

## ground truth
```datalog
task(make_tacos).
domain(make_tacos, cooking).
goal(make_tacos, taco).
needs(make_tacos, taco_shell).
needs(make_tacos, minced_beef).
needs(make_tacos, taco_seasoning).
needs(make_tacos, water).
needs(make_tacos, lettuce).
needs(make_tacos, cheese).
needs(make_tacos, frying_pan).
needs(make_tacos, knife).
needs(make_tacos, oven).
takes_time(make_tacos, 14, 18, minute).
step(make_tacos, make_tacos_s1).
action(make_tacos_s1, brown).
object_of(make_tacos_s1, minced_beef).
uses(make_tacos_s1, frying_pan).
role(make_tacos_s1, frying_pan, heat).
takes_time(make_tacos_s1, 6, 8, minute).
step(make_tacos, make_tacos_s2).
action(make_tacos_s2, shred).
object_of(make_tacos_s2, lettuce).
uses(make_tacos_s2, knife).
role(make_tacos_s2, knife, cut).
overlaps(make_tacos_s2, make_tacos_s1).
takes_time(make_tacos_s2, 3, 4, minute).
step(make_tacos, make_tacos_s3).
action(make_tacos_s3, stir).
object_of(make_tacos_s3, taco_seasoning).
uses(make_tacos_s3, water).
role(make_tacos_s3, water, mix).
into(make_tacos_s3, minced_beef).
step(make_tacos, make_tacos_s4).
action(make_tacos_s4, simmer).
object_of(make_tacos_s4, minced_beef).
takes_time(make_tacos_s4, 4, 5, minute).
step(make_tacos, make_tacos_s5).
action(make_tacos_s5, warm).
object_of(make_tacos_s5, taco_shell).
uses(make_tacos_s5, oven).
role(make_tacos_s5, oven, heat).
overlaps(make_tacos_s5, make_tacos_s4).
takes_time(make_tacos_s5, 3, 5, minute).
step(make_tacos, make_tacos_s6).
action(make_tacos_s6, fill).
object_of(make_tacos_s6, taco_shell).
uses(make_tacos_s6, minced_beef).
role(make_tacos_s6, minced_beef, fill).
uses(make_tacos_s6, lettuce).
role(make_tacos_s6, lettuce, fill).
uses(make_tacos_s6, cheese).
role(make_tacos_s6, cheese, fill).
takes_time(make_tacos_s6, 2, 3, minute).
before(make_tacos_s1, make_tacos_s3).
before(make_tacos_s2, make_tacos_s3).
before(make_tacos_s3, make_tacos_s4).
before(make_tacos_s1, make_tacos_s5).
before(make_tacos_s4, make_tacos_s6).
before(make_tacos_s5, make_tacos_s6).
```

## aliases
```datalog
task_alias(make_tacos, "making beef tacos").
task_alias(make_tacos, "beef tacos").
task_alias(make_tacos, "taco night").
alias(taco_shell, "taco shells").
alias(taco_shell, "hard shells").
alias(taco_shell, "shells").
alias(minced_beef, "mince").
alias(minced_beef, "beef").
alias(taco_seasoning, "seasoning").
alias(cheese, "grated cheese").
alias(frying_pan, "pan").
alias(stir, "stir in").
alias(black_bean, "black beans").
alias(black_bean, "tin of black beans").
```

## teach
- To make beef tacos you need taco shells, minced beef, taco seasoning, water, lettuce, grated cheese, a frying pan, a knife and an oven.
- Making beef tacos takes 14 to 18 minutes.
- Step 1 of making beef tacos: brown the minced beef in the frying pan, breaking it up, for 6 to 8 minutes.
- While step 1 runs, step 2 of making beef tacos: shred the lettuce with the knife for 3 to 4 minutes.
- Step 3 of making beef tacos: stir in the taco seasoning with a splash of water.
- Step 4 of making beef tacos: simmer the beef for 4 to 5 minutes.
- While step 4 runs, step 5 of making beef tacos: warm the taco shells in the oven for 3 to 5 minutes.
- Step 6 of making beef tacos: fill the taco shells with the beef, the lettuce and the cheese, for 2 to 3 minutes.

## variant
- change: ten people: brown the beef and warm the shells in two batches
- request: Taco night for ten! My frying pan only fits one packet of mince at a time. How do I manage?
- situation: Ten people are eating. I have taco shells, two packs of minced beef, taco seasoning, water, lettuce, grated cheese, a frying pan, a knife and an oven.
- expect: The reply browns the beef in two batches and warms the shells in two trays, taking about 25 to 30 minutes in total.

```datalog situation
has(user, taco_shell).
has(user, minced_beef).
has(user, taco_seasoning).
has(user, water).
has(user, lettuce).
has(user, cheese).
has(user, frying_pan).
has(user, knife).
has(user, oven).
count(user, guest, 10).
```

```datalog difference
change(make_tacos_s1, repeat, 1, 2).
change(make_tacos_s5, repeat, 1, 2).
change(make_tacos, takes_time, 14_18, 25_30).
```
