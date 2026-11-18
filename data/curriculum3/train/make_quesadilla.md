---
id: make_quesadilla
stage: procedures
task: make_quesadilla
schema: v3
---
# Make a quesadilla

## requests
- I want a quick cheesy lunch. How do I make a quesadilla?
- Can you show me how to make a cheese quesadilla in a pan?
- What are the steps for a crispy quesadilla?

## situation
- I have two tortillas, grated cheese, a frying pan, a spatula and a knife.
- I have about 15 minutes for lunch.

## ground truth
```datalog
task(make_quesadilla).
domain(make_quesadilla, cooking).
goal(make_quesadilla, quesadilla).
needs(make_quesadilla, tortilla).
needs(make_quesadilla, cheese).
needs(make_quesadilla, frying_pan).
needs(make_quesadilla, spatula).
needs(make_quesadilla, knife).
takes_time(make_quesadilla, 8, 12, minute).
step(make_quesadilla, make_quesadilla_s1).
action(make_quesadilla_s1, heat).
object_of(make_quesadilla_s1, frying_pan).
setting(make_quesadilla_s1, heat, medium, level).
requires_state(make_quesadilla_s1, pan_dry).
takes_time(make_quesadilla_s1, 2, 3, minute).
step(make_quesadilla, make_quesadilla_s2).
action(make_quesadilla_s2, sprinkle).
object_of(make_quesadilla_s2, cheese).
uses(make_quesadilla_s2, tortilla).
role(make_quesadilla_s2, tortilla, hold).
onto(make_quesadilla_s2, tortilla).
step(make_quesadilla, make_quesadilla_s3).
action(make_quesadilla_s3, put).
object_of(make_quesadilla_s3, tortilla).
onto(make_quesadilla_s3, cheese).
takes_time(make_quesadilla_s3, 1, 1, minute).
step(make_quesadilla, make_quesadilla_s4).
action(make_quesadilla_s4, fry).
object_of(make_quesadilla_s4, quesadilla).
uses(make_quesadilla_s4, frying_pan).
role(make_quesadilla_s4, frying_pan, heat).
with_part(make_quesadilla_s4, bottom).
until(make_quesadilla_s4, golden).
takes_time(make_quesadilla_s4, 2, 3, minute).
step(make_quesadilla, make_quesadilla_s5).
action(make_quesadilla_s5, flip).
object_of(make_quesadilla_s5, quesadilla).
uses(make_quesadilla_s5, spatula).
role(make_quesadilla_s5, spatula, lift).
step(make_quesadilla, make_quesadilla_s6).
action(make_quesadilla_s6, fry).
object_of(make_quesadilla_s6, quesadilla).
uses(make_quesadilla_s6, frying_pan).
role(make_quesadilla_s6, frying_pan, heat).
with_part(make_quesadilla_s6, other_side).
until(make_quesadilla_s6, melted).
takes_time(make_quesadilla_s6, 2, 3, minute).
step(make_quesadilla, make_quesadilla_s7).
action(make_quesadilla_s7, cut).
object_of(make_quesadilla_s7, quesadilla).
uses(make_quesadilla_s7, knife).
role(make_quesadilla_s7, knife, cut).
achieves(make_quesadilla_s7, wedge).
before(make_quesadilla_s1, make_quesadilla_s2).
before(make_quesadilla_s2, make_quesadilla_s3).
before(make_quesadilla_s3, make_quesadilla_s4).
before(make_quesadilla_s4, make_quesadilla_s5).
before(make_quesadilla_s5, make_quesadilla_s6).
before(make_quesadilla_s6, make_quesadilla_s7).
```

## aliases
```datalog
task_alias(make_quesadilla, "making a quesadilla").
task_alias(make_quesadilla, "cheese quesadilla").
task_alias(make_quesadilla, "crispy quesadilla").
alias(tortilla, "tortillas").
alias(tortilla, "second tortilla").
alias(cheese, "grated cheese").
alias(frying_pan, "pan").
alias(frying_pan, "dry frying pan").
alias(melted, "the cheese melts").
alias(wedge, "wedges").
alias(medium, "medium heat").
alias(chicken, "cooked chicken").
alias(chicken, "leftover roast chicken").
```

## teach
- To make a quesadilla you need two tortillas, grated cheese, a frying pan, a spatula and a knife.
- Making a quesadilla takes 8 to 12 minutes.
- Step 1 of making a quesadilla: heat the dry frying pan over medium heat for 2 to 3 minutes.
- Step 2 of making a quesadilla: sprinkle the cheese over one tortilla.
- Step 3 of making a quesadilla: put the second tortilla on top for 1 minute.
- Step 4 of making a quesadilla: fry the quesadilla in the pan until the bottom is golden, for 2 to 3 minutes.
- Step 5 of making a quesadilla: flip the quesadilla with the spatula.
- Step 6 of making a quesadilla: fry the other side until the cheese melts, for 2 to 3 minutes.
- Step 7 of making a quesadilla: cut the quesadilla into wedges with the knife.

## checklist
- names what is needed: two tortillas, grated cheese, a frying pan, a spatula and a knife
- gives the steps in order: heat frying pan, sprinkle cheese, fry quesadilla, flip quesadilla, cut quesadilla
- gives a total time of about 8 to 12 minutes
- closes politely

## variant
- change: leftover chicken: scatter it over the cheese before closing the quesadilla
- request: I've got some leftover roast chicken. Can I put it in a quesadilla?
- situation: I have two tortillas, grated cheese, some cooked chicken, a frying pan, a spatula and a knife.
- expect: The reply scatters the chicken over the cheese before adding the second tortilla and keeps the fry, flip and cut steps.

```datalog situation
has(user, tortilla).
has(user, cheese).
has(user, chicken).
has(user, frying_pan).
has(user, spatula).
has(user, knife).
```

```datalog difference
add_step(make_quesadilla_s9, make_quesadilla_s2).
step(make_quesadilla, make_quesadilla_s9).
action(make_quesadilla_s9, scatter).
object_of(make_quesadilla_s9, chicken).
onto(make_quesadilla_s9, cheese).
before(make_quesadilla_s9, make_quesadilla_s3).
extra_need(chicken).
```
