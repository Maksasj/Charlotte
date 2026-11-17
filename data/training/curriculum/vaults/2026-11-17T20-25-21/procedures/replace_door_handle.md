---
id: replace_door_handle
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# replace door handle

Learned in conversation.

```datalog
task(replace_door_handle). @conf 1 @source ""
needs(replace_door_handle, handle_set). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
needs(replace_door_handle, screwdriver). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
takes_time(replace_door_handle, 12, 20, minute). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
step(replace_door_handle, replace_door_handle_s1). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
action(replace_door_handle_s1, unscrew). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
object_of(replace_door_handle_s1, handle). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
takes_time(replace_door_handle_s1, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
step(replace_door_handle, replace_door_handle_s3). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
action(replace_door_handle_s3, slide). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
object_of(replace_door_handle_s3, spindle). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
uses(replace_door_handle_s3, handle_set). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
takes_time(replace_door_handle_s3, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
before(replace_door_handle_s2, replace_door_handle_s3). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
step(replace_door_handle, replace_door_handle_s4). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
action(replace_door_handle_s4, screw). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
object_of(replace_door_handle_s4, handle). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
uses(replace_door_handle_s4, screwdriver). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
takes_time(replace_door_handle_s4, 5, 8, minute). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
before(replace_door_handle_s3, replace_door_handle_s4). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
step(replace_door_handle, replace_door_handle_s2). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
action(replace_door_handle_s2, pull_out). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
object_of(replace_door_handle_s2, handle). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
object_of(replace_door_handle_s2, spindle). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
takes_time(replace_door_handle_s2, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
before(replace_door_handle_s1, replace_door_handle_s2). @conf 1 @source teacher:curriculum @conv s1794946451073-131028-133
```
