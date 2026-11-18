---
id: vacuum_room
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Vacuum a room

Clear the floor and vacuum it, corners included.

## ground truth
```datalog
task(vacuum_room).
domain(vacuum_room, cleaning).
goal(vacuum_room, clean_floor).
needs(vacuum_room, vacuum_cleaner).
needs(vacuum_room, crevice_tool).
takes_time(vacuum_room, 20, 30, minute).
step(vacuum_room, vacuum_room_s1).
action(vacuum_room_s1, clear).
object_of(vacuum_room_s1, floor).
takes_time(vacuum_room_s1, 5, 5, minute).
step(vacuum_room, vacuum_room_s2).
action(vacuum_room_s2, plug).
object_of(vacuum_room_s2, vacuum_cleaner).
step(vacuum_room, vacuum_room_s3).
action(vacuum_room_s3, vacuum).
object_of(vacuum_room_s3, floor).
uses(vacuum_room_s3, vacuum_cleaner).
role(vacuum_room_s3, vacuum_cleaner, clean).
takes_time(vacuum_room_s3, 10, 15, minute).
until(vacuum_room_s3, clean).
step(vacuum_room, vacuum_room_s4).
action(vacuum_room_s4, vacuum).
object_of(vacuum_room_s4, corner).
uses(vacuum_room_s4, crevice_tool).
role(vacuum_room_s4, crevice_tool, clean).
takes_time(vacuum_room_s4, 3, 5, minute).
until(vacuum_room_s4, clean).
step(vacuum_room, vacuum_room_s5).
action(vacuum_room_s5, empty).
object_of(vacuum_room_s5, dust_bin).
takes_time(vacuum_room_s5, 1, 2, minute).
until(vacuum_room_s5, empty).
before(vacuum_room_s1, vacuum_room_s2).
before(vacuum_room_s2, vacuum_room_s3).
before(vacuum_room_s3, vacuum_room_s4).
before(vacuum_room_s4, vacuum_room_s5).
```

## aliases
```datalog
task_alias(vacuum_room, "vacuuming a room").
task_alias(vacuum_room, "hoover the room").
task_alias(vacuum_room, "hoovering").
task_alias(vacuum_room, "vacuum the floor").
alias(vacuum_cleaner, "vacuum").
alias(vacuum_cleaner, "hoover").
alias(crevice_tool, "crevice tool").
alias(crevice_tool, "nozzle").
alias(corner, "corners").
alias(dust_bin, "dust container").
alias(vacuum, "hoover").
alias(plug, "plug in").
alias(clear, "pick things up").
```
