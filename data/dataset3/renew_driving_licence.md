---
id: renew_driving_licence
stage: procedures
task: renew_driving_licence
schema: v3
---
# Renew a driving licence

## requests
- How do I renew my driving licence by post?
- What's the process for getting my driving licence renewed?
- I need to send off my licence to be replaced. What do I have to do?

## situation
- I have my current licence, a pen, a payment card and an envelope.
- I'd like the new licence before my next holiday, so I can plan around about three weeks.

## ground truth
```datalog
task(renew_driving_licence).
domain(renew_driving_licence, errands).
goal(renew_driving_licence, valid_licence).
needs(renew_driving_licence, current_licence).
needs(renew_driving_licence, pen).
needs(renew_driving_licence, payment_card).
needs(renew_driving_licence, envelope).
takes_time(renew_driving_licence, 14, 21, day).
step(renew_driving_licence, renew_driving_licence_s1).
action(renew_driving_licence_s1, fill).
object_of(renew_driving_licence_s1, renewal_form).
uses(renew_driving_licence_s1, pen).
role(renew_driving_licence_s1, pen, write).
takes_time(renew_driving_licence_s1, 10, 15, minute).
step(renew_driving_licence, renew_driving_licence_s2).
action(renew_driving_licence_s2, pay).
object_of(renew_driving_licence_s2, renewal_fee).
uses(renew_driving_licence_s2, payment_card).
role(renew_driving_licence_s2, payment_card, pay).
step(renew_driving_licence, renew_driving_licence_s3).
action(renew_driving_licence_s3, put).
object_of(renew_driving_licence_s3, current_licence).
uses(renew_driving_licence_s3, envelope).
role(renew_driving_licence_s3, envelope, contain).
into(renew_driving_licence_s3, envelope).
step(renew_driving_licence, renew_driving_licence_s4).
action(renew_driving_licence_s4, post).
object_of(renew_driving_licence_s4, envelope).
step(renew_driving_licence, renew_driving_licence_s5).
action(renew_driving_licence_s5, receive).
object_of(renew_driving_licence_s5, new_licence).
takes_time(renew_driving_licence_s5, 14, 21, day).
before(renew_driving_licence_s1, renew_driving_licence_s2).
before(renew_driving_licence_s2, renew_driving_licence_s3).
before(renew_driving_licence_s3, renew_driving_licence_s4).
before(renew_driving_licence_s4, renew_driving_licence_s5).
```

## aliases
```datalog
task_alias(renew_driving_licence, "renew my driving licence").
task_alias(renew_driving_licence, "driving licence renewal").
alias(renewal_form, "application form").
alias(renewal_fee, "renewal fee").
alias(payment_card, "credit card").
alias(current_licence, "old licence").
alias(new_licence, "new licence").
alias(submit, "apply online").
alias(receive, "get").
alias(post, "send by post").
alias(valid_licence, "valid licence").
```

## teach
- To renew a driving licence you need a current licence, a pen, a payment card and an envelope.
- Renewing a driving licence takes 14 to 21 days.
- Step 1 of renewing a driving licence: fill in the renewal form with a pen, which takes 10 to 15 minutes.
- Step 2 of renewing a driving licence: pay the renewal fee with the payment card.
- Step 3 of renewing a driving licence: put the current licence into the envelope.
- Step 4 of renewing a driving licence: post the envelope to the licensing agency.
- Step 5 of renewing a driving licence: receive the new licence, which takes 14 to 21 days.

## checklist
- names what is needed, including the current licence, a pen, a payment card and an envelope
- gives the steps in order: fill in the form, pay the fee, put the licence in the envelope, post it, receive the new licence
- says the new licence takes about 14 to 21 days to arrive
- mentions that the form takes about 10 to 15 minutes to fill in

## variant
- change: apply online instead of posting, if you have a computer but no envelope
- request: I'd rather apply online than post my licence. I have a computer but no envelope. How do I do it?
- situation: I have my current licence, a pen, a payment card and a computer, but no envelope.
- expect: The reply says to fill in the form, pay the fee and apply online with the computer instead of posting, and that the new licence still takes 14 to 21 days to arrive.

```datalog situation
has(user, current_licence).
has(user, pen).
has(user, payment_card).
has(user, computer).
lacks(user, envelope).
```

```datalog difference
drop(renew_driving_licence_s3).
drop(renew_driving_licence_s4).
no_need(envelope).
extra_need(computer).
add_step(renew_driving_licence_s9, renew_driving_licence_s2).
step(renew_driving_licence, renew_driving_licence_s9).
action(renew_driving_licence_s9, submit).
object_of(renew_driving_licence_s9, renewal_form).
uses(renew_driving_licence_s9, computer).
role(renew_driving_licence_s9, computer, send).
before(renew_driving_licence_s2, renew_driving_licence_s9).
before(renew_driving_licence_s9, renew_driving_licence_s5).
```

- change: cannot renew without a way to pay the fee
- request: I want to renew my driving licence, but I don't have a card or any other way to pay the fee. What can I do?
- situation: I have my current licence, a pen and an envelope, but no payment card.
- expect: The reply says the renewal cannot be completed without a way to pay the fee, and says the form can still be filled in beforehand.

```datalog situation
has(user, current_licence).
has(user, pen).
has(user, envelope).
lacks(user, payment_card).
```

```datalog difference
cannot(no_payment_method).
```
