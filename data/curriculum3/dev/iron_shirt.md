---
id: iron_shirt
stage: household
task: iron_shirt
schema: v3
---
# Iron a shirt

## requests
- I have a job interview tomorrow and my shirt is all creased. How do I iron it?
- Could you explain how to iron a button-up shirt properly?
- My shirt is wrinkled and I've never really used an iron. Can you help?

## situation
- I have the shirt, an iron, an ironing board, some water and a hanger.
- I've got about 20 minutes.

## ground truth
```datalog
task(iron_shirt).
domain(iron_shirt, cleaning).
goal(iron_shirt, ironed_shirt).
needs(iron_shirt, shirt).
needs(iron_shirt, iron).
needs(iron_shirt, ironing_board).
needs(iron_shirt, water).
needs(iron_shirt, hanger).
takes_time(iron_shirt, 10, 15, minute).
step(iron_shirt, iron_shirt_s1).
action(iron_shirt_s1, fill).
object_of(iron_shirt_s1, iron).
uses(iron_shirt_s1, water).
role(iron_shirt_s1, water, fill).
step(iron_shirt, iron_shirt_s2).
action(iron_shirt_s2, heat).
object_of(iron_shirt_s2, iron).
setting(iron_shirt_s2, temperature, cotton, level).
takes_time(iron_shirt_s2, 2, 3, minute).
step(iron_shirt, iron_shirt_s3).
action(iron_shirt_s3, iron).
object_of(iron_shirt_s3, collar).
uses(iron_shirt_s3, ironing_board).
role(iron_shirt_s3, ironing_board, hold).
uses(iron_shirt_s3, iron).
role(iron_shirt_s3, iron, press).
requires_state(iron_shirt_s3, iron_hot).
takes_time(iron_shirt_s3, 1, 2, minute).
step(iron_shirt, iron_shirt_s4).
action(iron_shirt_s4, iron).
object_of(iron_shirt_s4, sleeve).
uses(iron_shirt_s4, ironing_board).
role(iron_shirt_s4, ironing_board, hold).
uses(iron_shirt_s4, iron).
role(iron_shirt_s4, iron, press).
takes_time(iron_shirt_s4, 3, 4, minute).
step(iron_shirt, iron_shirt_s5).
action(iron_shirt_s5, iron).
object_of(iron_shirt_s5, shirt).
with_part(iron_shirt_s5, front).
with_part(iron_shirt_s5, back).
uses(iron_shirt_s5, ironing_board).
role(iron_shirt_s5, ironing_board, hold).
uses(iron_shirt_s5, iron).
role(iron_shirt_s5, iron, press).
takes_time(iron_shirt_s5, 3, 5, minute).
step(iron_shirt, iron_shirt_s6).
action(iron_shirt_s6, hang).
object_of(iron_shirt_s6, shirt).
uses(iron_shirt_s6, hanger).
role(iron_shirt_s6, hanger, hold).
purpose(iron_shirt_s6, stays_smooth).
before(iron_shirt_s1, iron_shirt_s2).
before(iron_shirt_s2, iron_shirt_s3).
before(iron_shirt_s3, iron_shirt_s4).
before(iron_shirt_s4, iron_shirt_s5).
before(iron_shirt_s5, iron_shirt_s6).
```

## aliases
```datalog
task_alias(iron_shirt, "ironing a shirt").
task_alias(iron_shirt, "iron a button-up shirt").
alias(shirt, "button-up shirt").
alias(shirt, "body").
alias(sleeve, "sleeves").
alias(sleeve, "each sleeve").
alias(cotton, "cotton setting").
alias(front, "front and back").
alias(fill, "fill with water").
alias(stays_smooth, "stays crease-free").
alias(stays_smooth, "no creases").
```

## teach
- To iron a shirt you need the shirt, an iron, an ironing board, water and a hanger.
- Ironing a shirt takes 10 to 15 minutes.
- Step 1 of ironing a shirt: fill the iron with water.
- Step 2 of ironing a shirt: heat the iron on the cotton setting for 2 to 3 minutes.
- Step 3 of ironing a shirt: iron the collar on the ironing board for 1 to 2 minutes.
- Step 4 of ironing a shirt: iron each sleeve on the ironing board for 3 to 4 minutes.
- Step 5 of ironing a shirt: iron the front and back of the shirt on the ironing board for 3 to 5 minutes.
- Step 6 of ironing a shirt: hang the shirt on the hanger so it stays crease-free.

## checklist
- names what is needed, including an iron and an ironing board
- gives the steps in order: fill with water, heat, collar, sleeves, body, hang
- gives a total time of about 10 to 15 minutes
- closes politely
