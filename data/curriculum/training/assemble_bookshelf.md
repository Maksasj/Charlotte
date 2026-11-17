---
id: assemble_bookshelf
stage: tools
task: assemble_bookshelf
---
# Assemble a flat-pack bookshelf

## requests
- I just bought a flat-pack bookshelf. What's the best order to put it together?
- Can you walk me through building a bookcase from the box?
- How do I assemble a flat-pack shelf without it wobbling?

## situation
- I have the bookshelf parts, an allen key, a screwdriver, a hammer and the wall bracket from the box.
- I have the afternoon.

## ground truth
```datalog
task(assemble_bookshelf).
needs(assemble_bookshelf, allen_key).
needs(assemble_bookshelf, screwdriver).
needs(assemble_bookshelf, hammer).
needs(assemble_bookshelf, wall_bracket).
takes_time(assemble_bookshelf, 45, 70, minute).
step(assemble_bookshelf, assemble_bookshelf_s1).
action(assemble_bookshelf_s1, unpack).
object_of(assemble_bookshelf_s1, part).
takes_time(assemble_bookshelf_s1, 5, 10, minute).
step(assemble_bookshelf, assemble_bookshelf_s2).
action(assemble_bookshelf_s2, attach).
object_of(assemble_bookshelf_s2, side_panel).
uses(assemble_bookshelf_s2, screwdriver).
takes_time(assemble_bookshelf_s2, 10, 15, minute).
step(assemble_bookshelf, assemble_bookshelf_s3).
action(assemble_bookshelf_s3, insert).
object_of(assemble_bookshelf_s3, shelf).
uses(assemble_bookshelf_s3, allen_key).
takes_time(assemble_bookshelf_s3, 10, 15, minute).
step(assemble_bookshelf, assemble_bookshelf_s4).
action(assemble_bookshelf_s4, nail).
object_of(assemble_bookshelf_s4, back_panel).
uses(assemble_bookshelf_s4, hammer).
takes_time(assemble_bookshelf_s4, 10, 15, minute).
step(assemble_bookshelf, assemble_bookshelf_s5).
action(assemble_bookshelf_s5, anchor).
object_of(assemble_bookshelf_s5, bookshelf).
uses(assemble_bookshelf_s5, wall_bracket).
uses(assemble_bookshelf_s5, screwdriver).
takes_time(assemble_bookshelf_s5, 5, 10, minute).
before(assemble_bookshelf_s1, assemble_bookshelf_s2).
before(assemble_bookshelf_s2, assemble_bookshelf_s3).
before(assemble_bookshelf_s3, assemble_bookshelf_s4).
before(assemble_bookshelf_s4, assemble_bookshelf_s5).
```

## teach
- To assemble a flat-pack bookshelf you need an allen key, a screwdriver, a hammer and a wall bracket.
- Assembling a flat-pack bookshelf takes 45 to 70 minutes.
- Step 1 of assembling a flat-pack bookshelf: unpack the parts and check them against the parts list for 5 to 10 minutes.
- Step 2 of assembling a flat-pack bookshelf: attach the side panels to the base with the screws and the screwdriver for 10 to 15 minutes.
- Step 3 of assembling a flat-pack bookshelf: slide in the shelves and lock them with the allen key for 10 to 15 minutes.
- Step 4 of assembling a flat-pack bookshelf: nail the back panel on with the hammer to keep the frame square for 10 to 15 minutes.
- Step 5 of assembling a flat-pack bookshelf: screw the bookshelf to the wall with the wall bracket and the screwdriver so it cannot tip for 5 to 10 minutes.

## checklist
- names what is needed: an allen key, a screwdriver, a hammer and a wall bracket
- gives the steps in order: unpack part, attach side panel, insert shelf, nail back panel, anchor bookshelf
- gives a total time of about 45 to 70 minutes
- closes politely
