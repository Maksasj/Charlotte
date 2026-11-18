---
id: renew_passport
stage: social
task: renew_passport
schema: v3
---
# Renew a passport

## requests
- My passport runs out next year and I've got a holiday booked. How do I renew it?
- Can you walk me through renewing my passport by post?
- What do I need to do to get a new passport?

## situation
- I have my old passport, a payment card and a photo booth nearby.
- My trip is in four months.

## ground truth
```datalog
task(renew_passport).
domain(renew_passport, errands).
goal(renew_passport, new_passport).
needs(renew_passport, old_passport).
needs(renew_passport, photo).
needs(renew_passport, application_form).
needs(renew_passport, payment_card).
takes_time(renew_passport, 40, 60, minute).
step(renew_passport, renew_passport_s1).
action(renew_passport_s1, check).
object_of(renew_passport_s1, old_passport).
with_part(renew_passport_s1, expiry_date).
takes_time(renew_passport_s1, 1, 2, minute).
step(renew_passport, renew_passport_s2).
action(renew_passport_s2, take).
object_of(renew_passport_s2, photo).
amount(renew_passport_s2, photo, 2, piece).
takes_time(renew_passport_s2, 10, 15, minute).
step(renew_passport, renew_passport_s3).
action(renew_passport_s3, fill).
object_of(renew_passport_s3, application_form).
takes_time(renew_passport_s3, 15, 20, minute).
step(renew_passport, renew_passport_s4).
action(renew_passport_s4, pay).
object_of(renew_passport_s4, fee).
uses(renew_passport_s4, payment_card).
role(renew_passport_s4, payment_card, pay).
takes_time(renew_passport_s4, 5, 10, minute).
step(renew_passport, renew_passport_s5).
action(renew_passport_s5, send).
object_of(renew_passport_s5, application_form).
uses(renew_passport_s5, photo).
role(renew_passport_s5, photo, enclose).
uses(renew_passport_s5, old_passport).
role(renew_passport_s5, old_passport, enclose).
takes_time(renew_passport_s5, 10, 15, minute).
before(renew_passport_s1, renew_passport_s2).
before(renew_passport_s2, renew_passport_s3).
before(renew_passport_s3, renew_passport_s4).
before(renew_passport_s4, renew_passport_s5).
```

## aliases
```datalog
task_alias(renew_passport, "renewing a passport").
task_alias(renew_passport, "renew a passport").
task_alias(renew_passport, "renewing my passport by post").
task_alias(renew_passport, "get a new passport").
alias(old_passport, "old passport").
alias(old_passport, "my passport").
alias(photo, "passport photos").
alias(photo, "photos").
alias(application_form, "form").
alias(expiry_date, "expiry date").
alias(take, "get taken").
alias(fill, "fill in").
alias(fast_track_appointment, "fast-track appointment").
alias(passport_office, "passport office").
alias(fast_track_fee, "the higher fee").
```

## teach
- To renew a passport you need the old passport, two passport photos, an application form and a payment card.
- Renewing a passport takes 40 to 60 minutes.
- Step 1 of renewing a passport: check the expiry date on the old passport for 1 to 2 minutes.
- Step 2 of renewing a passport: get two new passport photos taken for 10 to 15 minutes.
- Step 3 of renewing a passport: fill in the application form for 15 to 20 minutes.
- Step 4 of renewing a passport: pay the fee with the payment card for 5 to 10 minutes.
- Step 5 of renewing a passport: send the application form, the photos and the old passport, for 10 to 15 minutes.

## checklist
- names what is needed: the old passport, two passport photos, an application form and a payment card
- gives the steps in order: check old passport, take photo, fill application form, pay fee, send application form
- gives a total time of about 40 to 60 minutes
- closes politely

## variant
- change: trip in three weeks: book a fast-track appointment instead of applying by post
- request: I just noticed my passport expires before my trip in three weeks! What can I do?
- situation: I have my old passport and a payment card. The trip is in three weeks.
- expect: The reply books a fast-track appointment at the passport office and brings the form, photos and old passport there instead of posting them, paying the higher fee.

```datalog situation
has(user, old_passport).
has(user, payment_card).
time_limit(user, 3, week).
```

```datalog difference
drop(renew_passport_s5).
change(renew_passport_s4, object_of, fee, fast_track_fee).
add_step(renew_passport_s9, renew_passport_s1).
step(renew_passport, renew_passport_s9).
action(renew_passport_s9, book).
object_of(renew_passport_s9, fast_track_appointment).
before(renew_passport_s1, renew_passport_s9).
before(renew_passport_s9, renew_passport_s2).
add_step(renew_passport_s10, renew_passport_s4).
step(renew_passport, renew_passport_s10).
action(renew_passport_s10, bring).
object_of(renew_passport_s10, application_form).
into(renew_passport_s10, passport_office).
uses(renew_passport_s10, photo).
role(renew_passport_s10, photo, enclose).
uses(renew_passport_s10, old_passport).
role(renew_passport_s10, old_passport, enclose).
before(renew_passport_s4, renew_passport_s10).
```
