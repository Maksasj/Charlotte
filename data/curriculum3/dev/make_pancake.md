---
id: make_pancake
stage: procedures
task: make_pancake
schema: v3
---
# Make pancakes

## requests
- The kids want pancakes this morning. How do I make them from scratch?
- Can you give me a simple way to make pancakes in a frying pan?
- I feel like pancakes for breakfast. What do I need and how do I cook them?

## situation
- I have flour, eggs, milk, butter, a bowl, a whisk, a frying pan and a spatula.
- We have about 20 minutes before school.

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

## checklist
- names what is needed, including flour, egg, milk and a pan
- gives the steps in order: mix batter, melt butter, pour, cook until bubbly, flip
- gives a total time of about 10 to 15 minutes
- closes politely

## variant
- change: no egg: use a mashed ripe banana in the batter instead
- request: I want to make pancakes but I just realised we're out of eggs. Is there a way around that?
- situation: I have flour, milk, butter, a ripe banana, a bowl, a whisk, a pan and a spatula, but no eggs.
- expect: The reply replaces the egg with a mashed banana in the batter and keeps the rest of the steps.

```datalog situation
has(user, flour).
has(user, milk).
has(user, butter).
has(user, banana).
has(user, bowl).
has(user, whisk).
has(user, pan).
has(user, spatula).
lacks(user, egg).
```

```datalog difference
swap(make_pancake_s1, egg, banana).
no_need(egg).
extra_need(banana).
add_step(make_pancake_s9, start).
step(make_pancake, make_pancake_s9).
action(make_pancake_s9, mash).
object_of(make_pancake_s9, banana).
before(make_pancake_s9, make_pancake_s1).
```
