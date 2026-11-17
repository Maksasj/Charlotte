---
id: boil_egg
stage: procedures
task: boil_egg
---
# Boil an egg

## requests
- I want a hard-boiled egg for breakfast. How do I cook it?
- How long do I boil an egg so the yolk is firm, and what are the steps?
- Could you tell me how to make a boiled egg? I never get it right.

## situation
- I have eggs, a pot, water and a bowl.
- I have about 20 minutes.

## ground truth
```datalog
task(boil_egg).
needs(boil_egg, egg).
needs(boil_egg, water).
needs(boil_egg, pot).
needs(boil_egg, bowl).
takes_time(boil_egg, 12, 16, minute).
step(boil_egg, boil_egg_s1).
action(boil_egg_s1, boil).
object_of(boil_egg_s1, water).
uses(boil_egg_s1, pot).
takes_time(boil_egg_s1, 3, 5, minute).
step(boil_egg, boil_egg_s2).
action(boil_egg_s2, lower).
object_of(boil_egg_s2, egg).
step(boil_egg, boil_egg_s3).
action(boil_egg_s3, boil).
object_of(boil_egg_s3, egg).
takes_time(boil_egg_s3, 7, 9, minute).
step(boil_egg, boil_egg_s4).
action(boil_egg_s4, cool).
object_of(boil_egg_s4, egg).
uses(boil_egg_s4, bowl).
takes_time(boil_egg_s4, 2, 2, minute).
before(boil_egg_s1, boil_egg_s2).
before(boil_egg_s2, boil_egg_s3).
before(boil_egg_s3, boil_egg_s4).
```

## teach
- To boil an egg you need an egg, water, a pot and a bowl.
- Boiling an egg takes 12 to 16 minutes.
- Step 1 of boiling an egg: boil the water in the pot for 3 to 5 minutes.
- Step 2 of boiling an egg: lower the egg into the water with a spoon.
- Step 3 of boiling an egg: boil the egg for 7 to 9 minutes.
- Step 4 of boiling an egg: cool the egg in a bowl of cold water for 2 minutes.

## checklist
- names what is needed: an egg, water, a pot and a bowl
- gives the steps in order: boil the water, lower the egg, boil the egg, cool it
- says the egg boils for about 7 to 9 minutes and the whole thing takes about 12 to 16 minutes
- closes politely

## variant
- change: soft-boiled with a runny yolk: boil the egg for only 5 to 6 minutes
- request: I'd like a soft-boiled egg with a runny yolk for dipping toast. How do I do that?
- situation: I have an egg, a pot of water and a bowl, and I want the yolk still runny.
- expect: The reply shortens the egg's boiling time to about 5 to 6 minutes and the total to about 10 to 13 minutes.
