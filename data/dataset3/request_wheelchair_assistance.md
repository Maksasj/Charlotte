---
id: request_wheelchair_assistance
stage: procedures
task: request_wheelchair_assistance
schema: v3
---
# Request wheelchair assistance

## requests
- How do I get a wheelchair at the airport? I can't walk very far.
- Could you walk me through booking wheelchair help for a flight?
- What's the process for asking the airline for a wheelchair when I fly?

## situation
- I have my phone and boarding pass, and my flight is in a few days.

## ground truth
```datalog
task(request_wheelchair_assistance).
domain(request_wheelchair_assistance, travel).
goal(request_wheelchair_assistance, wheelchair_assistance).
needs(request_wheelchair_assistance, phone).
needs(request_wheelchair_assistance, boarding_pass).
takes_time(request_wheelchair_assistance, 25, 45, minute).
step(request_wheelchair_assistance, request_wheelchair_assistance_s1).
action(request_wheelchair_assistance_s1, call).
object_of(request_wheelchair_assistance_s1, airline).
uses(request_wheelchair_assistance_s1, phone).
role(request_wheelchair_assistance_s1, phone, contact).
purpose(request_wheelchair_assistance_s1, help_booked).
takes_time(request_wheelchair_assistance_s1, 10, 15, minute).
step(request_wheelchair_assistance, request_wheelchair_assistance_s2).
action(request_wheelchair_assistance_s2, show).
object_of(request_wheelchair_assistance_s2, boarding_pass).
uses(request_wheelchair_assistance_s2, boarding_pass).
role(request_wheelchair_assistance_s2, boarding_pass, hold).
step(request_wheelchair_assistance, request_wheelchair_assistance_s3).
action(request_wheelchair_assistance_s3, wait).
object_of(request_wheelchair_assistance_s3, assistant).
until(request_wheelchair_assistance_s3, assistant_arrives).
takes_time(request_wheelchair_assistance_s3, 15, 30, minute).
step(request_wheelchair_assistance, request_wheelchair_assistance_s4).
action(request_wheelchair_assistance_s4, board).
object_of(request_wheelchair_assistance_s4, aircraft).
before(request_wheelchair_assistance_s1, request_wheelchair_assistance_s2).
before(request_wheelchair_assistance_s2, request_wheelchair_assistance_s3).
before(request_wheelchair_assistance_s3, request_wheelchair_assistance_s4).
```

## aliases
```datalog
task_alias(request_wheelchair_assistance, "wheelchair help at the airport").
task_alias(request_wheelchair_assistance, "book wheelchair assistance").
alias(phone, "mobile phone").
alias(boarding_pass, "boarding card").
alias(help_booked, "help is booked").
alias(assistant_arrives, "assistant arrives").
alias(board, "get on").
alias(aircraft, "plane").
```

## teach
- To request wheelchair assistance you need a phone and a boarding pass.
- Requesting wheelchair assistance takes 25 to 45 minutes.
- Step 1 of requesting wheelchair assistance: call the airline on your phone before your flight so help is booked, which takes 10 to 15 minutes.
- Step 2 of requesting wheelchair assistance: at check-in, show your boarding pass to the staff.
- Step 3 of requesting wheelchair assistance: wait until the assistant arrives, for 15 to 30 minutes.
- Step 4 of requesting wheelchair assistance: board the aircraft first.

## checklist
- names what is needed: a phone and a boarding pass
- gives the steps in order: call the airline before the flight, show the boarding pass, wait for the assistant, board first
- says the call is made before the flight, not at the airport
- gives a total time of about 25 to 45 minutes

## variant
- change: no phone, so ask for the wheelchair at the check-in desk instead of calling
- request: I'm already at the airport and forgot to book help. My phone is dead. Can I still get a wheelchair?
- situation: I have my boarding pass but no phone, and no help is booked.
- expect: The reply says to skip the call, ask at the check-in desk, show the boarding pass, then wait for the assistant, about 20 to 40 minutes.

```datalog situation
lacks(user, phone).
has(user, boarding_pass).
```

```datalog difference
drop(request_wheelchair_assistance_s1).
no_need(phone).
extra_need(check_in_desk).
add_step(request_wheelchair_assistance_s9, start).
step(request_wheelchair_assistance, request_wheelchair_assistance_s9).
action(request_wheelchair_assistance_s9, ask).
object_of(request_wheelchair_assistance_s9, staff).
uses(request_wheelchair_assistance_s9, check_in_desk).
role(request_wheelchair_assistance_s9, check_in_desk, contact).
takes_time(request_wheelchair_assistance_s9, 2, 5, minute).
before(request_wheelchair_assistance_s9, request_wheelchair_assistance_s2).
change(request_wheelchair_assistance, takes_time, 25_45, 20_40).
```

- change: already booked online, so skip the phone call
- request: I already booked wheelchair help online. Do I still have to phone the airline?
- situation: I have my phone and boarding pass, and I booked the help online last week.
- expect: The reply says no call is needed since help is already booked, so go straight to showing the boarding pass and waiting for the assistant, about 15 to 30 minutes.

```datalog situation
has(user, phone).
has(user, boarding_pass).
has(user, help_booked).
```

```datalog difference
drop(request_wheelchair_assistance_s1).
no_need(phone).
change(request_wheelchair_assistance, takes_time, 25_45, 15_30).
```
