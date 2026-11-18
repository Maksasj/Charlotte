---
id: do_laundry
stage: household
task: do_laundry
schema: v3
---
# Do the laundry

## requests
- I just moved out and have never done my own laundry. How does it work?
- My laundry basket is overflowing. Can you walk me through washing a load?
- How do I wash my clothes in the washing machine without ruining them?

## situation
- I have a washing machine, laundry detergent, a drying rack and a full basket of clothes.
- I've got the whole afternoon, a couple of hours at least.

## ground truth
```datalog
task(do_laundry).
domain(do_laundry, cleaning).
goal(do_laundry, clean_clothes).
needs(do_laundry, laundry).
needs(do_laundry, washing_machine).
needs(do_laundry, detergent).
needs(do_laundry, drying_rack).
takes_time(do_laundry, 75, 115, minute).
step(do_laundry, do_laundry_s1).
action(do_laundry_s1, sort).
object_of(do_laundry_s1, laundry).
purpose(do_laundry_s1, colours_do_not_run).
takes_time(do_laundry_s1, 3, 5, minute).
step(do_laundry, do_laundry_s2).
action(do_laundry_s2, load).
object_of(do_laundry_s2, laundry).
uses(do_laundry_s2, washing_machine).
role(do_laundry_s2, washing_machine, contain).
takes_time(do_laundry_s2, 2, 3, minute).
step(do_laundry, do_laundry_s3).
action(do_laundry_s3, add).
object_of(do_laundry_s3, detergent).
into(do_laundry_s3, drawer).
takes_time(do_laundry_s3, 1, 1, minute).
step(do_laundry, do_laundry_s4).
action(do_laundry_s4, run).
object_of(do_laundry_s4, washing_machine).
setting(do_laundry_s4, cycle, normal, level).
takes_time(do_laundry_s4, 60, 90, minute).
step(do_laundry, do_laundry_s5).
action(do_laundry_s5, hang).
object_of(do_laundry_s5, laundry).
uses(do_laundry_s5, drying_rack).
role(do_laundry_s5, drying_rack, hold).
takes_time(do_laundry_s5, 10, 15, minute).
before(do_laundry_s1, do_laundry_s2).
before(do_laundry_s2, do_laundry_s3).
before(do_laundry_s3, do_laundry_s4).
before(do_laundry_s4, do_laundry_s5).
```

## aliases
```datalog
task_alias(do_laundry, "doing the laundry").
task_alias(do_laundry, "washing a load").
task_alias(do_laundry, "wash my clothes").
alias(laundry, "clothes").
alias(laundry, "basket of clothes").
alias(laundry, "pile of laundry").
alias(laundry, "load").
alias(detergent, "laundry detergent").
alias(washing_machine, "machine").
alias(drawer, "detergent drawer").
alias(normal, "normal cycle").
alias(colours_do_not_run, "colours do not run").
```

## teach
- To do the laundry you need laundry, a washing machine, detergent and a drying rack.
- Doing the laundry takes 75 to 115 minutes.
- Step 1 of doing the laundry: sort the laundry into lights and darks so the colours do not run, for 3 to 5 minutes.
- Step 2 of doing the laundry: load one pile of laundry into the washing machine for 2 to 3 minutes.
- Step 3 of doing the laundry: add the detergent to the drawer for 1 minute.
- Step 4 of doing the laundry: run the washing machine on a normal cycle for 60 to 90 minutes.
- Step 5 of doing the laundry: hang the laundry on the drying rack for 10 to 15 minutes.

## checklist
- says to sort lights from darks before washing
- gives the steps in order: sort, load, add detergent, run the machine, hang to dry
- gives a total time of about 75 to 115 minutes, most of it the machine cycle
- closes politely

## variant
- change: no room to hang clothes: use a tumble dryer instead of the drying rack
- request: I need to wash my clothes but I've got nowhere to hang them. There's a dryer in the building though.
- situation: I have a washing machine, detergent, a tumble dryer and a basket of clothes, but no drying rack.
- expect: The reply ends by moving the laundry into the tumble dryer for about 45 to 60 minutes instead of hanging it on a rack.

```datalog situation
has(user, washing_machine).
has(user, detergent).
has(user, tumble_dryer).
has(user, laundry).
lacks(user, drying_rack).
```

```datalog difference
drop(do_laundry_s5).
no_need(drying_rack).
extra_need(tumble_dryer).
add_step(do_laundry_s9, do_laundry_s4).
step(do_laundry, do_laundry_s9).
action(do_laundry_s9, dry).
object_of(do_laundry_s9, laundry).
uses(do_laundry_s9, tumble_dryer).
role(do_laundry_s9, tumble_dryer, dry).
takes_time(do_laundry_s9, 45, 60, minute).
before(do_laundry_s4, do_laundry_s9).
```
