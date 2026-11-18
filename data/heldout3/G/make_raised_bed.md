---
id: make_raised_bed
stage: household
task: make_raised_bed
schema: v3
tier: G
---
# Make a raised bed

## requests
- My soil is heavy clay and nothing grows. How do I build a raised bed to grow veg in?
- I've bought a wooden raised bed kit. How do I put it together and fill it?
- What's the way to set up a raised vegetable bed on top of the lawn?

## situation
- I have a raised bed kit with four boards and screws, a screwdriver, a spirit level, flattened cardboard boxes, 300 litres of topsoil, compost and a rake.

## ground truth
```datalog
task(make_raised_bed).
domain(make_raised_bed, gardening).
goal(make_raised_bed, raised_bed).
needs(make_raised_bed, board).
needs(make_raised_bed, screw).
needs(make_raised_bed, screwdriver).
needs(make_raised_bed, spirit_level).
needs(make_raised_bed, cardboard).
needs(make_raised_bed, topsoil).
needs(make_raised_bed, compost).
needs(make_raised_bed, rake).
takes_time(make_raised_bed, 90, 120, minute).
step(make_raised_bed, make_raised_bed_s1).
action(make_raised_bed_s1, fasten).
object_of(make_raised_bed_s1, board).
with_part(make_raised_bed_s1, corner).
uses(make_raised_bed_s1, screw).
role(make_raised_bed_s1, screw, fasten).
uses(make_raised_bed_s1, screwdriver).
role(make_raised_bed_s1, screwdriver, fasten).
takes_time(make_raised_bed_s1, 20, 30, minute).
step(make_raised_bed, make_raised_bed_s2).
action(make_raised_bed_s2, level).
object_of(make_raised_bed_s2, frame).
uses(make_raised_bed_s2, spirit_level).
role(make_raised_bed_s2, spirit_level, measure).
step(make_raised_bed, make_raised_bed_s3).
action(make_raised_bed_s3, lay).
object_of(make_raised_bed_s3, cardboard).
into(make_raised_bed_s3, frame).
purpose(make_raised_bed_s3, smother_weeds).
step(make_raised_bed, make_raised_bed_s4).
action(make_raised_bed_s4, fill).
object_of(make_raised_bed_s4, frame).
uses(make_raised_bed_s4, topsoil).
role(make_raised_bed_s4, topsoil, fill).
amount(make_raised_bed_s4, topsoil, 300, litre).
takes_time(make_raised_bed_s4, 30, 45, minute).
step(make_raised_bed, make_raised_bed_s5).
action(make_raised_bed_s5, mix).
object_of(make_raised_bed_s5, compost).
into(make_raised_bed_s5, topsoil).
uses(make_raised_bed_s5, rake).
role(make_raised_bed_s5, rake, mix).
before(make_raised_bed_s1, make_raised_bed_s2).
before(make_raised_bed_s2, make_raised_bed_s3).
before(make_raised_bed_s3, make_raised_bed_s4).
before(make_raised_bed_s4, make_raised_bed_s5).
```

## aliases
```datalog
task_alias(make_raised_bed, "making a raised bed").
task_alias(make_raised_bed, "build a raised bed").
task_alias(make_raised_bed, "set up a raised vegetable bed").
alias(board, "boards").
alias(screw, "screws").
alias(frame, "the frame").
alias(cardboard, "cardboard boxes").
alias(corner, "corners").
alias(smother_weeds, "smothers the grass and weeds").
alias(raised_bed, "raised beds").
```

## teach
- To make a raised bed you need the boards and screws, a screwdriver, a spirit level, cardboard, 300 litres of topsoil, compost and a rake.
- Making a raised bed takes 90 to 120 minutes.
- Step 1 of making a raised bed: fasten the boards together at the corners with the screws and screwdriver, for 20 to 30 minutes.
- Step 2 of making a raised bed: level the frame where it will stand, checking with the spirit level.
- Step 3 of making a raised bed: lay cardboard in the bottom of the frame to smother the grass and weeds.
- Step 4 of making a raised bed: fill the frame with the 300 litres of topsoil, for 30 to 45 minutes.
- Step 5 of making a raised bed: mix the compost into the topsoil with the rake.

## checklist
- names what is needed: boards, screws, a screwdriver, a spirit level, cardboard, topsoil, compost and a rake
- gives the steps in order: screw frame, level, cardboard, fill, mix in compost
- says how much topsoil is needed
- gives a total time of about 90 to 120 minutes

## variant
- change: two beds: twice the topsoil and about twice as long
- request: I'm putting in two raised beds side by side this weekend. How much soil will I need, and what's the process?
- situation: I have two raised bed kits, a screwdriver, a spirit level, cardboard, 600 litres of topsoil, compost and a rake.
- expect: The reply repeats the steps for each bed, says 600 litres of topsoil in total, and gives about three to four hours.

```datalog situation
has(user, board).
has(user, screw).
has(user, screwdriver).
has(user, spirit_level).
has(user, cardboard).
has(user, topsoil).
has(user, compost).
has(user, rake).
count(user, raised_bed, 2).
```

```datalog difference
change(make_raised_bed_s4, amount, 300, 600).
change(make_raised_bed, takes_time, 90_120, 180_240).
```

## variant
- change: no cardboard: put the soil straight on the ground
- request: Do I have to line the bottom of my raised bed with cardboard? I've recycled all my boxes.
- situation: I have the raised bed kit, a screwdriver, a spirit level, topsoil, compost and a rake, but no cardboard.
- expect: The reply leaves out the cardboard step and fills the bed straight onto the ground.

```datalog situation
has(user, board).
has(user, screw).
has(user, screwdriver).
has(user, spirit_level).
has(user, topsoil).
has(user, compost).
has(user, rake).
lacks(user, cardboard).
```

```datalog difference
no_need(cardboard).
drop(make_raised_bed_s3).
```
