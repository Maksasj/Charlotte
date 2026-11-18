---
id: patch_drywall_hole__no_putty_knife
stage: tools
task: patch_drywall_hole
schema: v3
tier: S
base: patch_drywall_hole
---
# Patch a small hole in a wall

## requests
- A picture hook left a hole in my plasterboard. I've got filler and paint but no putty knife. Can I manage?

## ground truth
```datalog
task(patch_drywall_hole).
domain(patch_drywall_hole, repair).
goal(patch_drywall_hole, patched_wall).
needs(patch_drywall_hole, spackle).
needs(patch_drywall_hole, putty_knife).
needs(patch_drywall_hole, sandpaper).
needs(patch_drywall_hole, paint).
needs(patch_drywall_hole, paintbrush).
takes_time(patch_drywall_hole, 40, 60, minute).
step(patch_drywall_hole, patch_drywall_hole_s1).
action(patch_drywall_hole_s1, clean).
object_of(patch_drywall_hole_s1, hole).
takes_time(patch_drywall_hole_s1, 1, 2, minute).
step(patch_drywall_hole, patch_drywall_hole_s2).
action(patch_drywall_hole_s2, fill).
object_of(patch_drywall_hole_s2, hole).
uses(patch_drywall_hole_s2, spackle).
role(patch_drywall_hole_s2, spackle, fill).
uses(patch_drywall_hole_s2, putty_knife).
role(patch_drywall_hole_s2, putty_knife, spread).
step(patch_drywall_hole, patch_drywall_hole_s3).
action(patch_drywall_hole_s3, smooth).
object_of(patch_drywall_hole_s3, spackle).
uses(patch_drywall_hole_s3, putty_knife).
role(patch_drywall_hole_s3, putty_knife, spread).
achieves(patch_drywall_hole_s3, flat).
takes_time(patch_drywall_hole_s3, 2, 3, minute).
step(patch_drywall_hole, patch_drywall_hole_s4).
action(patch_drywall_hole_s4, dry).
object_of(patch_drywall_hole_s4, spackle).
takes_time(patch_drywall_hole_s4, 30, 40, minute).
step(patch_drywall_hole, patch_drywall_hole_s5).
action(patch_drywall_hole_s5, sand).
object_of(patch_drywall_hole_s5, spackle).
uses(patch_drywall_hole_s5, sandpaper).
role(patch_drywall_hole_s5, sandpaper, smooth).
requires_state(patch_drywall_hole_s5, spackle_dry).
until(patch_drywall_hole_s5, smooth).
check(patch_drywall_hole_s5, level_with_wall).
takes_time(patch_drywall_hole_s5, 2, 3, minute).
step(patch_drywall_hole, patch_drywall_hole_s6).
action(patch_drywall_hole_s6, paint).
object_of(patch_drywall_hole_s6, patch).
uses(patch_drywall_hole_s6, paint).
role(patch_drywall_hole_s6, paint, coat).
uses(patch_drywall_hole_s6, paintbrush).
role(patch_drywall_hole_s6, paintbrush, spread).
purpose(patch_drywall_hole_s6, matches_wall).
takes_time(patch_drywall_hole_s6, 5, 10, minute).
before(patch_drywall_hole_s1, patch_drywall_hole_s2).
before(patch_drywall_hole_s2, patch_drywall_hole_s3).
before(patch_drywall_hole_s3, patch_drywall_hole_s4).
before(patch_drywall_hole_s4, patch_drywall_hole_s5).
before(patch_drywall_hole_s5, patch_drywall_hole_s6).
```

## aliases
```datalog
task_alias(patch_drywall_hole, "patching a small hole in a wall").
task_alias(patch_drywall_hole, "patch a small hole in a wall").
task_alias(patch_drywall_hole, "filling a small hole in plasterboard").
task_alias(patch_drywall_hole, "patch a dent in the wall").
alias(hole, "small hole").
alias(hole, "dent").
alias(clean, "clean the loose bits").
alias(paint, "matching paint").
alias(paint, "paint that matches the wall").
alias(spackle_dry, "the dry spackle").
alias(level_with_wall, "level with the wall").
alias(matches_wall, "to match the wall").
alias(mesh_patch, "self-adhesive mesh patch").
alias(mesh_patch, "mesh").
```

## teach
- To patch a small hole in a wall you need spackle, a putty knife, sandpaper, matching paint and a paintbrush.
- Patching a small hole in a wall takes 40 to 60 minutes.
- Step 1 of patching a small hole in a wall: clean the loose bits from around the hole for 1 to 2 minutes.
- Step 2 of patching a small hole in a wall: fill the hole with spackle using the putty knife.
- Step 3 of patching a small hole in a wall: smooth the spackle flat for 2 to 3 minutes.
- Step 4 of patching a small hole in a wall: let the spackle dry for 30 to 40 minutes.
- Step 5 of patching a small hole in a wall: sand the dry spackle until it is smooth and level with the wall, for 2 to 3 minutes.
- Step 6 of patching a small hole in a wall: paint over the patch to match the wall for 5 to 10 minutes.

## variant
- change: no putty knife: spread and smooth the spackle with a flat butter knife
- request: A picture hook left a hole in my plasterboard. I've got filler and paint but no putty knife. Can I manage?
- situation: I have spackle, a butter knife, sandpaper, matching paint and a paintbrush, but no putty knife.
- expect: The reply fills and smooths the hole with the butter knife instead of a putty knife and keeps drying, sanding and painting.

```datalog situation
has(user, spackle).
has(user, knife).
has(user, sandpaper).
has(user, paint).
has(user, paintbrush).
lacks(user, putty_knife).
```

```datalog difference
swap(patch_drywall_hole_s2, putty_knife, knife).
swap(patch_drywall_hole_s3, putty_knife, knife).
no_need(putty_knife).
extra_need(knife).
```
