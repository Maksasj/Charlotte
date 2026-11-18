---
id: boil_egg
stage: procedures
task: boil_egg
schema: v3
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
domain(boil_egg, cooking).
goal(boil_egg, boiled_egg).
needs(boil_egg, egg).
needs(boil_egg, water).
needs(boil_egg, pot).
needs(boil_egg, bowl).
optional_need(boil_egg, spoon).
takes_time(boil_egg, 12, 16, minute).
step(boil_egg, boil_egg_s1).
action(boil_egg_s1, boil).
object_of(boil_egg_s1, water).
uses(boil_egg_s1, pot).
role(boil_egg_s1, pot, heat).
takes_time(boil_egg_s1, 3, 5, minute).
step(boil_egg, boil_egg_s2).
action(boil_egg_s2, lower).
object_of(boil_egg_s2, egg).
into(boil_egg_s2, water).
uses(boil_egg_s2, spoon).
role(boil_egg_s2, spoon, hold).
step(boil_egg, boil_egg_s3).
action(boil_egg_s3, boil).
object_of(boil_egg_s3, egg).
purpose(boil_egg_s3, yolk_set).
takes_time(boil_egg_s3, 7, 9, minute).
step(boil_egg, boil_egg_s4).
action(boil_egg_s4, cool).
object_of(boil_egg_s4, egg).
uses(boil_egg_s4, bowl).
role(boil_egg_s4, bowl, contain).
uses(boil_egg_s4, water).
role(boil_egg_s4, water, cool).
setting(boil_egg_s4, temperature, cold, level).
takes_time(boil_egg_s4, 2, 2, minute).
before(boil_egg_s1, boil_egg_s2).
before(boil_egg_s2, boil_egg_s3).
before(boil_egg_s3, boil_egg_s4).
```

## aliases
```datalog
task_alias(boil_egg, "boiled egg").
task_alias(boil_egg, "hard-boiled egg").
task_alias(boil_egg, "boiling an egg").
alias(egg, "eggs").
alias(water, "cold water").
alias(water, "pot of water").
alias(yolk_set, "yolk is firm").
alias(yolk_set, "yolk sets").
```

## teach
- To boil an egg you need an egg, water, a pot and a bowl.
- Boiling an egg takes 12 to 16 minutes.
- Step 1 of boiling an egg: boil the water in the pot for 3 to 5 minutes.
- Step 2 of boiling an egg: lower the egg into the water with a spoon.
- Step 3 of boiling an egg: boil the egg for 7 to 9 minutes so the yolk sets.
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

```datalog situation
has(user, egg).
has(user, pot).
has(user, water).
has(user, bowl).
wants(user, runny_yolk).
```

```datalog difference
change(boil_egg_s3, takes_time, 7_9, 5_6).
change(boil_egg_s3, purpose, yolk_set, runny_yolk).
```
