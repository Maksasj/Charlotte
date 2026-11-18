---
id: fix_wobbly_chair
stage: tools
task: fix_wobbly_chair
schema: v3
tier: T
---
# Fix a wobbly chair

## requests
- One of my dining chairs rocks when you sit on it. Can I fix it?
- How do you repair a wooden chair with a loose leg?
- Is there a way to stop a chair from wobbling?

## situation
- I have a screwdriver, wood glue, a damp cloth and a clamp.

## ground truth
```datalog
task(fix_wobbly_chair).
domain(fix_wobbly_chair, repair).
goal(fix_wobbly_chair, steady_chair).
needs(fix_wobbly_chair, screwdriver).
needs(fix_wobbly_chair, wood_glue).
needs(fix_wobbly_chair, cloth).
needs(fix_wobbly_chair, clamp).
takes_time(fix_wobbly_chair, 24, 25, hour).
step(fix_wobbly_chair, fix_wobbly_chair_s1).
action(fix_wobbly_chair_s1, turn_over).
object_of(fix_wobbly_chair_s1, chair).
step(fix_wobbly_chair, fix_wobbly_chair_s2).
action(fix_wobbly_chair_s2, tighten).
object_of(fix_wobbly_chair_s2, screw).
uses(fix_wobbly_chair_s2, screwdriver).
role(fix_wobbly_chair_s2, screwdriver, fasten).
step(fix_wobbly_chair, fix_wobbly_chair_s3).
action(fix_wobbly_chair_s3, glue).
object_of(fix_wobbly_chair_s3, joint).
uses(fix_wobbly_chair_s3, wood_glue).
role(fix_wobbly_chair_s3, wood_glue, fasten).
condition(fix_wobbly_chair_s3, joint_loose).
step(fix_wobbly_chair, fix_wobbly_chair_s4).
action(fix_wobbly_chair_s4, wipe).
object_of(fix_wobbly_chair_s4, joint).
uses(fix_wobbly_chair_s4, cloth).
role(fix_wobbly_chair_s4, cloth, wipe).
purpose(fix_wobbly_chair_s4, excess_glue_gone).
step(fix_wobbly_chair, fix_wobbly_chair_s5).
action(fix_wobbly_chair_s5, clamp).
object_of(fix_wobbly_chair_s5, joint).
uses(fix_wobbly_chair_s5, clamp).
role(fix_wobbly_chair_s5, clamp, hold).
step(fix_wobbly_chair, fix_wobbly_chair_s6).
action(fix_wobbly_chair_s6, leave).
object_of(fix_wobbly_chair_s6, chair).
takes_time(fix_wobbly_chair_s6, 24, 24, hour).
achieves(fix_wobbly_chair_s6, glue_dry).
check(fix_wobbly_chair_s6, steady).
before(fix_wobbly_chair_s1, fix_wobbly_chair_s2).
before(fix_wobbly_chair_s2, fix_wobbly_chair_s3).
before(fix_wobbly_chair_s3, fix_wobbly_chair_s4).
before(fix_wobbly_chair_s4, fix_wobbly_chair_s5).
before(fix_wobbly_chair_s5, fix_wobbly_chair_s6).
```

## aliases
```datalog
task_alias(fix_wobbly_chair, "fixing a wobbly chair").
task_alias(fix_wobbly_chair, "repairing a loose chair leg").
task_alias(fix_wobbly_chair, "stopping a chair rocking").
alias(chair, "dining chair").
alias(joint, "loose joint").
alias(turn_over, "turn upside down").
alias(wood_glue, "glue").
alias(excess_glue_gone, "removes the extra glue").
```

## teach
- To fix a wobbly chair you need a screwdriver, wood glue, a damp cloth and a clamp.
- Fixing a wobbly chair takes 24 to 25 hours, mostly waiting for the glue.
- Step 1 of fixing a wobbly chair: turn the chair upside down.
- Step 2 of fixing a wobbly chair: tighten every screw with the screwdriver.
- Step 3 of fixing a wobbly chair: if a joint is still loose, glue the joint with wood glue.
- Step 4 of fixing a wobbly chair: wipe the joint with the damp cloth to remove the extra glue.
- Step 5 of fixing a wobbly chair: clamp the joint with the clamp.
- Step 6 of fixing a wobbly chair: leave the chair for 24 hours until the glue is dry, then check it is steady.

## checklist
- names what is needed: a screwdriver, wood glue, a cloth and a clamp
- gives the steps in order: turn over, tighten screws, glue loose joint, wipe, clamp, leave 24 hours
- says to glue only if the joint is still loose

## variant
- change: no clamp: hold the joint tight with a leather belt
- request: I need to glue a chair leg but I've got nothing to clamp it with. What can I do?
- situation: I have a screwdriver, wood glue, a cloth and a leather belt.
- expect: The reply wraps the belt tightly around the legs to hold the glued joint while it dries, instead of a clamp.

```datalog situation
has(user, screwdriver).
has(user, wood_glue).
has(user, cloth).
has(user, belt).
lacks(user, clamp).
```

```datalog difference
swap(fix_wobbly_chair_s5, clamp, belt).
extra_need(belt).
no_need(clamp).
```

## variant
- change: no glue: just tighten the screws
- request: I don't have any wood glue. Can I still do something about my wobbly chair today?
- situation: I have a screwdriver and a cloth.
- expect: The reply tightens all the screws and leaves out the glue, wipe, clamp and waiting steps, noting the joint may loosen again.

```datalog situation
has(user, screwdriver).
has(user, cloth).
lacks(user, wood_glue).
```

```datalog difference
drop(fix_wobbly_chair_s3).
drop(fix_wobbly_chair_s4).
drop(fix_wobbly_chair_s5).
drop(fix_wobbly_chair_s6).
no_need(wood_glue).
no_need(clamp).
change(fix_wobbly_chair, takes_time, 24_25_hour, 10_15_minute).
```
