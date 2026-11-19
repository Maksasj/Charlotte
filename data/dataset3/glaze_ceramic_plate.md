---
id: glaze_ceramic_plate
stage: procedures
task: glaze_ceramic_plate
schema: v3
---
# Glaze a ceramic plate

## requests
- I've made an unglazed plate in pottery class. What's the process for putting glaze on it and firing it?
- Can you explain how to glaze a bisque-fired plate at home?
- Give me the steps for glazing a clay plate so it comes out food-safe and shiny.

## situation
- I have a bisque plate, a glaze, a brush, a spoon, a sponge and a kiln.
- I have a free weekend, so the drying and firing time is not a problem.

## ground truth
```datalog
task(glaze_ceramic_plate).
domain(glaze_ceramic_plate, crafts).
goal(glaze_ceramic_plate, glazed_plate).
needs(glaze_ceramic_plate, bisque_plate).
needs(glaze_ceramic_plate, glaze).
needs(glaze_ceramic_plate, brush).
needs(glaze_ceramic_plate, spoon).
needs(glaze_ceramic_plate, sponge).
needs(glaze_ceramic_plate, kiln).
takes_time(glaze_ceramic_plate, 18, 24, hour).
step(glaze_ceramic_plate, glaze_ceramic_plate_s1).
action(glaze_ceramic_plate_s1, wipe).
object_of(glaze_ceramic_plate_s1, bisque_plate).
uses(glaze_ceramic_plate_s1, sponge).
role(glaze_ceramic_plate_s1, sponge, wipe).
purpose(glaze_ceramic_plate_s1, glaze_sticks).
takes_time(glaze_ceramic_plate_s1, 2, 3, minute).
step(glaze_ceramic_plate, glaze_ceramic_plate_s2).
action(glaze_ceramic_plate_s2, stir).
object_of(glaze_ceramic_plate_s2, glaze).
uses(glaze_ceramic_plate_s2, spoon).
role(glaze_ceramic_plate_s2, spoon, stir).
purpose(glaze_ceramic_plate_s2, glaze_even).
overlaps(glaze_ceramic_plate_s2, glaze_ceramic_plate_s1).
takes_time(glaze_ceramic_plate_s2, 3, 5, minute).
step(glaze_ceramic_plate, glaze_ceramic_plate_s3).
action(glaze_ceramic_plate_s3, coat).
object_of(glaze_ceramic_plate_s3, bisque_plate).
uses(glaze_ceramic_plate_s3, brush).
role(glaze_ceramic_plate_s3, brush, spread).
uses(glaze_ceramic_plate_s3, glaze).
role(glaze_ceramic_plate_s3, glaze, coat).
warning(glaze_ceramic_plate_s3, drips).
takes_time(glaze_ceramic_plate_s3, 10, 15, minute).
step(glaze_ceramic_plate, glaze_ceramic_plate_s4).
action(glaze_ceramic_plate_s4, dry).
object_of(glaze_ceramic_plate_s4, bisque_plate).
until(glaze_ceramic_plate_s4, dry_to_touch).
takes_time(glaze_ceramic_plate_s4, 2, 4, hour).
step(glaze_ceramic_plate, glaze_ceramic_plate_s5).
action(glaze_ceramic_plate_s5, fire).
object_of(glaze_ceramic_plate_s5, bisque_plate).
uses(glaze_ceramic_plate_s5, kiln).
role(glaze_ceramic_plate_s5, kiln, heat).
setting(glaze_ceramic_plate_s5, temperature, 1000, celsius).
takes_time(glaze_ceramic_plate_s5, 8, 10, hour).
step(glaze_ceramic_plate, glaze_ceramic_plate_s6).
action(glaze_ceramic_plate_s6, cool).
object_of(glaze_ceramic_plate_s6, bisque_plate).
check(glaze_ceramic_plate_s6, cool_to_touch).
takes_time(glaze_ceramic_plate_s6, 8, 10, hour).
before(glaze_ceramic_plate_s1, glaze_ceramic_plate_s3).
before(glaze_ceramic_plate_s2, glaze_ceramic_plate_s3).
before(glaze_ceramic_plate_s3, glaze_ceramic_plate_s4).
before(glaze_ceramic_plate_s4, glaze_ceramic_plate_s5).
before(glaze_ceramic_plate_s5, glaze_ceramic_plate_s6).
```

