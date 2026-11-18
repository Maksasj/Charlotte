---
id: vacuum_room__broken_crevice_tool
task: vacuum_room
schema: v3
tier: S
base: vacuum_room
---
# Vacuum a room

## requests
- The thin nozzle for my hoover cracked in half. How do I get the dust out of the corners of the lounge?

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

## teach
- To vacuum a room you need a vacuum cleaner and its crevice tool.
- Vacuuming a room takes 20 to 30 minutes.
- Step 1 of vacuuming a room: clear the floor for 5 minutes.
- Step 2 of vacuuming a room: plug in the vacuum cleaner.
- Step 3 of vacuuming a room: vacuum the floor with the vacuum cleaner until it is clean, for 10 to 15 minutes.
- Step 4 of vacuuming a room: vacuum the corners with the crevice tool until they are clean, for 3 to 5 minutes.
- Step 5 of vacuuming a room: empty the dust bin until it is empty, for 1 to 2 minutes.

## variant
- change: broken crevice tool: brush the corners with a dustpan brush
- request: The thin nozzle for my hoover cracked in half. How do I get the dust out of the corners of the lounge?
- situation: The crevice tool is broken. I have the vacuum cleaner and a dustpan and brush.
- expect: The reply vacuums the floor as usual and does the corners with the dustpan brush instead of the crevice tool.

```datalog situation
has(user, vacuum_cleaner).
has(user, dustpan_brush).
lacks(user, crevice_tool).
```

```datalog difference
swap(vacuum_room_s4, crevice_tool, dustpan_brush).
no_need(crevice_tool).
extra_need(dustpan_brush).
```
