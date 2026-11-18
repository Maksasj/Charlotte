---
id: make_tacos
stage: procedures
task: make_tacos
schema: v3
---
# Make beef tacos

## requests
- It's taco night. How do I make beef tacos with hard shells?
- Can you tell me how to cook the mince for tacos?
- What are the steps for making tacos for the family?

## situation
- I have taco shells, minced beef, taco seasoning, water, lettuce, grated cheese, a frying pan, a knife and an oven.
- Dinner is in 30 minutes.

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

## checklist
- names what is needed: taco shells, minced beef, taco seasoning, water, lettuce, grated cheese, a frying pan, a knife and an oven
- gives the steps in order: brown minced beef, shred lettuce (while brown minced beef), stir taco seasoning, warm taco shell (while stir taco seasoning), fill taco shell
- gives a total time of about 14 to 18 minutes
- closes politely

## variant
- change: vegetarian: heat black beans with the seasoning instead of browning beef
- request: One of my guests is vegetarian. Can I make the tacos without meat?
- situation: I have taco shells, a tin of black beans, taco seasoning, water, lettuce, grated cheese, a frying pan, a knife and an oven.
- expect: The reply drains the black beans and heats them with the seasoning and water instead of browning beef, keeping the shred, warm and fill steps.

```datalog situation
has(user, taco_shell).
has(user, black_bean).
has(user, taco_seasoning).
has(user, water).
has(user, lettuce).
has(user, cheese).
has(user, frying_pan).
has(user, knife).
has(user, oven).
excludes(user, meat).
```

```datalog difference
add_step(make_tacos_s9, start).
step(make_tacos, make_tacos_s9).
action(make_tacos_s9, drain).
object_of(make_tacos_s9, black_bean).
before(make_tacos_s9, make_tacos_s1).
change(make_tacos_s1, action, brown, heat).
change(make_tacos_s1, object_of, minced_beef, black_bean).
change(make_tacos_s3, into, minced_beef, black_bean).
change(make_tacos_s4, object_of, minced_beef, black_bean).
swap(make_tacos_s6, minced_beef, black_bean).
extra_need(black_bean).
no_need(minced_beef).
```
