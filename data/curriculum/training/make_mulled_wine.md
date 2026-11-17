---
id: make_mulled_wine
stage: procedures
task: make_mulled_wine
---
# Make mulled wine

## requests
- We're having friends round on a cold evening. How do I make mulled wine?
- Can you tell me how to spice and warm a bottle of red wine?
- What do I need for a pot of mulled wine?

## situation
- I have a bottle of red wine, an orange, cinnamon sticks, cloves, sugar, a knife, a saucepan, a ladle and mugs.
- Guests arrive in 40 minutes.

## ground truth
```datalog
task(make_mulled_wine).
needs(make_mulled_wine, red_wine).
needs(make_mulled_wine, orange).
needs(make_mulled_wine, cinnamon_stick).
needs(make_mulled_wine, clove).
needs(make_mulled_wine, sugar).
needs(make_mulled_wine, knife).
needs(make_mulled_wine, saucepan).
needs(make_mulled_wine, ladle).
needs(make_mulled_wine, mug).
takes_time(make_mulled_wine, 18, 24, minute).
step(make_mulled_wine, make_mulled_wine_s1).
action(make_mulled_wine_s1, slice).
object_of(make_mulled_wine_s1, orange).
uses(make_mulled_wine_s1, knife).
takes_time(make_mulled_wine_s1, 1, 2, minute).
step(make_mulled_wine, make_mulled_wine_s2).
action(make_mulled_wine_s2, pour).
object_of(make_mulled_wine_s2, red_wine).
uses(make_mulled_wine_s2, saucepan).
step(make_mulled_wine, make_mulled_wine_s3).
action(make_mulled_wine_s3, add).
object_of(make_mulled_wine_s3, cinnamon_stick).
uses(make_mulled_wine_s3, orange).
uses(make_mulled_wine_s3, clove).
uses(make_mulled_wine_s3, sugar).
step(make_mulled_wine, make_mulled_wine_s4).
action(make_mulled_wine_s4, warm).
object_of(make_mulled_wine_s4, red_wine).
until(make_mulled_wine_s4, steaming).
takes_time(make_mulled_wine_s4, 15, 20, minute).
step(make_mulled_wine, make_mulled_wine_s5).
action(make_mulled_wine_s5, ladle).
object_of(make_mulled_wine_s5, red_wine).
uses(make_mulled_wine_s5, ladle).
uses(make_mulled_wine_s5, mug).
before(make_mulled_wine_s1, make_mulled_wine_s2).
before(make_mulled_wine_s2, make_mulled_wine_s3).
before(make_mulled_wine_s3, make_mulled_wine_s4).
before(make_mulled_wine_s4, make_mulled_wine_s5).
```

## teach
- To make mulled wine you need a bottle of red wine, an orange, cinnamon sticks, cloves, sugar, a knife, a saucepan, a ladle and mugs.
- Making mulled wine takes 18 to 24 minutes.
- Step 1 of making mulled wine: slice the orange with the knife for 1 to 2 minutes.
- Step 2 of making mulled wine: pour the red wine into the saucepan.
- Step 3 of making mulled wine: add the orange slices, the cinnamon sticks, the cloves and the sugar.
- Step 4 of making mulled wine: warm the wine gently until steaming, without letting it boil, for 15 to 20 minutes.
- Step 5 of making mulled wine: ladle the wine into the mugs.

## checklist
- names what is needed: a bottle of red wine, an orange, cinnamon sticks, cloves, sugar, a knife, a saucepan, a ladle and mugs
- gives the steps in order: slice orange, pour red wine, add cinnamon stick, warm red wine, ladle red wine
- gives a total time of about 18 to 24 minutes
- closes politely
