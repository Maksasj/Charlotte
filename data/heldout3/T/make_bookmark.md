---
id: make_bookmark
stage: tools
task: make_bookmark
schema: v3
tier: T
---
# Make a bookmark

## requests
- How can I make a bookmark for my book club friends?
- I keep folding page corners. Can you help me craft a proper bookmark?
- What's a fun way for kids to make their own bookmarks?

## situation
- I have card stock, a ruler, scissors, coloured markers, clear tape, a hole punch and ribbon.

## ground truth
```datalog
task(make_bookmark).
domain(make_bookmark, crafts).
goal(make_bookmark, bookmark).
goal(make_bookmark, gift).
needs(make_bookmark, card_stock).
needs(make_bookmark, ruler).
needs(make_bookmark, scissors).
needs(make_bookmark, marker).
needs(make_bookmark, clear_tape).
needs(make_bookmark, hole_punch).
needs(make_bookmark, ribbon).
takes_time(make_bookmark, 20, 30, minute).
step(make_bookmark, make_bookmark_s1).
action(make_bookmark_s1, cut).
object_of(make_bookmark_s1, card_stock).
uses(make_bookmark_s1, scissors).
role(make_bookmark_s1, scissors, cut).
uses(make_bookmark_s1, ruler).
role(make_bookmark_s1, ruler, measure).
setting(make_bookmark_s1, size, 5_by_15, centimetre).
step(make_bookmark, make_bookmark_s2).
action(make_bookmark_s2, decorate).
object_of(make_bookmark_s2, card_stock).
uses(make_bookmark_s2, marker).
role(make_bookmark_s2, marker, decorate).
step(make_bookmark, make_bookmark_s3).
action(make_bookmark_s3, cover).
object_of(make_bookmark_s3, card_stock).
uses(make_bookmark_s3, clear_tape).
role(make_bookmark_s3, clear_tape, cover).
purpose(make_bookmark_s3, lasts_longer).
step(make_bookmark, make_bookmark_s4).
action(make_bookmark_s4, punch).
object_of(make_bookmark_s4, hole).
uses(make_bookmark_s4, hole_punch).
role(make_bookmark_s4, hole_punch, pierce).
with_part(make_bookmark_s4, top).
step(make_bookmark, make_bookmark_s5).
action(make_bookmark_s5, tie).
object_of(make_bookmark_s5, ribbon).
into(make_bookmark_s5, hole).
before(make_bookmark_s1, make_bookmark_s2).
before(make_bookmark_s2, make_bookmark_s3).
before(make_bookmark_s3, make_bookmark_s4).
before(make_bookmark_s4, make_bookmark_s5).
```

## aliases
```datalog
task_alias(make_bookmark, "making a bookmark").
task_alias(make_bookmark, "a homemade bookmark").
task_alias(make_bookmark, "crafting bookmarks").
alias(card_stock, "card").
alias(marker, "coloured markers").
alias(clear_tape, "sticky tape").
alias(ribbon, "a ribbon tassel").
alias(lasts_longer, "it lasts longer").
```

## teach
- To make a bookmark you need card stock, a ruler, scissors, markers, clear tape, a hole punch and ribbon.
- Making a bookmark takes 20 to 30 minutes.
- Step 1 of making a bookmark: cut a strip of card 5 by 15 centimetres, measuring with the ruler and cutting with the scissors.
- Step 2 of making a bookmark: decorate the card with the markers.
- Step 3 of making a bookmark: cover the card with clear tape so it lasts longer.
- Step 4 of making a bookmark: punch a hole at the top with the hole punch.
- Step 5 of making a bookmark: tie the ribbon through the hole.

## checklist
- names what is needed, including card, scissors, markers and ribbon
- gives the steps in order: cut, decorate, cover with tape, punch hole, tie ribbon

## variant
- change: no hole punch: skip the hole and the ribbon
- request: I don't own a hole punch. Can I still make a bookmark?
- situation: I have card stock, a ruler, scissors, markers and clear tape.
- expect: The reply leaves out the hole and the ribbon tassel, so the bookmark is a plain decorated strip.

```datalog situation
has(user, card_stock).
has(user, ruler).
has(user, scissors).
has(user, marker).
has(user, clear_tape).
lacks(user, hole_punch).
```

```datalog difference
drop(make_bookmark_s4).
drop(make_bookmark_s5).
no_need(hole_punch).
no_need(ribbon).
```

## variant
- change: wants a pressed flower on it: glue one on before taping
- request: I've got some pressed flowers from the garden. Can I put one on the bookmark?
- situation: I have everything for the bookmark plus pressed flowers and a glue stick.
- expect: The reply adds gluing a pressed flower onto the card before covering it with tape, which also seals the flower in.

```datalog situation
has(user, card_stock).
has(user, ruler).
has(user, scissors).
has(user, marker).
has(user, clear_tape).
has(user, hole_punch).
has(user, ribbon).
has(user, pressed_flower).
has(user, glue_stick).
wants(user, pressed_flower).
```

```datalog difference
extra_need(pressed_flower).
extra_need(glue_stick).
add_step(make_bookmark_s9, make_bookmark_s2).
step(make_bookmark, make_bookmark_s9).
action(make_bookmark_s9, glue).
object_of(make_bookmark_s9, pressed_flower).
onto(make_bookmark_s9, card_stock).
uses(make_bookmark_s9, glue_stick).
role(make_bookmark_s9, glue_stick, fasten).
before(make_bookmark_s2, make_bookmark_s9).
before(make_bookmark_s9, make_bookmark_s3).
```
