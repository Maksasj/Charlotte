---
id: polish_shoes
stage: household
task: polish_shoes
schema: v3
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
domain(polish_shoes, cleaning).
goal(polish_shoes, shiny_shoe).
needs(polish_shoes, shoe_polish).
needs(polish_shoes, brush).
needs(polish_shoes, cloth).
needs(polish_shoes, newspaper).
takes_time(polish_shoes, 15, 25, minute).
step(polish_shoes, polish_shoes_s1).
action(polish_shoes_s1, remove).
object_of(polish_shoes_s1, lace).
from(polish_shoes_s1, shoe).
step(polish_shoes, polish_shoes_s2).
action(polish_shoes_s2, set).
object_of(polish_shoes_s2, shoe).
uses(polish_shoes_s2, newspaper).
role(polish_shoes_s2, newspaper, protect).
onto(polish_shoes_s2, newspaper).
takes_time(polish_shoes_s2, 1, 2, minute).
step(polish_shoes, polish_shoes_s3).
action(polish_shoes_s3, brush).
object_of(polish_shoes_s3, shoe).
uses(polish_shoes_s3, brush).
role(polish_shoes_s3, brush, clean).
takes_time(polish_shoes_s3, 2, 3, minute).
step(polish_shoes, polish_shoes_s4).
action(polish_shoes_s4, apply).
object_of(polish_shoes_s4, shoe_polish).
uses(polish_shoes_s4, cloth).
role(polish_shoes_s4, cloth, spread).
into(polish_shoes_s4, leather).
takes_time(polish_shoes_s4, 3, 5, minute).
step(polish_shoes, polish_shoes_s5).
action(polish_shoes_s5, dry).
object_of(polish_shoes_s5, shoe_polish).
takes_time(polish_shoes_s5, 5, 10, minute).
step(polish_shoes, polish_shoes_s6).
action(polish_shoes_s6, buff).
object_of(polish_shoes_s6, shoe).
uses(polish_shoes_s6, brush).
role(polish_shoes_s6, brush, polish).
until(polish_shoes_s6, shiny).
takes_time(polish_shoes_s6, 3, 5, minute).
before(polish_shoes_s1, polish_shoes_s2).
before(polish_shoes_s2, polish_shoes_s3).
before(polish_shoes_s3, polish_shoes_s4).
before(polish_shoes_s4, polish_shoes_s5).
before(polish_shoes_s5, polish_shoes_s6).
```

## aliases
```datalog
task_alias(polish_shoes, "polishing shoes").
task_alias(polish_shoes, "shine leather shoes").
task_alias(polish_shoes, "polish them").
alias(shoe, "shoes").
alias(shoe, "leather shoes").
alias(lace, "laces").
alias(remove, "take out").
alias(brush, "shoe brush").
alias(cloth, "soft cloth").
alias(shoe_polish, "polish").
alias(shiny, "they shine").
alias(suede_brush, "suede brush").
alias(suede_eraser, "suede eraser").
alias(suede_eraser, "eraser").
alias(suede_shoe, "suede shoes").
alias(mark, "marks").
```

## teach
- To polish shoes you need shoe polish, a shoe brush, a soft cloth and newspaper.
- Polishing shoes takes 15 to 25 minutes.
- Step 1 of polishing shoes: take the laces out.
- Step 2 of polishing shoes: set the shoes on the newspaper for 1 to 2 minutes.
- Step 3 of polishing shoes: brush the dirt off the shoes with the shoe brush for 2 to 3 minutes.
- Step 4 of polishing shoes: rub a little shoe polish into the leather in small circles with the cloth for 3 to 5 minutes.
- Step 5 of polishing shoes: let the polish dry for 5 to 10 minutes.
- Step 6 of polishing shoes: buff the shoes with the brush until they shine, for 3 to 5 minutes.

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

```datalog situation
has(user, suede_shoe).
has(user, suede_brush).
has(user, suede_eraser).
has(user, newspaper).
```

```datalog difference
drop(polish_shoes_s4).
drop(polish_shoes_s5).
drop(polish_shoes_s6).
no_need(shoe_polish).
no_need(cloth).
no_need(brush).
extra_need(suede_brush).
extra_need(suede_eraser).
swap(polish_shoes_s3, brush, suede_brush).
add_step(polish_shoes_s9, polish_shoes_s3).
step(polish_shoes, polish_shoes_s9).
action(polish_shoes_s9, rub).
object_of(polish_shoes_s9, mark).
uses(polish_shoes_s9, suede_eraser).
role(polish_shoes_s9, suede_eraser, clean).
before(polish_shoes_s3, polish_shoes_s9).
```
