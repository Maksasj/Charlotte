---
id: knead_bread_dough
stage: procedures
task: knead_bread_dough
schema: v3
---
# Knead bread dough

## requests
- How do I knead bread dough by hand?
- Walk me through making a basic bread dough from scratch.
- What's the method for mixing and kneading a simple yeast dough, and how long does it take?

## situation
- I have flour, water, yeast, salt, a bowl, a clean counter and a cloth.
- I have a free afternoon.

## ground truth
```datalog
task(knead_bread_dough).
domain(knead_bread_dough, cooking).
goal(knead_bread_dough, bread_dough).
needs(knead_bread_dough, flour).
needs(knead_bread_dough, water).
needs(knead_bread_dough, yeast).
needs(knead_bread_dough, salt).
needs(knead_bread_dough, bowl).
needs(knead_bread_dough, work_surface).
needs(knead_bread_dough, cloth).
takes_time(knead_bread_dough, 90, 120, minute).
step(knead_bread_dough, knead_bread_dough_s1).
action(knead_bread_dough_s1, pour).
object_of(knead_bread_dough_s1, water).
into(knead_bread_dough_s1, bowl).
uses(knead_bread_dough_s1, bowl).
role(knead_bread_dough_s1, bowl, contain).
amount(knead_bread_dough_s1, water, 300, millilitre).
setting(knead_bread_dough_s1, temperature, 35, celsius).
step(knead_bread_dough, knead_bread_dough_s2).
action(knead_bread_dough_s2, mix).
object_of(knead_bread_dough_s2, flour).
amount(knead_bread_dough_s2, flour, 500, gram).
into(knead_bread_dough_s2, bowl).
uses(knead_bread_dough_s2, yeast).
role(knead_bread_dough_s2, yeast, leaven).
uses(knead_bread_dough_s2, salt).
role(knead_bread_dough_s2, salt, season).
uses(knead_bread_dough_s2, bowl).
role(knead_bread_dough_s2, bowl, contain).
step(knead_bread_dough, knead_bread_dough_s3).
action(knead_bread_dough_s3, knead).
object_of(knead_bread_dough_s3, dough).
uses(knead_bread_dough_s3, work_surface).
role(knead_bread_dough_s3, work_surface, hold).
until(knead_bread_dough_s3, smooth).
check(knead_bread_dough_s3, elastic).
purpose(knead_bread_dough_s3, gluten_forms).
takes_time(knead_bread_dough_s3, 8, 10, minute).
step(knead_bread_dough, knead_bread_dough_s4).
action(knead_bread_dough_s4, cover).
object_of(knead_bread_dough_s4, dough).
uses(knead_bread_dough_s4, cloth).
role(knead_bread_dough_s4, cloth, cover).
purpose(knead_bread_dough_s4, surface_stays_moist).
step(knead_bread_dough, knead_bread_dough_s5).
action(knead_bread_dough_s5, rise).
object_of(knead_bread_dough_s5, dough).
until(knead_bread_dough_s5, doubled).
takes_time(knead_bread_dough_s5, 60, 90, minute).
before(knead_bread_dough_s1, knead_bread_dough_s2).
before(knead_bread_dough_s2, knead_bread_dough_s3).
before(knead_bread_dough_s3, knead_bread_dough_s4).
before(knead_bread_dough_s4, knead_bread_dough_s5).
```

## aliases
```datalog
task_alias(knead_bread_dough, "make bread dough").
task_alias(knead_bread_dough, "kneading bread dough").
task_alias(knead_bread_dough, "basic yeast dough").
alias(bowl, "mixing bowl").
alias(work_surface, "counter").
alias(cloth, "damp tea towel").
alias(yeast, "dried yeast").
alias(rise, "prove").
alias(doubled, "doubled in size").
alias(smooth, "smooth and elastic").
alias(gluten_forms, "gluten develops").
alias(surface_stays_moist, "does not dry out").
alias(pot, "large pot").
```

## teach
- To knead bread dough you need flour, water, yeast, salt, a bowl, a work surface and a cloth.
- Kneading bread dough takes 90 to 120 minutes.
- Step 1 of kneading bread dough: pour 300 millilitres of water at about 35 degrees into the bowl.
- Step 2 of kneading bread dough: mix 500 grams of flour with the yeast and salt in the bowl.
- Step 3 of kneading bread dough: knead the dough on the counter for 8 to 10 minutes until it is smooth and elastic, so the gluten forms.
- Step 4 of kneading bread dough: cover the dough with a damp cloth so the surface stays moist.
- Step 5 of kneading bread dough: let the dough rise for 60 to 90 minutes until it is doubled in size.

## checklist
- names what is needed: flour, water, yeast, salt, a bowl, a work surface and a cloth
- gives the five steps in order: pour water, mix with yeast and salt, knead until smooth and elastic, cover, let rise until doubled
- gives the water at about 35 degrees and 500 grams of flour
- gives a total time of about 90 to 120 minutes, most of it rising

## variant
- change: no mixing bowl: use a large pot in its place for steps 1 and 2
- request: I don't own a mixing bowl, but I have a big pot. Can I still make the dough?
- situation: I have flour, water, yeast, salt, a large pot, a clean counter and a cloth, but no bowl.
- expect: The reply uses the large pot in place of the bowl for pouring the water and mixing the flour, then keeps the kneading, covering and rising steps as they are.

```datalog situation
has(user, flour).
has(user, water).
has(user, yeast).
has(user, salt).
has(user, pot).
has(user, work_surface).
has(user, cloth).
lacks(user, bowl).
```

```datalog difference
swap(knead_bread_dough_s1, bowl, pot).
swap(knead_bread_dough_s2, bowl, pot).
```

- change: only about an hour: rise in a warm place for 30 to 40 minutes instead of 60 to 90
- request: I need bread dough ready within about an hour. Can you shorten the rising time?
- situation: I have flour, water, yeast, salt, a bowl, a clean counter and a cloth, and only about an hour.
- expect: The reply keeps the same mixing and kneading, shortens the rise to 30 to 40 minutes in a warm place, and says the dough should be ready in about 50 to 60 minutes in total.

```datalog situation
has(user, flour).
has(user, water).
has(user, yeast).
has(user, salt).
has(user, bowl).
has(user, work_surface).
has(user, cloth).
time_limit(user, 60, minute).
```

```datalog difference
change(knead_bread_dough_s5, takes_time, 60_90, 30_40).
change(knead_bread_dough, takes_time, 90_120, 50_60).
```
