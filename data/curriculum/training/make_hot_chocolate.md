---
id: make_hot_chocolate
stage: procedures
task: make_hot_chocolate
---
# Make hot chocolate

## requests
- It's freezing out and I want a proper mug of hot chocolate. How do I make it from cocoa powder?
- Can you tell me how to make hot cocoa on the stove?
- My kids are asking for hot chocolate before bed. What are the steps?

## situation
- I have milk, cocoa powder, sugar, a saucepan and a mug.
- I have about 15 minutes.

## ground truth
```datalog
task(make_hot_chocolate).
needs(make_hot_chocolate, milk).
needs(make_hot_chocolate, cocoa_powder).
needs(make_hot_chocolate, sugar).
needs(make_hot_chocolate, saucepan).
needs(make_hot_chocolate, mug).
takes_time(make_hot_chocolate, 6, 9, minute).
step(make_hot_chocolate, make_hot_chocolate_s1).
action(make_hot_chocolate_s1, heat).
object_of(make_hot_chocolate_s1, milk).
uses(make_hot_chocolate_s1, saucepan).
takes_time(make_hot_chocolate_s1, 4, 5, minute).
step(make_hot_chocolate, make_hot_chocolate_s2).
action(make_hot_chocolate_s2, mix).
object_of(make_hot_chocolate_s2, cocoa_powder).
uses(make_hot_chocolate_s2, sugar).
uses(make_hot_chocolate_s2, mug).
takes_time(make_hot_chocolate_s2, 1, 2, minute).
overlaps(make_hot_chocolate_s2, make_hot_chocolate_s1).
step(make_hot_chocolate, make_hot_chocolate_s3).
action(make_hot_chocolate_s3, pour).
object_of(make_hot_chocolate_s3, milk).
uses(make_hot_chocolate_s3, mug).
step(make_hot_chocolate, make_hot_chocolate_s4).
action(make_hot_chocolate_s4, stir).
object_of(make_hot_chocolate_s4, cocoa_powder).
until(make_hot_chocolate_s4, smooth).
takes_time(make_hot_chocolate_s4, 1, 2, minute).
before(make_hot_chocolate_s1, make_hot_chocolate_s3).
before(make_hot_chocolate_s2, make_hot_chocolate_s3).
before(make_hot_chocolate_s3, make_hot_chocolate_s4).
```

## teach
- To make hot chocolate you need milk, cocoa powder, sugar, a saucepan and a mug.
- Making hot chocolate takes 6 to 9 minutes.
- Step 1 of making hot chocolate: heat the milk in the saucepan over medium heat for 4 to 5 minutes.
- While step 1 runs, step 2 of making hot chocolate: mix the cocoa powder and the sugar in the mug with a splash of cold milk for 1 to 2 minutes.
- Step 3 of making hot chocolate: pour the hot milk into the mug.
- Step 4 of making hot chocolate: stir the cocoa powder into the milk until smooth, for 1 to 2 minutes.

## checklist
- names what is needed: milk, cocoa powder, sugar, a saucepan and a mug
- gives the steps in order: heat milk, mix cocoa powder (while heat milk), pour milk, stir cocoa powder
- gives a total time of about 6 to 9 minutes
- closes politely

## variant
- change: no cow's milk: heat oat milk instead
- request: I'm out of regular milk but I have oat milk. Can I still make hot chocolate?
- situation: I have oat milk, cocoa powder, sugar, a saucepan and a mug.
- expect: The reply heats the oat milk in place of milk and keeps the mix, pour and stir steps.
