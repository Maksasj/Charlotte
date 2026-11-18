---
id: boil_corn_on_cob
stage: procedures
task: boil_corn_on_cob
schema: v3
tier: T
---
# Boil corn on the cob

## requests
- How long do I boil corn on the cob for?
- I bought four ears of sweetcorn at the market. How do I cook them?
- What's the method for cooking whole corn cobs so they're juicy?

## situation
- I have four corn cobs, a large pot, water, butter, salt and tongs.

## ground truth
```datalog
task(boil_corn_on_cob).
domain(boil_corn_on_cob, cooking).
goal(boil_corn_on_cob, corn_on_cob).
goal(boil_corn_on_cob, side_dish).
needs(boil_corn_on_cob, corn_cob).
needs(boil_corn_on_cob, pot).
needs(boil_corn_on_cob, water).
needs(boil_corn_on_cob, butter).
needs(boil_corn_on_cob, salt).
needs(boil_corn_on_cob, tongs).
serves(boil_corn_on_cob, 4).
takes_time(boil_corn_on_cob, 15, 20, minute).
step(boil_corn_on_cob, boil_corn_on_cob_s1).
action(boil_corn_on_cob_s1, husk).
object_of(boil_corn_on_cob_s1, corn_cob).
purpose(boil_corn_on_cob_s1, clean_cob).
step(boil_corn_on_cob, boil_corn_on_cob_s2).
action(boil_corn_on_cob_s2, boil).
object_of(boil_corn_on_cob_s2, water).
uses(boil_corn_on_cob_s2, pot).
role(boil_corn_on_cob_s2, pot, contain).
takes_time(boil_corn_on_cob_s2, 8, 10, minute).
overlaps(boil_corn_on_cob_s2, boil_corn_on_cob_s1).
step(boil_corn_on_cob, boil_corn_on_cob_s3).
action(boil_corn_on_cob_s3, add).
object_of(boil_corn_on_cob_s3, corn_cob).
into(boil_corn_on_cob_s3, pot).
uses(boil_corn_on_cob_s3, tongs).
role(boil_corn_on_cob_s3, tongs, lift).
step(boil_corn_on_cob, boil_corn_on_cob_s4).
action(boil_corn_on_cob_s4, cook).
object_of(boil_corn_on_cob_s4, corn_cob).
takes_time(boil_corn_on_cob_s4, 5, 7, minute).
check(boil_corn_on_cob_s4, kernels_tender).
step(boil_corn_on_cob, boil_corn_on_cob_s5).
action(boil_corn_on_cob_s5, lift).
object_of(boil_corn_on_cob_s5, corn_cob).
uses(boil_corn_on_cob_s5, tongs).
role(boil_corn_on_cob_s5, tongs, lift).
step(boil_corn_on_cob, boil_corn_on_cob_s6).
action(boil_corn_on_cob_s6, spread).
object_of(boil_corn_on_cob_s6, butter).
onto(boil_corn_on_cob_s6, corn_cob).
uses(boil_corn_on_cob_s6, salt).
role(boil_corn_on_cob_s6, salt, season).
before(boil_corn_on_cob_s1, boil_corn_on_cob_s3).
before(boil_corn_on_cob_s2, boil_corn_on_cob_s3).
before(boil_corn_on_cob_s3, boil_corn_on_cob_s4).
before(boil_corn_on_cob_s4, boil_corn_on_cob_s5).
before(boil_corn_on_cob_s5, boil_corn_on_cob_s6).
```

## aliases
```datalog
task_alias(boil_corn_on_cob, "corn on the cob").
task_alias(boil_corn_on_cob, "boiled corn").
task_alias(boil_corn_on_cob, "cooking sweetcorn").
alias(corn_cob, "corn cobs").
alias(corn_cob, "sweetcorn").
alias(corn_cob, "ears of corn").
alias(husk, "strip the husks and silk").
alias(kernels_tender, "the kernels are tender").
```

## teach
- To boil corn on the cob you need corn cobs, a large pot of water, butter, salt and tongs.
- Boiling corn on the cob takes 15 to 20 minutes.
- Step 1 of boiling corn on the cob: husk the corn cobs, stripping the husks and silk.
- While step 1 runs, step 2 of boiling corn on the cob: boil the water in the pot for 8 to 10 minutes.
- Step 3 of boiling corn on the cob: add the corn cobs to the pot with the tongs.
- Step 4 of boiling corn on the cob: cook the corn for 5 to 7 minutes until the kernels are tender.
- Step 5 of boiling corn on the cob: lift the corn out with the tongs.
- Step 6 of boiling corn on the cob: spread butter on the corn and season it with salt.

## checklist
- names what is needed, including corn cobs, a pot, butter and tongs
- gives the steps in order: husk, boil water, add corn, cook, lift out, butter and salt
- says the corn cooks for about 5 to 7 minutes

## variant
- change: tongs broken: lower and lift the corn with a spatula
- request: The hinge on my tongs snapped. How do I get hot corn in and out of the pot?
- situation: I have four corn cobs, a pot, water, butter, salt and a spatula, but my tongs are broken.
- expect: The reply lowers and lifts the cobs with a spatula instead of the tongs, keeping hands away from the boiling water.

```datalog situation
has(user, corn_cob).
has(user, pot).
has(user, water).
has(user, butter).
has(user, salt).
has(user, spatula).
lacks(user, tongs).
```

```datalog difference
swap(boil_corn_on_cob_s3, tongs, spatula).
swap(boil_corn_on_cob_s5, tongs, spatula).
extra_need(spatula).
no_need(tongs).
```

## variant
- change: vegan: brush with olive oil instead of butter
- request: My friends coming tonight are vegan. What can I put on the corn instead of butter?
- situation: I have corn cobs, a pot, water, olive oil, salt and tongs. My guests don't eat dairy.
- expect: The reply brushes the corn with olive oil instead of butter, then salts it.

```datalog situation
has(user, corn_cob).
has(user, pot).
has(user, water).
has(user, olive_oil).
has(user, salt).
has(user, tongs).
excludes(user, dairy).
```

```datalog difference
change(boil_corn_on_cob_s6, object_of, butter, olive_oil).
extra_need(olive_oil).
no_need(butter).
```
