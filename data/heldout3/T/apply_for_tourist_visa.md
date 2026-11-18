---
id: apply_for_tourist_visa
stage: social
task: apply_for_tourist_visa
schema: v3
tier: T
---
# Apply for a tourist visa

## requests
- The country I'm visiting needs a tourist visa. How do I apply?
- What are the steps for getting a holiday visa?
- I need permission to enter for a two-week trip. Where do I start?

## situation
- I have a laptop, a digital passport photo, my passport and my bank card.

## ground truth
```datalog
task(apply_for_tourist_visa).
domain(apply_for_tourist_visa, travel).
goal(apply_for_tourist_visa, visa).
needs(apply_for_tourist_visa, laptop).
needs(apply_for_tourist_visa, photo).
needs(apply_for_tourist_visa, passport).
needs(apply_for_tourist_visa, payment_card).
takes_time(apply_for_tourist_visa, 2, 4, week).
step(apply_for_tourist_visa, apply_for_tourist_visa_s1).
action(apply_for_tourist_visa_s1, check).
object_of(apply_for_tourist_visa_s1, requirement).
uses(apply_for_tourist_visa_s1, laptop).
role(apply_for_tourist_visa_s1, laptop, inform).
step(apply_for_tourist_visa, apply_for_tourist_visa_s2).
action(apply_for_tourist_visa_s2, fill).
object_of(apply_for_tourist_visa_s2, application_form).
uses(apply_for_tourist_visa_s2, laptop).
role(apply_for_tourist_visa_s2, laptop, access).
step(apply_for_tourist_visa, apply_for_tourist_visa_s3).
action(apply_for_tourist_visa_s3, upload).
object_of(apply_for_tourist_visa_s3, photo).
step(apply_for_tourist_visa, apply_for_tourist_visa_s4).
action(apply_for_tourist_visa_s4, pay).
object_of(apply_for_tourist_visa_s4, fee).
uses(apply_for_tourist_visa_s4, payment_card).
role(apply_for_tourist_visa_s4, payment_card, pay).
step(apply_for_tourist_visa, apply_for_tourist_visa_s5).
action(apply_for_tourist_visa_s5, wait).
object_of(apply_for_tourist_visa_s5, decision).
takes_time(apply_for_tourist_visa_s5, 2, 3, week).
requires_state(apply_for_tourist_visa_s5, passport_valid_6_month).
before(apply_for_tourist_visa_s1, apply_for_tourist_visa_s2).
before(apply_for_tourist_visa_s2, apply_for_tourist_visa_s3).
before(apply_for_tourist_visa_s3, apply_for_tourist_visa_s4).
before(apply_for_tourist_visa_s4, apply_for_tourist_visa_s5).
```

## aliases
```datalog
task_alias(apply_for_tourist_visa, "getting a tourist visa").
task_alias(apply_for_tourist_visa, "a holiday visa").
task_alias(apply_for_tourist_visa, "applying for a visa").
alias(application_form, "the application form").
alias(photo, "passport photo").
alias(requirement, "the entry requirements").
alias(decision, "the decision").
alias(passport_valid_6_month, "a passport valid for six more months").
```

## teach
- To apply for a tourist visa you need a laptop, a passport photo, your passport and a payment card.
- Applying for a tourist visa takes 2 to 4 weeks.
- Step 1 of applying for a tourist visa: check the requirements on the laptop.
- Step 2 of applying for a tourist visa: fill in the application form on the laptop.
- Step 3 of applying for a tourist visa: upload the photo.
- Step 4 of applying for a tourist visa: pay the fee with your card.
- Step 5 of applying for a tourist visa: wait 2 to 3 weeks for the decision; your passport must be valid for six more months.

## checklist
- names what is needed: a laptop, a photo, a passport and a card
- gives the steps in order: check requirements, fill form, upload photo, pay, wait
- says the decision takes about 2 to 3 weeks

## variant
- change: trip in 3 days and no express service: cannot get the visa in time
- request: I fly in three days and only now found out I need a visa, and this embassy has no rush option. Can I still get one?
- situation: I have a laptop, a photo, my passport and my card, and 3 days.
- expect: The reply says the visa cannot be issued within 3 days and suggests moving the trip.

```datalog situation
has(user, laptop).
has(user, photo).
has(user, passport).
has(user, payment_card).
time_limit(user, 3, day).
```

```datalog difference
cannot(processing_too_slow).
```

## variant
- change: no laptop: apply on the phone
- request: I've only got my phone, no computer. Can I do the visa application on that?
- situation: I have my phone, a digital photo, my passport and my card.
- expect: The reply checks the requirements and fills in the form on the phone instead of a laptop.

```datalog situation
has(user, phone).
has(user, photo).
has(user, passport).
has(user, payment_card).
lacks(user, laptop).
```

```datalog difference
swap(apply_for_tourist_visa_s1, laptop, phone).
swap(apply_for_tourist_visa_s2, laptop, phone).
extra_need(phone).
no_need(laptop).
```
