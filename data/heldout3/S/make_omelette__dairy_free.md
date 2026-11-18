---
id: make_omelette__dairy_free
stage: procedures
task: make_omelette
schema: v3
tier: S
base: make_omelette
---
# Make an omelette

## requests
- I've gone dairy-free. How do I cook a folded egg omelette without butter?

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
- change: dairy-free: heat a little oil in the pan instead of melting butter
- request: I've gone dairy-free. How do I cook a folded egg omelette without butter?
- situation: I don't eat dairy. I have eggs, oil, salt, a fork, a bowl and a frying pan.
- expect: The reply heats a little oil in the pan instead of melting butter and keeps the crack, beat, cook and fold steps.

```datalog situation
has(user, egg).
has(user, oil).
has(user, salt).
has(user, fork).
has(user, bowl).
has(user, pan).
excludes(user, dairy).
```

```datalog difference
change(make_omelette_s3, action, melt, heat).
change(make_omelette_s3, object_of, butter, oil).
no_need(butter).
extra_need(oil).
```
