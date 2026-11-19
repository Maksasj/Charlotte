---
id: caramelise_onions
stage: procedures
task: caramelise_onions
schema: v3
---
# Caramelise onions

## requests
- How do I caramelise onions properly? I want them soft and golden for a sauce.
- Could you walk me through caramelising a couple of onions in a pan?
- What's the method for slow-cooked brown onions? I'd like them sweet.

## situation
- I have an onion, butter, salt, a knife, a chopping board, a frying pan, a wooden spoon and a hob.
- I've got about 50 minutes before lunch.

## ground truth
```datalog
task(caramelise_onions).
domain(caramelise_onions, cooking).
goal(caramelise_onions, caramelised_onions).
needs(caramelise_onions, onion).
needs(caramelise_onions, butter).
needs(caramelise_onions, salt).
needs(caramelise_onions, knife).
needs(caramelise_onions, chopping_board).
needs(caramelise_onions, frying_pan).
needs(caramelise_onions, wooden_spoon).
needs(caramelise_onions, hob).
takes_time(caramelise_onions, 40, 50, minute).
step(caramelise_onions, caramelise_onions_s1).
action(caramelise_onions_s1, cut).
object_of(caramelise_onions_s1, onion).
uses(caramelise_onions_s1, knife).
role(caramelise_onions_s1, knife, cut).
uses(caramelise_onions_s1, chopping_board).
role(caramelise_onions_s1, chopping_board, hold).
takes_time(caramelise_onions_s1, 5, 8, minute).
step(caramelise_onions, caramelise_onions_s2).
action(caramelise_onions_s2, heat).
object_of(caramelise_onions_s2, butter).
uses(caramelise_onions_s2, frying_pan).
role(caramelise_onions_s2, frying_pan, contain).
uses(caramelise_onions_s2, hob).
role(caramelise_onions_s2, hob, heat).
setting(caramelise_onions_s2, heat, low, level).
takes_time(caramelise_onions_s2, 3, 5, minute).
overlaps(caramelise_onions_s2, caramelise_onions_s1).
step(caramelise_onions, caramelise_onions_s3).
action(caramelise_onions_s3, add).
object_of(caramelise_onions_s3, onion).
into(caramelise_onions_s3, frying_pan).
step(caramelise_onions, caramelise_onions_s4).
action(caramelise_onions_s4, stir).
object_of(caramelise_onions_s4, onion).
uses(caramelise_onions_s4, wooden_spoon).
role(caramelise_onions_s4, wooden_spoon, stir).
until(caramelise_onions_s4, golden_brown).
takes_time(caramelise_onions_s4, 30, 40, minute).
warning(caramelise_onions_s4, onions_burn).
step(caramelise_onions, caramelise_onions_s5).
action(caramelise_onions_s5, season).
object_of(caramelise_onions_s5, onion).
uses(caramelise_onions_s5, salt).
role(caramelise_onions_s5, salt, season).
takes_time(caramelise_onions_s5, 1, 1, minute).
before(caramelise_onions_s1, caramelise_onions_s3).
before(caramelise_onions_s2, caramelise_onions_s3).
before(caramelise_onions_s3, caramelise_onions_s4).
before(caramelise_onions_s4, caramelise_onions_s5).
```

## aliases
```datalog
task_alias(caramelise_onions, "caramelised onions").
task_alias(caramelise_onions, "caramelizing onions").
alias(onion, "onions").
alias(frying_pan, "skillet").
alias(chopping_board, "cutting board").
alias(hob, "stove").
alias(golden_brown, "golden brown").
alias(onions_burn, "burn").
```

## teach
- To caramelise onions you need an onion, butter, salt, a knife, a chopping board, a frying pan, a wooden spoon and a hob.
- Caramelising onions takes 40 to 50 minutes.
- Step 1 of caramelise onions: cut the onion on the chopping board with the knife, for 5 to 8 minutes.
- While step 1 runs, step 2 of caramelise onions: heat the butter in the frying pan on the hob over low heat, for 3 to 5 minutes.
- Step 3 of caramelise onions: add the onion to the frying pan once the butter has melted.
- Step 4 of caramelise onions: stir the onion with the wooden spoon for 30 to 40 minutes, stirring often so it does not burn, until it is golden brown.
- Step 5 of caramelise onions: season the onion with salt for 1 minute.

## checklist
- names what is needed, including onions, butter, salt, a frying pan and a hob
- gives the steps in order: cut the onion, melt the butter, add the onion, stir until golden brown, season with salt
- says the cutting can be done while the butter heats
- gives a total time of about 40 to 50 minutes

## variant
- change: no wooden spoon, so stir with a metal spoon instead
- request: I want to caramelise onions but I only have a metal spoon, no wooden one. Will that still work?
- situation: I have an onion, butter, salt, a knife, a chopping board, a frying pan, a metal spoon and a hob, but no wooden spoon.
- expect: The reply gives the same steps and timings and says to stir with the metal spoon in place of the wooden spoon.

```datalog situation
has(user, onion).
has(user, butter).
has(user, salt).
has(user, knife).
has(user, chopping_board).
has(user, frying_pan).
has(user, metal_spoon).
has(user, hob).
lacks(user, wooden_spoon).
```

```datalog difference
swap(caramelise_onions_s4, wooden_spoon, metal_spoon).
no_need(wooden_spoon).
extra_need(metal_spoon).
```

- change: no hob and no other cooker, so the onions cannot be cooked
- request: My hob has broken and I have no other cooker. Can I still caramelise onions?
- situation: I have an onion, butter, salt, a knife, a chopping board, a frying pan and a wooden spoon, but no hob or other heat source.
- expect: The reply says onions cannot be caramelised without a heat source, so the request cannot be met, and gives no cooking steps.

```datalog situation
has(user, onion).
has(user, butter).
has(user, salt).
has(user, knife).
has(user, chopping_board).
has(user, frying_pan).
has(user, wooden_spoon).
lacks(user, hob).
```

```datalog difference
cannot(no_heat_source).
```
