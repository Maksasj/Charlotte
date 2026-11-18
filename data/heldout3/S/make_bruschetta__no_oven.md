---
id: make_bruschetta__no_oven
stage: procedures
task: make_bruschetta
schema: v3
tier: S
base: make_bruschetta
---
# Make bruschetta

## requests
- I want tomato toasts as a starter, but my flat has no oven, only an air fryer. Can I do them?

## ground truth
```datalog
task(make_bruschetta).
domain(make_bruschetta, cooking).
goal(make_bruschetta, bruschetta).
needs(make_bruschetta, baguette).
needs(make_bruschetta, tomato).
needs(make_bruschetta, garlic).
needs(make_bruschetta, basil).
needs(make_bruschetta, olive_oil).
needs(make_bruschetta, salt).
needs(make_bruschetta, knife).
needs(make_bruschetta, bowl).
needs(make_bruschetta, oven).
takes_time(make_bruschetta, 10, 14, minute).
step(make_bruschetta, make_bruschetta_s1).
action(make_bruschetta_s1, dice).
object_of(make_bruschetta_s1, tomato).
uses(make_bruschetta_s1, knife).
role(make_bruschetta_s1, knife, cut).
takes_time(make_bruschetta_s1, 5, 6, minute).
step(make_bruschetta, make_bruschetta_s2).
action(make_bruschetta_s2, tear).
object_of(make_bruschetta_s2, basil).
step(make_bruschetta, make_bruschetta_s3).
action(make_bruschetta_s3, toast).
object_of(make_bruschetta_s3, baguette).
uses(make_bruschetta_s3, oven).
role(make_bruschetta_s3, oven, heat).
overlaps(make_bruschetta_s3, make_bruschetta_s1).
until(make_bruschetta_s3, golden).
takes_time(make_bruschetta_s3, 4, 6, minute).
step(make_bruschetta, make_bruschetta_s4).
action(make_bruschetta_s4, mix).
object_of(make_bruschetta_s4, tomato).
uses(make_bruschetta_s4, olive_oil).
role(make_bruschetta_s4, olive_oil, flavour).
uses(make_bruschetta_s4, salt).
role(make_bruschetta_s4, salt, season).
uses(make_bruschetta_s4, bowl).
role(make_bruschetta_s4, bowl, contain).
takes_time(make_bruschetta_s4, 1, 2, minute).
step(make_bruschetta, make_bruschetta_s5).
action(make_bruschetta_s5, rub).
object_of(make_bruschetta_s5, garlic).
uses(make_bruschetta_s5, baguette).
role(make_bruschetta_s5, baguette, hold).
onto(make_bruschetta_s5, baguette).
requires_state(make_bruschetta_s5, garlic_cut).
takes_time(make_bruschetta_s5, 1, 1, minute).
step(make_bruschetta, make_bruschetta_s6).
action(make_bruschetta_s6, spoon).
object_of(make_bruschetta_s6, tomato).
uses(make_bruschetta_s6, baguette).
role(make_bruschetta_s6, baguette, hold).
onto(make_bruschetta_s6, baguette).
takes_time(make_bruschetta_s6, 1, 2, minute).
before(make_bruschetta_s1, make_bruschetta_s2).
before(make_bruschetta_s1, make_bruschetta_s4).
before(make_bruschetta_s2, make_bruschetta_s4).
before(make_bruschetta_s3, make_bruschetta_s4).
before(make_bruschetta_s4, make_bruschetta_s5).
before(make_bruschetta_s5, make_bruschetta_s6).
```

## aliases
```datalog
task_alias(make_bruschetta, "making bruschetta").
task_alias(make_bruschetta, "tomato bruschetta").
task_alias(make_bruschetta, "garlicky tomato toasts").
alias(tomato, "tomatoes").
alias(tomato, "ripe tomatoes").
alias(tomato, "tomato mix").
alias(baguette, "baguette slices").
alias(baguette, "toasted baguette").
alias(oven, "oven grill").
alias(oven, "grill").
alias(garlic, "cut garlic").
alias(garlic_cut, "the cut garlic").
```

## teach
- To make bruschetta you need a baguette, tomatoes, garlic, basil, olive oil, salt, a knife, a bowl and an oven.
- Making bruschetta takes 10 to 14 minutes.
- Step 1 of making bruschetta: dice the tomatoes with the knife for 5 to 6 minutes.
- Step 2 of making bruschetta: tear the basil.
- While step 1 runs, step 3 of making bruschetta: toast the baguette slices under the oven grill until golden, for 4 to 6 minutes.
- Step 4 of making bruschetta: mix the tomatoes with olive oil and salt in the bowl for 1 to 2 minutes.
- Step 5 of making bruschetta: rub the cut garlic over the toasted baguette for 1 minute.
- Step 6 of making bruschetta: spoon the tomato mix onto the baguette slices for 1 to 2 minutes.

## variant
- change: no oven: crisp the baguette slices in an air fryer
- request: I want tomato toasts as a starter, but my flat has no oven, only an air fryer. Can I do them?
- situation: I have a baguette, tomatoes, garlic, basil, olive oil, salt, a knife, a bowl and an air fryer, but no oven.
- expect: The reply toasts the baguette slices in the air fryer instead of under the oven grill and keeps the tomato topping steps.

```datalog situation
has(user, baguette).
has(user, tomato).
has(user, garlic).
has(user, basil).
has(user, olive_oil).
has(user, salt).
has(user, knife).
has(user, bowl).
has(user, air_fryer).
lacks(user, oven).
```

```datalog difference
swap(make_bruschetta_s3, oven, air_fryer).
no_need(oven).
extra_need(air_fryer).
```
