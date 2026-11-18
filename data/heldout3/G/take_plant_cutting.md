---
id: take_plant_cutting
stage: household
task: take_plant_cutting
schema: v3
tier: G
---
# Take a plant cutting

## requests
- My neighbour said I could have a bit of her lavender to grow my own. How do I take a cutting?
- How do I propagate a geranium from a cutting?
- What's the method for growing a new plant from a snipped-off shoot?

## situation
- I have a sharp knife, a small pot, cutting compost, rooting powder and a clear plastic bag.

## ground truth
```datalog
task(take_plant_cutting).
domain(take_plant_cutting, gardening).
goal(take_plant_cutting, new_plant).
needs(take_plant_cutting, small_pot).
needs(take_plant_cutting, cutting_compost).
needs(take_plant_cutting, knife).
optional_need(take_plant_cutting, rooting_powder).
needs(take_plant_cutting, clear_bag).
takes_time(take_plant_cutting, 10, 15, minute).
step(take_plant_cutting, take_plant_cutting_s1).
action(take_plant_cutting_s1, fill).
object_of(take_plant_cutting_s1, small_pot).
uses(take_plant_cutting_s1, cutting_compost).
role(take_plant_cutting_s1, cutting_compost, fill).
step(take_plant_cutting, take_plant_cutting_s2).
action(take_plant_cutting_s2, cut).
object_of(take_plant_cutting_s2, shoot).
with_part(take_plant_cutting_s2, leaf_node).
uses(take_plant_cutting_s2, knife).
role(take_plant_cutting_s2, knife, cut).
setting(take_plant_cutting_s2, length, 10, centimetre).
step(take_plant_cutting, take_plant_cutting_s3).
action(take_plant_cutting_s3, strip).
object_of(take_plant_cutting_s3, lower_leaf).
purpose(take_plant_cutting_s3, prevent_rot).
step(take_plant_cutting, take_plant_cutting_s4).
action(take_plant_cutting_s4, dip).
object_of(take_plant_cutting_s4, shoot).
into(take_plant_cutting_s4, rooting_powder).
uses(take_plant_cutting_s4, rooting_powder).
role(take_plant_cutting_s4, rooting_powder, coat).
optional(take_plant_cutting_s4).
step(take_plant_cutting, take_plant_cutting_s5).
action(take_plant_cutting_s5, push).
object_of(take_plant_cutting_s5, shoot).
into(take_plant_cutting_s5, small_pot).
setting(take_plant_cutting_s5, depth, 3, centimetre).
step(take_plant_cutting, take_plant_cutting_s6).
action(take_plant_cutting_s6, cover).
object_of(take_plant_cutting_s6, small_pot).
uses(take_plant_cutting_s6, clear_bag).
role(take_plant_cutting_s6, clear_bag, cover).
purpose(take_plant_cutting_s6, keeps_moist).
before(take_plant_cutting_s1, take_plant_cutting_s2).
before(take_plant_cutting_s2, take_plant_cutting_s3).
before(take_plant_cutting_s3, take_plant_cutting_s4).
before(take_plant_cutting_s4, take_plant_cutting_s5).
before(take_plant_cutting_s5, take_plant_cutting_s6).
```

## aliases
```datalog
task_alias(take_plant_cutting, "taking a plant cutting").
task_alias(take_plant_cutting, "propagate from a cutting").
task_alias(take_plant_cutting, "grow a new plant from a shoot").
alias(knife, "sharp knife").
alias(shoot, "cutting").
alias(shoot, "stem").
alias(leaf_node, "just below a leaf joint").
alias(lower_leaf, "lower leaves").
alias(clear_bag, "clear plastic bag").
alias(plastic_bottle, "cut-off plastic bottle").
alias(plastic_bottle, "drinks bottle").
alias(keeps_moist, "keeps it humid").
alias(prevent_rot, "so they don't rot").
```

## teach
- To take a plant cutting you need a small pot, cutting compost, a sharp knife and a clear plastic bag; rooting powder helps but is not required.
- Taking a plant cutting takes 10 to 15 minutes.
- Step 1 of taking a plant cutting: fill the small pot with cutting compost.
- Step 2 of taking a plant cutting: cut a 10 centimetre shoot with the knife, just below a leaf joint.
- Step 3 of taking a plant cutting: strip off the lower leaves so they don't rot.
- Step 4 of taking a plant cutting: if you have it, dip the cut end into rooting powder.
- Step 5 of taking a plant cutting: push the shoot about 3 centimetres into the compost in the pot.
- Step 6 of taking a plant cutting: cover the pot with the clear bag to keep it humid.

## checklist
- names what is needed: a small pot, cutting compost, a knife and a clear bag, with rooting powder optional
- gives the steps in order: fill pot, cut shoot, strip leaves, dip, push in, cover
- says to cut about 10 centimetres below a leaf joint
- gives a total time of about 10 to 15 minutes

## variant
- change: no rooting powder: skip the dipping step
- request: Do I need rooting powder for a lavender cutting? I haven't got any.
- situation: I have a knife, a small pot, cutting compost and a clear bag, but no rooting powder.
- expect: The reply leaves out the dipping step and keeps the rest of the steps.

```datalog situation
has(user, knife).
has(user, small_pot).
has(user, cutting_compost).
has(user, clear_bag).
lacks(user, rooting_powder).
```

```datalog difference
no_need(rooting_powder).
drop(take_plant_cutting_s4).
```

## variant
- change: no plastic bag: cover the pot with a cut-off plastic bottle
- request: I want to root a geranium cutting but there are no clear bags in the house. Could the top half of a drinks bottle work as a cover?
- situation: I have a knife, a small pot, cutting compost, rooting powder and an empty plastic bottle, but no clear bag.
- expect: The reply covers the pot with the cut-off plastic bottle instead of a bag and keeps the other steps.

```datalog situation
lacks(user, clear_bag).
has(user, plastic_bottle).
has(user, knife).
has(user, small_pot).
has(user, cutting_compost).
has(user, rooting_powder).
```

```datalog difference
no_need(clear_bag).
extra_need(plastic_bottle).
swap(take_plant_cutting_s6, clear_bag, plastic_bottle).
```
