---
id: cook_rice
stage: procedures
task: cook_rice
---
# Cook rice

## requests
- I need to cook some white rice to go with dinner. How do I do it on the stove?
- My rice always ends up mushy. What's the right way to make it in a pot?
- Could you explain how to cook plain rice? I want it fluffy.

## situation
- I have white rice, water, a pot with a lid and a sieve.
- Dinner is in about 40 minutes.

## ground truth
```datalog
task(cook_rice).
needs(cook_rice, rice).
needs(cook_rice, water).
needs(cook_rice, pot).
needs(cook_rice, sieve).
takes_time(cook_rice, 25, 30, minute).
step(cook_rice, cook_rice_s1).
action(cook_rice_s1, boil).
object_of(cook_rice_s1, water).
uses(cook_rice_s1, pot).
takes_time(cook_rice_s1, 5, 6, minute).
step(cook_rice, cook_rice_s2).
action(cook_rice_s2, rinse).
object_of(cook_rice_s2, rice).
uses(cook_rice_s2, sieve).
until(cook_rice_s2, clear).
takes_time(cook_rice_s2, 1, 2, minute).
overlaps(cook_rice_s2, cook_rice_s1).
step(cook_rice, cook_rice_s3).
action(cook_rice_s3, add).
object_of(cook_rice_s3, rice).
step(cook_rice, cook_rice_s4).
action(cook_rice_s4, simmer).
object_of(cook_rice_s4, rice).
takes_time(cook_rice_s4, 15, 18, minute).
step(cook_rice, cook_rice_s5).
action(cook_rice_s5, rest).
object_of(cook_rice_s5, rice).
takes_time(cook_rice_s5, 5, 6, minute).
before(cook_rice_s1, cook_rice_s3).
before(cook_rice_s2, cook_rice_s3).
before(cook_rice_s3, cook_rice_s4).
before(cook_rice_s4, cook_rice_s5).
```

## teach
- To cook rice you need rice, water, a pot and a sieve.
- Cooking rice takes 25 to 30 minutes.
- Step 1 of cooking rice: boil the water in the pot for 5 to 6 minutes.
- While step 1 runs, step 2 of cooking rice: rinse the rice in the sieve until the water runs clear, for 1 to 2 minutes.
- Step 3 of cooking rice: add the rice to the boiling water.
- Step 4 of cooking rice: simmer the rice with the lid on for 15 to 18 minutes.
- Step 5 of cooking rice: rest the rice off the heat for 5 to 6 minutes.

## checklist
- names what is needed: rice, water, a pot and a sieve
- gives the steps in order: boil water, rinse rice (while the water heats), add rice, simmer, rest
- gives a total time of about 25 to 30 minutes
- closes politely

## variant
- change: brown rice instead of white: simmer for 35 to 40 minutes
- request: I bought brown rice instead of white this time. How do I cook it?
- situation: I have brown rice, water, a pot with a lid and a sieve, and I've got about an hour.
- expect: The reply keeps the same steps but lengthens the simmer to about 35 to 40 minutes and the total to about 45 to 52 minutes.
