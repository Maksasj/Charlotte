---
id: make_greeting_card
stage: tools
task: make_greeting_card
schema: v3
tier: T
---
# Make a handmade greeting card

## requests
- I'd like to make my mum a card by hand this year. How should I go about it?
- Can you show me how to craft a simple greeting card at home?
- Shop-bought cards feel boring. How do I make my own card for a friend's birthday?

## situation
- I have a sheet of card stock, scissors, an old magazine, a glue stick, glitter and a pen.

## ground truth
```datalog
task(make_greeting_card).
domain(make_greeting_card, crafts).
goal(make_greeting_card, greeting_card).
goal(make_greeting_card, gift).
needs(make_greeting_card, card_stock).
needs(make_greeting_card, scissors).
needs(make_greeting_card, magazine).
needs(make_greeting_card, glue_stick).
needs(make_greeting_card, glitter).
needs(make_greeting_card, pen).
takes_time(make_greeting_card, 30, 45, minute).
step(make_greeting_card, make_greeting_card_s1).
action(make_greeting_card_s1, fold).
object_of(make_greeting_card_s1, card_stock).
check(make_greeting_card_s1, sharp_crease).
step(make_greeting_card, make_greeting_card_s2).
action(make_greeting_card_s2, cut).
object_of(make_greeting_card_s2, picture).
from(make_greeting_card_s2, magazine).
uses(make_greeting_card_s2, scissors).
role(make_greeting_card_s2, scissors, cut).
step(make_greeting_card, make_greeting_card_s3).
action(make_greeting_card_s3, glue).
object_of(make_greeting_card_s3, picture).
onto(make_greeting_card_s3, card_stock).
uses(make_greeting_card_s3, glue_stick).
role(make_greeting_card_s3, glue_stick, fasten).
step(make_greeting_card, make_greeting_card_s4).
action(make_greeting_card_s4, decorate).
object_of(make_greeting_card_s4, card_stock).
uses(make_greeting_card_s4, glitter).
role(make_greeting_card_s4, glitter, decorate).
step(make_greeting_card, make_greeting_card_s5).
action(make_greeting_card_s5, write).
object_of(make_greeting_card_s5, message).
uses(make_greeting_card_s5, pen).
role(make_greeting_card_s5, pen, write).
step(make_greeting_card, make_greeting_card_s6).
action(make_greeting_card_s6, dry).
object_of(make_greeting_card_s6, card_stock).
takes_time(make_greeting_card_s6, 10, 15, minute).
before(make_greeting_card_s1, make_greeting_card_s2).
before(make_greeting_card_s2, make_greeting_card_s3).
before(make_greeting_card_s3, make_greeting_card_s4).
before(make_greeting_card_s4, make_greeting_card_s5).
before(make_greeting_card_s5, make_greeting_card_s6).
```

## aliases
```datalog
task_alias(make_greeting_card, "making a card").
task_alias(make_greeting_card, "a homemade card").
task_alias(make_greeting_card, "crafting a greeting card").
alias(card_stock, "card").
alias(picture, "a picture").
alias(glue_stick, "glue").
alias(message, "a message inside").
alias(sharp_crease, "a sharp crease").
```

## teach
- To make a handmade greeting card you need card stock, scissors, a magazine, a glue stick, glitter and a pen.
- Making a handmade greeting card takes 30 to 45 minutes.
- Step 1 of making a greeting card: fold the card stock in half with a sharp crease.
- Step 2 of making a greeting card: cut a picture from the magazine with the scissors.
- Step 3 of making a greeting card: glue the picture onto the card with the glue stick.
- Step 4 of making a greeting card: decorate the card with glitter.
- Step 5 of making a greeting card: write a message inside with the pen.
- Step 6 of making a greeting card: let the card dry for 10 to 15 minutes.

## checklist
- names what is needed, including card, scissors, glue and a pen
- gives the steps in order: fold, cut picture, glue, decorate, write, dry

## variant
- change: no glue stick: staple the picture on
- request: I've run out of glue. How else can I stick a picture onto my card?
- situation: I have card stock, scissors, a magazine, a stapler, glitter and a pen.
- expect: The reply fixes the picture onto the card with a stapler, a staple in each corner, instead of glue.

```datalog situation
has(user, card_stock).
has(user, scissors).
has(user, magazine).
has(user, stapler).
has(user, glitter).
has(user, pen).
lacks(user, glue_stick).
```

```datalog difference
swap(make_greeting_card_s3, glue_stick, stapler).
extra_need(stapler).
no_need(glue_stick).
```

## variant
- change: no glitter: leave the decoration out
- request: I have no glitter or stickers. Can I still make a nice card?
- situation: I have card stock, scissors, a magazine, a glue stick and a pen.
- expect: The reply skips the decorating step and keeps the picture and the message.

```datalog situation
has(user, card_stock).
has(user, scissors).
has(user, magazine).
has(user, glue_stick).
has(user, pen).
lacks(user, glitter).
```

```datalog difference
drop(make_greeting_card_s4).
no_need(glitter).
```
