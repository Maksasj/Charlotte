---
id: stain_wood_table
stage: procedures
task: stain_wood_table
schema: v3
---
# Stain a wood table

## requests
- I want to stain my wooden dining table. What do I need to do?
- Can you give me the steps for staining a wood table?
- Please explain how to put stain on a timber table properly.

## situation
- I have a wooden table, stain, a brush, a cloth and sandpaper.
- I have a full weekend to do it.

## ground truth
```datalog
task(stain_wood_table).
domain(stain_wood_table, crafts).
goal(stain_wood_table, stained_table).
needs(stain_wood_table, table).
needs(stain_wood_table, sandpaper).
needs(stain_wood_table, cloth).
needs(stain_wood_table, stain).
needs(stain_wood_table, brush).
takes_time(stain_wood_table, 300, 360, minute).
step(stain_wood_table, stain_wood_table_s1).
action(stain_wood_table_s1, sand).
object_of(stain_wood_table_s1, table).
uses(stain_wood_table_s1, sandpaper).
role(stain_wood_table_s1, sandpaper, scrub).
purpose(stain_wood_table_s1, smooth_surface).
takes_time(stain_wood_table_s1, 20, 30, minute).
step(stain_wood_table, stain_wood_table_s2).
action(stain_wood_table_s2, wipe).
object_of(stain_wood_table_s2, table).
uses(stain_wood_table_s2, cloth).
role(stain_wood_table_s2, cloth, wipe).
purpose(stain_wood_table_s2, dust_removed).
takes_time(stain_wood_table_s2, 5, 10, minute).
step(stain_wood_table, stain_wood_table_s3).
action(stain_wood_table_s3, coat).
object_of(stain_wood_table_s3, table).
uses(stain_wood_table_s3, stain).
role(stain_wood_table_s3, stain, coat).
uses(stain_wood_table_s3, brush).
role(stain_wood_table_s3, brush, spread).
warning(stain_wood_table_s3, drips).
takes_time(stain_wood_table_s3, 15, 20, minute).
step(stain_wood_table, stain_wood_table_s4).
action(stain_wood_table_s4, wipe).
object_of(stain_wood_table_s4, table).
uses(stain_wood_table_s4, cloth).
role(stain_wood_table_s4, cloth, wipe).
purpose(stain_wood_table_s4, excess_removed).
takes_time(stain_wood_table_s4, 5, 10, minute).
step(stain_wood_table, stain_wood_table_s5).
action(stain_wood_table_s5, dry).
object_of(stain_wood_table_s5, table).
until(stain_wood_table_s5, dry).
check(stain_wood_table_s5, no_tack).
takes_time(stain_wood_table_s5, 240, 300, minute).
before(stain_wood_table_s1, stain_wood_table_s2).
before(stain_wood_table_s2, stain_wood_table_s3).
before(stain_wood_table_s3, stain_wood_table_s4).
before(stain_wood_table_s4, stain_wood_table_s5).
```

## aliases
```datalog
task_alias(stain_wood_table, "staining a wooden table").
task_alias(stain_wood_table, "staining a table").
alias(stain, "wood stain").
alias(sandpaper, "sanding paper").
alias(cloth, "rag").
alias(brush, "paintbrush").
alias(dry, "dry to the touch").
alias(smooth_surface, "smooth").
alias(no_tack, "not sticky").
```

## teach
- To stain a wooden table you need a table, sandpaper, a cloth, stain and a brush.
- Staining a wooden table takes 300 to 360 minutes.
- Step 1 of staining a wooden table: sand the table top with sandpaper until it is smooth.
- Step 2 of staining a wooden table: wipe the table with the cloth to remove dust.
- Step 3 of staining a wooden table: coat the table with stain using the brush, and do not let the stain drip.
- Step 4 of staining a wooden table: wipe off the extra stain with the cloth.
- Step 5 of staining a wooden table: leave the table to dry for 240 to 300 minutes until it is dry to the touch.

## checklist
- names what is needed, including a table, sandpaper, a cloth, stain and a brush
- gives the steps in order: sand, wipe dust, coat with stain, wipe off extra, leave to dry
- warns not to let the stain drip while coating
- gives a total time of about 300 to 360 minutes

## variant
- change: the user wants a darker colour, so a second stain coat goes on after the first wipe-off
- request: I want a darker finish on my table. Can I put a second coat of stain on it?
- situation: I have a table, stain, a brush, a cloth and sandpaper, and I want a darker colour.
- expect: The reply gives the usual steps and adds a second stain coat with the brush after the first wipe-off and before the table is left to dry.

```datalog situation
has(user, table).
has(user, stain).
has(user, brush).
has(user, cloth).
has(user, sandpaper).
wants(user, darker_colour).
```

```datalog difference
add_step(stain_wood_table_s9, stain_wood_table_s4).
step(stain_wood_table, stain_wood_table_s9).
action(stain_wood_table_s9, coat).
object_of(stain_wood_table_s9, table).
uses(stain_wood_table_s9, stain).
role(stain_wood_table_s9, stain, coat).
uses(stain_wood_table_s9, brush).
role(stain_wood_table_s9, brush, spread).
takes_time(stain_wood_table_s9, 15, 20, minute).
before(stain_wood_table_s4, stain_wood_table_s9).
before(stain_wood_table_s9, stain_wood_table_s5).
```

## variant
- change: the table is already smooth, so the sanding step is dropped
- request: My table is already smooth and I have no sandpaper. How do I stain it?
- situation: I have a table, stain, a brush and a cloth, but no sandpaper.
- expect: The reply skips the sanding step and gives the other four steps: wipe dust, coat with stain, wipe off extra, leave to dry.

```datalog situation
has(user, table).
has(user, stain).
has(user, brush).
has(user, cloth).
lacks(user, sandpaper).
```

```datalog difference
drop(stain_wood_table_s1).
no_need(sandpaper).
change(stain_wood_table, takes_time, 300_360, 280_340).
```
