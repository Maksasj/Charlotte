---
id: set_table__eight_guests
task: set_table
schema: v3
tier: S
base: set_table
---
# Set the table

## requests
- Eight people are coming for dinner. How do I lay the table for all of them?

## ground truth
```datalog
task(set_table).
domain(set_table, cooking).
goal(set_table, laid_table).
needs(set_table, tablecloth).
needs(set_table, plate).
needs(set_table, fork).
needs(set_table, knife).
needs(set_table, glass).
needs(set_table, napkin).
takes_time(set_table, 5, 10, minute).
step(set_table, set_table_s1).
action(set_table_s1, spread).
object_of(set_table_s1, tablecloth).
takes_time(set_table_s1, 1, 2, minute).
onto(set_table_s1, table).
step(set_table, set_table_s2).
action(set_table_s2, place).
object_of(set_table_s2, plate).
takes_time(set_table_s2, 1, 2, minute).
onto(set_table_s2, table).
step(set_table, set_table_s3).
action(set_table_s3, place).
object_of(set_table_s3, fork).
takes_time(set_table_s3, 1, 2, minute).
step(set_table, set_table_s4).
action(set_table_s4, place).
object_of(set_table_s4, knife).
takes_time(set_table_s4, 1, 2, minute).
step(set_table, set_table_s5).
action(set_table_s5, place).
object_of(set_table_s5, glass).
takes_time(set_table_s5, 1, 2, minute).
step(set_table, set_table_s6).
action(set_table_s6, fold).
object_of(set_table_s6, napkin).
takes_time(set_table_s6, 1, 2, minute).
before(set_table_s1, set_table_s2).
before(set_table_s2, set_table_s3).
before(set_table_s2, set_table_s4).
before(set_table_s1, set_table_s5).
before(set_table_s2, set_table_s6).
```

## aliases
```datalog
task_alias(set_table, "setting the table").
task_alias(set_table, "lay the table").
task_alias(set_table, "laying the table").
alias(tablecloth, "cloth").
alias(tablecloth, "table cloth").
alias(plate, "plates").
alias(fork, "forks").
alias(knife, "knives").
alias(glass, "glasses").
alias(napkin, "napkins").
alias(place, "lay").
alias(spread, "lay").
```

## teach
- To set the table you need a tablecloth, plates, forks, knives, glasses and napkins.
- Setting the table takes 5 to 10 minutes.
- Step 1 of setting the table: spread the tablecloth onto the table for 1 to 2 minutes.
- Step 2 of setting the table: place the plates onto the table for 1 to 2 minutes.
- Step 3 of setting the table: after the plates, place the forks for 1 to 2 minutes.
- Step 4 of setting the table: after the plates, place the knives for 1 to 2 minutes.
- Step 5 of setting the table: after the cloth, place the glasses for 1 to 2 minutes.
- Step 6 of setting the table: after the plates, fold the napkins for 1 to 2 minutes.

## variant
- change: eight guests: one place setting per guest
- request: Eight people are coming for dinner. How do I lay the table for all of them?
- situation: Eight guests are coming. I have a tablecloth and eight each of plates, forks, knives, glasses and napkins.
- expect: The reply lays the cloth once and then a plate, fork, knife, glass and napkin for each of the eight places.

```datalog situation
has(user, tablecloth).
has(user, plate).
has(user, fork).
has(user, knife).
has(user, glass).
has(user, napkin).
count(user, guest, 8).
```

```datalog difference
change(set_table_s2, repeat, 1, 8).
change(set_table_s3, repeat, 1, 8).
change(set_table_s4, repeat, 1, 8).
change(set_table_s5, repeat, 1, 8).
change(set_table_s6, repeat, 1, 8).
change(set_table, takes_time, 5_10, 15_25).
```
