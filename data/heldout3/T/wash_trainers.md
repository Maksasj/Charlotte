---
id: wash_trainers
stage: household
task: wash_trainers
schema: v3
tier: T
---
# Wash trainers

## requests
- My white sneakers are filthy. How do I clean them?
- How should I wash a pair of trainers by hand?
- I wore my running shoes through a muddy park. How do I get them looking decent again?

## situation
- I have a soft brush, a sponge, washing-up liquid, a bowl of warm water and some old newspaper.

## ground truth
```datalog
task(wash_trainers).
domain(wash_trainers, cleaning).
goal(wash_trainers, clean_trainer).
needs(wash_trainers, brush).
needs(wash_trainers, sponge).
needs(wash_trainers, dish_soap).
needs(wash_trainers, bowl).
needs(wash_trainers, warm_water).
needs(wash_trainers, newspaper).
takes_time(wash_trainers, 8, 24, hour).
step(wash_trainers, wash_trainers_s1).
action(wash_trainers_s1, remove).
object_of(wash_trainers_s1, lace).
step(wash_trainers, wash_trainers_s2).
action(wash_trainers_s2, brush).
object_of(wash_trainers_s2, sole).
uses(wash_trainers_s2, brush).
role(wash_trainers_s2, brush, clean).
purpose(wash_trainers_s2, dirt_off).
step(wash_trainers, wash_trainers_s3).
action(wash_trainers_s3, scrub).
object_of(wash_trainers_s3, trainer).
uses(wash_trainers_s3, sponge).
role(wash_trainers_s3, sponge, scrub).
uses(wash_trainers_s3, dish_soap).
role(wash_trainers_s3, dish_soap, clean).
uses(wash_trainers_s3, warm_water).
role(wash_trainers_s3, warm_water, wet).
step(wash_trainers, wash_trainers_s4).
action(wash_trainers_s4, wash).
object_of(wash_trainers_s4, lace).
into(wash_trainers_s4, bowl).
uses(wash_trainers_s4, bowl).
role(wash_trainers_s4, bowl, contain).
uses(wash_trainers_s4, dish_soap).
role(wash_trainers_s4, dish_soap, clean).
overlaps(wash_trainers_s4, wash_trainers_s3).
step(wash_trainers, wash_trainers_s5).
action(wash_trainers_s5, stuff).
object_of(wash_trainers_s5, trainer).
uses(wash_trainers_s5, newspaper).
role(wash_trainers_s5, newspaper, absorb).
purpose(wash_trainers_s5, keeps_shape).
step(wash_trainers, wash_trainers_s6).
action(wash_trainers_s6, dry).
object_of(wash_trainers_s6, trainer).
takes_time(wash_trainers_s6, 8, 24, hour).
avoid(wash_trainers_s6, put, radiator).
before(wash_trainers_s1, wash_trainers_s2).
before(wash_trainers_s2, wash_trainers_s3).
before(wash_trainers_s1, wash_trainers_s4).
before(wash_trainers_s3, wash_trainers_s5).
before(wash_trainers_s4, wash_trainers_s6).
before(wash_trainers_s5, wash_trainers_s6).
```

## aliases
```datalog
task_alias(wash_trainers, "washing trainers").
task_alias(wash_trainers, "cleaning sneakers").
task_alias(wash_trainers, "washing running shoes").
alias(trainer, "trainers").
alias(trainer, "sneakers").
alias(lace, "laces").
alias(dish_soap, "washing-up liquid").
alias(keeps_shape, "keeps their shape").
```

## teach
- To wash trainers you need a soft brush, a sponge, washing-up liquid, a bowl of warm water and newspaper.
- Washing trainers takes 8 to 24 hours, mostly drying.
- Step 1 of washing trainers: remove the laces.
- Step 2 of washing trainers: brush the soles with the brush to get the dirt off.
- Step 3 of washing trainers: scrub the trainers with the sponge and warm soapy water.
- While step 3 runs, step 4 of washing trainers: wash the laces in a bowl of soapy water.
- Step 5 of washing trainers: stuff the trainers with newspaper so they keep their shape and dry faster.
- Step 6 of washing trainers: let the trainers dry for 8 to 24 hours; do not put them on a radiator.

## checklist
- names what is needed, including a brush, a sponge, soapy water and newspaper
- gives the steps in order: laces out, brush soles, scrub, wash laces, stuff with paper, dry
- warns not to dry them on a radiator

## variant
- change: no newspaper: stuff them with paper towels
- request: We don't get a newspaper. What can I stuff my wet trainers with?
- situation: I have a brush, a sponge, washing-up liquid, warm water, a bowl and a roll of paper towels.
- expect: The reply stuffs the trainers with scrunched paper towels instead of newspaper.

```datalog situation
has(user, brush).
has(user, sponge).
has(user, dish_soap).
has(user, bowl).
has(user, warm_water).
has(user, paper_towel).
lacks(user, newspaper).
```

```datalog difference
swap(wash_trainers_s5, newspaper, paper_towel).
extra_need(paper_towel).
no_need(newspaper).
```

## variant
- change: needs them in 3 hours: dry in front of a fan and change the paper often
- request: I need these trainers clean for a run this evening, about three hours from now. Possible?
- situation: I have a brush, a sponge, washing-up liquid, a bowl, warm water, newspaper and an electric fan, and about 3 hours.
- expect: The reply dries the trainers in front of the fan, swapping the wet newspaper for dry every half hour, so they are wearable in about 3 hours.

```datalog situation
has(user, brush).
has(user, sponge).
has(user, dish_soap).
has(user, bowl).
has(user, warm_water).
has(user, newspaper).
has(user, fan).
time_limit(user, 3, hour).
```

```datalog difference
extra_need(fan).
change(wash_trainers_s6, takes_time, 8_24_hour, 2_3_hour).
add_step(wash_trainers_s9, wash_trainers_s5).
step(wash_trainers, wash_trainers_s9).
action(wash_trainers_s9, place).
object_of(wash_trainers_s9, trainer).
uses(wash_trainers_s9, fan).
role(wash_trainers_s9, fan, dry).
before(wash_trainers_s5, wash_trainers_s9).
overlaps(wash_trainers_s9, wash_trainers_s6).
change(wash_trainers, takes_time, 8_24_hour, 3_3_hour).
```
