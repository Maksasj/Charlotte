---
id: patch_bicycle_inner_tube
stage: procedures
task: patch_bicycle_inner_tube
schema: v3
---
# Patch a bicycle inner tube

## requests
- My bike has a flat. How do I find the hole in the tube and patch it?
- What's the method for fixing a punctured bike inner tube with a patch?
- Can you talk me through repairing a puncture in my bicycle tube, step by step?

## situation
- I have a tyre lever, a bowl of water, sandpaper, glue, a rubber patch and a pump.
- I have about half an hour.

## ground truth
```datalog
task(patch_bicycle_inner_tube).
domain(patch_bicycle_inner_tube, repair).
goal(patch_bicycle_inner_tube, fix_flat_tyre).
needs(patch_bicycle_inner_tube, tyre_lever).
needs(patch_bicycle_inner_tube, bowl_of_water).
needs(patch_bicycle_inner_tube, sandpaper).
needs(patch_bicycle_inner_tube, glue).
needs(patch_bicycle_inner_tube, rubber_patch).
needs(patch_bicycle_inner_tube, pump).
takes_time(patch_bicycle_inner_tube, 30, 40, minute).
step(patch_bicycle_inner_tube, patch_bicycle_inner_tube_s1).
action(patch_bicycle_inner_tube_s1, loosen).
object_of(patch_bicycle_inner_tube_s1, tyre).
uses(patch_bicycle_inner_tube_s1, tyre_lever).
role(patch_bicycle_inner_tube_s1, tyre_lever, lift).
step(patch_bicycle_inner_tube, patch_bicycle_inner_tube_s2).
action(patch_bicycle_inner_tube_s2, submerge).
object_of(patch_bicycle_inner_tube_s2, inner_tube).
uses(patch_bicycle_inner_tube_s2, bowl_of_water).
role(patch_bicycle_inner_tube_s2, bowl_of_water, contain).
purpose(patch_bicycle_inner_tube_s2, find_hole).
step(patch_bicycle_inner_tube, patch_bicycle_inner_tube_s3).
action(patch_bicycle_inner_tube_s3, scrub).
object_of(patch_bicycle_inner_tube_s3, inner_tube).
uses(patch_bicycle_inner_tube_s3, sandpaper).
role(patch_bicycle_inner_tube_s3, sandpaper, scrub).
purpose(patch_bicycle_inner_tube_s3, rough_surface).
step(patch_bicycle_inner_tube, patch_bicycle_inner_tube_s4).
action(patch_bicycle_inner_tube_s4, coat).
object_of(patch_bicycle_inner_tube_s4, inner_tube).
uses(patch_bicycle_inner_tube_s4, glue).
role(patch_bicycle_inner_tube_s4, glue, coat).
step(patch_bicycle_inner_tube, patch_bicycle_inner_tube_s5).
action(patch_bicycle_inner_tube_s5, press).
object_of(patch_bicycle_inner_tube_s5, inner_tube).
uses(patch_bicycle_inner_tube_s5, rubber_patch).
role(patch_bicycle_inner_tube_s5, rubber_patch, cover).
purpose(patch_bicycle_inner_tube_s5, seal_hole).
step(patch_bicycle_inner_tube, patch_bicycle_inner_tube_s6).
action(patch_bicycle_inner_tube_s6, pump).
object_of(patch_bicycle_inner_tube_s6, inner_tube).
uses(patch_bicycle_inner_tube_s6, pump).
role(patch_bicycle_inner_tube_s6, pump, inflate).
until(patch_bicycle_inner_tube_s6, firm).
check(patch_bicycle_inner_tube_s6, firm).
before(patch_bicycle_inner_tube_s1, patch_bicycle_inner_tube_s2).
before(patch_bicycle_inner_tube_s2, patch_bicycle_inner_tube_s3).
before(patch_bicycle_inner_tube_s3, patch_bicycle_inner_tube_s4).
before(patch_bicycle_inner_tube_s4, patch_bicycle_inner_tube_s5).
before(patch_bicycle_inner_tube_s5, patch_bicycle_inner_tube_s6).
```

## aliases
```datalog
task_alias(patch_bicycle_inner_tube, "bike tube repair").
task_alias(patch_bicycle_inner_tube, "fixing a bike puncture").
alias(tyre_lever, "tire lever").
alias(inner_tube, "bike tube").
alias(rubber_patch, "patch").
alias(pump, "bike pump").
alias(glue, "rubber cement").
alias(submerge, "dunk").
alias(find_hole, "locate the leak").
alias(firm, "firm to the touch").
```

## teach
- To patch a bicycle inner tube you need a tyre lever, a bowl of water, sandpaper, glue, a rubber patch and a pump.
- Patching a bicycle inner tube takes 30 to 40 minutes.
- Step 1 of patching a bicycle inner tube: loosen the tyre with the tyre lever.
- Step 2 of patching a bicycle inner tube: submerge the inner tube in the bowl of water to find the hole.
- Step 3 of patching a bicycle inner tube: scrub the inner tube with sandpaper so the glue sticks.
- Step 4 of patching a bicycle inner tube: coat the inner tube with glue.
- Step 5 of patching a bicycle inner tube: press the rubber patch onto the hole.
- Step 6 of patching a bicycle inner tube: pump the inner tube until it is firm.

## checklist
- names what is needed, including a tyre lever, a bowl of water, sandpaper, glue, a rubber patch and a pump
- gives the steps in order: loosen tyre, submerge tube, scrub, coat with glue, press patch, pump until firm
- says the water test is how to find the hole
- gives a total time of about 30 to 40 minutes

## variant
- change: no bowl at hand, so use the sink to find the hole instead
- request: I have a flat but no bowl. Can I still find the hole in the tube?
- situation: I have a sink, a tyre lever, sandpaper, glue, a rubber patch and a pump.
- expect: The reply uses the sink, filled with water, in place of the bowl to submerge the inner tube, then continues with the same steps.

```datalog situation
lacks(user, bowl_of_water).
has(user, sink).
```

```datalog difference
swap(patch_bicycle_inner_tube_s2, bowl_of_water, sink).
no_need(bowl_of_water).
extra_need(sink).
```

## variant
- change: wait 10 minutes for the glue to dry before pressing the patch on
- request: I have about 50 minutes, so I want the glue to set properly before I press the patch. How long should I wait?
- situation: I have all the kit and 50 minutes.
- expect: The reply adds a wait of about 10 minutes after the glue step, until the glue is dry, before pressing the patch, and says the total is about 40 to 50 minutes.

```datalog situation
has(user, glue).
time_limit(user, 50, minute).
```

```datalog difference
add_step(patch_bicycle_inner_tube_s9, patch_bicycle_inner_tube_s4).
step(patch_bicycle_inner_tube, patch_bicycle_inner_tube_s9).
action(patch_bicycle_inner_tube_s9, wait).
object_of(patch_bicycle_inner_tube_s9, glue).
until(patch_bicycle_inner_tube_s9, dry).
takes_time(patch_bicycle_inner_tube_s9, 10, 10, minute).
before(patch_bicycle_inner_tube_s4, patch_bicycle_inner_tube_s9).
before(patch_bicycle_inner_tube_s9, patch_bicycle_inner_tube_s5).
change(patch_bicycle_inner_tube, takes_time, 30_40, 40_50).
```
