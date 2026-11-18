---
id: make_chili_con_carne__35_minutes
stage: procedures
task: make_chili_con_carne
schema: v3
tier: S
base: make_chili_con_carne
---
# Make chili con carne

## requests
- I've only got about 35 minutes to get a beef and bean chili on the table. Doable?

## ground truth
```datalog
task(make_chili_con_carne).
domain(make_chili_con_carne, cooking).
goal(make_chili_con_carne, chili).
needs(make_chili_con_carne, minced_beef).
needs(make_chili_con_carne, onion).
needs(make_chili_con_carne, kidney_bean).
needs(make_chili_con_carne, chopped_tomato).
needs(make_chili_con_carne, chili_powder).
needs(make_chili_con_carne, oil).
needs(make_chili_con_carne, knife).
needs(make_chili_con_carne, pot).
takes_time(make_chili_con_carne, 45, 60, minute).
step(make_chili_con_carne, make_chili_con_carne_s1).
action(make_chili_con_carne_s1, chop).
object_of(make_chili_con_carne_s1, onion).
uses(make_chili_con_carne_s1, knife).
role(make_chili_con_carne_s1, knife, cut).
takes_time(make_chili_con_carne_s1, 3, 4, minute).
step(make_chili_con_carne, make_chili_con_carne_s2).
action(make_chili_con_carne_s2, fry).
object_of(make_chili_con_carne_s2, onion).
uses(make_chili_con_carne_s2, pot).
role(make_chili_con_carne_s2, pot, heat).
uses(make_chili_con_carne_s2, oil).
role(make_chili_con_carne_s2, oil, heat).
until(make_chili_con_carne_s2, soft).
takes_time(make_chili_con_carne_s2, 5, 6, minute).
step(make_chili_con_carne, make_chili_con_carne_s3).
action(make_chili_con_carne_s3, brown).
object_of(make_chili_con_carne_s3, minced_beef).
uses(make_chili_con_carne_s3, pot).
role(make_chili_con_carne_s3, pot, heat).
takes_time(make_chili_con_carne_s3, 5, 6, minute).
step(make_chili_con_carne, make_chili_con_carne_s4).
action(make_chili_con_carne_s4, add).
object_of(make_chili_con_carne_s4, chopped_tomato).
uses(make_chili_con_carne_s4, kidney_bean).
role(make_chili_con_carne_s4, kidney_bean, mix).
uses(make_chili_con_carne_s4, chili_powder).
role(make_chili_con_carne_s4, chili_powder, flavour).
into(make_chili_con_carne_s4, pot).
takes_time(make_chili_con_carne_s4, 1, 2, minute).
step(make_chili_con_carne, make_chili_con_carne_s5).
action(make_chili_con_carne_s5, simmer).
object_of(make_chili_con_carne_s5, chili).
uses(make_chili_con_carne_s5, pot).
role(make_chili_con_carne_s5, pot, heat).
setting(make_chili_con_carne_s5, lid, half_on, level).
takes_time(make_chili_con_carne_s5, 30, 40, minute).
step(make_chili_con_carne, make_chili_con_carne_s6).
action(make_chili_con_carne_s6, stir).
object_of(make_chili_con_carne_s6, chili).
overlaps(make_chili_con_carne_s6, make_chili_con_carne_s5).
before(make_chili_con_carne_s1, make_chili_con_carne_s2).
before(make_chili_con_carne_s2, make_chili_con_carne_s3).
before(make_chili_con_carne_s3, make_chili_con_carne_s4).
before(make_chili_con_carne_s4, make_chili_con_carne_s5).
```

## aliases
```datalog
task_alias(make_chili_con_carne, "making chili con carne").
task_alias(make_chili_con_carne, "chili con carne").
task_alias(make_chili_con_carne, "pot of chili").
task_alias(make_chili_con_carne, "batch of chili").
alias(minced_beef, "mince").
alias(minced_beef, "beef").
alias(kidney_bean, "kidney beans").
alias(kidney_bean, "tin of kidney beans").
alias(chopped_tomato, "chopped tomatoes").
alias(chopped_tomato, "tin of chopped tomatoes").
alias(pot, "large pot").
alias(half_on, "lid half on").
alias(lentil, "lentils").
alias(lentil, "tin of lentils").
alias(lentil, "drained lentils").
```

## teach
- To make chili con carne you need minced beef, an onion, a tin of kidney beans, a tin of chopped tomatoes, chili powder, oil, a knife and a large pot.
- Making chili con carne takes 45 to 60 minutes.
- Step 1 of making chili con carne: chop the onion with the knife for 3 to 4 minutes.
- Step 2 of making chili con carne: fry the onion in the oil in the pot until soft, for 5 to 6 minutes.
- Step 3 of making chili con carne: brown the minced beef with the onion in the pot for 5 to 6 minutes.
- Step 4 of making chili con carne: add the chopped tomatoes, the kidney beans and the chili powder, for 1 to 2 minutes.
- Step 5 of making chili con carne: simmer the chili in the pot with the lid half on for 30 to 40 minutes.
- While step 5 runs, step 6 of making chili con carne: stir the chili now and then.

## variant
- change: only 35 minutes: simmer for 15 minutes instead of 30 to 40
- request: I've only got about 35 minutes to get a beef and bean chili on the table. Doable?
- situation: I have minced beef, an onion, kidney beans, chopped tomatoes, chili powder, oil, a knife and a large pot, and about 35 minutes.
- expect: The reply keeps all the steps but cuts the simmer to about 15 minutes so the whole dish takes about 30 minutes.

```datalog situation
has(user, minced_beef).
has(user, onion).
has(user, kidney_bean).
has(user, chopped_tomato).
has(user, chili_powder).
has(user, oil).
has(user, knife).
has(user, pot).
time_limit(user, 35, minute).
```

```datalog difference
change(make_chili_con_carne_s5, takes_time, 30_40, 15_15).
change(make_chili_con_carne, takes_time, 45_60, 30_33).
```
