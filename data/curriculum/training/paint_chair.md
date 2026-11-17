---
id: paint_chair
stage: tools
task: paint_chair
---
# Paint a wooden chair

## requests
- I found an old wooden chair at a flea market. How do I paint it?
- Can you walk me through repainting a kitchen chair?
- What's the right way to paint wooden furniture so the paint doesn't peel?

## situation
- I have the chair, sandpaper, a damp cloth, primer, paint, a paintbrush and newspaper.
- I have a whole free afternoon.

## ground truth
```datalog
task(paint_chair).
needs(paint_chair, sandpaper).
needs(paint_chair, cloth).
needs(paint_chair, primer).
needs(paint_chair, paint).
needs(paint_chair, paintbrush).
needs(paint_chair, newspaper).
takes_time(paint_chair, 120, 180, minute).
step(paint_chair, paint_chair_s1).
action(paint_chair_s1, sand).
object_of(paint_chair_s1, chair).
uses(paint_chair_s1, sandpaper).
takes_time(paint_chair_s1, 20, 30, minute).
step(paint_chair, paint_chair_s2).
action(paint_chair_s2, wipe).
object_of(paint_chair_s2, chair).
uses(paint_chair_s2, cloth).
takes_time(paint_chair_s2, 5, 5, minute).
step(paint_chair, paint_chair_s3).
action(paint_chair_s3, prime).
object_of(paint_chair_s3, chair).
uses(paint_chair_s3, primer).
uses(paint_chair_s3, paintbrush).
uses(paint_chair_s3, newspaper).
takes_time(paint_chair_s3, 15, 20, minute).
step(paint_chair, paint_chair_s4).
action(paint_chair_s4, dry).
object_of(paint_chair_s4, primer).
takes_time(paint_chair_s4, 60, 90, minute).
step(paint_chair, paint_chair_s5).
action(paint_chair_s5, paint).
object_of(paint_chair_s5, chair).
uses(paint_chair_s5, paint).
uses(paint_chair_s5, paintbrush).
takes_time(paint_chair_s5, 20, 30, minute).
before(paint_chair_s1, paint_chair_s2).
before(paint_chair_s2, paint_chair_s3).
before(paint_chair_s3, paint_chair_s4).
before(paint_chair_s4, paint_chair_s5).
```

## teach
- To paint a wooden chair you need sandpaper, a damp cloth, primer, paint, a paintbrush and newspaper.
- Painting a wooden chair takes 120 to 180 minutes.
- Step 1 of painting a wooden chair: sand the chair all over with the sandpaper for 20 to 30 minutes.
- Step 2 of painting a wooden chair: wipe the dust off the chair with the damp cloth for 5 minutes.
- Step 3 of painting a wooden chair: set the chair on the newspaper and brush on a coat of primer for 15 to 20 minutes.
- Step 4 of painting a wooden chair: let the primer dry for 60 to 90 minutes.
- Step 5 of painting a wooden chair: paint the chair with thin, even coats, for 20 to 30 minutes.

## checklist
- names what is needed: sandpaper, a damp cloth, primer, paint, a paintbrush and newspaper
- gives the steps in order: sand chair, wipe chair, prime chair, dry primer, paint chair
- gives a total time of about 120 to 180 minutes
- closes politely

## variant
- change: chair already painted and smooth: give it a light sand and skip the primer
- request: My chair is already painted white, I just want it blue. Do I need to do everything?
- situation: I have the painted chair, sandpaper, a damp cloth, blue paint, a paintbrush and newspaper.
- expect: The reply gives the chair a light sand only, wipes it and paints it without the primer and its drying time, so the job takes about 50 to 70 minutes.
