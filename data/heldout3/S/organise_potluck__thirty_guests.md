---
id: organise_potluck__thirty_guests
stage: social
task: organise_potluck
schema: v3
tier: S
base: organise_potluck
---
# Organise a potluck

## requests
- The bring-a-dish party has grown to thirty people and I only have plates for twelve. What do I need to sort?

## ground truth
```datalog
task(organise_potluck).
domain(organise_potluck, planning).
goal(organise_potluck, potluck).
needs(organise_potluck, sign_up_list).
needs(organise_potluck, plate).
needs(organise_potluck, cutlery).
needs(organise_potluck, serving_spoon).
needs(organise_potluck, table).
takes_time(organise_potluck, 40, 60, minute).
step(organise_potluck, organise_potluck_s1).
action(organise_potluck_s1, choose).
object_of(organise_potluck_s1, date).
takes_time(organise_potluck_s1, 5, 10, minute).
step(organise_potluck, organise_potluck_s2).
action(organise_potluck_s2, invite).
object_of(organise_potluck_s2, guest).
step(organise_potluck, organise_potluck_s3).
action(organise_potluck_s3, ask).
object_of(organise_potluck_s3, guest).
purpose(organise_potluck_s3, guest_brings_dish).
takes_time(organise_potluck_s3, 10, 15, minute).
step(organise_potluck, organise_potluck_s4).
action(organise_potluck_s4, make).
object_of(organise_potluck_s4, sign_up_list).
purpose(organise_potluck_s4, dishes_not_overlap).
takes_time(organise_potluck_s4, 10, 15, minute).
step(organise_potluck, organise_potluck_s5).
action(organise_potluck_s5, set).
object_of(organise_potluck_s5, table).
uses(organise_potluck_s5, plate).
role(organise_potluck_s5, plate, serve).
uses(organise_potluck_s5, cutlery).
role(organise_potluck_s5, cutlery, serve).
uses(organise_potluck_s5, serving_spoon).
role(organise_potluck_s5, serving_spoon, serve).
takes_time(organise_potluck_s5, 15, 20, minute).
before(organise_potluck_s1, organise_potluck_s2).
before(organise_potluck_s2, organise_potluck_s3).
before(organise_potluck_s3, organise_potluck_s4).
before(organise_potluck_s4, organise_potluck_s5).
```

## aliases
```datalog
task_alias(organise_potluck, "organising a potluck").
task_alias(organise_potluck, "potluck dinner").
task_alias(organise_potluck, "bring-a-dish party").
task_alias(organise_potluck, "host a potluck").
alias(guest, "guests").
alias(guest, "neighbours").
alias(plate, "plates").
alias(serving_spoon, "serving spoons").
alias(sign_up_list, "sign-up list").
alias(table, "big table").
alias(set, "set out").
alias(guest_brings_dish, "bring a dish").
alias(dishes_not_overlap, "the dishes do not overlap").
alias(nut, "nuts").
alias(nut, "nut allergy").
alias(ingredient_column, "a column for ingredients").
```

## teach
- To organise a potluck you need a sign-up list, plates, cutlery, serving spoons and a table.
- Organising a potluck takes 40 to 60 minutes.
- Step 1 of organising a potluck: choose a date and a place for the potluck for 5 to 10 minutes.
- Step 2 of organising a potluck: invite the guests.
- Step 3 of organising a potluck: ask each guest to bring a dish for 10 to 15 minutes.
- Step 4 of organising a potluck: make a sign-up list with mains, sides, salads and desserts so the dishes do not overlap, for 10 to 15 minutes.
- Step 5 of organising a potluck: set out the plates, the cutlery and the serving spoons on the table, for 15 to 20 minutes.

## variant
- change: thirty guests: borrow extra plates before setting the table
- request: The bring-a-dish party has grown to thirty people and I only have plates for twelve. What do I need to sort?
- situation: Thirty guests are coming. I have a big table, twelve plates, cutlery, serving spoons and a group chat.
- expect: The reply keeps the date, invites, dish requests and sign-up list, adds borrowing extra plates, then sets the table.

```datalog situation
has(user, table).
has(user, plate).
has(user, cutlery).
has(user, serving_spoon).
count(user, guest, 30).
```

```datalog difference
add_step(organise_potluck_s9, organise_potluck_s4).
step(organise_potluck, organise_potluck_s9).
action(organise_potluck_s9, borrow).
object_of(organise_potluck_s9, plate).
before(organise_potluck_s4, organise_potluck_s9).
before(organise_potluck_s9, organise_potluck_s5).
```
