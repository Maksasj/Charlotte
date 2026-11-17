---
id: pan_fry_salmon
stage: procedures
task: pan_fry_salmon
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
uses(pan_fry_salmon_s1, salt).
uses(pan_fry_salmon_s1, pepper).
takes_time(pan_fry_salmon_s1, 1, 2, minute).
step(pan_fry_salmon, pan_fry_salmon_s2).
action(pan_fry_salmon_s2, heat).
object_of(pan_fry_salmon_s2, oil).
uses(pan_fry_salmon_s2, frying_pan).
takes_time(pan_fry_salmon_s2, 2, 3, minute).
step(pan_fry_salmon, pan_fry_salmon_s3).
action(pan_fry_salmon_s3, fry).
object_of(pan_fry_salmon_s3, salmon_fillet).
uses(pan_fry_salmon_s3, frying_pan).
until(pan_fry_salmon_s3, crisp).
takes_time(pan_fry_salmon_s3, 4, 5, minute).
step(pan_fry_salmon, pan_fry_salmon_s4).
action(pan_fry_salmon_s4, flip).
object_of(pan_fry_salmon_s4, salmon_fillet).
uses(pan_fry_salmon_s4, spatula).
takes_time(pan_fry_salmon_s4, 2, 3, minute).
step(pan_fry_salmon, pan_fry_salmon_s5).
action(pan_fry_salmon_s5, rest).
object_of(pan_fry_salmon_s5, salmon_fillet).
uses(pan_fry_salmon_s5, lemon).
takes_time(pan_fry_salmon_s5, 2, 2, minute).
before(pan_fry_salmon_s1, pan_fry_salmon_s2).
before(pan_fry_salmon_s2, pan_fry_salmon_s3).
before(pan_fry_salmon_s3, pan_fry_salmon_s4).
before(pan_fry_salmon_s4, pan_fry_salmon_s5).
```

## teach
- To pan-fry salmon you need a salmon fillet, oil, salt, pepper, a lemon, paper towels, a frying pan and a spatula.
- Pan-frying salmon takes 12 to 16 minutes.
- Step 1 of pan-frying salmon: pat the salmon fillet dry with paper towels and season it with salt and pepper for 1 to 2 minutes.
- Step 2 of pan-frying salmon: heat the oil in the frying pan for 2 to 3 minutes.
- Step 3 of pan-frying salmon: fry the salmon fillet skin side down until the skin is crisp, for 4 to 5 minutes.
- Step 4 of pan-frying salmon: flip the fillet with the spatula and cook the other side for 2 to 3 minutes.
- Step 5 of pan-frying salmon: rest the salmon fillet and serve it with a wedge of lemon for 2 minutes.

## checklist
- names what is needed: a salmon fillet, oil, salt, pepper, a lemon, paper towels, a frying pan and a spatula
- gives the steps in order: pat salmon fillet, heat oil, fry salmon fillet, flip salmon fillet, rest salmon fillet
- gives a total time of about 12 to 16 minutes
- closes politely
