---
id: cancel_flight_booking
stage: procedures
task: cancel_flight_booking
schema: v3
---
# Cancel a flight booking

## requests
- How do I cancel a flight booking through the airline's website or app?
- My trip is off. What's the way to cancel my flight reservation?
- Can you show me the steps to cancel a booked flight?

## situation
- I have my phone, my booking reference and my email address.
- I'd like the cancellation done and confirmed today.

## ground truth
```datalog
task(cancel_flight_booking).
domain(cancel_flight_booking, travel).
goal(cancel_flight_booking, flight_cancelled).
needs(cancel_flight_booking, phone).
needs(cancel_flight_booking, booking_reference).
needs(cancel_flight_booking, email_address).
takes_time(cancel_flight_booking, 10, 15, minute).
step(cancel_flight_booking, cancel_flight_booking_s1).
action(cancel_flight_booking_s1, sign_in).
object_of(cancel_flight_booking_s1, account).
uses(cancel_flight_booking_s1, phone).
role(cancel_flight_booking_s1, phone, hold).
step(cancel_flight_booking, cancel_flight_booking_s2).
action(cancel_flight_booking_s2, find).
object_of(cancel_flight_booking_s2, booking).
uses(cancel_flight_booking_s2, booking_reference).
role(cancel_flight_booking_s2, booking_reference, hold).
step(cancel_flight_booking, cancel_flight_booking_s3).
action(cancel_flight_booking_s3, select).
object_of(cancel_flight_booking_s3, cancel_option).
step(cancel_flight_booking, cancel_flight_booking_s4).
action(cancel_flight_booking_s4, confirm).
object_of(cancel_flight_booking_s4, cancellation).
check(cancel_flight_booking_s4, cancellation_confirmed).
step(cancel_flight_booking, cancel_flight_booking_s5).
action(cancel_flight_booking_s5, save).
object_of(cancel_flight_booking_s5, confirmation_email).
uses(cancel_flight_booking_s5, email_address).
role(cancel_flight_booking_s5, email_address, contain).
before(cancel_flight_booking_s1, cancel_flight_booking_s2).
before(cancel_flight_booking_s2, cancel_flight_booking_s3).
before(cancel_flight_booking_s3, cancel_flight_booking_s4).
before(cancel_flight_booking_s4, cancel_flight_booking_s5).
```

## aliases
```datalog
task_alias(cancel_flight_booking, "cancel a flight").
task_alias(cancel_flight_booking, "cancelling a flight booking").
task_alias(cancel_flight_booking, "cancel flight reservation").
alias(phone, "mobile phone").
alias(booking_reference, "confirmation code").
alias(email_address, "email").
alias(cancel_option, "cancel button").
alias(cancellation, "cancellation").
alias(confirmation_email, "confirmation email").
alias(sign_in, "log in").
```

## teach
- To cancel a flight booking you need a phone, a booking reference and an email address.
- Cancelling a flight booking takes 10 to 15 minutes.
- Step 1 of cancelling a flight booking: sign in to the airline account on your phone.
- Step 2 of cancelling a flight booking: find the booking using the booking reference.
- Step 3 of cancelling a flight booking: select the cancel option for the flight.
- Step 4 of cancelling a flight booking: confirm the cancellation so you can see it is confirmed.
- Step 5 of cancelling a flight booking: save the confirmation to your email address.

## checklist
- names what is needed, including a phone, a booking reference and an email address
- gives the steps in order: sign in, find the booking, select cancel, confirm, save the confirmation
- says the cancellation should be confirmed on screen before finishing
- gives a total time of about 10 to 15 minutes

## variant
- change: no phone, only a laptop: sign in on the laptop instead
- request: I don't have my phone with me. Can I cancel the flight from my laptop?
- situation: I have a laptop, my booking reference and my email address.
```datalog situation
lacks(user, phone).
has(user, laptop).
has(user, booking_reference).
has(user, email_address).
```
```datalog difference
swap(cancel_flight_booking_s1, phone, laptop).
extra_need(laptop).
no_need(phone).
```
- expect: The reply says to sign in to the airline account on the laptop instead of the phone, then continues with the same steps: find the booking, select cancel, confirm, and save the confirmation to email.

- change: add a printed receipt at the end
- request: I'd like a paper copy of the cancellation too. Can I print it after cancelling?
- situation: I have my phone, booking reference, email address and a printer.
```datalog situation
has(user, printer).
wants(user, printed_receipt).
has(user, phone).
```
```datalog difference
add_step(cancel_flight_booking_s9, cancel_flight_booking_s5).
step(cancel_flight_booking, cancel_flight_booking_s9).
action(cancel_flight_booking_s9, print).
object_of(cancel_flight_booking_s9, receipt).
uses(cancel_flight_booking_s9, printer).
role(cancel_flight_booking_s9, printer, hold).
before(cancel_flight_booking_s5, cancel_flight_booking_s9).
extra_need(printer).
```
- expect: The reply gives the same five steps and adds a final step: print the cancellation receipt on the printer after saving the confirmation.
