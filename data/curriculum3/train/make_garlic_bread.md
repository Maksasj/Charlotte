---
id: make_garlic_bread
stage: procedures
task: make_garlic_bread
schema: v3
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
domain(make_garlic_bread, cooking).
goal(make_garlic_bread, garlic_bread).
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
setting(make_garlic_bread_s1, temperature, 190, celsius).
takes_time(make_garlic_bread_s1, 10, 12, minute).
step(make_garlic_bread, make_garlic_bread_s2).
action(make_garlic_bread_s2, mix).
object_of(make_garlic_bread_s2, butter).
uses(make_garlic_bread_s2, garlic).
role(make_garlic_bread_s2, garlic, flavour).
requires_state(make_garlic_bread_s2, butter_soft).
requires_state(make_garlic_bread_s2, garlic_crushed).
overlaps(make_garlic_bread_s2, make_garlic_bread_s1).
takes_time(make_garlic_bread_s2, 3, 4, minute).
step(make_garlic_bread, make_garlic_bread_s3).
action(make_garlic_bread_s3, cut).
object_of(make_garlic_bread_s3, baguette).
uses(make_garlic_bread_s3, knife).
role(make_garlic_bread_s3, knife, cut).
avoid(make_garlic_bread_s3, cut_through, baguette).
step(make_garlic_bread, make_garlic_bread_s4).
action(make_garlic_bread_s4, spread).
object_of(make_garlic_bread_s4, butter).
uses(make_garlic_bread_s4, baguette).
role(make_garlic_bread_s4, baguette, hold).
into(make_garlic_bread_s4, baguette).
takes_time(make_garlic_bread_s4, 2, 3, minute).
step(make_garlic_bread, make_garlic_bread_s5).
action(make_garlic_bread_s5, wrap).
object_of(make_garlic_bread_s5, baguette).
uses(make_garlic_bread_s5, foil).
role(make_garlic_bread_s5, foil, cover).
takes_time(make_garlic_bread_s5, 1, 1, minute).
step(make_garlic_bread, make_garlic_bread_s6).
action(make_garlic_bread_s6, bake).
object_of(make_garlic_bread_s6, baguette).
uses(make_garlic_bread_s6, oven).
role(make_garlic_bread_s6, oven, heat).
until(make_garlic_bread_s6, crisp).
takes_time(make_garlic_bread_s6, 10, 12, minute).
before(make_garlic_bread_s1, make_garlic_bread_s3).
before(make_garlic_bread_s2, make_garlic_bread_s3).
before(make_garlic_bread_s3, make_garlic_bread_s4).
before(make_garlic_bread_s4, make_garlic_bread_s5).
before(make_garlic_bread_s5, make_garlic_bread_s6).
```

## aliases
```datalog
task_alias(make_garlic_bread, "making garlic bread").
task_alias(make_garlic_bread, "homemade garlic bread").
task_alias(make_garlic_bread, "buttery garlic bread").
alias(butter, "soft butter").
alias(butter, "garlic butter").
alias(garlic, "crushed garlic").
alias(baguette, "slices").
alias(foil, "tin foil").
alias(preheat, "heat").
alias(cut_through, "cutting all the way through").
```

## teach
- To make garlic bread you need a baguette, butter, garlic, a knife, foil and an oven.
- Making garlic bread takes 23 to 28 minutes.
- Step 1 of making garlic bread: preheat the oven to 190 degrees for 10 to 12 minutes.
- While step 1 runs, step 2 of making garlic bread: mix the soft butter with crushed garlic for 3 to 4 minutes.
- Step 3 of making garlic bread: cut the baguette with the knife into slices without cutting all the way through.
- Step 4 of making garlic bread: spread the garlic butter between the slices for 2 to 3 minutes.
- Step 5 of making garlic bread: wrap the baguette in foil for 1 minute.
- Step 6 of making garlic bread: bake the baguette in the oven until crisp, for 10 to 12 minutes.

## checklist
- names what is needed: a baguette, butter, garlic, a knife, foil and an oven
- gives the steps in order: preheat oven, mix butter (while preheat oven), spread butter, wrap baguette, bake baguette
- gives a total time of about 23 to 28 minutes
- closes politely
