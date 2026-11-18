---
id: sort_recycling
stage: household
task: sort_recycling
schema: v3
---
# Sort the recycling

## requests
- Our recycling is a mess. How should I sort it properly?
- Can you tell me how to get the recycling ready for collection?
- What's the right way to sort bottles, tins and cardboard for recycling?

## situation
- I have a pile of mixed recycling, water and separate recycling bins.
- Collection is tomorrow morning.

## ground truth
```datalog
task(sort_recycling).
domain(sort_recycling, cleaning).
goal(sort_recycling, sorted_recycling).
needs(sort_recycling, water).
needs(sort_recycling, recycling_bin).
takes_time(sort_recycling, 10, 15, minute).
step(sort_recycling, sort_recycling_s1).
action(sort_recycling_s1, rinse).
object_of(sort_recycling_s1, container).
uses(sort_recycling_s1, water).
role(sort_recycling_s1, water, clean).
takes_time(sort_recycling_s1, 3, 5, minute).
step(sort_recycling, sort_recycling_s2).
action(sort_recycling_s2, flatten).
object_of(sort_recycling_s2, box).
takes_time(sort_recycling_s2, 2, 3, minute).
step(sort_recycling, sort_recycling_s3).
action(sort_recycling_s3, separate).
object_of(sort_recycling_s3, glass).
takes_time(sort_recycling_s3, 2, 3, minute).
step(sort_recycling, sort_recycling_s4).
action(sort_recycling_s4, put).
object_of(sort_recycling_s4, waste).
uses(sort_recycling_s4, recycling_bin).
role(sort_recycling_s4, recycling_bin, contain).
into(sort_recycling_s4, recycling_bin).
takes_time(sort_recycling_s4, 2, 3, minute).
before(sort_recycling_s1, sort_recycling_s2).
before(sort_recycling_s2, sort_recycling_s3).
before(sort_recycling_s3, sort_recycling_s4).
```

## aliases
```datalog
task_alias(sort_recycling, "sorting the recycling").
task_alias(sort_recycling, "sort the recycling").
task_alias(sort_recycling, "get the recycling ready for collection").
alias(container, "food containers").
alias(container, "tins").
alias(box, "cardboard boxes").
alias(box, "cardboard").
alias(glass, "glass bottles").
alias(glass, "bottles").
alias(recycling_bin, "recycling bins").
alias(recycling_bin, "separate recycling bins").
alias(waste, "each kind of waste").
alias(waste, "mixed recycling").
```

## teach
- To sort the recycling you need water and recycling bins.
- Sorting the recycling takes 10 to 15 minutes.
- Step 1 of sorting the recycling: rinse the food containers and tins with water for 3 to 5 minutes.
- Step 2 of sorting the recycling: flatten the cardboard boxes for 2 to 3 minutes.
- Step 3 of sorting the recycling: separate the glass bottles from the plastic and the paper for 2 to 3 minutes.
- Step 4 of sorting the recycling: put each kind of waste in the right recycling bin for 2 to 3 minutes.

## checklist
- names what is needed: water and recycling bins
- gives the steps in order: rinse container, flatten box, separate glass, put waste
- gives a total time of about 10 to 15 minutes
- closes politely
