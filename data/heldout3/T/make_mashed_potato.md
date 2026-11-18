---
id: make_mashed_potato
stage: procedures
task: make_mashed_potato
schema: v3
tier: T
---
# Make mashed potatoes

## requests
- What's the best way to make smooth mashed potatoes for four people?
- I want a creamy mash to go with sausages tonight. Where do I start?
- How do I turn a bag of potatoes into a proper potato mash?

## situation
- I have potatoes, a peeler, a knife, a saucepan, a colander, a potato masher, butter, milk and salt.
- Dinner is in about 40 minutes.

## ground truth
```datalog
task(make_mashed_potato).
domain(make_mashed_potato, cooking).
goal(make_mashed_potato, mashed_potato).
goal(make_mashed_potato, side_dish).
needs(make_mashed_potato, potato).
needs(make_mashed_potato, peeler).
needs(make_mashed_potato, knife).
needs(make_mashed_potato, saucepan).
needs(make_mashed_potato, water).
needs(make_mashed_potato, colander).
needs(make_mashed_potato, potato_masher).
needs(make_mashed_potato, butter).
needs(make_mashed_potato, milk).
needs(make_mashed_potato, salt).
takes_time(make_mashed_potato, 30, 40, minute).
step(make_mashed_potato, make_mashed_potato_s1).
action(make_mashed_potato_s1, peel).
object_of(make_mashed_potato_s1, potato).
uses(make_mashed_potato_s1, peeler).
role(make_mashed_potato_s1, peeler, cut).
takes_time(make_mashed_potato_s1, 5, 8, minute).
step(make_mashed_potato, make_mashed_potato_s2).
action(make_mashed_potato_s2, cut).
object_of(make_mashed_potato_s2, potato).
uses(make_mashed_potato_s2, knife).
role(make_mashed_potato_s2, knife, cut).
purpose(make_mashed_potato_s2, cooks_evenly).
step(make_mashed_potato, make_mashed_potato_s3).
action(make_mashed_potato_s3, boil).
object_of(make_mashed_potato_s3, potato).
uses(make_mashed_potato_s3, saucepan).
role(make_mashed_potato_s3, saucepan, heat).
uses(make_mashed_potato_s3, water).
role(make_mashed_potato_s3, water, heat).
until(make_mashed_potato_s3, soft).
takes_time(make_mashed_potato_s3, 15, 20, minute).
step(make_mashed_potato, make_mashed_potato_s4).
action(make_mashed_potato_s4, drain).
object_of(make_mashed_potato_s4, potato).
uses(make_mashed_potato_s4, colander).
role(make_mashed_potato_s4, colander, strain).
step(make_mashed_potato, make_mashed_potato_s5).
action(make_mashed_potato_s5, mash).
object_of(make_mashed_potato_s5, potato).
uses(make_mashed_potato_s5, potato_masher).
role(make_mashed_potato_s5, potato_masher, press).
uses(make_mashed_potato_s5, butter).
role(make_mashed_potato_s5, butter, flavour).
uses(make_mashed_potato_s5, milk).
role(make_mashed_potato_s5, milk, moisten).
check(make_mashed_potato_s5, smooth).
step(make_mashed_potato, make_mashed_potato_s6).
action(make_mashed_potato_s6, season).
object_of(make_mashed_potato_s6, potato).
uses(make_mashed_potato_s6, salt).
role(make_mashed_potato_s6, salt, season).
before(make_mashed_potato_s1, make_mashed_potato_s2).
before(make_mashed_potato_s2, make_mashed_potato_s3).
before(make_mashed_potato_s3, make_mashed_potato_s4).
before(make_mashed_potato_s4, make_mashed_potato_s5).
before(make_mashed_potato_s5, make_mashed_potato_s6).
```

## aliases
```datalog
task_alias(make_mashed_potato, "mashed potatoes").
task_alias(make_mashed_potato, "potato mash").
task_alias(make_mashed_potato, "making mash").
alias(potato, "potatoes").
alias(potato_masher, "masher").
alias(mashed_potato, "mash").
alias(soft, "tender").
alias(cooks_evenly, "cook evenly").
```

## teach
- To make mashed potatoes you need potatoes, a peeler, a knife, a saucepan, water, a colander, a potato masher, butter, milk and salt.
- Making mashed potatoes takes 30 to 40 minutes.
- Step 1 of making mashed potatoes: peel the potatoes with the peeler for 5 to 8 minutes.
- Step 2 of making mashed potatoes: cut the potatoes into even chunks with the knife so they cook evenly.
- Step 3 of making mashed potatoes: boil the potatoes in a saucepan of water until they are soft, for 15 to 20 minutes.
- Step 4 of making mashed potatoes: drain the potatoes in the colander.
- Step 5 of making mashed potatoes: mash the potatoes with the potato masher, butter and milk until smooth.
- Step 6 of making mashed potatoes: season the mash with salt.

## checklist
- names what is needed, including potatoes, butter, milk and a masher
- gives the steps in order: peel, cut, boil until soft, drain, mash with butter and milk, season
- gives a total time of about 30 to 40 minutes

## variant
- change: no potato masher: mash with a fork instead
- request: I want mash but I don't own a masher. Can I still do it?
- situation: I have potatoes, a peeler, a knife, a saucepan, a colander, a fork, butter, milk and salt.
- expect: The reply keeps peeling, cutting, boiling and draining, then mashes the potatoes with a fork in place of the masher.

```datalog situation
has(user, potato).
has(user, peeler).
has(user, knife).
has(user, saucepan).
has(user, water).
has(user, colander).
has(user, fork).
has(user, butter).
has(user, milk).
has(user, salt).
lacks(user, potato_masher).
```

```datalog difference
swap(make_mashed_potato_s5, potato_masher, fork).
extra_need(fork).
no_need(potato_masher).
```

## variant
- change: dairy-free guest: use olive oil and vegetable stock instead of butter and milk
- request: One of my guests can't have dairy. How do I make the mash without butter or milk?
- situation: I have potatoes, a peeler, a knife, a saucepan, a colander, a masher, olive oil, vegetable stock and salt. One guest avoids dairy.
- expect: The reply mashes the potatoes with olive oil for flavour and warm vegetable stock to loosen it, leaving out the butter and milk.

```datalog situation
has(user, potato).
has(user, peeler).
has(user, knife).
has(user, saucepan).
has(user, water).
has(user, colander).
has(user, potato_masher).
has(user, olive_oil).
has(user, vegetable_stock).
has(user, salt).
excludes(user, dairy).
```

```datalog difference
swap(make_mashed_potato_s5, butter, olive_oil).
swap(make_mashed_potato_s5, milk, vegetable_stock).
extra_need(olive_oil).
extra_need(vegetable_stock).
no_need(butter).
no_need(milk).
```
