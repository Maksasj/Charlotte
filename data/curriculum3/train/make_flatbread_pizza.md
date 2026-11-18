---
id: make_flatbread_pizza
stage: procedures
task: make_flatbread_pizza
schema: v3
---
# Make flatbread pizza

## requests
- I want pizza but I don't have time to make dough. Can I use a flatbread?
- How do I make a quick pizza on a flatbread in the oven?
- Can you give me the steps for an easy flatbread margherita?

## situation
- I have a flatbread, tomato sauce, mozzarella, basil, a baking tray and an oven.
- I have about half an hour.

## ground truth
```datalog
task(make_flatbread_pizza).
domain(make_flatbread_pizza, cooking).
goal(make_flatbread_pizza, pizza).
needs(make_flatbread_pizza, flatbread).
needs(make_flatbread_pizza, tomato_sauce).
needs(make_flatbread_pizza, mozzarella).
needs(make_flatbread_pizza, basil).
needs(make_flatbread_pizza, baking_tray).
needs(make_flatbread_pizza, oven).
takes_time(make_flatbread_pizza, 18, 24, minute).
step(make_flatbread_pizza, make_flatbread_pizza_s1).
action(make_flatbread_pizza_s1, preheat).
object_of(make_flatbread_pizza_s1, oven).
setting(make_flatbread_pizza_s1, temperature, 220, celsius).
takes_time(make_flatbread_pizza_s1, 10, 12, minute).
step(make_flatbread_pizza, make_flatbread_pizza_s2).
action(make_flatbread_pizza_s2, spread).
object_of(make_flatbread_pizza_s2, tomato_sauce).
uses(make_flatbread_pizza_s2, flatbread).
role(make_flatbread_pizza_s2, flatbread, hold).
uses(make_flatbread_pizza_s2, baking_tray).
role(make_flatbread_pizza_s2, baking_tray, hold).
onto(make_flatbread_pizza_s2, flatbread).
overlaps(make_flatbread_pizza_s2, make_flatbread_pizza_s1).
takes_time(make_flatbread_pizza_s2, 1, 2, minute).
step(make_flatbread_pizza, make_flatbread_pizza_s3).
action(make_flatbread_pizza_s3, tear).
object_of(make_flatbread_pizza_s3, mozzarella).
uses(make_flatbread_pizza_s3, flatbread).
role(make_flatbread_pizza_s3, flatbread, hold).
onto(make_flatbread_pizza_s3, tomato_sauce).
overlaps(make_flatbread_pizza_s3, make_flatbread_pizza_s1).
takes_time(make_flatbread_pizza_s3, 1, 1, minute).
step(make_flatbread_pizza, make_flatbread_pizza_s4).
action(make_flatbread_pizza_s4, bake).
object_of(make_flatbread_pizza_s4, flatbread).
uses(make_flatbread_pizza_s4, oven).
role(make_flatbread_pizza_s4, oven, heat).
until(make_flatbread_pizza_s4, bubbling).
takes_time(make_flatbread_pizza_s4, 8, 10, minute).
step(make_flatbread_pizza, make_flatbread_pizza_s5).
action(make_flatbread_pizza_s5, scatter).
object_of(make_flatbread_pizza_s5, basil).
onto(make_flatbread_pizza_s5, flatbread).
before(make_flatbread_pizza_s2, make_flatbread_pizza_s3).
before(make_flatbread_pizza_s1, make_flatbread_pizza_s4).
before(make_flatbread_pizza_s3, make_flatbread_pizza_s4).
before(make_flatbread_pizza_s4, make_flatbread_pizza_s5).
```

## aliases
```datalog
task_alias(make_flatbread_pizza, "making flatbread pizza").
task_alias(make_flatbread_pizza, "flatbread pizza").
task_alias(make_flatbread_pizza, "flatbread margherita").
task_alias(make_flatbread_pizza, "quick pizza on a flatbread").
alias(tomato_sauce, "sauce").
alias(basil, "basil leaves").
alias(mozzarella, "cheese").
alias(cheddar, "grated cheddar").
alias(bubbling, "the cheese bubbles").
alias(preheat, "heat").
alias(baking_tray, "baking sheet").
```

## teach
- To make flatbread pizza you need a flatbread, tomato sauce, mozzarella, basil, a baking tray and an oven.
- Making flatbread pizza takes 18 to 24 minutes.
- Step 1 of making flatbread pizza: preheat the oven to 220 degrees for 10 to 12 minutes.
- While step 1 runs, step 2 of making flatbread pizza: spread the tomato sauce over the flatbread on the baking tray for 1 to 2 minutes.
- While step 1 runs, step 3 of making flatbread pizza: tear the mozzarella over the sauce for 1 minute.
- Step 4 of making flatbread pizza: bake the flatbread in the oven until the cheese bubbles, for 8 to 10 minutes.
- Step 5 of making flatbread pizza: scatter the basil leaves over the top.

## checklist
- names what is needed: a flatbread, tomato sauce, mozzarella, basil, a baking tray and an oven
- gives the steps in order: preheat oven, spread tomato sauce (while preheat oven), tear mozzarella (while preheat oven), bake flatbread, scatter basil
- gives a total time of about 18 to 24 minutes
- closes politely

## variant
- change: no mozzarella: use grated cheddar instead
- request: I'm out of mozzarella. Can I still make flatbread pizza with cheddar?
- situation: I have a flatbread, tomato sauce, grated cheddar, basil, a baking tray and an oven.
- expect: The reply sprinkles the cheddar in place of the mozzarella and keeps the preheat, spread, bake and basil steps.

```datalog situation
has(user, flatbread).
has(user, tomato_sauce).
has(user, cheddar).
has(user, basil).
has(user, baking_tray).
has(user, oven).
lacks(user, mozzarella).
```

```datalog difference
change(make_flatbread_pizza_s3, action, tear, sprinkle).
change(make_flatbread_pizza_s3, object_of, mozzarella, cheddar).
extra_need(cheddar).
no_need(mozzarella).
```