## aliases
```datalog
task_alias(glaze_ceramic_plate, "glazing a ceramic plate").
task_alias(glaze_ceramic_plate, "glazing a plate").
alias(bisque_plate, "bisque plate").
alias(bisque_plate, "unglazed plate").
alias(kiln, "kiln").
alias(stir, "mix").
alias(coat, "brush on").
alias(dry_to_touch, "dry to the touch").
alias(dry_to_touch, "dry").
alias(glaze_even, "even glaze").
alias(glaze_sticks, "glaze sticks").
```

## teach
- To glaze a ceramic plate you need a bisque plate, a glaze, a brush, a spoon, a sponge and a kiln.
- Glazing a ceramic plate takes 18 to 24 hours.
- Step 1 of glazing a ceramic plate: wipe the bisque plate with the sponge to remove dust, for 2 to 3 minutes so the glaze sticks.
- While step 1 runs, step 2 of glazing a ceramic plate: stir the glaze with the spoon for 3 to 5 minutes so it is even.
- Step 3 of glazing a ceramic plate: brush the glaze onto the bisque plate, for 10 to 15 minutes, and take care the glaze does not drip.
- Step 4 of glazing a ceramic plate: let the glaze dry on the plate for 2 to 4 hours until it is dry to the touch.
- Step 5 of glazing a ceramic plate: fire the plate in the kiln at 1000 degrees for 8 to 10 hours.
- Step 6 of glazing a ceramic plate: let the plate cool in the kiln for 8 to 10 hours until it is cool to the touch.

## checklist
- names what is needed, including a bisque plate, a glaze, a brush, a spoon, a sponge and a kiln
- gives the steps in order: wipe, stir the glaze while wiping, brush on the glaze, dry, fire at about 1000 degrees, cool
- says the glaze must be stirred while the plate is wiped
- gives a total time of about 18 to 24 hours

## variant
- change: add a second glaze coat after the first, for a thicker finish
- request: I want a thicker glaze on my plate. Can I put on a second coat?
- situation: I have a bisque plate, a glaze, a brush, a spoon, a sponge and a kiln.
- expect: The reply keeps the base steps and adds a second brushed coat of glaze after the first coat has been put on and before the plate is left to dry, with the total time going up by about one hour.

```datalog situation
has(user, glaze).
has(user, brush).
```

```datalog difference
add_step(glaze_ceramic_plate_s9, glaze_ceramic_plate_s3).
step(glaze_ceramic_plate, glaze_ceramic_plate_s9).
action(glaze_ceramic_plate_s9, coat).
object_of(glaze_ceramic_plate_s9, bisque_plate).
uses(glaze_ceramic_plate_s9, brush).
role(glaze_ceramic_plate_s9, brush, spread).
uses(glaze_ceramic_plate_s9, glaze).
role(glaze_ceramic_plate_s9, glaze, coat).
takes_time(glaze_ceramic_plate_s9, 10, 15, minute).
before(glaze_ceramic_plate_s3, glaze_ceramic_plate_s9).
before(glaze_ceramic_plate_s9, glaze_ceramic_plate_s4).
change(glaze_ceramic_plate, takes_time, 18_24, 19_26).
```

## variant
- change: no kiln is available, so the plate cannot be fired at home
- request: I don't have a kiln. Can I still glaze this plate at home?
- situation: I have a bisque plate, a glaze, a brush, a spoon and a sponge, but no kiln.
- expect: The reply says the glaze must be fired in a kiln to become food-safe and glossy, so the plate cannot be finished at home without one, and suggests a pottery studio or a kiln firing service.

```datalog situation
lacks(user, kiln).
has(user, bisque_plate).
```

```datalog difference
cannot(no_heat_source).
```
