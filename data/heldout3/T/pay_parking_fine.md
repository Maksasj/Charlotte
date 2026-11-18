---
id: pay_parking_fine
stage: social
task: pay_parking_fine
schema: v3
tier: T
---
# Pay a parking fine

## requests
- I came back to my car and found a parking ticket. How do I pay it?
- What's the process for paying a penalty charge notice online?
- I got fined for parking. How do I settle it before it goes up?

## situation
- I have the penalty notice, my phone and my bank card.

## ground truth
```datalog
task(pay_parking_fine).
domain(pay_parking_fine, errands).
goal(pay_parking_fine, fine_paid).
needs(pay_parking_fine, penalty_notice).
needs(pay_parking_fine, phone).
needs(pay_parking_fine, payment_card).
takes_time(pay_parking_fine, 10, 15, minute).
step(pay_parking_fine, pay_parking_fine_s1).
action(pay_parking_fine_s1, read).
object_of(pay_parking_fine_s1, penalty_notice).
check(pay_parking_fine_s1, deadline).
step(pay_parking_fine, pay_parking_fine_s2).
action(pay_parking_fine_s2, open).
object_of(pay_parking_fine_s2, council_website).
uses(pay_parking_fine_s2, phone).
role(pay_parking_fine_s2, phone, access).
step(pay_parking_fine, pay_parking_fine_s3).
action(pay_parking_fine_s3, enter).
object_of(pay_parking_fine_s3, reference_number).
from(pay_parking_fine_s3, penalty_notice).
step(pay_parking_fine, pay_parking_fine_s4).
action(pay_parking_fine_s4, pay).
object_of(pay_parking_fine_s4, fine).
uses(pay_parking_fine_s4, payment_card).
role(pay_parking_fine_s4, payment_card, pay).
condition(pay_parking_fine_s4, within_discount_period).
step(pay_parking_fine, pay_parking_fine_s5).
action(pay_parking_fine_s5, save).
object_of(pay_parking_fine_s5, receipt).
purpose(pay_parking_fine_s5, proof).
before(pay_parking_fine_s1, pay_parking_fine_s2).
before(pay_parking_fine_s2, pay_parking_fine_s3).
before(pay_parking_fine_s3, pay_parking_fine_s4).
before(pay_parking_fine_s4, pay_parking_fine_s5).
```

## aliases
```datalog
task_alias(pay_parking_fine, "paying a parking ticket").
task_alias(pay_parking_fine, "paying a penalty charge").
task_alias(pay_parking_fine, "settling a parking fine").
alias(penalty_notice, "parking ticket").
alias(penalty_notice, "penalty charge notice").
alias(council_website, "the council's website").
alias(reference_number, "the reference number").
alias(within_discount_period, "within 14 days for the reduced rate").
```

## teach
- To pay a parking fine you need the penalty notice, a phone and a payment card.
- Paying a parking fine takes 10 to 15 minutes.
- Step 1 of paying a parking fine: read the penalty notice and check the deadline.
- Step 2 of paying a parking fine: open the council's website on your phone.
- Step 3 of paying a parking fine: enter the reference number from the notice.
- Step 4 of paying a parking fine: pay the fine with your card, within 14 days for the reduced rate.
- Step 5 of paying a parking fine: save the receipt as proof.

## checklist
- names what is needed: the notice, a phone and a card
- gives the steps in order: read notice, open website, enter reference, pay, save receipt
- mentions paying early for the reduced rate

## variant
- change: phone screen smashed: pay on a laptop
- request: I need to pay a parking ticket but my phone's screen is shattered. How else can I do it?
- situation: I have the notice, a laptop and my bank card; my phone is broken.
- expect: The reply opens the council's website on the laptop instead of the phone and pays there.

```datalog situation
has(user, penalty_notice).
has(user, laptop).
has(user, payment_card).
lacks(user, phone).
```

```datalog difference
swap(pay_parking_fine_s2, phone, laptop).
extra_need(laptop).
no_need(phone).
```

## variant
- change: wants to appeal: gather photos and appeal before paying
- request: I think the parking ticket was unfair, the sign was hidden. Can I challenge it?
- situation: I have the notice, my phone with photos of the hidden sign, and my card.
- expect: The reply submits an appeal with the photos on the council website before paying, instead of paying straight away.

```datalog situation
has(user, penalty_notice).
has(user, phone).
has(user, payment_card).
wants(user, appeal).
```

```datalog difference
add_step(pay_parking_fine_s9, pay_parking_fine_s3).
step(pay_parking_fine, pay_parking_fine_s9).
action(pay_parking_fine_s9, submit).
object_of(pay_parking_fine_s9, appeal).
uses(pay_parking_fine_s9, phone).
role(pay_parking_fine_s9, phone, access).
purpose(pay_parking_fine_s9, fine_cancelled).
before(pay_parking_fine_s3, pay_parking_fine_s9).
drop(pay_parking_fine_s4).
```
