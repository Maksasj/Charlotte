---
id: make_salad
stage: procedures
task: make_salad
schema: v3
---
# Make a salad

## requests
- I want a fresh green salad as a side tonight. How should I put it together?
- Can you tell me how to make a simple salad with a homemade dressing?
- I need something light and quick, maybe a salad. What do I do?

## situation
- I have lettuce, tomatoes, a cucumber, olive oil, a lemon, a knife and a big bowl.
- I've got about 15 minutes.

## ground truth
```datalog
task(make_salad).
domain(make_salad, cooking).
goal(make_salad, salad).
needs(make_salad, lettuce).
needs(make_salad, tomato).
needs(make_salad, cucumber).
needs(make_salad, olive_oil).
needs(make_salad, lemon).
needs(make_salad, knife).
needs(make_salad, bowl).
takes_time(make_salad, 10, 15, minute).
step(make_salad, make_salad_s1).
action(make_salad_s1, wash).
object_of(make_salad_s1, lettuce).
takes_time(make_salad_s1, 2, 3, minute).
step(make_salad, make_salad_s2).
action(make_salad_s2, chop).
object_of(make_salad_s2, tomato).
uses(make_salad_s2, knife).
role(make_salad_s2, knife, cut).
step(make_salad, make_salad_s3).
action(make_salad_s3, slice).
object_of(make_salad_s3, cucumber).
uses(make_salad_s3, knife).
role(make_salad_s3, knife, cut).
step(make_salad, make_salad_s4).
action(make_salad_s4, mix).
object_of(make_salad_s4, olive_oil).
uses(make_salad_s4, lemon).
role(make_salad_s4, lemon, flavour).
step(make_salad, make_salad_s5).
action(make_salad_s5, toss).
object_of(make_salad_s5, salad).
uses(make_salad_s5, bowl).
role(make_salad_s5, bowl, contain).
before(make_salad_s1, make_salad_s2).
before(make_salad_s2, make_salad_s3).
before(make_salad_s3, make_salad_s4).
before(make_salad_s4, make_salad_s5).
```

## aliases
```datalog
task_alias(make_salad, "making a salad").
task_alias(make_salad, "green salad").
task_alias(make_salad, "simple salad").
alias(salad, "green salad").
alias(tomato, "tomatoes").
alias(bowl, "big bowl").
alias(lemon, "juice of the lemon").
alias(lemon, "lemon juice").
alias(lemon, "lemons").
alias(mix, "mix dressing").
alias(vinegar, "some vinegar").
```

## teach
- To make a salad you need lettuce, a tomato, a cucumber, olive oil, a lemon, a knife and a bowl.
- Making a salad takes 10 to 15 minutes.
- Step 1 of making a salad: wash the lettuce for 2 to 3 minutes.
- Step 2 of making a salad: chop the tomato with the knife.
- Step 3 of making a salad: slice the cucumber with the knife.
- Step 4 of making a salad: mix the olive oil with the juice of the lemon.
- Step 5 of making a salad: toss the salad with the dressing in the bowl.

## checklist
- names what is needed, including lettuce, tomato, cucumber, olive oil and lemon
- gives the steps in order: wash, chop, slice, mix dressing, toss
- gives a total time of about 10 to 15 minutes
- closes politely

## variant
- change: no lemon: use vinegar in the dressing
- request: I'm making a salad but I don't have any lemons for the dressing. What can I do?
- situation: I have lettuce, tomatoes, a cucumber, olive oil, some vinegar, a knife and a bowl, but no lemon.
- expect: The reply mixes the olive oil with vinegar instead of lemon juice and keeps the other steps.

```datalog situation
has(user, lettuce).
has(user, tomato).
has(user, cucumber).
has(user, olive_oil).
has(user, vinegar).
has(user, knife).
has(user, bowl).
lacks(user, lemon).
```

```datalog difference
swap(make_salad_s4, lemon, vinegar).
no_need(lemon).
extra_need(vinegar).
```
