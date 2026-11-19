---
id: claim_flight_delay_compensation
stage: procedures
task: claim_flight_delay_compensation
schema: v3
---
# Claim flight delay compensation

## requests
- My flight was delayed by more than three hours. How do I get compensation from the airline?
- What's the process for claiming delay compensation on a flight?
- Can you explain how I claim money back for a late flight?

## situation
- My flight arrived four hours late because of a technical fault.
- I have my booking reference, boarding pass, phone and email account.

## ground truth
```datalog
task(claim_flight_delay_compensation).
domain(claim_flight_delay_compensation, travel).
goal(claim_flight_delay_compensation, compensation_claimed).
needs(claim_flight_delay_compensation, booking_reference).
needs(claim_flight_delay_compensation, boarding_pass).
needs(claim_flight_delay_compensation, phone).
needs(claim_flight_delay_compensation, email_account).
takes_time(claim_flight_delay_compensation, 20, 35, minute).
step(claim_flight_delay_compensation, claim_flight_delay_compensation_s1).
action(claim_flight_delay_compensation_s1, check).
object_of(claim_flight_delay_compensation_s1, flight_status).
uses(claim_flight_delay_compensation_s1, phone).
role(claim_flight_delay_compensation_s1, phone, hold).
purpose(claim_flight_delay_compensation_s1, delay_confirmed).
takes_time(claim_flight_delay_compensation_s1, 5, 10, minute).
step(claim_flight_delay_compensation, claim_flight_delay_compensation_s2).
action(claim_flight_delay_compensation_s2, fill).
object_of(claim_flight_delay_compensation_s2, claim_form).
uses(claim_flight_delay_compensation_s2, booking_reference).
role(claim_flight_delay_compensation_s2, booking_reference, cite).
takes_time(claim_flight_delay_compensation_s2, 10, 15, minute).
step(claim_flight_delay_compensation, claim_flight_delay_compensation_s3).
action(claim_flight_delay_compensation_s3, attach).
object_of(claim_flight_delay_compensation_s3, claim_form).
uses(claim_flight_delay_compensation_s3, boarding_pass).
role(claim_flight_delay_compensation_s3, boarding_pass, attach).
takes_time(claim_flight_delay_compensation_s3, 3, 5, minute).
step(claim_flight_delay_compensation, claim_flight_delay_compensation_s4).
action(claim_flight_delay_compensation_s4, submit).
object_of(claim_flight_delay_compensation_s4, claim_form).
uses(claim_flight_delay_compensation_s4, email_account).
role(claim_flight_delay_compensation_s4, email_account, send).
takes_time(claim_flight_delay_compensation_s4, 2, 3, minute).
before(claim_flight_delay_compensation_s1, claim_flight_delay_compensation_s2).
before(claim_flight_delay_compensation_s2, claim_flight_delay_compensation_s3).
before(claim_flight_delay_compensation_s3, claim_flight_delay_compensation_s4).
```

## aliases
```datalog
task_alias(claim_flight_delay_compensation, "flight delay compensation").
task_alias(claim_flight_delay_compensation, "claim compensation for a late flight").
alias(booking_reference, "booking reference").
alias(boarding_pass, "boarding pass").
alias(email_account, "email account").
alias(claim_form, "claim form").
alias(flight_status, "flight status").
alias(e_ticket_receipt, "e-ticket receipt").
alias(delay_confirmed, "delay is confirmed").
```

## teach
- To claim flight delay compensation you need a booking reference, a boarding pass, a phone and an email account.
- Claiming flight delay compensation takes 20 to 35 minutes.
- Step 1 of claim flight delay compensation: check the flight status on your phone so the delay is confirmed, for 5 to 10 minutes.
- Step 2 of claim flight delay compensation: fill in the claim form with your booking reference, for 10 to 15 minutes.
- Step 3 of claim flight delay compensation: attach your boarding pass to the claim form, for 3 to 5 minutes.
- Step 4 of claim flight delay compensation: submit the claim form from your email account, for 2 to 3 minutes.

## checklist
- names what is needed, including a booking reference, a boarding pass, a phone and an email account
- gives the steps in order: check the flight status, fill in the claim form, attach the boarding pass, submit by email
- says the status check is there to confirm the delay
- gives a total time of about 20 to 35 minutes

## variant
- change: boarding pass lost: attach the e-ticket receipt instead
- request: My paper boarding pass got lost, but I have my e-ticket receipt. Can I still claim delay compensation?
- situation: I have my booking reference, phone, email account and e-ticket receipt, but I lost my boarding pass.
- expect: The reply uses the e-ticket receipt in place of the boarding pass when attaching to the claim form and otherwise follows the four steps.

```datalog situation
has(user, booking_reference).
has(user, phone).
has(user, email_account).
has(user, e_ticket_receipt).
lacks(user, boarding_pass).
```

```datalog difference
swap(claim_flight_delay_compensation_s3, boarding_pass, e_ticket_receipt).
extra_need(e_ticket_receipt).
no_need(boarding_pass).
```

## variant
- change: already have a written delay letter: skip the status check
- request: The airline sent me a letter confirming my flight was delayed. Do I still need to check the flight status before claiming?
- situation: I have my booking reference, boarding pass, phone, email account and the airline's written delay letter.
- expect: The reply skips the flight status check because the letter already confirms the delay and gives about 15 to 25 minutes in total.

```datalog situation
has(user, booking_reference).
has(user, boarding_pass).
has(user, phone).
has(user, email_account).
has(user, delay_letter).
```

```datalog difference
drop(claim_flight_delay_compensation_s1).
change(claim_flight_delay_compensation, takes_time, 20_35, 15_25).
```
