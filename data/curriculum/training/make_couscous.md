---
id: make_couscous
stage: procedures
task: make_couscous
---
# Make couscous

## requests
- How do I make couscous? The packet instructions are worn off.
- Can you explain how to make fluffy couscous with just a kettle?
- I need a fast side for dinner. How do I prepare couscous?

## situation
- I have couscous, salt, water, a kettle, a bowl, a plate and a fork.
- I have about 15 minutes.

## ground truth
```datalog
task(make_couscous).
needs(make_couscous, couscous).
needs(make_couscous, salt).
needs(make_couscous, water).
needs(make_couscous, kettle).
needs(make_couscous, bowl).
needs(make_couscous, plate).
needs(make_couscous, fork).
takes_time(make_couscous, 9, 11, minute).
step(make_couscous, make_couscous_s1).
action(make_couscous_s1, boil).
object_of(make_couscous_s1, water).
uses(make_couscous_s1, kettle).
takes_time(make_couscous_s1, 3, 4, minute).
step(make_couscous, make_couscous_s2).
action(make_couscous_s2, measure).
object_of(make_couscous_s2, couscous).
uses(make_couscous_s2, bowl).
uses(make_couscous_s2, salt).
takes_time(make_couscous_s2, 1, 1, minute).
overlaps(make_couscous_s2, make_couscous_s1).
step(make_couscous, make_couscous_s3).
action(make_couscous_s3, pour).
object_of(make_couscous_s3, water).
uses(make_couscous_s3, bowl).
until(make_couscous_s3, covered).
step(make_couscous, make_couscous_s4).
action(make_couscous_s4, cover).
object_of(make_couscous_s4, bowl).
uses(make_couscous_s4, plate).
takes_time(make_couscous_s4, 5, 5, minute).
step(make_couscous, make_couscous_s5).
action(make_couscous_s5, fluff).
object_of(make_couscous_s5, couscous).
uses(make_couscous_s5, fork).
takes_time(make_couscous_s5, 1, 1, minute).
before(make_couscous_s1, make_couscous_s3).
before(make_couscous_s2, make_couscous_s3).
before(make_couscous_s3, make_couscous_s4).
before(make_couscous_s4, make_couscous_s5).
```

## teach
- To make couscous you need couscous, salt, water, a kettle, a bowl, a plate and a fork.
- Making couscous takes 9 to 11 minutes.
- Step 1 of making couscous: boil the water in the kettle for 3 to 4 minutes.
- While step 1 runs, step 2 of making couscous: measure the couscous into the bowl with a pinch of salt for 1 minute.
- Step 3 of making couscous: pour the boiling water over the couscous in the bowl until it is just covered.
- Step 4 of making couscous: cover the bowl with the plate and let it stand for 5 minutes.
- Step 5 of making couscous: fluff the couscous with the fork for 1 minute.

## checklist
- names what is needed: couscous, salt, water, a kettle, a bowl, a plate and a fork
- gives the steps in order: boil water, measure couscous (while boil water), pour water, cover bowl, fluff couscous
- gives a total time of about 9 to 11 minutes
- closes politely

## variant
- change: hot stock instead of water: pour stock over the couscous for more flavour
- request: I've got a carton of chicken stock. Can I use it to make couscous taste better?
- situation: I have couscous, chicken stock, a saucepan, a bowl, a plate and a fork.
- expect: The reply heats the stock in the saucepan and pours it over the couscous in place of boiled water, keeping the cover and fluff steps.
