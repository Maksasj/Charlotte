---
id: make_quesadilla
stage: procedures
task: make_quesadilla
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
needs(make_quesadilla, tortilla).
needs(make_quesadilla, cheese).
needs(make_quesadilla, frying_pan).
needs(make_quesadilla, spatula).
needs(make_quesadilla, knife).
takes_time(make_quesadilla, 8, 12, minute).
step(make_quesadilla, make_quesadilla_s1).
action(make_quesadilla_s1, heat).
object_of(make_quesadilla_s1, frying_pan).
takes_time(make_quesadilla_s1, 2, 3, minute).
step(make_quesadilla, make_quesadilla_s2).
action(make_quesadilla_s2, sprinkle).
object_of(make_quesadilla_s2, cheese).
uses(make_quesadilla_s2, tortilla).
takes_time(make_quesadilla_s2, 1, 1, minute).
step(make_quesadilla, make_quesadilla_s3).
action(make_quesadilla_s3, fry).
object_of(make_quesadilla_s3, quesadilla).
uses(make_quesadilla_s3, frying_pan).
until(make_quesadilla_s3, golden).
takes_time(make_quesadilla_s3, 2, 3, minute).
step(make_quesadilla, make_quesadilla_s4).
action(make_quesadilla_s4, flip).
object_of(make_quesadilla_s4, quesadilla).
uses(make_quesadilla_s4, spatula).
until(make_quesadilla_s4, melted).
takes_time(make_quesadilla_s4, 2, 3, minute).
step(make_quesadilla, make_quesadilla_s5).
action(make_quesadilla_s5, cut).
object_of(make_quesadilla_s5, quesadilla).
uses(make_quesadilla_s5, knife).
before(make_quesadilla_s1, make_quesadilla_s2).
before(make_quesadilla_s2, make_quesadilla_s3).
before(make_quesadilla_s3, make_quesadilla_s4).
before(make_quesadilla_s4, make_quesadilla_s5).
```

## teach
- To make a quesadilla you need two tortillas, grated cheese, a frying pan, a spatula and a knife.
- Making a quesadilla takes 8 to 12 minutes.
- Step 1 of making a quesadilla: heat the dry frying pan over medium heat for 2 to 3 minutes.
- Step 2 of making a quesadilla: sprinkle the cheese over one tortilla and put the second tortilla on top for 1 minute.
- Step 3 of making a quesadilla: fry the quesadilla in the pan until the bottom is golden, for 2 to 3 minutes.
- Step 4 of making a quesadilla: flip the quesadilla with the spatula and fry the other side until the cheese melts, for 2 to 3 minutes.
- Step 5 of making a quesadilla: cut the quesadilla into wedges with the knife.

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
