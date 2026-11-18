---
id: write_thank_you_note
stage: social
task: write_thank_you_note
schema: v3
---
# Write a thank-you note

## requests
- My aunt sent me a lovely scarf for my birthday. How do I write her a proper thank-you card?
- Could you help me write a thank-you note for a wedding gift?
- I want to send a handwritten thank-you to my neighbour for watering my plants. What should I write?

## situation
- I have a blank card, an envelope, a pen and a stamp.
- I've got about 15 minutes.

## ground truth
```datalog
task(write_thank_you_note).
domain(write_thank_you_note, crafts).
goal(write_thank_you_note, thank_you_note).
needs(write_thank_you_note, card).
needs(write_thank_you_note, pen).
needs(write_thank_you_note, envelope).
needs(write_thank_you_note, stamp).
takes_time(write_thank_you_note, 10, 15, minute).
step(write_thank_you_note, write_thank_you_note_s1).
action(write_thank_you_note_s1, write).
object_of(write_thank_you_note_s1, greeting).
uses(write_thank_you_note_s1, card).
role(write_thank_you_note_s1, card, hold).
uses(write_thank_you_note_s1, pen).
role(write_thank_you_note_s1, pen, write).
step(write_thank_you_note, write_thank_you_note_s2).
action(write_thank_you_note_s2, thank).
object_of(write_thank_you_note_s2, gift).
step(write_thank_you_note, write_thank_you_note_s3).
action(write_thank_you_note_s3, describe).
object_of(write_thank_you_note_s3, gift).
step(write_thank_you_note, write_thank_you_note_s4).
action(write_thank_you_note_s4, sign).
object_of(write_thank_you_note_s4, card).
uses(write_thank_you_note_s4, pen).
role(write_thank_you_note_s4, pen, write).
step(write_thank_you_note, write_thank_you_note_s5).
action(write_thank_you_note_s5, seal).
object_of(write_thank_you_note_s5, envelope).
step(write_thank_you_note, write_thank_you_note_s6).
action(write_thank_you_note_s6, stick).
object_of(write_thank_you_note_s6, stamp).
onto(write_thank_you_note_s6, envelope).
before(write_thank_you_note_s1, write_thank_you_note_s2).
before(write_thank_you_note_s2, write_thank_you_note_s3).
before(write_thank_you_note_s3, write_thank_you_note_s4).
before(write_thank_you_note_s4, write_thank_you_note_s5).
before(write_thank_you_note_s5, write_thank_you_note_s6).
```

## aliases
```datalog
task_alias(write_thank_you_note, "writing a thank-you note").
task_alias(write_thank_you_note, "thank-you card").
task_alias(write_thank_you_note, "handwritten thank-you").
alias(card, "blank card").
alias(card, "thank-you card").
alias(gift, "present").
alias(greeting, "greeting with the person's name").
alias(sign, "signature").
alias(thank, "thanks for the gift").
```

## teach
- To write a thank-you note you need a card, a pen, an envelope and a stamp.
- Writing a thank-you note takes 10 to 15 minutes.
- Step 1 of writing a thank-you note: write a greeting with the person's name on the card with the pen.
- Step 2 of writing a thank-you note: thank them for the gift by name.
- Step 3 of writing a thank-you note: describe how you will use or enjoy the gift.
- Step 4 of writing a thank-you note: sign the card with the pen.
- Step 5 of writing a thank-you note: seal the envelope.
- Step 6 of writing a thank-you note: stick the stamp on the envelope.

## checklist
- gives the parts in order: greeting, thanks for the gift, how you will use it, signature
- suggests naming the gift specifically
- mentions sealing the envelope and adding a stamp
- closes politely
