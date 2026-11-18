---
id: make_french_toast
stage: procedures
task: make_french_toast
schema: v3
---
# Make French toast

## requests
- I've got some stale bread. Can you tell me how to turn it into French toast?
- What's the method for French toast? I want it for breakfast.
- How do I make eggy bread in a frying pan?

## situation
- I have bread, eggs, milk, butter, a frying pan and a bowl.
- Breakfast is in about 20 minutes.

## ground truth
```datalog
task(make_french_toast).
domain(make_french_toast, cooking).
goal(make_french_toast, french_toast).
needs(make_french_toast, bread).
needs(make_french_toast, egg).
needs(make_french_toast, milk).
needs(make_french_toast, butter).
needs(make_french_toast, bowl).
needs(make_french_toast, frying_pan).
takes_time(make_french_toast, 12, 15, minute).
step(make_french_toast, make_french_toast_s1).
action(make_french_toast_s1, whisk).
object_of(make_french_toast_s1, egg).
uses(make_french_toast_s1, milk).
role(make_french_toast_s1, milk, mix).
uses(make_french_toast_s1, bowl).
role(make_french_toast_s1, bowl, contain).
takes_time(make_french_toast_s1, 2, 3, minute).
step(make_french_toast, make_french_toast_s2).
action(make_french_toast_s2, melt).
object_of(make_french_toast_s2, butter).
uses(make_french_toast_s2, frying_pan).
role(make_french_toast_s2, frying_pan, heat).
setting(make_french_toast_s2, heat, medium, level).
overlaps(make_french_toast_s2, make_french_toast_s1).
takes_time(make_french_toast_s2, 2, 3, minute).
step(make_french_toast, make_french_toast_s3).
action(make_french_toast_s3, dip).
object_of(make_french_toast_s3, bread).
uses(make_french_toast_s3, bowl).
role(make_french_toast_s3, bowl, contain).
into(make_french_toast_s3, egg).
takes_time(make_french_toast_s3, 1, 2, minute).
step(make_french_toast, make_french_toast_s4).
action(make_french_toast_s4, fry).
object_of(make_french_toast_s4, bread).
uses(make_french_toast_s4, frying_pan).
role(make_french_toast_s4, frying_pan, heat).
until(make_french_toast_s4, golden).
check(make_french_toast_s4, golden_both_sides).
takes_time(make_french_toast_s4, 6, 8, minute).
before(make_french_toast_s1, make_french_toast_s3).
before(make_french_toast_s2, make_french_toast_s3).
before(make_french_toast_s3, make_french_toast_s4).
```

## aliases
```datalog
task_alias(make_french_toast, "making French toast").
task_alias(make_french_toast, "French toast").
task_alias(make_french_toast, "eggy bread").
alias(egg, "eggs").
alias(egg, "egg mixture").
alias(bread, "stale bread").
alias(bread, "slice of bread").
alias(frying_pan, "pan").
alias(golden, "golden brown").
alias(golden_both_sides, "golden brown on both sides").
alias(medium, "medium heat").
alias(vegetable_oil, "oil").
alias(vegetable_oil, "vegetable oil").
```

## teach
- To make French toast you need bread, eggs, milk, butter, a bowl and a frying pan.
- Making French toast takes 12 to 15 minutes.
- Step 1 of making French toast: whisk the eggs and the milk in the bowl for 2 to 3 minutes.
- While step 1 runs, step 2 of making French toast: melt the butter in the frying pan over medium heat for 2 to 3 minutes.
- Step 3 of making French toast: dip each slice of bread in the egg mixture in the bowl, turning it once, for 1 to 2 minutes.
- Step 4 of making French toast: fry the bread in the frying pan until golden brown on both sides, for 6 to 8 minutes.

## checklist
- names what is needed: bread, eggs, milk, butter, a bowl and a frying pan
- gives the steps in order: whisk egg, melt butter (while whisk egg), dip bread, fry bread
- gives a total time of about 12 to 15 minutes
- closes politely

## variant
- change: no butter: heat a little vegetable oil in the pan instead
- request: I've run out of butter. Can I still make French toast?
- situation: I have bread, eggs, milk, vegetable oil, a frying pan and a bowl, but no butter.
- expect: The reply heats a little oil in the pan instead of melting butter and keeps the whisk, dip and fry steps.

```datalog situation
has(user, bread).
has(user, egg).
has(user, milk).
has(user, vegetable_oil).
has(user, frying_pan).
has(user, bowl).
lacks(user, butter).
```

```datalog difference
change(make_french_toast_s2, action, melt, heat).
change(make_french_toast_s2, object_of, butter, vegetable_oil).
extra_need(vegetable_oil).
no_need(butter).
```
