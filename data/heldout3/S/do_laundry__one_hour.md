---
id: do_laundry__one_hour
stage: household
task: do_laundry
schema: v3
tier: S
base: do_laundry
---
# Do the laundry

## requests
- I need a load of clothes washed and hung up within the hour before I head out. How?

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

## variant
- change: only one hour: run the quick cycle
- request: I need a load of clothes washed and hung up within the hour before I head out. How?
- situation: I have a washing machine with a quick cycle, detergent, a drying rack and a basket of clothes, and one hour.
- expect: The reply runs the quick 30-minute cycle instead of the normal one and keeps sorting, loading, detergent and hanging.

```datalog situation
has(user, laundry).
has(user, washing_machine).
has(user, detergent).
has(user, drying_rack).
time_limit(user, 60, minute).
```

```datalog difference
change(do_laundry_s4, setting, normal, quick).
change(do_laundry_s4, takes_time, 60_90, 30_30).
change(do_laundry, takes_time, 75_115, 45_55).
```
