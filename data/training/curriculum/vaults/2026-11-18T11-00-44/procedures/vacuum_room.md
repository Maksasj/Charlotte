---
id: vacuum_room
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Vacuum a room

Clear the floor and vacuum it, corners included.

```datalog
task(vacuum_room).
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
takes_time(vacuum_room_s3, 10, 15, minute).
until(vacuum_room_s3, clean).
step(vacuum_room, vacuum_room_s4).
action(vacuum_room_s4, vacuum).
object_of(vacuum_room_s4, corner).
uses(vacuum_room_s4, crevice_tool).
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
