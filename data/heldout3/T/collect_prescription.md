---
id: collect_prescription
stage: social
task: collect_prescription
schema: v3
tier: T
---
# Collect a prescription

## requests
- The doctor sent my prescription to the chemist. How do I pick it up?
- What do I need to bring when collecting medicine from the pharmacy?
- I've got to get my repeat prescription today. What's the drill?

## situation
- I have my phone, a photo ID and my bank card.

## ground truth
```datalog
task(collect_prescription).
domain(collect_prescription, errands).
goal(collect_prescription, medicine_collected).
needs(collect_prescription, phone).
needs(collect_prescription, photo_id).
needs(collect_prescription, payment_card).
takes_time(collect_prescription, 20, 40, minute).
step(collect_prescription, collect_prescription_s1).
action(collect_prescription_s1, check).
object_of(collect_prescription_s1, prescription).
uses(collect_prescription_s1, phone).
role(collect_prescription_s1, phone, inform).
check(collect_prescription_s1, ready).
step(collect_prescription, collect_prescription_s2).
action(collect_prescription_s2, go).
object_of(collect_prescription_s2, pharmacy).
step(collect_prescription, collect_prescription_s3).
action(collect_prescription_s3, give).
object_of(collect_prescription_s3, name).
uses(collect_prescription_s3, photo_id).
role(collect_prescription_s3, photo_id, inform).
step(collect_prescription, collect_prescription_s4).
action(collect_prescription_s4, pay).
object_of(collect_prescription_s4, charge).
uses(collect_prescription_s4, payment_card).
role(collect_prescription_s4, payment_card, pay).
step(collect_prescription, collect_prescription_s5).
action(collect_prescription_s5, check).
object_of(collect_prescription_s5, medicine).
check(collect_prescription_s5, right_dose).
before(collect_prescription_s1, collect_prescription_s2).
before(collect_prescription_s2, collect_prescription_s3).
before(collect_prescription_s3, collect_prescription_s4).
before(collect_prescription_s4, collect_prescription_s5).
```

## aliases
```datalog
task_alias(collect_prescription, "picking up a prescription").
task_alias(collect_prescription, "collecting medicine").
task_alias(collect_prescription, "getting my prescription").
alias(pharmacy, "chemist").
alias(photo_id, "driving licence").
alias(charge, "the prescription charge").
alias(right_dose, "the dose on the label").
```

## teach
- To collect a prescription you need your phone, a photo ID and a payment card.
- Collecting a prescription takes 20 to 40 minutes.
- Step 1 of collecting a prescription: check on your phone that the prescription is ready.
- Step 2 of collecting a prescription: go to the pharmacy.
- Step 3 of collecting a prescription: give your name and show your photo ID.
- Step 4 of collecting a prescription: pay the charge with your card.
- Step 5 of collecting a prescription: check the medicine and the dose on the label.

## checklist
- names what is needed: a phone, photo ID and a card
- gives the steps in order: check ready, go, give name with ID, pay, check medicine

## variant
- change: free prescriptions: no charge to pay
- request: I get free prescriptions now that I'm over sixty. Do I still need my card to pick up my pills?
- situation: I have my phone and a photo ID, and I qualify for free prescriptions.
- expect: The reply leaves out paying the charge and keeps the other steps.

```datalog situation
has(user, phone).
has(user, photo_id).
lacks(user, payment_card).
```

```datalog difference
drop(collect_prescription_s4).
no_need(payment_card).
```

## variant
- change: no driving licence: show a passport as ID
- request: I don't drive, so I've no licence. What ID can I show at the chemist?
- situation: I have my phone, my passport and my bank card.
- expect: The reply shows the passport as photo ID instead of a driving licence.

```datalog situation
has(user, phone).
has(user, passport).
has(user, payment_card).
lacks(user, photo_id).
```

```datalog difference
swap(collect_prescription_s3, photo_id, passport).
extra_need(passport).
no_need(photo_id).
```
