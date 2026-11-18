---
id: dust_shelves
stage: household
task: dust_shelves
schema: v3
---
# Dust the shelves

## requests
- My bookshelves are covered in dust. What's the best way to clean them?
- How do I dust shelves without just moving the dust around?
- Can you tell me how to dust the shelves in the living room?

## situation
- I have a microfibre cloth, water and a step stool.
- I've got about half an hour.

## ground truth
```datalog
task(dust_shelves).
domain(dust_shelves, cleaning).
goal(dust_shelves, dust_free_shelf).
needs(dust_shelves, cloth).
needs(dust_shelves, water).
needs(dust_shelves, step_stool).
takes_time(dust_shelves, 15, 25, minute).
step(dust_shelves, dust_shelves_s1).
action(dust_shelves_s1, clear).
object_of(dust_shelves_s1, shelf).
uses(dust_shelves_s1, step_stool).
role(dust_shelves_s1, step_stool, reach).
takes_time(dust_shelves_s1, 3, 5, minute).
step(dust_shelves, dust_shelves_s2).
action(dust_shelves_s2, dust).
object_of(dust_shelves_s2, shelf).
uses(dust_shelves_s2, cloth).
role(dust_shelves_s2, cloth, wipe).
requires_state(dust_shelves_s2, cloth_dry).
takes_time(dust_shelves_s2, 5, 10, minute).
step(dust_shelves, dust_shelves_s3).
action(dust_shelves_s3, wipe).
object_of(dust_shelves_s3, item).
uses(dust_shelves_s3, cloth).
role(dust_shelves_s3, cloth, wipe).
uses(dust_shelves_s3, water).
role(dust_shelves_s3, water, clean).
requires_state(dust_shelves_s3, cloth_damp).
takes_time(dust_shelves_s3, 5, 7, minute).
step(dust_shelves, dust_shelves_s4).
action(dust_shelves_s4, put).
object_of(dust_shelves_s4, item).
onto(dust_shelves_s4, shelf).
takes_time(dust_shelves_s4, 3, 5, minute).
before(dust_shelves_s1, dust_shelves_s2).
before(dust_shelves_s2, dust_shelves_s3).
before(dust_shelves_s3, dust_shelves_s4).
```

## aliases
```datalog
task_alias(dust_shelves, "dusting the shelves").
task_alias(dust_shelves, "dust shelves").
alias(cloth, "microfibre cloth").
alias(cloth, "dry cloth").
alias(shelf, "shelves").
alias(shelf, "bookshelves").
alias(shelf, "top shelf").
alias(item, "items").
alias(put, "put back").
alias(cloth_damp, "dampened in water").
```

## teach
- To dust the shelves you need a microfibre cloth, water and a step stool.
- Dusting the shelves takes 15 to 25 minutes.
- Step 1 of dusting the shelves: clear the items off each shelf, using the step stool for the top shelf, for 3 to 5 minutes.
- Step 2 of dusting the shelves: dust each shelf from the top down with the dry cloth for 5 to 10 minutes.
- Step 3 of dusting the shelves: wipe each item with the cloth dampened in water for 5 to 7 minutes.
- Step 4 of dusting the shelves: put the items back on the shelves for 3 to 5 minutes.

## checklist
- names what is needed: a microfibre cloth, water and a step stool
- gives the steps in order: clear shelf, dust shelf, wipe item, put item
- gives a total time of about 15 to 25 minutes
- closes politely
