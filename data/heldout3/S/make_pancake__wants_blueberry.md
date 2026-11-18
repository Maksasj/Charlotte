---
id: make_pancake__wants_blueberry
stage: procedures
task: make_pancake
schema: v3
tier: S
base: make_pancake
---
# Make pancakes

## requests
- I've got a punnet of blueberries I want to use up. Can they go into this morning's pancakes?

## ground truth
```datalog
task(make_pancake).
domain(make_pancake, cooking).
goal(make_pancake, pancake).
needs(make_pancake, flour).
needs(make_pancake, egg).
needs(make_pancake, milk).
needs(make_pancake, butter).
needs(make_pancake, bowl).
needs(make_pancake, whisk).
needs(make_pancake, pan).
needs(make_pancake, spatula).
takes_time(make_pancake, 10, 15, minute).
step(make_pancake, make_pancake_s1).
action(make_pancake_s1, mix).
object_of(make_pancake_s1, batter).
uses(make_pancake_s1, flour).
role(make_pancake_s1, flour, thicken).
uses(make_pancake_s1, egg).
role(make_pancake_s1, egg, bind).
uses(make_pancake_s1, milk).
role(make_pancake_s1, milk, moisten).
uses(make_pancake_s1, bowl).
role(make_pancake_s1, bowl, contain).
uses(make_pancake_s1, whisk).
role(make_pancake_s1, whisk, beat).
takes_time(make_pancake_s1, 2, 3, minute).
step(make_pancake, make_pancake_s2).
action(make_pancake_s2, melt).
object_of(make_pancake_s2, butter).
uses(make_pancake_s2, pan).
role(make_pancake_s2, pan, heat).
purpose(make_pancake_s2, no_sticking).
takes_time(make_pancake_s2, 1, 2, minute).
step(make_pancake, make_pancake_s3).
action(make_pancake_s3, pour).
object_of(make_pancake_s3, batter).
uses(make_pancake_s3, pan).
role(make_pancake_s3, pan, heat).
step(make_pancake, make_pancake_s4).
action(make_pancake_s4, cook).
object_of(make_pancake_s4, batter).
until(make_pancake_s4, bubbly).
takes_time(make_pancake_s4, 2, 3, minute).
step(make_pancake, make_pancake_s5).
action(make_pancake_s5, flip).
object_of(make_pancake_s5, pancake).
uses(make_pancake_s5, spatula).
role(make_pancake_s5, spatula, lift).
step(make_pancake, make_pancake_s6).
action(make_pancake_s6, cook).
object_of(make_pancake_s6, pancake).
takes_time(make_pancake_s6, 1, 2, minute).
before(make_pancake_s1, make_pancake_s3).
before(make_pancake_s2, make_pancake_s3).
before(make_pancake_s3, make_pancake_s4).
before(make_pancake_s4, make_pancake_s5).
before(make_pancake_s5, make_pancake_s6).
```

## aliases
```datalog
task_alias(make_pancake, "making pancakes").
task_alias(make_pancake, "make pancakes").
task_alias(make_pancake, "pancakes from scratch").
alias(pancake, "pancakes").
alias(egg, "eggs").
alias(pan, "frying pan").
alias(bubbly, "bubbles appear").
alias(no_sticking, "do not stick").
alias(banana, "ripe banana").
alias(banana, "mashed banana").
alias(banana, "mashed ripe banana").
```

## teach
- To make pancakes you need flour, an egg, milk, butter, a bowl, a whisk, a frying pan and a spatula.
- Making pancakes takes 10 to 15 minutes.
- Step 1 of making pancakes: mix the batter from the flour, the egg and the milk in the bowl with the whisk for 2 to 3 minutes.
- Step 2 of making pancakes: melt a little butter in the pan for 1 to 2 minutes so the pancakes do not stick.
- Step 3 of making pancakes: pour some batter into the pan.
- Step 4 of making pancakes: cook the batter until bubbles appear, for 2 to 3 minutes.
- Step 5 of making pancakes: flip the pancake with the spatula.
- Step 6 of making pancakes: cook the pancake for 1 to 2 minutes more.

## variant
- change: add blueberries: scatter them onto the batter in the pan
- request: I've got a punnet of blueberries I want to use up. Can they go into this morning's pancakes?
- situation: I have flour, eggs, milk, butter, a bowl, a whisk, a frying pan, a spatula and blueberries.
- expect: The reply scatters a few blueberries onto each pancake right after pouring the batter, before it cooks through and is flipped.

```datalog situation
has(user, flour).
has(user, egg).
has(user, milk).
has(user, butter).
has(user, bowl).
has(user, whisk).
has(user, pan).
has(user, spatula).
has(user, blueberry).
wants(user, blueberry).
```

```datalog difference
extra_need(blueberry).
add_step(make_pancake_s9, make_pancake_s3).
step(make_pancake, make_pancake_s9).
action(make_pancake_s9, scatter).
object_of(make_pancake_s9, blueberry).
onto(make_pancake_s9, batter).
before(make_pancake_s3, make_pancake_s9).
before(make_pancake_s9, make_pancake_s4).
```
