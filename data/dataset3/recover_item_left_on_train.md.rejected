---
id: recover_item_left_on_train
stage: procedures
task: recover_item_left_on_train
schema: v3
---
# Recover an item left on a train

## requests
- I left my bag on a train this morning. How do I get it back?
- What's the process for reclaiming something I forgot on a train?
- Someone told me to call the rail company and fill in a form for my lost bag. What steps should I follow?

## situation
- I have my phone, my ticket, a pen and a notebook.
- I left my bag on a train this morning.

## ground truth
```datalog
task(recover_item_left_on_train).
domain(recover_item_left_on_train, travel).
goal(recover_item_left_on_train, item_returned).
needs(recover_item_left_on_train, phone).
needs(recover_item_left_on_train, ticket).
needs(recover_item_left_on_train, pen).
needs(recover_item_left_on_train, notebook).
takes_time(recover_item_left_on_train, 30, 50, minute).
step(recover_item_left_on_train, recover_item_left_on_train_s1).
action(recover_item_left_on_train_s1, call).
object_of(recover_item_left_on_train_s1, train_operator).
uses(recover_item_left_on_train_s1, phone).
role(recover_item_left_on_train_s1, phone, hold).
purpose(recover_item_left_on_train_s1, reference_number).
takes_time(recover_item_left_on_train_s1, 10, 15, minute).
step(recover_item_left_on_train, recover_item_left_on_train_s2).
action(recover_item_left_on_train_s2, write).
object_of(recover_item_left_on_train_s2, reference_number).
uses(recover_item_left_on_train_s2, notebook).
role(recover_item_left_on_train_s2, notebook, contain).
uses(recover_item_left_on_train_s2, pen).
role(recover_item_left_on_train_s2, pen, mark).
takes_time(recover_item_left_on_train_s2, 2, 3, minute).
step(recover_item_left_on_train, recover_item_left_on_train_s3).
action(recover_item_left_on_train_s3, fill).
object_of(recover_item_left_on_train_s3, lost_property_form).
uses(recover_item_left_on_train_s3, pen).
role(recover_item_left_on_train_s3, pen, mark).
takes_time(recover_item_left_on_train_s3, 5, 10, minute).
step(recover_item_left_on_train, recover_item_left_on_train_s4).
action(recover_item_left_on_train_s4, send).
object_of(recover_item_left_on_train_s4, lost_property_form).
uses(recover_item_left_on_train_s4, phone).
role(recover_item_left_on_train_s4, phone, hold).
takes_time(recover_item_left_on_train_s4, 2, 5, minute).
step(recover_item_left_on_train, recover_item_left_on_train_s5).
action(recover_item_left_on_train_s5, collect).
object_of(recover_item_left_on_train_s5, lost_item).
uses(recover_item_left_on_train_s5, ticket).
role(recover_item_left_on_train_s5, ticket, hold).
check(recover_item_left_on_train_s5, item_in_hand).
takes_time(recover_item_left_on_train_s5, 10, 20, minute).
before(recover_item_left_on_train_s1, recover_item_left_on_train_s2).
before(recover_item_left_on_train_s2, recover_item_left_on_train_s3).
before(recover_item_left_on_train_s3, recover_item_left_on_train_s4).
before(recover_item_left_on_train_s4, recover_item_left_on_train_s5).
```

## aliases
```datalog
task_alias(recover_item_left_on_train, "getting back something left on a train").
task_alias(recover_item_left_on_train, "recovering lost luggage").
alias(lost_item, "bag").
alias(train_operator, "rail company").
alias(lost_property_form, "form").
alias(lost_property_office, "lost property").
alias(reference_number, "reference").
alias(booking_email, "booking email").
alias(item_in_hand, "item in hand").
```

## teach
- To recover an item left on a train you need a phone, a ticket, a pen and a notebook.
- Recovering an item left on a train takes 30 to 50 minutes.
- Step 1 of recovering an item left on a train: call the train operator on the phone to get a reference number, for 10 to 15 minutes.
- Step 2 of recovering an item left on a train: write the reference number in the notebook with a pen, for 2 to 3 minutes.
- Step 3 of recovering an item left on a train: fill in the lost property form with the pen, for 5 to 10 minutes.
- Step 4 of recovering an item left on a train: send the form to the lost property office from the phone, for 2 to 5 minutes.
- Step 5 of recovering an item left on a train: collect the bag from the lost property office, showing the ticket, for 10 to 20 minutes.

## checklist
- names what is needed, including a phone, a ticket, a pen and a notebook
- gives the steps in order: call the operator, write the reference, fill in the form, send the form, collect the bag
- says the reference number from the call is needed for the form
- gives a total time of about 30 to 50 minutes

## variant
- change: no ticket, so show the booking confirmation email instead when collecting the bag.
- request: I lost my ticket but I still have the booking email. Can I still get my bag back?
- situation: I have a phone, a pen, a notebook and the booking email, but no ticket.
- expect: The reply keeps all five steps and says to show the booking confirmation email at the lost property office in place of the ticket.

```datalog situation
has(user, phone).
has(user, pen).
has(user, notebook).
has(user, booking_email).
lacks(user, ticket).
```

```datalog difference
swap(recover_item_left_on_train_s5, ticket, booking_email).
role(recover_item_left_on_train_s5, booking_email, hold).
no_need(ticket).
extra_need(booking_email).
```

## variant
- change: I am standing at the station and the lost property office is open, so I hand the form in myself and skip sending it online.
- request: I'm already at the station and the lost property office is open. Do I still need to send the form online?
- situation: I am at the station with my phone, ticket, pen and notebook, and the lost property office is open now.
- expect: The reply drops the step of sending the form from the phone and says to hand the completed form in at the office, cutting the total to about 20 to 35 minutes.

```datalog situation
has(user, phone).
has(user, ticket).
has(user, pen).
has(user, notebook).
has(user, lost_property_office_open).
```

```datalog difference
drop(recover_item_left_on_train_s4).
change(recover_item_left_on_train, takes_time, 30_50, 20_35).
```
