---
id: descale_kettle
stage: household
task: descale_kettle
schema: v3
tier: T
---
# Descale a kettle

## requests
- There's white crusty stuff inside my kettle. How do I get rid of it?
- How do I descale a kettle?
- My tea has little flakes floating in it lately. Is there a way to clean the kettle out?

## situation
- I have a kettle, a bottle of white vinegar, water and a cloth.

## ground truth
```datalog
task(descale_kettle).
domain(descale_kettle, cleaning).
goal(descale_kettle, no_limescale).
goal(descale_kettle, clean_kettle).
needs(descale_kettle, kettle).
needs(descale_kettle, white_vinegar).
needs(descale_kettle, water).
needs(descale_kettle, cloth).
takes_time(descale_kettle, 45, 50, minute).
step(descale_kettle, descale_kettle_s1).
action(descale_kettle_s1, fill).
object_of(descale_kettle_s1, kettle).
uses(descale_kettle_s1, white_vinegar).
role(descale_kettle_s1, white_vinegar, clean).
uses(descale_kettle_s1, water).
role(descale_kettle_s1, water, wet).
amount(descale_kettle_s1, white_vinegar, 250, millilitre).
amount(descale_kettle_s1, water, 250, millilitre).
step(descale_kettle, descale_kettle_s2).
action(descale_kettle_s2, boil).
object_of(descale_kettle_s2, kettle).
uses(descale_kettle_s2, kettle).
role(descale_kettle_s2, kettle, heat).
takes_time(descale_kettle_s2, 2, 3, minute).
step(descale_kettle, descale_kettle_s3).
action(descale_kettle_s3, leave).
object_of(descale_kettle_s3, kettle).
takes_time(descale_kettle_s3, 30, 30, minute).
purpose(descale_kettle_s3, limescale_dissolves).
step(descale_kettle, descale_kettle_s4).
action(descale_kettle_s4, empty).
object_of(descale_kettle_s4, kettle).
warning(descale_kettle_s4, hot_water).
step(descale_kettle, descale_kettle_s5).
action(descale_kettle_s5, rinse).
object_of(descale_kettle_s5, kettle).
uses(descale_kettle_s5, water).
role(descale_kettle_s5, water, clean).
repeat(descale_kettle_s5, 2).
step(descale_kettle, descale_kettle_s6).
action(descale_kettle_s6, wipe).
object_of(descale_kettle_s6, kettle).
uses(descale_kettle_s6, cloth).
role(descale_kettle_s6, cloth, wipe).
with_part(descale_kettle_s6, inside).
check(descale_kettle_s6, no_limescale).
before(descale_kettle_s1, descale_kettle_s2).
before(descale_kettle_s2, descale_kettle_s3).
before(descale_kettle_s3, descale_kettle_s4).
before(descale_kettle_s4, descale_kettle_s5).
before(descale_kettle_s5, descale_kettle_s6).
```

## aliases
```datalog
task_alias(descale_kettle, "descaling a kettle").
task_alias(descale_kettle, "getting limescale out of a kettle").
task_alias(descale_kettle, "cleaning the inside of a kettle").
alias(white_vinegar, "vinegar").
alias(limescale_dissolves, "the limescale loosens").
alias(no_limescale, "the scale is gone").
alias(leave, "let it sit").
```

## teach
- To descale a kettle you need the kettle, white vinegar, water and a cloth.
- Descaling a kettle takes 45 to 50 minutes.
- Step 1 of descaling a kettle: fill the kettle with 250 millilitres of white vinegar and 250 millilitres of water.
- Step 2 of descaling a kettle: boil the kettle for 2 to 3 minutes.
- Step 3 of descaling a kettle: leave it for 30 minutes so the limescale loosens.
- Step 4 of descaling a kettle: empty the kettle; watch out, the water is hot.
- Step 5 of descaling a kettle: rinse the kettle with clean water twice.
- Step 6 of descaling a kettle: wipe the inside with the cloth until the scale is gone.

## checklist
- names what is needed: the kettle, white vinegar, water and a cloth
- gives the steps in order: fill with vinegar and water, boil, leave 30 minutes, empty, rinse twice, wipe
- warns the water is hot when emptying

## variant
- change: no vinegar: use lemon juice instead
- request: I want to get the scale out of my kettle but I don't have any vinegar. Anything else work?
- situation: I have a kettle, two lemons, water and a cloth, but no vinegar.
- expect: The reply fills the kettle with lemon juice and water instead of vinegar and otherwise keeps the steps.

```datalog situation
has(user, kettle).
has(user, lemon).
has(user, water).
has(user, cloth).
lacks(user, white_vinegar).
```

```datalog difference
swap(descale_kettle_s1, white_vinegar, lemon).
extra_need(lemon).
no_need(white_vinegar).
```

## variant
- change: only 15 minutes: boil twice and skip the long soak
- request: Guests are coming in a quarter of an hour and my kettle is all furred up. Can I do a quick descale?
- situation: I have a kettle, white vinegar, water and a cloth, and about 15 minutes.
- expect: The reply boils the vinegar mix twice and leaves it only about 5 minutes instead of 30 before rinsing.

```datalog situation
has(user, kettle).
has(user, white_vinegar).
has(user, water).
has(user, cloth).
time_limit(user, 15, minute).
```

```datalog difference
change(descale_kettle_s2, repeat, 1, 2).
change(descale_kettle_s3, takes_time, 30_30, 5_5).
change(descale_kettle, takes_time, 45_50, 15_15).
```
