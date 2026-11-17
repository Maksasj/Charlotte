---
id: bake_banana_bread
stage: procedures
task: bake_banana_bread
---
# Bake banana bread

## requests
- I've got three brown bananas going soft. How do I make banana bread?
- Can you walk me through baking a loaf of banana bread?
- What's a simple banana bread method for a beginner?

## situation
- I have ripe bananas, flour, sugar, eggs, butter, baking soda, a bowl, a fork, a loaf tin and an oven.
- I have the afternoon free.

## ground truth
```datalog
task(bake_banana_bread).
needs(bake_banana_bread, banana).
needs(bake_banana_bread, flour).
needs(bake_banana_bread, sugar).
needs(bake_banana_bread, egg).
needs(bake_banana_bread, butter).
needs(bake_banana_bread, baking_soda).
needs(bake_banana_bread, bowl).
needs(bake_banana_bread, fork).
needs(bake_banana_bread, loaf_tin).
needs(bake_banana_bread, oven).
takes_time(bake_banana_bread, 65, 80, minute).
step(bake_banana_bread, bake_banana_bread_s1).
action(bake_banana_bread_s1, preheat).
object_of(bake_banana_bread_s1, oven).
takes_time(bake_banana_bread_s1, 10, 15, minute).
step(bake_banana_bread, bake_banana_bread_s2).
action(bake_banana_bread_s2, mash).
object_of(bake_banana_bread_s2, banana).
uses(bake_banana_bread_s2, bowl).
uses(bake_banana_bread_s2, fork).
takes_time(bake_banana_bread_s2, 3, 5, minute).
overlaps(bake_banana_bread_s2, bake_banana_bread_s1).
step(bake_banana_bread, bake_banana_bread_s3).
action(bake_banana_bread_s3, mix).
object_of(bake_banana_bread_s3, flour).
uses(bake_banana_bread_s3, butter).
uses(bake_banana_bread_s3, sugar).
uses(bake_banana_bread_s3, egg).
uses(bake_banana_bread_s3, baking_soda).
takes_time(bake_banana_bread_s3, 3, 5, minute).
step(bake_banana_bread, bake_banana_bread_s4).
action(bake_banana_bread_s4, pour).
object_of(bake_banana_bread_s4, batter).
uses(bake_banana_bread_s4, loaf_tin).
takes_time(bake_banana_bread_s4, 1, 1, minute).
step(bake_banana_bread, bake_banana_bread_s5).
action(bake_banana_bread_s5, bake).
object_of(bake_banana_bread_s5, banana_bread).
uses(bake_banana_bread_s5, oven).
until(bake_banana_bread_s5, clean).
takes_time(bake_banana_bread_s5, 50, 60, minute).
before(bake_banana_bread_s1, bake_banana_bread_s3).
before(bake_banana_bread_s2, bake_banana_bread_s3).
before(bake_banana_bread_s3, bake_banana_bread_s4).
before(bake_banana_bread_s4, bake_banana_bread_s5).
```

## teach
- To bake banana bread you need ripe bananas, flour, sugar, eggs, melted butter, baking soda, a bowl, a fork, a loaf tin and an oven.
- Baking banana bread takes 65 to 80 minutes.
- Step 1 of baking banana bread: preheat the oven to 175 degrees for 10 to 15 minutes.
- While step 1 runs, step 2 of baking banana bread: mash the bananas in the bowl with the fork for 3 to 5 minutes.
- Step 3 of baking banana bread: mix in the melted butter, the sugar, the eggs, the flour and the baking soda, for 3 to 5 minutes.
- Step 4 of baking banana bread: pour the batter into the greased loaf tin for 1 minute.
- Step 5 of baking banana bread: bake the banana bread in the oven until a skewer comes out clean, for 50 to 60 minutes.

## checklist
- names what is needed: ripe bananas, flour, sugar, eggs, melted butter, baking soda, a bowl, a fork, a loaf tin and an oven
- gives the steps in order: preheat oven, mash banana (while preheat oven), mix flour, pour batter, bake banana bread
- gives a total time of about 65 to 80 minutes
- closes politely

## variant
- change: no loaf tin: bake the batter in a square cake tin for 35 to 40 minutes
- request: I want to bake banana bread but I don't own a loaf tin. What can I use?
- situation: I have ripe bananas, flour, sugar, eggs, butter, baking soda, a bowl, a fork, a square cake tin and an oven.
- expect: The reply bakes the batter in the square cake tin and shortens the bake to about 35 to 40 minutes, keeping the mash, mix and pour steps.
