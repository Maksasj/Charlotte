---
id: make_omelette__no_fork
stage: procedures
task: make_omelette
schema: v3
tier: S
base: make_omelette
---
# Make an omelette

## requests
- Every fork I own is in the dishwasher halfway through a cycle. I've got a whisk though. Can I still do a quick plain omelette?

## ground truth
```datalog
task(make_omelette).
domain(make_omelette, cooking).
goal(make_omelette, omelette).
needs(make_omelette, egg).
needs(make_omelette, butter).
needs(make_omelette, salt).
needs(make_omelette, bowl).
needs(make_omelette, fork).
needs(make_omelette, pan).
takes_time(make_omelette, 6, 10, minute).
step(make_omelette, make_omelette_s1).
action(make_omelette_s1, crack).
object_of(make_omelette_s1, egg).
uses(make_omelette_s1, bowl).
role(make_omelette_s1, bowl, contain).
step(make_omelette, make_omelette_s2).
action(make_omelette_s2, beat).
object_of(make_omelette_s2, egg).
uses(make_omelette_s2, fork).
role(make_omelette_s2, fork, beat).
uses(make_omelette_s2, salt).
role(make_omelette_s2, salt, season).
amount(make_omelette_s2, salt, 1, pinch).
takes_time(make_omelette_s2, 1, 1, minute).
step(make_omelette, make_omelette_s3).
action(make_omelette_s3, melt).
object_of(make_omelette_s3, butter).
uses(make_omelette_s3, pan).
role(make_omelette_s3, pan, heat).
takes_time(make_omelette_s3, 1, 2, minute).
step(make_omelette, make_omelette_s4).
action(make_omelette_s4, cook).
object_of(make_omelette_s4, egg).
uses(make_omelette_s4, pan).
role(make_omelette_s4, pan, heat).
until(make_omelette_s4, set).
takes_time(make_omelette_s4, 3, 4, minute).
step(make_omelette, make_omelette_s5).
action(make_omelette_s5, fold).
object_of(make_omelette_s5, omelette).
before(make_omelette_s1, make_omelette_s2).
before(make_omelette_s2, make_omelette_s3).
before(make_omelette_s3, make_omelette_s4).
before(make_omelette_s4, make_omelette_s5).
```

## aliases
```datalog
task_alias(make_omelette, "making an omelette").
task_alias(make_omelette, "plain omelette").
task_alias(make_omelette, "cook an omelette").
alias(egg, "eggs").
alias(pan, "frying pan").
alias(fold, "fold in half").
alias(cheese, "grated cheese").
alias(almost_set, "almost set").
alias(set, "set").
```

## teach
- To make an omelette you need eggs, butter, salt, a bowl, a fork and a frying pan.
- Making an omelette takes 6 to 10 minutes.
- Step 1 of making an omelette: crack the eggs into the bowl.
- Step 2 of making an omelette: beat the eggs with a pinch of salt using the fork for 1 minute.
- Step 3 of making an omelette: melt the butter in the pan for 1 to 2 minutes.
- Step 4 of making an omelette: cook the eggs in the pan until they are set, for 3 to 4 minutes.
- Step 5 of making an omelette: fold the omelette in half.

## variant
- change: no fork: beat the eggs with a whisk
- request: Every fork I own is in the dishwasher halfway through a cycle. I've got a whisk though. Can I still do a quick plain omelette?
- situation: I have eggs, butter, salt, a bowl, a whisk and a frying pan, but no clean fork.
- expect: The reply beats the eggs with the whisk in place of the fork and keeps the crack, melt, cook and fold steps.

```datalog situation
has(user, egg).
has(user, butter).
has(user, salt).
has(user, bowl).
has(user, whisk).
has(user, pan).
lacks(user, fork).
```

```datalog difference
swap(make_omelette_s2, fork, whisk).
no_need(fork).
extra_need(whisk).
```
