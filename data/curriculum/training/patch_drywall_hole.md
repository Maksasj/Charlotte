---
id: patch_drywall_hole
stage: tools
task: patch_drywall_hole
---
# Patch a small hole in a wall

## requests
- There's a hole in the wall where a picture hook was ripped out. How do I fix it?
- Can you walk me through filling a small hole in plasterboard?
- How do I patch a dent in the wall so you can't see it?

## situation
- I have spackle, a putty knife, sandpaper, paint that matches the wall and a paintbrush.
- I have the morning.

## ground truth
```datalog
task(patch_drywall_hole).
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
uses(patch_drywall_hole_s2, putty_knife).
takes_time(patch_drywall_hole_s2, 2, 3, minute).
step(patch_drywall_hole, patch_drywall_hole_s3).
action(patch_drywall_hole_s3, dry).
object_of(patch_drywall_hole_s3, spackle).
takes_time(patch_drywall_hole_s3, 30, 40, minute).
step(patch_drywall_hole, patch_drywall_hole_s4).
action(patch_drywall_hole_s4, sand).
object_of(patch_drywall_hole_s4, spackle).
uses(patch_drywall_hole_s4, sandpaper).
until(patch_drywall_hole_s4, smooth).
takes_time(patch_drywall_hole_s4, 2, 3, minute).
step(patch_drywall_hole, patch_drywall_hole_s5).
action(patch_drywall_hole_s5, paint).
object_of(patch_drywall_hole_s5, patch).
uses(patch_drywall_hole_s5, paint).
uses(patch_drywall_hole_s5, paintbrush).
takes_time(patch_drywall_hole_s5, 5, 10, minute).
before(patch_drywall_hole_s1, patch_drywall_hole_s2).
before(patch_drywall_hole_s2, patch_drywall_hole_s3).
before(patch_drywall_hole_s3, patch_drywall_hole_s4).
before(patch_drywall_hole_s4, patch_drywall_hole_s5).
```

## teach
- To patch a small hole in a wall you need spackle, a putty knife, sandpaper, matching paint and a paintbrush.
- Patching a small hole in a wall takes 40 to 60 minutes.
- Step 1 of patching a small hole in a wall: clean the loose bits from around the hole for 1 to 2 minutes.
- Step 2 of patching a small hole in a wall: fill the hole with spackle using the putty knife and smooth it flat for 2 to 3 minutes.
- Step 3 of patching a small hole in a wall: let the spackle dry for 30 to 40 minutes.
- Step 4 of patching a small hole in a wall: sand the dry spackle until it is smooth and level with the wall, for 2 to 3 minutes.
- Step 5 of patching a small hole in a wall: paint over the patch to match the wall for 5 to 10 minutes.

## checklist
- names what is needed: spackle, a putty knife, sandpaper, matching paint and a paintbrush
- gives the steps in order: clean hole, fill hole, dry spackle, sand spackle, paint patch
- gives a total time of about 40 to 60 minutes
- closes politely

## variant
- change: bigger hole, about 5 centimetres: cover it with a self-adhesive mesh patch before the spackle
- request: The doorknob went through the wall and left a hole about the size of a fist. Can I still patch it myself?
- situation: I have a self-adhesive mesh patch, spackle, a putty knife, sandpaper, matching paint and a paintbrush.
- expect: The reply sticks the mesh patch over the hole first, spreads the spackle over the mesh in thin coats and keeps the dry, sand and paint steps.
