---
id: organise_potluck
stage: social
task: organise_potluck
---
# Organise a potluck

## requests
- I'd like to host a potluck dinner where everyone brings a dish. How do I organise it?
- Can you help me plan a bring-a-dish party with my neighbours?
- How do I make sure a potluck doesn't end up with five desserts and no mains?

## situation
- I have a big table, plates, cutlery and serving spoons, and a group chat with the guests.
- The potluck is next Saturday.

## ground truth
```datalog
task(organise_potluck).
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
takes_time(organise_potluck_s2, 10, 15, minute).
step(organise_potluck, organise_potluck_s3).
action(organise_potluck_s3, make).
object_of(organise_potluck_s3, sign_up_list).
takes_time(organise_potluck_s3, 10, 15, minute).
step(organise_potluck, organise_potluck_s4).
action(organise_potluck_s4, set).
object_of(organise_potluck_s4, table).
uses(organise_potluck_s4, plate).
uses(organise_potluck_s4, cutlery).
uses(organise_potluck_s4, serving_spoon).
takes_time(organise_potluck_s4, 15, 20, minute).
before(organise_potluck_s1, organise_potluck_s2).
before(organise_potluck_s2, organise_potluck_s3).
before(organise_potluck_s3, organise_potluck_s4).
```

## teach
- To organise a potluck you need a sign-up list, plates, cutlery, serving spoons and a table.
- Organising a potluck takes 40 to 60 minutes.
- Step 1 of organising a potluck: choose a date and a place for the potluck for 5 to 10 minutes.
- Step 2 of organising a potluck: invite the guests and ask each one to bring a dish for 10 to 15 minutes.
- Step 3 of organising a potluck: make a sign-up list with mains, sides, salads and desserts so the dishes do not overlap, for 10 to 15 minutes.
- Step 4 of organising a potluck: set out the plates, the cutlery and the serving spoons on the table, for 15 to 20 minutes.

## checklist
- names what is needed: a sign-up list, plates, cutlery, serving spoons and a table
- gives the steps in order: choose date, invite guest, make sign up list, set table
- gives a total time of about 40 to 60 minutes
- closes politely

## variant
- change: a guest has a nut allergy: ask everyone to label their dish and keep nuts out
- request: One of the guests at my potluck has a nut allergy. How do I plan around that?
- situation: One guest is allergic to nuts. I have plates, cutlery, serving spoons, a table and a group chat with the guests.
- expect: The reply tells the guests about the allergy when inviting them, adds a column for ingredients to the sign-up list and asks for labels on each dish, keeping the other steps.
