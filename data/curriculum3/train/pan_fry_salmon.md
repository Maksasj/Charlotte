---
id: pan_fry_salmon
stage: procedures
task: pan_fry_salmon
schema: v3
---
# Pan-fry salmon

## requests
- How do I cook a salmon fillet in a pan so the skin goes crispy?
- Can you walk me through pan-frying salmon for dinner?
- I bought a fresh salmon fillet. What's a simple way to cook it?

## situation
- I have a salmon fillet, oil, salt, pepper, a lemon, paper towels, a frying pan and a spatula.
- I have about 20 minutes.

## ground truth
```datalog
task(pan_fry_salmon).
domain(pan_fry_salmon, cooking).
goal(pan_fry_salmon, cooked_salmon).
needs(pan_fry_salmon, salmon_fillet).
needs(pan_fry_salmon, oil).
needs(pan_fry_salmon, salt).
needs(pan_fry_salmon, pepper).
needs(pan_fry_salmon, lemon).
needs(pan_fry_salmon, paper_towel).
needs(pan_fry_salmon, frying_pan).
needs(pan_fry_salmon, spatula).
takes_time(pan_fry_salmon, 12, 16, minute).
step(pan_fry_salmon, pan_fry_salmon_s1).
action(pan_fry_salmon_s1, pat).
object_of(pan_fry_salmon_s1, salmon_fillet).
uses(pan_fry_salmon_s1, paper_towel).
role(pan_fry_salmon_s1, paper_towel, dry).
achieves(pan_fry_salmon_s1, dry).
purpose(pan_fry_salmon_s1, crispy_skin).
step(pan_fry_salmon, pan_fry_salmon_s2).
action(pan_fry_salmon_s2, season).
object_of(pan_fry_salmon_s2, salmon_fillet).
uses(pan_fry_salmon_s2, salt).
role(pan_fry_salmon_s2, salt, season).
uses(pan_fry_salmon_s2, pepper).
role(pan_fry_salmon_s2, pepper, season).
takes_time(pan_fry_salmon_s2, 1, 2, minute).
step(pan_fry_salmon, pan_fry_salmon_s3).
action(pan_fry_salmon_s3, heat).
object_of(pan_fry_salmon_s3, oil).
uses(pan_fry_salmon_s3, frying_pan).
role(pan_fry_salmon_s3, frying_pan, heat).
takes_time(pan_fry_salmon_s3, 2, 3, minute).
step(pan_fry_salmon, pan_fry_salmon_s4).
action(pan_fry_salmon_s4, fry).
object_of(pan_fry_salmon_s4, salmon_fillet).
uses(pan_fry_salmon_s4, frying_pan).
role(pan_fry_salmon_s4, frying_pan, heat).
with_part(pan_fry_salmon_s4, skin).
until(pan_fry_salmon_s4, crisp).
takes_time(pan_fry_salmon_s4, 4, 5, minute).
step(pan_fry_salmon, pan_fry_salmon_s5).
action(pan_fry_salmon_s5, flip).
object_of(pan_fry_salmon_s5, salmon_fillet).
uses(pan_fry_salmon_s5, spatula).
role(pan_fry_salmon_s5, spatula, lift).
step(pan_fry_salmon, pan_fry_salmon_s6).
action(pan_fry_salmon_s6, cook).
object_of(pan_fry_salmon_s6, salmon_fillet).
with_part(pan_fry_salmon_s6, other_side).
takes_time(pan_fry_salmon_s6, 2, 3, minute).
step(pan_fry_salmon, pan_fry_salmon_s7).
action(pan_fry_salmon_s7, rest).
object_of(pan_fry_salmon_s7, salmon_fillet).
takes_time(pan_fry_salmon_s7, 2, 2, minute).
step(pan_fry_salmon, pan_fry_salmon_s8).
action(pan_fry_salmon_s8, serve).
object_of(pan_fry_salmon_s8, salmon_fillet).
uses(pan_fry_salmon_s8, lemon).
role(pan_fry_salmon_s8, lemon, flavour).
before(pan_fry_salmon_s1, pan_fry_salmon_s2).
before(pan_fry_salmon_s2, pan_fry_salmon_s3).
before(pan_fry_salmon_s3, pan_fry_salmon_s4).
before(pan_fry_salmon_s4, pan_fry_salmon_s5).
before(pan_fry_salmon_s5, pan_fry_salmon_s6).
before(pan_fry_salmon_s6, pan_fry_salmon_s7).
before(pan_fry_salmon_s7, pan_fry_salmon_s8).
```

## aliases
```datalog
task_alias(pan_fry_salmon, "pan-frying salmon").
task_alias(pan_fry_salmon, "pan-fry salmon").
task_alias(pan_fry_salmon, "cook a salmon fillet in a pan").
alias(salmon_fillet, "salmon").
alias(salmon_fillet, "fillet").
alias(salmon_fillet, "fresh salmon fillet").
alias(paper_towel, "paper towels").
alias(frying_pan, "pan").
alias(lemon, "wedge of lemon").
alias(skin, "skin side down").
alias(crisp, "the skin is crisp").
alias(crispy_skin, "the skin crisps").
alias(other_side, "the other side").
```

## teach
- To pan-fry salmon you need a salmon fillet, oil, salt, pepper, a lemon, paper towels, a frying pan and a spatula.
- Pan-frying salmon takes 12 to 16 minutes.
- Step 1 of pan-frying salmon: pat the salmon fillet dry with paper towels so the skin crisps.
- Step 2 of pan-frying salmon: season the salmon fillet with salt and pepper for 1 to 2 minutes.
- Step 3 of pan-frying salmon: heat the oil in the frying pan for 2 to 3 minutes.
- Step 4 of pan-frying salmon: fry the salmon fillet skin side down until the skin is crisp, for 4 to 5 minutes.
- Step 5 of pan-frying salmon: flip the fillet with the spatula.
- Step 6 of pan-frying salmon: cook the other side for 2 to 3 minutes.
- Step 7 of pan-frying salmon: rest the salmon fillet for 2 minutes.
- Step 8 of pan-frying salmon: serve it with a wedge of lemon.

## checklist
- names what is needed: a salmon fillet, oil, salt, pepper, a lemon, paper towels, a frying pan and a spatula
- gives the steps in order: pat salmon fillet, heat oil, fry salmon fillet, flip salmon fillet, rest salmon fillet
- gives a total time of about 12 to 16 minutes
- closes politely
