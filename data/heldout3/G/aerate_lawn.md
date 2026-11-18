---
id: aerate_lawn
stage: household
task: aerate_lawn
schema: v3
tier: G
---
# Aerate a lawn

## requests
- My lawn is hard as concrete and puddles every time it rains. What can I do about it?
- How do I spike the grass with a garden fork to let air in?
- Can you explain how to aerate a lawn in autumn?

## situation
- I have a garden fork, a rake, a bag of sharp sand and a stiff broom.
- The lawn is small, about the size of a room.

## ground truth
```datalog
task(aerate_lawn).
domain(aerate_lawn, gardening).
goal(aerate_lawn, healthy_lawn).
needs(aerate_lawn, rake).
needs(aerate_lawn, garden_fork).
optional_need(aerate_lawn, sharp_sand).
optional_need(aerate_lawn, broom).
takes_time(aerate_lawn, 40, 60, minute).
step(aerate_lawn, aerate_lawn_s1).
action(aerate_lawn_s1, rake).
object_of(aerate_lawn_s1, lawn).
uses(aerate_lawn_s1, rake).
role(aerate_lawn_s1, rake, gather).
purpose(aerate_lawn_s1, remove_thatch).
takes_time(aerate_lawn_s1, 10, 15, minute).
step(aerate_lawn, aerate_lawn_s2).
action(aerate_lawn_s2, spike).
object_of(aerate_lawn_s2, lawn).
uses(aerate_lawn_s2, garden_fork).
role(aerate_lawn_s2, garden_fork, pierce).
setting(aerate_lawn_s2, depth, 10, centimetre).
purpose(aerate_lawn_s2, air_reaches_roots).
takes_time(aerate_lawn_s2, 20, 30, minute).
step(aerate_lawn, aerate_lawn_s3).
action(aerate_lawn_s3, spread).
object_of(aerate_lawn_s3, sharp_sand).
onto(aerate_lawn_s3, lawn).
optional(aerate_lawn_s3).
step(aerate_lawn, aerate_lawn_s4).
action(aerate_lawn_s4, brush).
object_of(aerate_lawn_s4, sharp_sand).
into(aerate_lawn_s4, hole).
uses(aerate_lawn_s4, broom).
role(aerate_lawn_s4, broom, spread).
purpose(aerate_lawn_s4, better_drainage).
optional(aerate_lawn_s4).
before(aerate_lawn_s1, aerate_lawn_s2).
before(aerate_lawn_s2, aerate_lawn_s3).
before(aerate_lawn_s3, aerate_lawn_s4).
```

## aliases
```datalog
task_alias(aerate_lawn, "aerating a lawn").
task_alias(aerate_lawn, "spike the lawn").
task_alias(aerate_lawn, "let air into the grass").
alias(lawn, "grass").
alias(garden_fork, "fork").
alias(garden_fork, "digging fork").
alias(sharp_sand, "sand").
alias(broom, "stiff broom").
alias(hole, "holes").
alias(remove_thatch, "pulls out the dead thatch").
alias(no_spiking_tool, "nothing to make the holes with").
```

## teach
- To aerate a lawn you need a rake and a garden fork; sharp sand and a broom help but are not required.
- Aerating a lawn takes 40 to 60 minutes.
- Step 1 of aerating a lawn: rake the lawn hard to pull out the dead thatch, for 10 to 15 minutes.
- Step 2 of aerating a lawn: spike the lawn all over with the garden fork about 10 centimetres deep so air reaches the roots, for 20 to 30 minutes.
- Step 3 of aerating a lawn: if you have it, spread sharp sand over the lawn.
- Step 4 of aerating a lawn: if you spread sand, brush it into the holes with the broom for better drainage.

## checklist
- names what is needed: a rake and a garden fork, with sand and a broom as optional
- gives the steps in order: rake, spike with fork, spread sand, brush into holes
- says to spike about 10 centimetres deep
- gives a total time of about 40 to 60 minutes

## variant
- change: garden fork broken and nothing else to make holes: cannot aerate, only rake
- request: Both tines snapped off my garden fork last week and I've got nothing else with spikes. Can I still aerate the lawn today?
- situation: My garden fork is broken. I have a rake, sharp sand and a broom, and nothing else that can spike the lawn.
- expect: The reply says the lawn cannot be aerated without something to make the holes, and suggests raking for now and borrowing or buying a fork or aerator.

```datalog situation
lacks(user, garden_fork).
has(user, rake).
has(user, sharp_sand).
has(user, broom).
```

```datalog difference
cannot(no_spiking_tool).
```

## variant
- change: no sand: skip spreading and brushing in sand
- request: Do I really need sand to aerate the lawn? I haven't got any.
- situation: I have a garden fork and a rake, but no sharp sand.
- expect: The reply rakes and spikes the lawn and leaves out the sand and brushing steps.

```datalog situation
has(user, garden_fork).
has(user, rake).
lacks(user, sharp_sand).
```

```datalog difference
no_need(sharp_sand).
no_need(broom).
drop(aerate_lawn_s3).
drop(aerate_lawn_s4).
```
