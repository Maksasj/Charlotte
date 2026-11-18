---
id: make_omelette
stage: procedures
task: make_omelette
schema: v3
---
# Make an omelette

## requests
- I want to make an omelette for lunch. Can you tell me how?
- How do you cook a simple plain omelette in a frying pan?
- I've got some eggs and fancy an omelette. What are the steps?

## situation
- I have eggs, butter, salt, a fork, a bowl and a frying pan.
- I've got about 15 minutes.

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

## checklist
- names what is needed, including eggs, butter and a pan
- gives the steps in order: crack, beat, melt butter, cook until set, fold
- gives a total time of about 6 to 10 minutes
- closes politely

## variant
- change: add grated cheese before folding
- request: Can you tell me how to make a cheese omelette? I love it when it's gooey inside.
- situation: I have eggs, butter, salt, some grated cheese, a bowl, a fork and a pan.
- expect: The reply adds cheese, sprinkling it over the eggs once they are almost set and before folding.

```datalog situation
has(user, egg).
has(user, butter).
has(user, salt).
has(user, cheese).
has(user, bowl).
has(user, fork).
has(user, pan).
wants(user, cheese).
```

```datalog difference
extra_need(cheese).
add_step(make_omelette_s9, make_omelette_s4).
step(make_omelette, make_omelette_s9).
action(make_omelette_s9, sprinkle).
object_of(make_omelette_s9, cheese).
onto(make_omelette_s9, egg).
requires_state(make_omelette_s9, almost_set).
overlaps(make_omelette_s9, make_omelette_s4).
before(make_omelette_s9, make_omelette_s5).
```
