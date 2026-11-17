---
id: iron_shirt
stage: household
task: iron_shirt
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
step(iron_shirt, iron_shirt_s2).
action(iron_shirt_s2, heat).
object_of(iron_shirt_s2, iron).
takes_time(iron_shirt_s2, 2, 3, minute).
step(iron_shirt, iron_shirt_s3).
action(iron_shirt_s3, iron).
object_of(iron_shirt_s3, collar).
uses(iron_shirt_s3, ironing_board).
takes_time(iron_shirt_s3, 1, 2, minute).
step(iron_shirt, iron_shirt_s4).
action(iron_shirt_s4, iron).
object_of(iron_shirt_s4, sleeve).
uses(iron_shirt_s4, ironing_board).
takes_time(iron_shirt_s4, 3, 4, minute).
step(iron_shirt, iron_shirt_s5).
action(iron_shirt_s5, iron).
object_of(iron_shirt_s5, shirt).
uses(iron_shirt_s5, ironing_board).
takes_time(iron_shirt_s5, 3, 5, minute).
step(iron_shirt, iron_shirt_s6).
action(iron_shirt_s6, hang).
object_of(iron_shirt_s6, shirt).
uses(iron_shirt_s6, hanger).
before(iron_shirt_s1, iron_shirt_s2).
before(iron_shirt_s2, iron_shirt_s3).
before(iron_shirt_s3, iron_shirt_s4).
before(iron_shirt_s4, iron_shirt_s5).
before(iron_shirt_s5, iron_shirt_s6).
```

## teach
- To iron a shirt you need the shirt, an iron, an ironing board, water and a hanger.
- Ironing a shirt takes 10 to 15 minutes.
- Step 1 of ironing a shirt: fill the iron with water.
- Step 2 of ironing a shirt: heat the iron on the cotton setting for 2 to 3 minutes.
- Step 3 of ironing a shirt: iron the collar on the ironing board for 1 to 2 minutes.
- Step 4 of ironing a shirt: iron each sleeve on the ironing board for 3 to 4 minutes.
- Step 5 of ironing a shirt: iron the front and back of the shirt on the ironing board for 3 to 5 minutes.
- Step 6 of ironing a shirt: hang the shirt on the hanger.

## checklist
- names what is needed, including an iron and an ironing board
- gives the steps in order: fill with water, heat, collar, sleeves, body, hang
- gives a total time of about 10 to 15 minutes
- closes politely
