---
id: plant_tree_sapling
stage: household
task: plant_tree_sapling
schema: v3
tier: G
---
# Plant a tree sapling

## requests
- I've been given a little apple tree for my birthday. How do I get it in the ground so it survives?
- What's the right way to plant a young tree with a stake?
- How do I plant a bare-root sapling in the back garden?

## situation
- I have the sapling, a spade, a garden fork, a wooden stake, a mallet, a tree tie, a bag of compost and a garden hose.

## ground truth
```datalog
task(plant_tree_sapling).
domain(plant_tree_sapling, gardening).
goal(plant_tree_sapling, planted_tree).
needs(plant_tree_sapling, sapling).
needs(plant_tree_sapling, spade).
needs(plant_tree_sapling, garden_fork).
needs(plant_tree_sapling, stake).
needs(plant_tree_sapling, mallet).
needs(plant_tree_sapling, compost).
needs(plant_tree_sapling, tree_tie).
needs(plant_tree_sapling, hose).
takes_time(plant_tree_sapling, 45, 60, minute).
step(plant_tree_sapling, plant_tree_sapling_s1).
action(plant_tree_sapling_s1, dig).
object_of(plant_tree_sapling_s1, hole).
uses(plant_tree_sapling_s1, spade).
role(plant_tree_sapling_s1, spade, lift).
setting(plant_tree_sapling_s1, width, 60, centimetre).
takes_time(plant_tree_sapling_s1, 15, 20, minute).
step(plant_tree_sapling, plant_tree_sapling_s2).
action(plant_tree_sapling_s2, loosen).
object_of(plant_tree_sapling_s2, hole).
with_part(plant_tree_sapling_s2, bottom).
uses(plant_tree_sapling_s2, garden_fork).
role(plant_tree_sapling_s2, garden_fork, loosen).
step(plant_tree_sapling, plant_tree_sapling_s3).
action(plant_tree_sapling_s3, drive).
object_of(plant_tree_sapling_s3, stake).
into(plant_tree_sapling_s3, hole).
uses(plant_tree_sapling_s3, mallet).
role(plant_tree_sapling_s3, mallet, strike).
step(plant_tree_sapling, plant_tree_sapling_s4).
action(plant_tree_sapling_s4, set).
object_of(plant_tree_sapling_s4, sapling).
into(plant_tree_sapling_s4, hole).
check(plant_tree_sapling_s4, soil_mark_level).
step(plant_tree_sapling, plant_tree_sapling_s5).
action(plant_tree_sapling_s5, fill).
object_of(plant_tree_sapling_s5, hole).
uses(plant_tree_sapling_s5, spade).
role(plant_tree_sapling_s5, spade, lift).
uses(plant_tree_sapling_s5, compost).
role(plant_tree_sapling_s5, compost, fill).
step(plant_tree_sapling, plant_tree_sapling_s6).
action(plant_tree_sapling_s6, fasten).
object_of(plant_tree_sapling_s6, sapling).
onto(plant_tree_sapling_s6, stake).
uses(plant_tree_sapling_s6, tree_tie).
role(plant_tree_sapling_s6, tree_tie, fasten).
step(plant_tree_sapling, plant_tree_sapling_s7).
action(plant_tree_sapling_s7, water).
object_of(plant_tree_sapling_s7, sapling).
uses(plant_tree_sapling_s7, hose).
role(plant_tree_sapling_s7, hose, pour).
until(plant_tree_sapling_s7, soaked).
before(plant_tree_sapling_s1, plant_tree_sapling_s2).
before(plant_tree_sapling_s2, plant_tree_sapling_s3).
before(plant_tree_sapling_s3, plant_tree_sapling_s4).
before(plant_tree_sapling_s4, plant_tree_sapling_s5).
before(plant_tree_sapling_s5, plant_tree_sapling_s6).
before(plant_tree_sapling_s6, plant_tree_sapling_s7).
```

## aliases
```datalog
task_alias(plant_tree_sapling, "planting a tree sapling").
task_alias(plant_tree_sapling, "plant a young tree").
task_alias(plant_tree_sapling, "get a tree in the ground").
alias(sapling, "apple tree").
alias(sapling, "young tree").
alias(stake, "wooden stake").
alias(tree_tie, "tree tie").
alias(hose, "garden hose").
alias(garden_fork, "fork").
alias(soil_mark_level, "the old soil mark on the stem is level with the ground").
alias(hammer, "claw hammer").
alias(old_tights, "old pair of tights").
alias(old_tights, "tights").
alias(soaked, "the ground is soaked").
```

## teach
- To plant a tree sapling you need the sapling, a spade, a garden fork, a stake, a mallet, compost, a tree tie and a hose.
- Planting a tree sapling takes 45 to 60 minutes.
- Step 1 of planting a tree sapling: dig a hole about 60 centimetres wide with the spade, for 15 to 20 minutes.
- Step 2 of planting a tree sapling: loosen the bottom of the hole with the garden fork.
- Step 3 of planting a tree sapling: drive the stake into the hole with the mallet.
- Step 4 of planting a tree sapling: set the sapling in the hole so the old soil mark on the stem is level with the ground.
- Step 5 of planting a tree sapling: fill the hole with soil and compost using the spade.
- Step 6 of planting a tree sapling: fasten the sapling to the stake with the tree tie.
- Step 7 of planting a tree sapling: water the sapling with the hose until the ground is soaked.

## checklist
- names what is needed: the sapling, a spade, a fork, a stake, a mallet, compost, a tree tie and a hose
- gives the steps in order: dig, loosen, stake, set tree, fill, tie, water
- says to plant at the old soil mark
- gives a total time of about 45 to 60 minutes

## variant
- change: no mallet: drive the stake in with a hammer
- request: I'm putting in a young tree and need to bang the stake in, but there's no mallet. I've got an ordinary hammer.
- situation: I have the sapling, a spade, a garden fork, a stake, a tree tie, compost, a hose and a hammer, but no mallet.
- expect: The reply drives the stake in with the hammer instead of a mallet and keeps the other steps.

```datalog situation
lacks(user, mallet).
has(user, hammer).
has(user, sapling).
has(user, spade).
has(user, garden_fork).
has(user, stake).
has(user, tree_tie).
has(user, compost).
has(user, hose).
```

```datalog difference
no_need(mallet).
extra_need(hammer).
swap(plant_tree_sapling_s3, mallet, hammer).
```

## variant
- change: tree tie broken: tie the tree to the stake with an old pair of tights
- request: The plastic tree tie that came with my sapling cracked as I did it up. I have some old tights - can I use those to hold the tree to its stake?
- situation: My tree tie is broken. I have the sapling, a spade, a garden fork, a stake, a mallet, compost, a hose and an old pair of tights.
- expect: The reply ties the sapling to the stake with the tights in a figure of eight instead of the tree tie and keeps the other steps.

```datalog situation
lacks(user, tree_tie).
has(user, old_tights).
has(user, sapling).
has(user, spade).
has(user, garden_fork).
has(user, stake).
has(user, mallet).
has(user, compost).
has(user, hose).
```

```datalog difference
no_need(tree_tie).
extra_need(old_tights).
swap(plant_tree_sapling_s6, tree_tie, old_tights).
```
