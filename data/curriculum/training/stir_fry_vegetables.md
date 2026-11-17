---
id: stir_fry_vegetables
stage: procedures
task: stir_fry_vegetables
---
# Stir-fry vegetables

## requests
- How do I make a quick vegetable stir fry?
- I've got peppers, carrots and broccoli. Can I stir fry them in my wok?
- What's the right order for stir frying veg so it stays crunchy?

## situation
- I have mixed vegetables, garlic, oil, soy sauce, a knife and a wok.
- I'm hungry and have about 20 minutes.

## ground truth
```datalog
task(stir_fry_vegetables).
needs(stir_fry_vegetables, vegetable).
needs(stir_fry_vegetables, garlic).
needs(stir_fry_vegetables, oil).
needs(stir_fry_vegetables, soy_sauce).
needs(stir_fry_vegetables, knife).
needs(stir_fry_vegetables, wok).
takes_time(stir_fry_vegetables, 14, 19, minute).
step(stir_fry_vegetables, stir_fry_vegetables_s1).
action(stir_fry_vegetables_s1, chop).
object_of(stir_fry_vegetables_s1, vegetable).
uses(stir_fry_vegetables_s1, knife).
uses(stir_fry_vegetables_s1, garlic).
takes_time(stir_fry_vegetables_s1, 8, 10, minute).
step(stir_fry_vegetables, stir_fry_vegetables_s2).
action(stir_fry_vegetables_s2, heat).
object_of(stir_fry_vegetables_s2, oil).
uses(stir_fry_vegetables_s2, wok).
until(stir_fry_vegetables_s2, hot).
takes_time(stir_fry_vegetables_s2, 1, 2, minute).
step(stir_fry_vegetables, stir_fry_vegetables_s3).
action(stir_fry_vegetables_s3, fry).
object_of(stir_fry_vegetables_s3, garlic).
uses(stir_fry_vegetables_s3, wok).
until(stir_fry_vegetables_s3, fragrant).
step(stir_fry_vegetables, stir_fry_vegetables_s4).
action(stir_fry_vegetables_s4, fry).
object_of(stir_fry_vegetables_s4, vegetable).
uses(stir_fry_vegetables_s4, wok).
until(stir_fry_vegetables_s4, tender).
takes_time(stir_fry_vegetables_s4, 4, 6, minute).
step(stir_fry_vegetables, stir_fry_vegetables_s5).
action(stir_fry_vegetables_s5, add).
object_of(stir_fry_vegetables_s5, soy_sauce).
takes_time(stir_fry_vegetables_s5, 1, 1, minute).
before(stir_fry_vegetables_s1, stir_fry_vegetables_s2).
before(stir_fry_vegetables_s2, stir_fry_vegetables_s3).
before(stir_fry_vegetables_s3, stir_fry_vegetables_s4).
before(stir_fry_vegetables_s4, stir_fry_vegetables_s5).
```

## teach
- To stir-fry vegetables you need vegetables, garlic, oil, soy sauce, a knife and a wok.
- Stir-frying vegetables takes 14 to 19 minutes.
- Step 1 of stir-frying vegetables: chop the vegetables into thin strips and slice the garlic with the knife for 8 to 10 minutes.
- Step 2 of stir-frying vegetables: heat the oil in the wok until very hot, for 1 to 2 minutes.
- Step 3 of stir-frying vegetables: fry the garlic in the wok until fragrant.
- Step 4 of stir-frying vegetables: fry the vegetables in the wok over high heat, tossing them all the time, until just tender, for 4 to 6 minutes.
- Step 5 of stir-frying vegetables: add the soy sauce and toss to coat for 1 minute.

## checklist
- names what is needed: vegetables, garlic, oil, soy sauce, a knife and a wok
- gives the steps in order: chop vegetable, heat oil, fry garlic, fry vegetable, add soy sauce
- gives a total time of about 14 to 19 minutes
- closes politely

## variant
- change: no wok: use the largest frying pan and cook the vegetables in two batches
- request: I don't have a wok, just normal frying pans. Can I still do a stir fry?
- situation: I have mixed vegetables, garlic, oil, soy sauce, a knife and a large frying pan.
- expect: The reply uses the large frying pan and fries the vegetables in two batches so they do not stew, keeping the chop, garlic and soy sauce steps.
