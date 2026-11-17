---
id: make_tacos
stage: procedures
task: make_tacos
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
takes_time(make_tacos_s1, 6, 8, minute).
step(make_tacos, make_tacos_s2).
action(make_tacos_s2, shred).
object_of(make_tacos_s2, lettuce).
uses(make_tacos_s2, knife).
takes_time(make_tacos_s2, 3, 4, minute).
overlaps(make_tacos_s2, make_tacos_s1).
step(make_tacos, make_tacos_s3).
action(make_tacos_s3, stir).
object_of(make_tacos_s3, taco_seasoning).
uses(make_tacos_s3, water).
takes_time(make_tacos_s3, 4, 5, minute).
step(make_tacos, make_tacos_s4).
action(make_tacos_s4, warm).
object_of(make_tacos_s4, taco_shell).
uses(make_tacos_s4, oven).
takes_time(make_tacos_s4, 3, 5, minute).
overlaps(make_tacos_s4, make_tacos_s3).
step(make_tacos, make_tacos_s5).
action(make_tacos_s5, fill).
object_of(make_tacos_s5, taco_shell).
uses(make_tacos_s5, minced_beef).
uses(make_tacos_s5, lettuce).
uses(make_tacos_s5, cheese).
takes_time(make_tacos_s5, 2, 3, minute).
before(make_tacos_s1, make_tacos_s3).
before(make_tacos_s2, make_tacos_s3).
before(make_tacos_s1, make_tacos_s4).
before(make_tacos_s3, make_tacos_s5).
before(make_tacos_s4, make_tacos_s5).
```

## teach
- To make beef tacos you need taco shells, minced beef, taco seasoning, water, lettuce, grated cheese, a frying pan, a knife and an oven.
- Making beef tacos takes 14 to 18 minutes.
- Step 1 of making beef tacos: brown the minced beef in the frying pan, breaking it up, for 6 to 8 minutes.
- While step 1 runs, step 2 of making beef tacos: shred the lettuce with the knife for 3 to 4 minutes.
- Step 3 of making beef tacos: stir in the taco seasoning with a splash of water and simmer for 4 to 5 minutes.
- While step 3 runs, step 4 of making beef tacos: warm the taco shells in the oven for 3 to 5 minutes.
- Step 5 of making beef tacos: fill the taco shells with the beef, the lettuce and the cheese, for 2 to 3 minutes.

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
