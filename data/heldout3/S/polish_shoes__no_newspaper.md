---
id: polish_shoes__no_newspaper
stage: household
task: polish_shoes
schema: v3
tier: S
base: polish_shoes
---
# Polish shoes

## requests
- I want to shine my leather shoes for a wedding but we don't get a paper any more, so there's nothing to put down.

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

## variant
- change: no newspaper: skip laying the shoes on newspaper
- request: I want to shine my leather shoes for a wedding but we don't get a paper any more, so there's nothing to put down.
- situation: I have leather shoes, shoe polish, a shoe brush and a soft cloth, but no newspaper.
- expect: The reply leaves out the newspaper step and keeps removing laces, brushing, polishing, drying and buffing.

```datalog situation
has(user, shoe_polish).
has(user, brush).
has(user, cloth).
lacks(user, newspaper).
```

```datalog difference
drop(polish_shoes_s2).
no_need(newspaper).
```
