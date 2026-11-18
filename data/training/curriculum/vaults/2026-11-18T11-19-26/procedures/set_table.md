---
id: set_table
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Set the table

Lay the cloth, then a place setting for each guest.

```datalog
task(set_table).
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
step(set_table, set_table_s2).
action(set_table_s2, place).
object_of(set_table_s2, plate).
takes_time(set_table_s2, 1, 2, minute).
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
