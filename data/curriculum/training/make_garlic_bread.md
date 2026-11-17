---
id: make_garlic_bread
stage: procedures
task: make_garlic_bread
---
# Make garlic bread

## requests
- I want garlic bread with the lasagne tonight. How do I make it from a baguette?
- How do you make homemade garlic bread in the oven?
- Can you give me the steps for buttery garlic bread?

## situation
- I have a baguette, butter, garlic, a knife, foil and an oven.
- I have about half an hour.

## ground truth
```datalog
task(make_garlic_bread).
needs(make_garlic_bread, baguette).
needs(make_garlic_bread, butter).
needs(make_garlic_bread, garlic).
needs(make_garlic_bread, knife).
needs(make_garlic_bread, foil).
needs(make_garlic_bread, oven).
takes_time(make_garlic_bread, 23, 28, minute).
step(make_garlic_bread, make_garlic_bread_s1).
action(make_garlic_bread_s1, preheat).
object_of(make_garlic_bread_s1, oven).
takes_time(make_garlic_bread_s1, 10, 12, minute).
step(make_garlic_bread, make_garlic_bread_s2).
action(make_garlic_bread_s2, mix).
object_of(make_garlic_bread_s2, butter).
uses(make_garlic_bread_s2, garlic).
takes_time(make_garlic_bread_s2, 3, 4, minute).
overlaps(make_garlic_bread_s2, make_garlic_bread_s1).
step(make_garlic_bread, make_garlic_bread_s3).
action(make_garlic_bread_s3, spread).
object_of(make_garlic_bread_s3, butter).
uses(make_garlic_bread_s3, baguette).
uses(make_garlic_bread_s3, knife).
takes_time(make_garlic_bread_s3, 2, 3, minute).
step(make_garlic_bread, make_garlic_bread_s4).
action(make_garlic_bread_s4, wrap).
object_of(make_garlic_bread_s4, baguette).
uses(make_garlic_bread_s4, foil).
takes_time(make_garlic_bread_s4, 1, 1, minute).
step(make_garlic_bread, make_garlic_bread_s5).
action(make_garlic_bread_s5, bake).
object_of(make_garlic_bread_s5, baguette).
uses(make_garlic_bread_s5, oven).
until(make_garlic_bread_s5, crisp).
takes_time(make_garlic_bread_s5, 10, 12, minute).
before(make_garlic_bread_s1, make_garlic_bread_s3).
before(make_garlic_bread_s2, make_garlic_bread_s3).
before(make_garlic_bread_s3, make_garlic_bread_s4).
before(make_garlic_bread_s4, make_garlic_bread_s5).
```

## teach
- To make garlic bread you need a baguette, butter, garlic, a knife, foil and an oven.
- Making garlic bread takes 23 to 28 minutes.
- Step 1 of making garlic bread: preheat the oven to 190 degrees for 10 to 12 minutes.
- While step 1 runs, step 2 of making garlic bread: mix the soft butter with crushed garlic for 3 to 4 minutes.
- Step 3 of making garlic bread: cut the baguette with the knife into slices without cutting all the way through and spread the garlic butter between them for 2 to 3 minutes.
- Step 4 of making garlic bread: wrap the baguette in foil for 1 minute.
- Step 5 of making garlic bread: bake the baguette in the oven until crisp, for 10 to 12 minutes.

## checklist
- names what is needed: a baguette, butter, garlic, a knife, foil and an oven
- gives the steps in order: preheat oven, mix butter (while preheat oven), spread butter, wrap baguette, bake baguette
- gives a total time of about 23 to 28 minutes
- closes politely
