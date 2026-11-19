---
id: sand_wood_surface
stage: procedures
task: sand_wood_surface
schema: v3
---
# Sand a wooden surface

## requests
- How do I sand a wooden tabletop so it's smooth before I varnish it?
- What's the right way to sand down a wood surface until it feels silky?
- Can you walk me through sanding a piece of wood by hand, step by step?

## situation
- I have sandpaper, a cloth and a brush, and a wooden table to sand.
- I have about an hour free.

## ground truth
```datalog
task(sand_wood_surface).
domain(sand_wood_surface, crafts).
goal(sand_wood_surface, smooth_surface).
needs(sand_wood_surface, sandpaper).
needs(sand_wood_surface, cloth).
needs(sand_wood_surface, brush).
takes_time(sand_wood_surface, 30, 45, minute).
step(sand_wood_surface, sand_wood_surface_s1).
action(sand_wood_surface_s1, wipe).
object_of(sand_wood_surface_s1, wood_surface).
uses(sand_wood_surface_s1, cloth).
role(sand_wood_surface_s1, cloth, wipe).
purpose(sand_wood_surface_s1, dust_removed).
step(sand_wood_surface, sand_wood_surface_s2).
action(sand_wood_surface_s2, sand).
object_of(sand_wood_surface_s2, wood_surface).
uses(sand_wood_surface_s2, sandpaper).
role(sand_wood_surface_s2, sandpaper, scrub).
setting(sand_wood_surface_s2, grit, fine, level).
until(sand_wood_surface_s2, smooth).
takes_time(sand_wood_surface_s2, 20, 30, minute).
step(sand_wood_surface, sand_wood_surface_s3).
action(sand_wood_surface_s3, clean).
object_of(sand_wood_surface_s3, sanding_dust).
uses(sand_wood_surface_s3, brush).
role(sand_wood_surface_s3, brush, clean).
step(sand_wood_surface, sand_wood_surface_s4).
action(sand_wood_surface_s4, wipe).
object_of(sand_wood_surface_s4, wood_surface).
uses(sand_wood_surface_s4, cloth).
role(sand_wood_surface_s4, cloth, wipe).
achieves(sand_wood_surface_s4, dry).
before(sand_wood_surface_s1, sand_wood_surface_s2).
before(sand_wood_surface_s2, sand_wood_surface_s3).
before(sand_wood_surface_s3, sand_wood_surface_s4).
```

## aliases
```datalog
task_alias(sand_wood_surface, "sanding a wooden surface").
task_alias(sand_wood_surface, "sand a wood surface").
task_alias(sand_wood_surface, "sanding wood").
alias(wood_surface, "wooden surface").
alias(wood_surface, "wooden tabletop").
alias(sanding_dust, "sanding dust").
alias(sanding_dust, "sawdust").
alias(smooth, "silky").
alias(smooth, "smooth").
alias(dust_removed, "dust removed").
alias(dry, "dry").
```

## teach
- To sand a wooden surface you need sandpaper, a cloth and a brush.
- Sanding a wooden surface takes 30 to 45 minutes.
- Step 1 of sanding a wooden surface: wipe the wooden surface with the cloth to remove dust.
- Step 2 of sanding a wooden surface: sand the wooden surface with the fine sandpaper for 20 to 30 minutes until it is smooth.
- Step 3 of sanding a wooden surface: brush off the sanding dust with the brush.
- Step 4 of sanding a wooden surface: wipe the wooden surface with the cloth until it is dry.

## checklist
- names what is needed: sandpaper, a cloth and a brush
- gives the steps in order: wipe to remove dust, sand with fine sandpaper until smooth, brush off the dust, wipe dry
- says to use fine sandpaper for the sanding step
- gives a total time of about 30 to 45 minutes

## variant
- change: no brush: wipe the sanding dust off with the cloth instead
- request: I want to sand a wooden shelf but I don't have a brush. What do I do instead?
- situation: I have sandpaper and a cloth, and no brush.
- expect: The reply sands the surface as usual, then uses the cloth in place of the brush to wipe the sanding dust off.

```datalog situation
lacks(user, brush).
has(user, sandpaper).
has(user, cloth).
```

```datalog difference
swap(sand_wood_surface_s3, brush, cloth).
no_need(brush).
```

## variant
- change: only 20 minutes: sand for a shorter time and finish in about 15 to 20 minutes
- request: I only have 20 minutes to sand a wooden surface. Can I still do it?
- situation: I have sandpaper, a cloth and a brush, and 20 minutes.
- expect: The reply sands for about 10 to 15 minutes, then brushes and wipes the surface, for a total of about 15 to 20 minutes.

```datalog situation
has(user, sandpaper).
has(user, cloth).
has(user, brush).
time_limit(user, 20, minute).
```

```datalog difference
change(sand_wood_surface, takes_time, 30_45, 15_20).
change(sand_wood_surface_s2, takes_time, 20_30, 10_15).
```
