---
id: replace_door_handle
stage: tools
task: replace_door_handle
schema: v3
---
# Replace a door handle

## requests
- The handle on my bedroom door is loose and broken. How do I swap it for a new one?
- Can you explain how to change an interior door handle?
- I bought new door handles. How do I fit them?

## situation
- I have a new door handle set with its spindle and a screwdriver.
- I have about half an hour.

## ground truth
```datalog
task(replace_door_handle).
domain(replace_door_handle, repair).
goal(replace_door_handle, working_door_handle).
needs(replace_door_handle, new_handle).
needs(replace_door_handle, screwdriver).
takes_time(replace_door_handle, 12, 20, minute).
step(replace_door_handle, replace_door_handle_s1).
action(replace_door_handle_s1, unscrew).
object_of(replace_door_handle_s1, handle).
uses(replace_door_handle_s1, screwdriver).
role(replace_door_handle_s1, screwdriver, loosen).
takes_time(replace_door_handle_s1, 3, 5, minute).
step(replace_door_handle, replace_door_handle_s2).
action(replace_door_handle_s2, pull).
object_of(replace_door_handle_s2, spindle).
takes_time(replace_door_handle_s2, 1, 2, minute).
step(replace_door_handle, replace_door_handle_s3).
action(replace_door_handle_s3, slide).
object_of(replace_door_handle_s3, spindle).
uses(replace_door_handle_s3, new_handle).
role(replace_door_handle_s3, new_handle, fasten).
into(replace_door_handle_s3, latch).
takes_time(replace_door_handle_s3, 1, 2, minute).
step(replace_door_handle, replace_door_handle_s4).
action(replace_door_handle_s4, screw).
object_of(replace_door_handle_s4, new_handle).
uses(replace_door_handle_s4, screwdriver).
role(replace_door_handle_s4, screwdriver, fasten).
takes_time(replace_door_handle_s4, 5, 8, minute).
step(replace_door_handle, replace_door_handle_s5).
action(replace_door_handle_s5, test).
object_of(replace_door_handle_s5, new_handle).
until(replace_door_handle_s5, smooth).
check(replace_door_handle_s5, latch_springs_back).
takes_time(replace_door_handle_s5, 1, 2, minute).
before(replace_door_handle_s1, replace_door_handle_s2).
before(replace_door_handle_s2, replace_door_handle_s3).
before(replace_door_handle_s3, replace_door_handle_s4).
before(replace_door_handle_s4, replace_door_handle_s5).
```

## aliases
```datalog
task_alias(replace_door_handle, "replacing a door handle").
task_alias(replace_door_handle, "replace a door handle").
task_alias(replace_door_handle, "change an interior door handle").
task_alias(replace_door_handle, "fit new door handles").
alias(handle, "old handle").
alias(handle, "old handles").
alias(new_handle, "new handle set").
alias(new_handle, "new handles").
alias(new_handle, "new door handle set").
alias(pull, "pull out").
alias(latch_springs_back, "the latch springs back").
alias(smooth, "smoothly").
```

## teach
- To replace a door handle you need a new handle set and a screwdriver.
- Replacing a door handle takes 12 to 20 minutes.
- Step 1 of replacing a door handle: unscrew the old handle on both sides of the door for 3 to 5 minutes.
- Step 2 of replacing a door handle: pull out the old handles and the spindle for 1 to 2 minutes.
- Step 3 of replacing a door handle: slide the spindle of the new handle set through the latch for 1 to 2 minutes.
- Step 4 of replacing a door handle: screw the new handles on both sides with the screwdriver for 5 to 8 minutes.
- Step 5 of replacing a door handle: test the new handle until the latch springs back smoothly, for 1 to 2 minutes.

## checklist
- names what is needed: a new handle set and a screwdriver
- gives the steps in order: unscrew handle, pull spindle, slide spindle, screw new handle, test new handle
- gives a total time of about 12 to 20 minutes
- closes politely
