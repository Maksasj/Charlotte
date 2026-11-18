---
id: paint_chair
stage: tools
task: paint_chair
schema: v3
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
domain(paint_chair, crafts).
goal(paint_chair, painted_chair).
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
role(paint_chair_s1, sandpaper, smooth).
takes_time(paint_chair_s1, 20, 30, minute).
step(paint_chair, paint_chair_s2).
action(paint_chair_s2, wipe).
object_of(paint_chair_s2, chair).
uses(paint_chair_s2, cloth).
role(paint_chair_s2, cloth, wipe).
takes_time(paint_chair_s2, 5, 5, minute).
step(paint_chair, paint_chair_s3).
action(paint_chair_s3, set).
object_of(paint_chair_s3, chair).
uses(paint_chair_s3, newspaper).
role(paint_chair_s3, newspaper, protect).
onto(paint_chair_s3, newspaper).
step(paint_chair, paint_chair_s4).
action(paint_chair_s4, prime).
object_of(paint_chair_s4, chair).
uses(paint_chair_s4, primer).
role(paint_chair_s4, primer, coat).
uses(paint_chair_s4, paintbrush).
role(paint_chair_s4, paintbrush, spread).
takes_time(paint_chair_s4, 15, 20, minute).
step(paint_chair, paint_chair_s5).
action(paint_chair_s5, dry).
object_of(paint_chair_s5, primer).
takes_time(paint_chair_s5, 60, 90, minute).
step(paint_chair, paint_chair_s6).
action(paint_chair_s6, paint).
object_of(paint_chair_s6, chair).
uses(paint_chair_s6, paint).
role(paint_chair_s6, paint, coat).
uses(paint_chair_s6, paintbrush).
role(paint_chair_s6, paintbrush, spread).
takes_time(paint_chair_s6, 20, 30, minute).
before(paint_chair_s1, paint_chair_s2).
before(paint_chair_s2, paint_chair_s3).
before(paint_chair_s3, paint_chair_s4).
before(paint_chair_s4, paint_chair_s5).
before(paint_chair_s5, paint_chair_s6).
```

## aliases
```datalog
task_alias(paint_chair, "painting a wooden chair").
task_alias(paint_chair, "paint a wooden chair").
task_alias(paint_chair, "repainting a kitchen chair").
task_alias(paint_chair, "paint wooden furniture").
alias(chair, "wooden chair").
alias(chair, "kitchen chair").
alias(chair, "painted chair").
alias(cloth, "damp cloth").
alias(paint, "blue paint").
alias(prime, "brush on a coat of primer").
```

## teach
- To paint a wooden chair you need sandpaper, a damp cloth, primer, paint, a paintbrush and newspaper.
- Painting a wooden chair takes 120 to 180 minutes.
- Step 1 of painting a wooden chair: sand the chair all over with the sandpaper for 20 to 30 minutes.
- Step 2 of painting a wooden chair: wipe the dust off the chair with the damp cloth for 5 minutes.
- Step 3 of painting a wooden chair: set the chair on the newspaper.
- Step 4 of painting a wooden chair: brush on a coat of primer for 15 to 20 minutes.
- Step 5 of painting a wooden chair: let the primer dry for 60 to 90 minutes.
- Step 6 of painting a wooden chair: paint the chair with thin, even coats, for 20 to 30 minutes.

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

```datalog situation
has(user, chair).
has(user, sandpaper).
has(user, cloth).
has(user, paint).
has(user, paintbrush).
has(user, newspaper).
```

```datalog difference
drop(paint_chair_s4).
drop(paint_chair_s5).
no_need(primer).
change(paint_chair, takes_time, 120_180, 50_70).
```
