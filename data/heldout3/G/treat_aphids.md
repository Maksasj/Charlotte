---
id: treat_aphids
stage: household
task: treat_aphids
schema: v3
tier: G
---
# Treat aphids

## requests
- The new shoots on my roses are crawling with little green bugs. How do I get rid of them?
- What's a gentle way to deal with greenfly on my plants?
- How do I treat aphids with a soap spray?

## situation
- I have a spray bottle, washing-up liquid, water and a garden hose.

## ground truth
```datalog
task(treat_aphids).
domain(treat_aphids, gardening).
goal(treat_aphids, pest_free_plant).
needs(treat_aphids, spray_bottle).
needs(treat_aphids, dish_soap).
needs(treat_aphids, water).
needs(treat_aphids, hose).
takes_time(treat_aphids, 10, 15, minute).
step(treat_aphids, treat_aphids_s1).
action(treat_aphids_s1, remove).
object_of(treat_aphids_s1, infested_shoot).
purpose(treat_aphids_s1, fewer_aphids).
step(treat_aphids, treat_aphids_s2).
action(treat_aphids_s2, mix).
object_of(treat_aphids_s2, dish_soap).
into(treat_aphids_s2, spray_bottle).
uses(treat_aphids_s2, water).
role(treat_aphids_s2, water, mix).
amount(treat_aphids_s2, dish_soap, 1, teaspoon).
amount(treat_aphids_s2, water, 500, millilitre).
step(treat_aphids, treat_aphids_s3).
action(treat_aphids_s3, spray).
object_of(treat_aphids_s3, aphid).
with_part(treat_aphids_s3, leaf_underside).
uses(treat_aphids_s3, spray_bottle).
role(treat_aphids_s3, spray_bottle, spray).
repeat(treat_aphids_s3, 3).
step(treat_aphids, treat_aphids_s4).
action(treat_aphids_s4, rinse).
object_of(treat_aphids_s4, plant).
uses(treat_aphids_s4, hose).
role(treat_aphids_s4, hose, rinse).
condition(treat_aphids_s4, next_day).
purpose(treat_aphids_s4, leaves_not_scorched).
before(treat_aphids_s1, treat_aphids_s2).
before(treat_aphids_s2, treat_aphids_s3).
before(treat_aphids_s3, treat_aphids_s4).
```

## aliases
```datalog
task_alias(treat_aphids, "treating aphids").
task_alias(treat_aphids, "get rid of greenfly").
task_alias(treat_aphids, "deal with aphids").
alias(aphid, "aphids").
alias(aphid, "greenfly").
alias(aphid, "little green bugs").
alias(dish_soap, "washing-up liquid").
alias(dish_soap, "soap").
alias(infested_shoot, "worst shoots").
alias(leaf_underside, "undersides of the leaves").
alias(hose, "garden hose").
alias(ladybird, "ladybirds").
alias(ladybird, "ladybird larvae").
alias(natural_predator, "bugs that eat them").
```

## teach
- To treat aphids you need a spray bottle, washing-up liquid, water and a hose.
- Treating aphids takes 10 to 15 minutes each time.
- Step 1 of treating aphids: remove the worst infested shoots by hand so there are fewer aphids.
- Step 2 of treating aphids: mix 1 teaspoon of washing-up liquid with 500 millilitres of water in the spray bottle.
- Step 3 of treating aphids: spray the aphids, including the undersides of the leaves, and repeat every few days, three times in all.
- Step 4 of treating aphids: the next day, rinse the plant with the hose so the leaves don't scorch.

## checklist
- names what is needed: a spray bottle, washing-up liquid, water and a hose
- gives the steps in order: remove worst shoots, mix spray, spray, rinse
- gives the mix of 1 teaspoon to 500 millilitres and says to repeat three times
- gives a time of about 10 to 15 minutes each time

## variant
- change: no soap wanted: knock the aphids off with a plain water spray instead
- request: I'd rather not put any soap on my plants because of the bees. How else can I deal with the greenfly?
- situation: I have a spray bottle, water and a hose, and I don't want to use soap.
- expect: The reply leaves out mixing the soap and fills the spray bottle with plain water to blast the aphids off, keeping the repeat sprays.

```datalog situation
excludes(user, dish_soap).
has(user, spray_bottle).
has(user, water).
has(user, hose).
```

```datalog difference
no_need(dish_soap).
drop(treat_aphids_s2).
add_step(treat_aphids_s9, treat_aphids_s1).
step(treat_aphids, treat_aphids_s9).
action(treat_aphids_s9, fill).
object_of(treat_aphids_s9, spray_bottle).
uses(treat_aphids_s9, water).
role(treat_aphids_s9, water, fill).
before(treat_aphids_s1, treat_aphids_s9).
before(treat_aphids_s9, treat_aphids_s3).
```

## variant
- change: wants natural predators: release ladybirds after spraying
- request: I bought a pot of ladybird larvae online. How do I clear the aphids on my roses and use the ladybirds too?
- situation: I have a spray bottle, washing-up liquid, water, a hose and a pot of ladybird larvae.
- expect: The reply adds a step after rinsing to release the ladybirds onto the plant in the evening.

```datalog situation
has(user, spray_bottle).
has(user, dish_soap).
has(user, water).
has(user, hose).
has(user, ladybird).
wants(user, natural_predator).
```

```datalog difference
extra_need(ladybird).
add_step(treat_aphids_s9, treat_aphids_s4).
step(treat_aphids, treat_aphids_s9).
action(treat_aphids_s9, release).
object_of(treat_aphids_s9, ladybird).
onto(treat_aphids_s9, plant).
condition(treat_aphids_s9, evening).
before(treat_aphids_s4, treat_aphids_s9).
```
