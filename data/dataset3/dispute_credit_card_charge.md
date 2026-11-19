---
id: dispute_credit_card_charge
stage: procedures
task: dispute_credit_card_charge
schema: v3
---
# Dispute a credit card charge

## requests
- A charge on my card statement looks wrong. How do I dispute it with the bank?
- What's the process for contesting a card charge I don't recognise?
- Can you tell me how to challenge a credit card payment and get it reversed?

## situation
- I have my card statement, a purchase receipt, a pen and an envelope.
- The charge is for an amount I don't recognise from a shop I didn't visit.

## ground truth
```datalog
task(dispute_credit_card_charge).
domain(dispute_credit_card_charge, errands).
goal(dispute_credit_card_charge, resolve_charge).
needs(dispute_credit_card_charge, statement).
needs(dispute_credit_card_charge, receipt).
needs(dispute_credit_card_charge, pen).
needs(dispute_credit_card_charge, envelope).
takes_time(dispute_credit_card_charge, 30, 45, minute).
step(dispute_credit_card_charge, dispute_credit_card_charge_s1).
action(dispute_credit_card_charge_s1, review).
object_of(dispute_credit_card_charge_s1, statement).
takes_time(dispute_credit_card_charge_s1, 5, 5, minute).
step(dispute_credit_card_charge, dispute_credit_card_charge_s2).
action(dispute_credit_card_charge_s2, write).
object_of(dispute_credit_card_charge_s2, letter).
uses(dispute_credit_card_charge_s2, pen).
role(dispute_credit_card_charge_s2, pen, mark).
takes_time(dispute_credit_card_charge_s2, 10, 15, minute).
step(dispute_credit_card_charge, dispute_credit_card_charge_s3).
action(dispute_credit_card_charge_s3, attach).
object_of(dispute_credit_card_charge_s3, receipt).
onto(dispute_credit_card_charge_s3, letter).
overlaps(dispute_credit_card_charge_s3, dispute_credit_card_charge_s2).
step(dispute_credit_card_charge, dispute_credit_card_charge_s4).
action(dispute_credit_card_charge_s4, send).
object_of(dispute_credit_card_charge_s4, letter).
uses(dispute_credit_card_charge_s4, envelope).
role(dispute_credit_card_charge_s4, envelope, contain).
purpose(dispute_credit_card_charge_s4, bank_receives_dispute).
step(dispute_credit_card_charge, dispute_credit_card_charge_s5).
action(dispute_credit_card_charge_s5, call).
object_of(dispute_credit_card_charge_s5, bank).
takes_time(dispute_credit_card_charge_s5, 15, 20, minute).
before(dispute_credit_card_charge_s1, dispute_credit_card_charge_s2).
before(dispute_credit_card_charge_s3, dispute_credit_card_charge_s4).
before(dispute_credit_card_charge_s4, dispute_credit_card_charge_s5).
```

## aliases
```datalog
task_alias(dispute_credit_card_charge, "dispute a charge").
task_alias(dispute_credit_card_charge, "contest a card charge").
task_alias(dispute_credit_card_charge, "challenge a credit card charge").
alias(statement, "card statement").
alias(receipt, "purchase receipt").
alias(letter, "dispute letter").
alias(bank, "card issuer").
alias(review, "check").
alias(bank_receives_dispute, "bank receives the dispute").
```

## teach
- To dispute a credit card charge you need a card statement, a purchase receipt, a pen and an envelope.
- Disputing a credit card charge takes 30 to 45 minutes.
- Step 1 of dispute a credit card charge: review the card statement for the charge.
- Step 2 of dispute a credit card charge: write the dispute letter with a pen.
- While step 2 runs, step 3 of dispute a credit card charge: attach the purchase receipt onto the letter.
- Step 4 of dispute a credit card charge: send the letter in the envelope so the bank receives the dispute.
- Step 5 of dispute a credit card charge: call the card issuer to confirm the dispute.

## checklist
- names what is needed, including a card statement, a purchase receipt, a pen and an envelope
- gives the steps in order: review statement, write letter, attach receipt, send letter, call the bank
- says the receipt can be attached while the letter is being written
- gives a total time of about 30 to 45 minutes

## variant
- change: only 25 minutes and no post: dispute by phone only, skipping the letter
- request: I can only spare about 25 minutes and I don't want to post anything. Can I dispute this just by phone?
- situation: I have my card statement and a phone, but no envelope, and I have about 25 minutes.
- expect: The reply says to skip the letter, check the statement, and call the card issuer to dispute the charge, which fits in about 20 to 25 minutes.

```datalog situation
has(user, statement).
has(user, phone).
lacks(user, envelope).
time_limit(user, 25, minute).
```

```datalog difference
drop(dispute_credit_card_charge_s2).
drop(dispute_credit_card_charge_s3).
drop(dispute_credit_card_charge_s4).
no_need(pen).
no_need(envelope).
change(dispute_credit_card_charge, takes_time, 30_45, 20_25).
```

## variant
- change: no pen, so the letter is written in pencil instead
- request: I don't have a pen, only a pencil. Will that still work for the dispute letter?
- situation: I have my card statement, a receipt, a pencil and an envelope, but no pen.
- expect: The reply says to write the dispute letter in pencil instead of pen, then attach the receipt, send it in the envelope and call the card issuer.

```datalog situation
has(user, statement).
has(user, receipt).
has(user, pencil).
has(user, envelope).
lacks(user, pen).
```

```datalog difference
swap(dispute_credit_card_charge_s2, pen, pencil).
no_need(pen).
extra_need(pencil).
```
