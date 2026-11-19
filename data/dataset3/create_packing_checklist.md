---
id: create_packing_checklist
stage: procedures
task: create_packing_checklist
schema: v3
---
# Make a packing checklist

## requests
- I'm going away next week and keep forgetting things. How should I put together a packing list?
- Can you show me how to make a checklist for a trip so I don't leave anything behind?
- What's the best way to draw up a list of things to pack before a holiday?

## situation
- I have some paper and a suitcase.
- I'm packing for a week-long trip.

## ground truth
```datalog
task(create_packing_checklist).
domain(create_packing_checklist, planning).
goal(create_packing_checklist, packing_ready).
needs(create_packing_checklist, paper).
needs(create_packing_checklist, suitcase).
takes_time(create_packing_checklist, 15, 25, minute).
step(create_packing_checklist, create_packing_checklist_s1).
action(create_packing_checklist_s1, list).
object_of(create_packing_checklist_s1, trip_items).
uses(create_packing_checklist_s1, paper).
role(create_packing_checklist_s1, paper, hold).
takes_time(create_packing_checklist_s1, 5, 8, minute).
step(create_packing_checklist, create_packing_checklist_s2).
action(create_packing_checklist_s2, sort).
object_of(create_packing_checklist_s2, trip_items).
takes_time(create_packing_checklist_s2, 2, 4, minute).
step(create_packing_checklist, create_packing_checklist_s3).
action(create_packing_checklist_s3, pack).
object_of(create_packing_checklist_s3, trip_items).
uses(create_packing_checklist_s3, suitcase).
role(create_packing_checklist_s3, suitcase, contain).
takes_time(create_packing_checklist_s3, 5, 8, minute).
step(create_packing_checklist, create_packing_checklist_s4).
action(create_packing_checklist_s4, check).
object_of(create_packing_checklist_s4, trip_items).
takes_time(create_packing_checklist_s4, 2, 3, minute).
step(create_packing_checklist, create_packing_checklist_s5).
action(create_packing_checklist_s5, tick).
object_of(create_packing_checklist_s5, checklist).
uses(create_packing_checklist_s5, paper).
role(create_packing_checklist_s5, paper, hold).
before(create_packing_checklist_s1, create_packing_checklist_s2).
before(create_packing_checklist_s2, create_packing_checklist_s3).
before(create_packing_checklist_s3, create_packing_checklist_s4).
before(create_packing_checklist_s4, create_packing_checklist_s5).
```

## aliases
```datalog
task_alias(create_packing_checklist, "packing checklist").
task_alias(create_packing_checklist, "packing list").
task_alias(create_packing_checklist, "list for packing").
alias(paper, "piece of paper").
alias(suitcase, "luggage").
alias(suitcase, "suitcase").
alias(trip_items, "things to pack").
alias(checklist, "list").
alias(tick, "tick off").
alias(packing_ready, "ready to pack").
alias(backpack, "rucksack").
```

## teach
- To make a packing checklist you need paper and a suitcase.
- Making a packing checklist takes 15 to 25 minutes.
- Step 1 of making a packing checklist: list the things to pack on the paper.
- Step 2 of making a packing checklist: sort the things to pack into groups.
- Step 3 of making a packing checklist: pack the things to pack into the suitcase.
- Step 4 of making a packing checklist: check the things to pack against the list.
- Step 5 of making a packing checklist: tick off each item on the paper.

## checklist
- names what is needed, including paper and a suitcase
- gives the steps in order: list the items, sort them, pack them, check them, tick off the list
- gives a total time of about 15 to 25 minutes
- says the list is written on paper before packing starts

## variant
- change: no suitcase, so use a backpack instead for packing
- request: I'm going away for a week but I don't own a suitcase. Can I still make a packing list with a backpack?
- situation: I have some paper, a backpack and no suitcase.
- expect: The reply keeps the list on paper and tells the user to pack the items into the backpack in place of the suitcase.

```datalog situation
has(user, paper).
has(user, backpack).
lacks(user, suitcase).
```

```datalog difference
swap(create_packing_checklist_s3, suitcase, backpack).
extra_need(backpack).
no_need(suitcase).
```

- change: only 5 minutes, so skip the sorting step and pack straight from the list
- request: I'm leaving in five minutes. Can you give me a quick packing list and skip the sorting?
- situation: I have paper and a suitcase, and only 5 minutes.
- expect: The reply writes a short list, packs straight from it without sorting, and says this takes about 5 to 8 minutes.

```datalog situation
has(user, paper).
has(user, suitcase).
time_limit(user, 5, minute).
```

```datalog difference
drop(create_packing_checklist_s2).
change(create_packing_checklist, takes_time, 15_25, 5_8).
```
