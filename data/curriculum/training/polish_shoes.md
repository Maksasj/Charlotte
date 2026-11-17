---
id: polish_shoes
stage: household
task: polish_shoes
---
# Polish shoes

## requests
- I have an interview tomorrow and my leather shoes look scuffed. How do I polish them?
- Can you show me how to shine leather shoes properly?
- What's the right way to use shoe polish and a brush?

## situation
- I have leather shoes, shoe polish, a shoe brush, a soft cloth and some newspaper.
- I have about half an hour tonight.

## ground truth
```datalog
task(polish_shoes).
needs(polish_shoes, shoe_polish).
needs(polish_shoes, brush).
needs(polish_shoes, cloth).
needs(polish_shoes, newspaper).
takes_time(polish_shoes, 15, 25, minute).
step(polish_shoes, polish_shoes_s1).
action(polish_shoes_s1, remove).
object_of(polish_shoes_s1, lace).
uses(polish_shoes_s1, newspaper).
takes_time(polish_shoes_s1, 1, 2, minute).
step(polish_shoes, polish_shoes_s2).
action(polish_shoes_s2, brush).
object_of(polish_shoes_s2, shoe).
uses(polish_shoes_s2, brush).
takes_time(polish_shoes_s2, 2, 3, minute).
step(polish_shoes, polish_shoes_s3).
action(polish_shoes_s3, apply).
object_of(polish_shoes_s3, shoe_polish).
uses(polish_shoes_s3, cloth).
takes_time(polish_shoes_s3, 3, 5, minute).
step(polish_shoes, polish_shoes_s4).
action(polish_shoes_s4, dry).
object_of(polish_shoes_s4, shoe_polish).
takes_time(polish_shoes_s4, 5, 10, minute).
step(polish_shoes, polish_shoes_s5).
action(polish_shoes_s5, buff).
object_of(polish_shoes_s5, shoe).
uses(polish_shoes_s5, brush).
until(polish_shoes_s5, shiny).
takes_time(polish_shoes_s5, 3, 5, minute).
before(polish_shoes_s1, polish_shoes_s2).
before(polish_shoes_s2, polish_shoes_s3).
before(polish_shoes_s3, polish_shoes_s4).
before(polish_shoes_s4, polish_shoes_s5).
```

## teach
- To polish shoes you need shoe polish, a shoe brush, a soft cloth and newspaper.
- Polishing shoes takes 15 to 25 minutes.
- Step 1 of polishing shoes: take the laces out and set the shoes on the newspaper for 1 to 2 minutes.
- Step 2 of polishing shoes: brush the dirt off the shoes with the shoe brush for 2 to 3 minutes.
- Step 3 of polishing shoes: rub a little shoe polish into the leather in small circles with the cloth for 3 to 5 minutes.
- Step 4 of polishing shoes: let the polish dry for 5 to 10 minutes.
- Step 5 of polishing shoes: buff the shoes with the brush until they shine, for 3 to 5 minutes.

## checklist
- names what is needed: shoe polish, a shoe brush, a soft cloth and newspaper
- gives the steps in order: remove lace, brush shoe, apply shoe polish, dry shoe polish, buff shoe
- gives a total time of about 15 to 25 minutes
- closes politely

## variant
- change: suede shoes: no polish, brush the nap with a suede brush and lift marks with a suede eraser
- request: My good shoes are suede, not smooth leather. How do I clean them up for tomorrow?
- situation: I have suede shoes, a suede brush, a suede eraser and some newspaper.
- expect: The reply leaves out the polish and drying steps, brushes the suede in one direction with the suede brush and rubs marks out with the eraser.
