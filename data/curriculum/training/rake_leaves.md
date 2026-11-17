---
id: rake_leaves
stage: household
task: rake_leaves
---
# Rake leaves

## requests
- The garden is covered in autumn leaves. How do I clear them efficiently?
- What's the easiest way to rake up a lawn full of leaves?
- Can you tell me how to rake and bag the leaves in my yard?

## situation
- I have a rake, a tarp, garden waste bags and gloves.
- I have about an hour.

## ground truth
```datalog
task(rake_leaves).
needs(rake_leaves, rake).
needs(rake_leaves, tarp).
needs(rake_leaves, garden_bag).
needs(rake_leaves, glove).
takes_time(rake_leaves, 35, 50, minute).
step(rake_leaves, rake_leaves_s1).
action(rake_leaves_s1, lay).
object_of(rake_leaves_s1, tarp).
takes_time(rake_leaves_s1, 1, 2, minute).
step(rake_leaves, rake_leaves_s2).
action(rake_leaves_s2, rake).
object_of(rake_leaves_s2, leaf).
uses(rake_leaves_s2, rake).
takes_time(rake_leaves_s2, 20, 30, minute).
step(rake_leaves, rake_leaves_s3).
action(rake_leaves_s3, fill).
object_of(rake_leaves_s3, garden_bag).
uses(rake_leaves_s3, tarp).
uses(rake_leaves_s3, glove).
takes_time(rake_leaves_s3, 10, 15, minute).
step(rake_leaves, rake_leaves_s4).
action(rake_leaves_s4, tie).
object_of(rake_leaves_s4, garden_bag).
takes_time(rake_leaves_s4, 1, 2, minute).
before(rake_leaves_s1, rake_leaves_s2).
before(rake_leaves_s2, rake_leaves_s3).
before(rake_leaves_s3, rake_leaves_s4).
```

## teach
- To rake leaves you need a rake, a tarp, garden waste bags and gloves.
- Raking leaves takes 35 to 50 minutes.
- Step 1 of raking leaves: lay the tarp out at the edge of the lawn for 1 to 2 minutes.
- Step 2 of raking leaves: rake the leaves onto the tarp, working in rows, for 20 to 30 minutes.
- Step 3 of raking leaves: fill the garden bags from the tarp, wearing the gloves, for 10 to 15 minutes.
- Step 4 of raking leaves: tie the garden bags closed for 1 to 2 minutes.

## checklist
- names what is needed: a rake, a tarp, garden waste bags and gloves
- gives the steps in order: lay tarp, rake leaf, fill garden bag, tie garden bag
- gives a total time of about 35 to 50 minutes
- closes politely

## variant
- change: no tarp: rake the leaves into small piles and scoop them into the bags by hand
- request: I want to rake the leaves but I don't own a tarp. How should I do it?
- situation: I have a rake, garden waste bags and gloves, but no tarp.
- expect: The reply rakes the leaves into several small piles instead of onto a tarp and scoops each pile into the bags with gloved hands, keeping the tie step.
