---
id: buy_postage_stamps
stage: procedures
task: buy_postage_stamps
schema: v3
---
# Buy postage stamps

## requests
- I need to mail a letter. How do I get the stamps at the post office?
- What's the process for buying postage for an envelope?
- Could you tell me the steps to buy stamps and put one on my letter?

## situation
- I have a letter, an envelope, a kitchen scale and a debit card.
- I want to post it today.

## ground truth
```datalog
task(buy_postage_stamps).
domain(buy_postage_stamps, errands).
goal(buy_postage_stamps, stamps_bought).
needs(buy_postage_stamps, letter).
needs(buy_postage_stamps, envelope).
needs(buy_postage_stamps, scale).
needs(buy_postage_stamps, card).
takes_time(buy_postage_stamps, 10, 20, minute).
step(buy_postage_stamps, buy_postage_stamps_s1).
action(buy_postage_stamps_s1, weigh).
object_of(buy_postage_stamps_s1, letter).
uses(buy_postage_stamps_s1, scale).
role(buy_postage_stamps_s1, scale, measure).
purpose(buy_postage_stamps_s1, correct_postage).
step(buy_postage_stamps, buy_postage_stamps_s2).
action(buy_postage_stamps_s2, ask).
object_of(buy_postage_stamps_s2, clerk).
takes_time(buy_postage_stamps_s2, 5, 10, minute).
step(buy_postage_stamps, buy_postage_stamps_s3).
action(buy_postage_stamps_s3, pay).
object_of(buy_postage_stamps_s3, stamp).
uses(buy_postage_stamps_s3, card).
role(buy_postage_stamps_s3, card, pay).
step(buy_postage_stamps, buy_postage_stamps_s4).
action(buy_postage_stamps_s4, stick).
object_of(buy_postage_stamps_s4, stamp).
onto(buy_postage_stamps_s4, envelope).
uses(buy_postage_stamps_s4, envelope).
role(buy_postage_stamps_s4, envelope, hold).
before(buy_postage_stamps_s1, buy_postage_stamps_s2).
before(buy_postage_stamps_s2, buy_postage_stamps_s3).
before(buy_postage_stamps_s3, buy_postage_stamps_s4).
```

## aliases
```datalog
task_alias(buy_postage_stamps, "buy stamps").
task_alias(buy_postage_stamps, "buying postage").
alias(stamp, "postage stamp").
alias(clerk, "post office counter clerk").
alias(card, "debit card").
alias(correct_postage, "right postage").
alias(stamps_bought, "stamps bought").
```

## teach
- To buy postage stamps you need a letter, an envelope, a scale and a card.
- Buying postage stamps takes 10 to 20 minutes.
- Step 1 of buying postage stamps: weigh the letter on the scale so you know the right postage.
- Step 2 of buying postage stamps: ask the clerk for stamps of the right value, which takes 5 to 10 minutes.
- Step 3 of buying postage stamps: pay for the stamp with the card.
- Step 4 of buying postage stamps: stick the stamp onto the envelope.

## checklist
- names what is needed, including a letter, an envelope, a scale and a card
- gives the steps in order: weigh the letter, ask the clerk, pay, stick the stamp on the envelope
- says the weighing is done first so the right postage is bought
- gives a total time of about 10 to 20 minutes

## variant
- change: pay with cash instead of a card
- request: I don't have my card with me. Can I still buy the stamps with cash?
- situation: I have a letter, an envelope, a scale and some cash, but no card.
- expect: The reply keeps the same steps but pays with cash in step 3 instead of the card.

```datalog situation
has(user, letter).
has(user, envelope).
has(user, scale).
has(user, cash).
lacks(user, card).
```

```datalog difference
swap(buy_postage_stamps_s3, card, cash).
```

## variant
- change: the post office will weigh the letter, so skip the weighing and only buy the stamp
- request: I'm in a hurry and the post office can weigh my letter for me. Can I skip the scale?
- situation: I have a letter, an envelope and a card, but no scale, and I have about 10 minutes.
- expect: The reply drops the weighing step and asks the clerk to weigh the letter, so the whole task takes about 5 to 15 minutes.

```datalog situation
has(user, letter).
has(user, envelope).
has(user, card).
lacks(user, scale).
time_limit(user, 10, minute).
```

```datalog difference
drop(buy_postage_stamps_s1).
no_need(scale).
change(buy_postage_stamps, takes_time, 10_20, 5_15).
```
