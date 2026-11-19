---
id: schedule_dentist_appointment
stage: procedures
task: schedule_dentist_appointment
schema: v3
---
# Schedule a dentist appointment

## requests
- I need to book a dentist appointment. What's the process?
- How do I go about getting a check-up booked with my dentist?
- Can you tell me the steps to make a dentist appointment?

## situation
- I have my phone, my calendar and my insurance card with me.
- I'm free on weekday mornings next week.

## ground truth
```datalog
task(schedule_dentist_appointment).
domain(schedule_dentist_appointment, planning).
goal(schedule_dentist_appointment, dentist_appointment_booked).
needs(schedule_dentist_appointment, phone).
needs(schedule_dentist_appointment, calendar).
needs(schedule_dentist_appointment, insurance_card).
takes_time(schedule_dentist_appointment, 10, 15, minute).
step(schedule_dentist_appointment, schedule_dentist_appointment_s1).
action(schedule_dentist_appointment_s1, look_up).
object_of(schedule_dentist_appointment_s1, free_slot).
uses(schedule_dentist_appointment_s1, calendar).
role(schedule_dentist_appointment_s1, calendar, contain).
step(schedule_dentist_appointment, schedule_dentist_appointment_s2).
action(schedule_dentist_appointment_s2, call).
object_of(schedule_dentist_appointment_s2, clinic).
uses(schedule_dentist_appointment_s2, phone).
role(schedule_dentist_appointment_s2, phone, call).
takes_time(schedule_dentist_appointment_s2, 5, 10, minute).
achieves(schedule_dentist_appointment_s2, booked).
step(schedule_dentist_appointment, schedule_dentist_appointment_s3).
action(schedule_dentist_appointment_s3, read).
object_of(schedule_dentist_appointment_s3, insurance_number).
uses(schedule_dentist_appointment_s3, insurance_card).
role(schedule_dentist_appointment_s3, insurance_card, hold).
from(schedule_dentist_appointment_s3, insurance_card).
step(schedule_dentist_appointment, schedule_dentist_appointment_s4).
action(schedule_dentist_appointment_s4, write).
object_of(schedule_dentist_appointment_s4, appointment).
uses(schedule_dentist_appointment_s4, calendar).
role(schedule_dentist_appointment_s4, calendar, contain).
into(schedule_dentist_appointment_s4, calendar).
before(schedule_dentist_appointment_s1, schedule_dentist_appointment_s2).
before(schedule_dentist_appointment_s2, schedule_dentist_appointment_s3).
before(schedule_dentist_appointment_s3, schedule_dentist_appointment_s4).
```

## aliases
```datalog
task_alias(schedule_dentist_appointment, "book a dentist appointment").
task_alias(schedule_dentist_appointment, "dentist check-up booking").
alias(insurance_card, "insurance card").
alias(clinic, "dental office").
alias(free_slot, "free appointment time").
alias(look_up, "look up").
alias(booked, "booked").
```

## teach
- To schedule a dentist appointment you need a phone, a calendar and your insurance card.
- Scheduling a dentist appointment takes 10 to 15 minutes.
- Step 1 of scheduling a dentist appointment: look up a free appointment time in the calendar.
- Step 2 of scheduling a dentist appointment: call the dental office on the phone to book the slot, which takes 5 to 10 minutes.
- Step 3 of scheduling a dentist appointment: read your insurance number from your insurance card.
- Step 4 of scheduling a dentist appointment: write the booked time into the calendar.

## checklist
- names what is needed: a phone, a calendar and the insurance card
- gives the steps in order: find a free time, call the dental office, read the insurance number, write the appointment in the calendar
- gives a total time of about 10 to 15 minutes
- says the call is what books the appointment

## variant
- change: no phone: book through the clinic website on a laptop instead of calling
- request: I don't have my phone with me. Can I still book the appointment?
- situation: I have a laptop, my calendar and my insurance card, but no phone.
- expect: The reply says to find a free time in the calendar, book through the dental office website on the laptop instead of calling, then read the insurance number and write the booked time into the calendar.

```datalog situation
lacks(user, phone).
has(user, laptop).
has(user, calendar).
has(user, insurance_card).
```

```datalog difference
swap(schedule_dentist_appointment_s2, phone, laptop).
no_need(phone).
extra_need(laptop).
```

- change: add a reminder the day before the appointment
- request: Once it's booked, can you also remind me to go?
- situation: I have my phone, my calendar and my insurance card, and I want a reminder set for the day before.
- expect: The reply gives the same four steps and then adds a final step to set a reminder on the phone for the day before the appointment.

```datalog situation
has(user, phone).
has(user, calendar).
has(user, insurance_card).
wants(user, reminder).
```

```datalog difference
add_step(schedule_dentist_appointment_s9, schedule_dentist_appointment_s4).
step(schedule_dentist_appointment, schedule_dentist_appointment_s9).
action(schedule_dentist_appointment_s9, set).
object_of(schedule_dentist_appointment_s9, reminder).
uses(schedule_dentist_appointment_s9, phone).
role(schedule_dentist_appointment_s9, phone, remind).
before(schedule_dentist_appointment_s4, schedule_dentist_appointment_s9).
extra_need(reminder).
```
