---
id: divide_perennial
stage: household
task: divide_perennial
schema: v3
tier: G
---
# Divide a perennial

## requests
- My hosta clump has got huge and the middle is dying off. Can I split it into several plants?
- How do you divide an overgrown perennial in the border?
- What's the way to split a big clump of daylilies so I can replant them?

## situation
- I have a spade, a garden fork, a watering can and water.

## ground truth
```datalog
task(divide_perennial).
domain(divide_perennial, gardening).
goal(divide_perennial, more_plants).
needs(divide_perennial, spade).
needs(divide_perennial, garden_fork).
needs(divide_perennial, watering_can).
needs(divide_perennial, water).
takes_time(divide_perennial, 25, 40, minute).
step(divide_perennial, divide_perennial_s1).
action(divide_perennial_s1, dig).
object_of(divide_perennial_s1, clump).
uses(divide_perennial_s1, spade).
role(divide_perennial_s1, spade, lift).
takes_time(divide_perennial_s1, 5, 10, minute).
step(divide_perennial, divide_perennial_s2).
action(divide_perennial_s2, lift).
object_of(divide_perennial_s2, clump).
uses(divide_perennial_s2, garden_fork).
role(divide_perennial_s2, garden_fork, lift).
step(divide_perennial, divide_perennial_s3).
action(divide_perennial_s3, split).
object_of(divide_perennial_s3, clump).
uses(divide_perennial_s3, spade).
role(divide_perennial_s3, spade, cut).
check(divide_perennial_s3, each_piece_has_roots).
step(divide_perennial, divide_perennial_s4).
action(divide_perennial_s4, plant).
object_of(divide_perennial_s4, piece).
into(divide_perennial_s4, hole).
takes_time(divide_perennial_s4, 10, 15, minute).
step(divide_perennial, divide_perennial_s5).
action(divide_perennial_s5, water).
object_of(divide_perennial_s5, piece).
uses(divide_perennial_s5, watering_can).
role(divide_perennial_s5, watering_can, pour).
uses(divide_perennial_s5, water).
role(divide_perennial_s5, water, wet).
before(divide_perennial_s1, divide_perennial_s2).
before(divide_perennial_s2, divide_perennial_s3).
before(divide_perennial_s3, divide_perennial_s4).
before(divide_perennial_s4, divide_perennial_s5).
```

## aliases
```datalog
task_alias(divide_perennial, "dividing a perennial").
task_alias(divide_perennial, "split a clump").
task_alias(divide_perennial, "split it into several plants").
alias(clump, "hosta clump").
alias(clump, "daylilies").
alias(piece, "pieces").
alias(piece, "divisions").
alias(garden_fork, "fork").
alias(pruning_saw, "pruning saw").
alias(pruning_saw, "old saw").
alias(bone_meal, "bonemeal").
alias(each_piece_has_roots, "every piece has roots and shoots").
```

## teach
- To divide a perennial you need a spade, a garden fork, a watering can and water.
- Dividing a perennial takes 25 to 40 minutes.
- Step 1 of dividing a perennial: dig around the clump with the spade, for 5 to 10 minutes.
- Step 2 of dividing a perennial: lift the clump out with the garden fork.
- Step 3 of dividing a perennial: split the clump by chopping down through it with the spade, so every piece has roots and shoots.
- Step 4 of dividing a perennial: plant the pieces in their own holes, for 10 to 15 minutes.
- Step 5 of dividing a perennial: water the pieces well with the watering can.

## checklist
- names what is needed: a spade, a garden fork and water
- gives the steps in order: dig round, lift, split, replant, water
- says each piece needs roots and shoots
- gives a total time of about 25 to 40 minutes

## variant
- change: spade broken: dig round with the fork and cut the clump with a pruning saw
- request: The handle on my spade snapped this morning. I need to split my hosta - I've got a fork and an old pruning saw. Will that work?
- situation: My spade is broken. I have a garden fork, a pruning saw, a watering can and water.
- expect: The reply loosens round the clump with the fork, saws the clump into pieces with the pruning saw, and keeps the other steps.

```datalog situation
lacks(user, spade).
has(user, garden_fork).
has(user, pruning_saw).
has(user, watering_can).
has(user, water).
```

```datalog difference
no_need(spade).
extra_need(pruning_saw).
swap(divide_perennial_s1, spade, garden_fork).
swap(divide_perennial_s3, spade, pruning_saw).
```

## variant
- change: wants to feed the new pieces: sprinkle bone meal in each hole before planting
- request: When I split up my daylilies I'd like to give them a feed so they settle in. How do I fit that in?
- situation: I have a spade, a garden fork, a watering can, water and a box of bone meal.
- expect: The reply adds a step to sprinkle a handful of bone meal into each hole before planting the pieces.

```datalog situation
has(user, spade).
has(user, garden_fork).
has(user, watering_can).
has(user, water).
has(user, bone_meal).
wants(user, bone_meal).
```

```datalog difference
extra_need(bone_meal).
add_step(divide_perennial_s9, divide_perennial_s3).
step(divide_perennial, divide_perennial_s9).
action(divide_perennial_s9, sprinkle).
object_of(divide_perennial_s9, bone_meal).
into(divide_perennial_s9, hole).
before(divide_perennial_s3, divide_perennial_s9).
before(divide_perennial_s9, divide_perennial_s4).
```
