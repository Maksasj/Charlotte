---
id: assemble_bookshelf__no_hammer
stage: tools
task: assemble_bookshelf
schema: v3
tier: S
base: assemble_bookshelf
---
# Assemble a flat-pack bookshelf

## requests
- Putting together a flat-pack bookcase and there's no hammer for the back panel nails. I do have a rubber mallet.

## ground truth
```datalog
task(assemble_bookshelf).
domain(assemble_bookshelf, repair).
goal(assemble_bookshelf, assembled_bookshelf).
needs(assemble_bookshelf, allen_key).
needs(assemble_bookshelf, screwdriver).
needs(assemble_bookshelf, hammer).
needs(assemble_bookshelf, wall_bracket).
takes_time(assemble_bookshelf, 45, 70, minute).
step(assemble_bookshelf, assemble_bookshelf_s1).
action(assemble_bookshelf_s1, unpack).
object_of(assemble_bookshelf_s1, part).
step(assemble_bookshelf, assemble_bookshelf_s2).
action(assemble_bookshelf_s2, check).
object_of(assemble_bookshelf_s2, part).
takes_time(assemble_bookshelf_s2, 5, 10, minute).
step(assemble_bookshelf, assemble_bookshelf_s3).
action(assemble_bookshelf_s3, attach).
object_of(assemble_bookshelf_s3, side_panel).
uses(assemble_bookshelf_s3, screwdriver).
role(assemble_bookshelf_s3, screwdriver, fasten).
onto(assemble_bookshelf_s3, base).
takes_time(assemble_bookshelf_s3, 10, 15, minute).
step(assemble_bookshelf, assemble_bookshelf_s4).
action(assemble_bookshelf_s4, insert).
object_of(assemble_bookshelf_s4, shelf).
step(assemble_bookshelf, assemble_bookshelf_s5).
action(assemble_bookshelf_s5, lock).
object_of(assemble_bookshelf_s5, shelf).
uses(assemble_bookshelf_s5, allen_key).
role(assemble_bookshelf_s5, allen_key, fasten).
takes_time(assemble_bookshelf_s5, 10, 15, minute).
step(assemble_bookshelf, assemble_bookshelf_s6).
action(assemble_bookshelf_s6, nail).
object_of(assemble_bookshelf_s6, back_panel).
uses(assemble_bookshelf_s6, hammer).
role(assemble_bookshelf_s6, hammer, strike).
purpose(assemble_bookshelf_s6, frame_square).
takes_time(assemble_bookshelf_s6, 10, 15, minute).
step(assemble_bookshelf, assemble_bookshelf_s7).
action(assemble_bookshelf_s7, anchor).
object_of(assemble_bookshelf_s7, bookshelf).
uses(assemble_bookshelf_s7, wall_bracket).
role(assemble_bookshelf_s7, wall_bracket, fasten).
uses(assemble_bookshelf_s7, screwdriver).
role(assemble_bookshelf_s7, screwdriver, fasten).
onto(assemble_bookshelf_s7, wall).
purpose(assemble_bookshelf_s7, cannot_tip).
takes_time(assemble_bookshelf_s7, 5, 10, minute).
before(assemble_bookshelf_s1, assemble_bookshelf_s2).
before(assemble_bookshelf_s2, assemble_bookshelf_s3).
before(assemble_bookshelf_s3, assemble_bookshelf_s4).
before(assemble_bookshelf_s4, assemble_bookshelf_s5).
before(assemble_bookshelf_s5, assemble_bookshelf_s6).
before(assemble_bookshelf_s6, assemble_bookshelf_s7).
```

## aliases
```datalog
task_alias(assemble_bookshelf, "assembling a flat-pack bookshelf").
task_alias(assemble_bookshelf, "flat-pack bookshelf").
task_alias(assemble_bookshelf, "building a bookcase").
task_alias(assemble_bookshelf, "assemble a flat-pack shelf").
alias(part, "parts").
alias(side_panel, "side panels").
alias(shelf, "shelves").
alias(bookshelf, "bookcase").
alias(bookshelf, "flat-pack shelf").
alias(wall_bracket, "bracket").
alias(insert, "slide in").
alias(anchor, "screw to the wall").
alias(attach, "screw on").
alias(frame_square, "keep the frame square").
alias(cannot_tip, "it cannot tip").
```

## teach
- To assemble a flat-pack bookshelf you need an allen key, a screwdriver, a hammer and a wall bracket.
- Assembling a flat-pack bookshelf takes 45 to 70 minutes.
- Step 1 of assembling a flat-pack bookshelf: unpack the parts.
- Step 2 of assembling a flat-pack bookshelf: check the parts against the parts list for 5 to 10 minutes.
- Step 3 of assembling a flat-pack bookshelf: attach the side panels to the base with the screws and the screwdriver for 10 to 15 minutes.
- Step 4 of assembling a flat-pack bookshelf: slide in the shelves.
- Step 5 of assembling a flat-pack bookshelf: lock the shelves with the allen key for 10 to 15 minutes.
- Step 6 of assembling a flat-pack bookshelf: nail the back panel on with the hammer to keep the frame square for 10 to 15 minutes.
- Step 7 of assembling a flat-pack bookshelf: screw the bookshelf to the wall with the wall bracket and the screwdriver so it cannot tip for 5 to 10 minutes.

## variant
- change: no hammer: tap the back panel nails in with a rubber mallet
- request: Putting together a flat-pack bookcase and there's no hammer for the back panel nails. I do have a rubber mallet.
- situation: I have the bookshelf parts, an allen key, a screwdriver, the wall bracket and a rubber mallet, but no hammer.
- expect: The reply nails the back panel with the rubber mallet instead of a hammer and keeps the other steps.

```datalog situation
has(user, part).
has(user, allen_key).
has(user, screwdriver).
has(user, wall_bracket).
has(user, rubber_mallet).
lacks(user, hammer).
```

```datalog difference
swap(assemble_bookshelf_s6, hammer, rubber_mallet).
no_need(hammer).
extra_need(rubber_mallet).
```
