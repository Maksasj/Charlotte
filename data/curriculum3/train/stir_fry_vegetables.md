---
id: stir_fry_vegetables
stage: procedures
task: stir_fry_vegetables
schema: v3
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
domain(stir_fry_vegetables, cooking).
goal(stir_fry_vegetables, stir_fry).
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
role(stir_fry_vegetables_s1, knife, cut).
step(stir_fry_vegetables, stir_fry_vegetables_s2).
action(stir_fry_vegetables_s2, slice).
object_of(stir_fry_vegetables_s2, garlic).
uses(stir_fry_vegetables_s2, knife).
role(stir_fry_vegetables_s2, knife, cut).
takes_time(stir_fry_vegetables_s2, 8, 10, minute).
step(stir_fry_vegetables, stir_fry_vegetables_s3).
action(stir_fry_vegetables_s3, heat).
object_of(stir_fry_vegetables_s3, oil).
uses(stir_fry_vegetables_s3, wok).
role(stir_fry_vegetables_s3, wok, heat).
until(stir_fry_vegetables_s3, hot).
takes_time(stir_fry_vegetables_s3, 1, 2, minute).
step(stir_fry_vegetables, stir_fry_vegetables_s4).
action(stir_fry_vegetables_s4, fry).
object_of(stir_fry_vegetables_s4, garlic).
uses(stir_fry_vegetables_s4, wok).
role(stir_fry_vegetables_s4, wok, heat).
until(stir_fry_vegetables_s4, fragrant).
step(stir_fry_vegetables, stir_fry_vegetables_s5).
action(stir_fry_vegetables_s5, fry).
object_of(stir_fry_vegetables_s5, vegetable).
uses(stir_fry_vegetables_s5, wok).
role(stir_fry_vegetables_s5, wok, heat).
setting(stir_fry_vegetables_s5, heat, high, level).
until(stir_fry_vegetables_s5, tender).
takes_time(stir_fry_vegetables_s5, 4, 6, minute).
step(stir_fry_vegetables, stir_fry_vegetables_s6).
action(stir_fry_vegetables_s6, add).
object_of(stir_fry_vegetables_s6, soy_sauce).
step(stir_fry_vegetables, stir_fry_vegetables_s7).
action(stir_fry_vegetables_s7, toss).
object_of(stir_fry_vegetables_s7, vegetable).
purpose(stir_fry_vegetables_s7, vegetable_coated).
takes_time(stir_fry_vegetables_s7, 1, 1, minute).
before(stir_fry_vegetables_s1, stir_fry_vegetables_s2).
before(stir_fry_vegetables_s2, stir_fry_vegetables_s3).
before(stir_fry_vegetables_s3, stir_fry_vegetables_s4).
before(stir_fry_vegetables_s4, stir_fry_vegetables_s5).
before(stir_fry_vegetables_s5, stir_fry_vegetables_s6).
before(stir_fry_vegetables_s6, stir_fry_vegetables_s7).
```

## aliases
```datalog
task_alias(stir_fry_vegetables, "stir-frying vegetables").
task_alias(stir_fry_vegetables, "stir-fry vegetables").
task_alias(stir_fry_vegetables, "vegetable stir fry").
task_alias(stir_fry_vegetables, "stir fry").
alias(vegetable, "vegetables").
alias(vegetable, "mixed vegetables").
alias(vegetable, "veg").
alias(vegetable, "peppers, carrots and broccoli").
alias(hot, "very hot").
alias(tender, "just tender").
alias(high, "high heat").
alias(frying_pan, "large frying pan").
alias(frying_pan, "frying pans").
alias(vegetable_coated, "toss to coat").
alias(batch, "two batches").
```

## teach
- To stir-fry vegetables you need vegetables, garlic, oil, soy sauce, a knife and a wok.
- Stir-frying vegetables takes 14 to 19 minutes.
- Step 1 of stir-frying vegetables: chop the vegetables into thin strips with the knife.
- Step 2 of stir-frying vegetables: slice the garlic with the knife for 8 to 10 minutes.
- Step 3 of stir-frying vegetables: heat the oil in the wok until very hot, for 1 to 2 minutes.
- Step 4 of stir-frying vegetables: fry the garlic in the wok until fragrant.
- Step 5 of stir-frying vegetables: fry the vegetables in the wok over high heat, tossing them all the time, until just tender, for 4 to 6 minutes.
- Step 6 of stir-frying vegetables: add the soy sauce.
- Step 7 of stir-frying vegetables: toss to coat for 1 minute.

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

```datalog situation
has(user, vegetable).
has(user, garlic).
has(user, oil).
has(user, soy_sauce).
has(user, knife).
has(user, frying_pan).
lacks(user, wok).
```

```datalog difference
no_need(wok).
extra_need(frying_pan).
swap(stir_fry_vegetables_s3, wok, frying_pan).
swap(stir_fry_vegetables_s4, wok, frying_pan).
swap(stir_fry_vegetables_s5, wok, frying_pan).
change(stir_fry_vegetables_s5, repeat, 1, 2).
```
