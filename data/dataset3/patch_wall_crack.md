---
id: patch_wall_crack
stage: procedures
task: patch_wall_crack
schema: v3
---
# Patch a wall crack

## requests
- My plaster wall has a small crack. How do I patch it properly before I repaint?
- What's the method for filling a hairline crack in a wall and painting over it?
- Can you tell me the steps to repair a crack in drywall so the repair doesn't show?

## situation
- I have a hairline crack in a plaster wall, about 5 mm wide, plus wall filler, a putty knife, sandpaper, a stiff brush, a damp cloth, paint and a paint brush.
- I have most of the afternoon free.

## ground truth
```datalog
task(patch_wall_crack).
domain(patch_wall_crack, repair).
goal(patch_wall_crack, patched_wall).
needs(patch_wall_crack, brush).
needs(patch_wall_crack, cloth).
needs(patch_wall_crack, wall_filler).
needs(patch_wall_crack, putty_knife).
needs(patch_wall_crack, sandpaper).
needs(patch_wall_crack, paint).
needs(patch_wall_crack, paint_brush).
takes_time(patch_wall_crack, 60, 90, minute).
step(patch_wall_crack, patch_wall_crack_s1).
action(patch_wall_crack_s1, scrub).
object_of(patch_wall_crack_s1, crack).
uses(patch_wall_crack_s1, brush).
role(patch_wall_crack_s1, brush, scrub).
purpose(patch_wall_crack_s1, filler_grips).
takes_time(patch_wall_crack_s1, 5, 10, minute).
step(patch_wall_crack, patch_wall_crack_s2).
action(patch_wall_crack_s2, wipe).
object_of(patch_wall_crack_s2, crack).
uses(patch_wall_crack_s2, cloth).
role(patch_wall_crack_s2, cloth, wipe).
check(patch_wall_crack_s2, dust_gone).
takes_time(patch_wall_crack_s2, 2, 5, minute).
step(patch_wall_crack, patch_wall_crack_s3).
action(patch_wall_crack_s3, spread).
object_of(patch_wall_crack_s3, wall_filler).
into(patch_wall_crack_s3, crack).
uses(patch_wall_crack_s3, putty_knife).
role(patch_wall_crack_s3, putty_knife, spread).
takes_time(patch_wall_crack_s3, 10, 15, minute).
step(patch_wall_crack, patch_wall_crack_s4).
action(patch_wall_crack_s4, sand).
object_of(patch_wall_crack_s4, wall_filler).
uses(patch_wall_crack_s4, sandpaper).
role(patch_wall_crack_s4, sandpaper, scrub).
requires_state(patch_wall_crack_s4, filler_dry).
takes_time(patch_wall_crack_s4, 15, 20, minute).
step(patch_wall_crack, patch_wall_crack_s5).
action(patch_wall_crack_s5, paint).
object_of(patch_wall_crack_s5, crack).
uses(patch_wall_crack_s5, paint_brush).
role(patch_wall_crack_s5, paint_brush, coat).
uses(patch_wall_crack_s5, paint).
role(patch_wall_crack_s5, paint, cover).
takes_time(patch_wall_crack_s5, 30, 40, minute).
before(patch_wall_crack_s1, patch_wall_crack_s2).
before(patch_wall_crack_s2, patch_wall_crack_s3).
before(patch_wall_crack_s3, patch_wall_crack_s4).
before(patch_wall_crack_s4, patch_wall_crack_s5).
```

## aliases
```datalog
task_alias(patch_wall_crack, "patch a wall crack").
task_alias(patch_wall_crack, "fix a wall crack").
task_alias(patch_wall_crack, "repair a crack in a wall").
alias(crack, "wall crack").
alias(brush, "stiff brush").
alias(cloth, "damp cloth").
alias(wall_filler, "filler").
alias(wall_filler, "spackle").
alias(putty_knife, "filling knife").
alias(sandpaper, "sanding paper").
alias(paint_brush, "brush for paint").
alias(dust_gone, "no dust left").
alias(filler_dry, "dry").
alias(filler_grips, "filler sticks").
alias(butter_knife, "butter knife").
```

## teach
- To patch a wall crack you need a stiff brush, a damp cloth, wall filler, a putty knife, sandpaper, paint and a paint brush.
- Patching a wall crack takes 60 to 90 minutes.
- Step 1 of patching a wall crack: scrub inside the crack with the stiff brush for 5 to 10 minutes so the filler sticks.
- Step 2 of patching a wall crack: wipe the crack with the damp cloth for 2 to 5 minutes until no dust is left.
- Step 3 of patching a wall crack: spread the wall filler into the crack with the putty knife for 10 to 15 minutes.
- Step 4 of patching a wall crack: once the filler is dry, sand it with sandpaper for 15 to 20 minutes.
- Step 5 of patching a wall crack: paint over the crack with the paint brush, covering it with paint, for 30 to 40 minutes.

## checklist
- names what is needed, including a stiff brush, damp cloth, wall filler, putty knife, sandpaper, paint and paint brush
- gives the steps in order: scrub, wipe, fill, sand once dry, paint
- says the sanding waits until the filler is dry
- gives a total time of about 60 to 90 minutes

## variant
- change: no putty knife: spread the filler with a butter knife instead
- request: I want to fix the crack but I only have a butter knife, not a putty knife. Will that work?
- situation: I have a butter knife, wall filler, a stiff brush, a damp cloth, sandpaper, paint and a paint brush, but no putty knife.
- expect: The reply spreads the filler with the butter knife in place of the putty knife and otherwise follows the base steps.

```datalog situation
lacks(user, putty_knife).
has(user, butter_knife).
has(user, wall_filler).
has(user, brush).
has(user, cloth).
has(user, sandpaper).
has(user, paint).
has(user, paint_brush).
```

```datalog difference
swap(patch_wall_crack_s3, putty_knife, butter_knife).
no_need(putty_knife).
extra_need(butter_knife).
```

## variant
- change: no sandpaper and a short time: skip the sanding and paint once the filler is dry
- request: I'm short on time and I don't have any sandpaper. Can I skip sanding and still paint it?
- situation: I have a stiff brush, a damp cloth, wall filler, a putty knife, paint and a paint brush, but no sandpaper, and I have about 50 minutes.
- expect: The reply drops the sanding step, paints the dry filler directly for a total of about 47 to 70 minutes, and notes the finish will be less smooth.

```datalog situation
lacks(user, sandpaper).
has(user, brush).
has(user, cloth).
has(user, wall_filler).
has(user, putty_knife).
has(user, paint).
has(user, paint_brush).
time_limit(user, 50, minute).
```

```datalog difference
drop(patch_wall_crack_s4).
no_need(sandpaper).
change(patch_wall_crack, takes_time, 60_90, 47_70).
```
