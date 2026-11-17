---
id: make_bruschetta
stage: procedures
task: make_bruschetta
---
# Make bruschetta

## requests
- How do I make tomato bruschetta as a starter?
- Can you give me the steps for garlicky tomato toasts?
- I've got ripe tomatoes and a baguette. How do I turn them into bruschetta?

## situation
- I have a baguette, tomatoes, garlic, basil, olive oil, salt, a knife, a bowl and an oven with a grill.
- Guests arrive in 30 minutes.

## ground truth
```datalog
task(make_bruschetta).
needs(make_bruschetta, baguette).
needs(make_bruschetta, tomato).
needs(make_bruschetta, garlic).
needs(make_bruschetta, basil).
needs(make_bruschetta, olive_oil).
needs(make_bruschetta, salt).
needs(make_bruschetta, knife).
needs(make_bruschetta, bowl).
needs(make_bruschetta, oven).
takes_time(make_bruschetta, 10, 14, minute).
step(make_bruschetta, make_bruschetta_s1).
action(make_bruschetta_s1, dice).
object_of(make_bruschetta_s1, tomato).
uses(make_bruschetta_s1, knife).
uses(make_bruschetta_s1, basil).
takes_time(make_bruschetta_s1, 5, 6, minute).
step(make_bruschetta, make_bruschetta_s2).
action(make_bruschetta_s2, toast).
object_of(make_bruschetta_s2, baguette).
uses(make_bruschetta_s2, oven).
until(make_bruschetta_s2, golden).
takes_time(make_bruschetta_s2, 4, 6, minute).
overlaps(make_bruschetta_s2, make_bruschetta_s1).
step(make_bruschetta, make_bruschetta_s3).
action(make_bruschetta_s3, mix).
object_of(make_bruschetta_s3, tomato).
uses(make_bruschetta_s3, olive_oil).
uses(make_bruschetta_s3, salt).
uses(make_bruschetta_s3, bowl).
takes_time(make_bruschetta_s3, 1, 2, minute).
step(make_bruschetta, make_bruschetta_s4).
action(make_bruschetta_s4, rub).
object_of(make_bruschetta_s4, garlic).
uses(make_bruschetta_s4, baguette).
takes_time(make_bruschetta_s4, 1, 1, minute).
step(make_bruschetta, make_bruschetta_s5).
action(make_bruschetta_s5, spoon).
object_of(make_bruschetta_s5, tomato).
uses(make_bruschetta_s5, baguette).
takes_time(make_bruschetta_s5, 1, 2, minute).
before(make_bruschetta_s1, make_bruschetta_s3).
before(make_bruschetta_s2, make_bruschetta_s3).
before(make_bruschetta_s3, make_bruschetta_s4).
before(make_bruschetta_s4, make_bruschetta_s5).
```

## teach
- To make bruschetta you need a baguette, tomatoes, garlic, basil, olive oil, salt, a knife, a bowl and an oven.
- Making bruschetta takes 10 to 14 minutes.
- Step 1 of making bruschetta: dice the tomatoes with the knife and tear the basil for 5 to 6 minutes.
- While step 1 runs, step 2 of making bruschetta: toast the baguette slices under the oven grill until golden, for 4 to 6 minutes.
- Step 3 of making bruschetta: mix the tomatoes with olive oil and salt in the bowl for 1 to 2 minutes.
- Step 4 of making bruschetta: rub the cut garlic over the toasted baguette for 1 minute.
- Step 5 of making bruschetta: spoon the tomato mix onto the baguette slices for 1 to 2 minutes.

## checklist
- names what is needed: a baguette, tomatoes, garlic, basil, olive oil, salt, a knife, a bowl and an oven
- gives the steps in order: dice tomato, toast baguette (while dice tomato), mix tomato, rub garlic, spoon tomato
- gives a total time of about 10 to 14 minutes
- closes politely
