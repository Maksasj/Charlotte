---
id: make_couscous
stage: procedures
task: make_couscous
schema: v3
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
domain(make_couscous, cooking).
goal(make_couscous, cooked_couscous).
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
role(make_couscous_s1, kettle, heat).
takes_time(make_couscous_s1, 3, 4, minute).
step(make_couscous, make_couscous_s2).
action(make_couscous_s2, measure).
object_of(make_couscous_s2, couscous).
uses(make_couscous_s2, bowl).
role(make_couscous_s2, bowl, contain).
uses(make_couscous_s2, salt).
role(make_couscous_s2, salt, season).
into(make_couscous_s2, bowl).
amount(make_couscous_s2, salt, 1, pinch).
overlaps(make_couscous_s2, make_couscous_s1).
takes_time(make_couscous_s2, 1, 1, minute).
step(make_couscous, make_couscous_s3).
action(make_couscous_s3, pour).
object_of(make_couscous_s3, water).
uses(make_couscous_s3, bowl).
role(make_couscous_s3, bowl, contain).
onto(make_couscous_s3, couscous).
requires_state(make_couscous_s3, water_boiling).
until(make_couscous_s3, covered).
step(make_couscous, make_couscous_s4).
action(make_couscous_s4, cover).
object_of(make_couscous_s4, bowl).
uses(make_couscous_s4, plate).
role(make_couscous_s4, plate, cover).
step(make_couscous, make_couscous_s5).
action(make_couscous_s5, stand).
object_of(make_couscous_s5, couscous).
purpose(make_couscous_s5, water_absorbed).
takes_time(make_couscous_s5, 5, 5, minute).
step(make_couscous, make_couscous_s6).
action(make_couscous_s6, fluff).
object_of(make_couscous_s6, couscous).
uses(make_couscous_s6, fork).
role(make_couscous_s6, fork, loosen).
takes_time(make_couscous_s6, 1, 1, minute).
before(make_couscous_s1, make_couscous_s3).
before(make_couscous_s2, make_couscous_s3).
before(make_couscous_s3, make_couscous_s4).
before(make_couscous_s4, make_couscous_s5).
before(make_couscous_s5, make_couscous_s6).
```

## aliases
```datalog
task_alias(make_couscous, "making couscous").
task_alias(make_couscous, "fluffy couscous").
task_alias(make_couscous, "prepare couscous").
alias(water, "boiling water").
alias(water, "boiled water").
alias(salt, "pinch of salt").
alias(covered, "just covered").
alias(stand, "let it stand").
alias(water_absorbed, "the couscous soaks up the water").
alias(chicken_stock, "stock").
alias(chicken_stock, "hot stock").
alias(chicken_stock, "carton of chicken stock").
```

## teach
- To make couscous you need couscous, salt, water, a kettle, a bowl, a plate and a fork.
- Making couscous takes 9 to 11 minutes.
- Step 1 of making couscous: boil the water in the kettle for 3 to 4 minutes.
- While step 1 runs, step 2 of making couscous: measure the couscous into the bowl with a pinch of salt for 1 minute.
- Step 3 of making couscous: pour the boiling water over the couscous in the bowl until it is just covered.
- Step 4 of making couscous: cover the bowl with the plate.
- Step 5 of making couscous: let it stand for 5 minutes so the couscous soaks up the water.
- Step 6 of making couscous: fluff the couscous with the fork for 1 minute.

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

```datalog situation
has(user, couscous).
has(user, chicken_stock).
has(user, saucepan).
has(user, bowl).
has(user, plate).
has(user, fork).
```

```datalog difference
change(make_couscous_s1, action, boil, heat).
change(make_couscous_s1, object_of, water, chicken_stock).
swap(make_couscous_s1, kettle, saucepan).
change(make_couscous_s3, object_of, water, chicken_stock).
extra_need(chicken_stock).
extra_need(saucepan).
no_need(water).
no_need(kettle).
```
