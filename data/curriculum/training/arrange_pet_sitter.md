---
id: arrange_pet_sitter
stage: social
task: arrange_pet_sitter
---
# Arrange a pet sitter

## requests
- I'm going away for a week and need someone to look after my dog. How do I set that up?
- Can you help me arrange a cat sitter while I'm on holiday?
- What should I sort out before leaving my pet with a sitter?

## situation
- I have pet food, a spare key, paper and a pen.
- I leave in two weeks.

## ground truth
```datalog
task(arrange_pet_sitter).
needs(arrange_pet_sitter, pet_food).
needs(arrange_pet_sitter, key).
needs(arrange_pet_sitter, paper).
needs(arrange_pet_sitter, pen).
takes_time(arrange_pet_sitter, 65, 90, minute).
step(arrange_pet_sitter, arrange_pet_sitter_s1).
action(arrange_pet_sitter_s1, find).
object_of(arrange_pet_sitter_s1, pet_sitter).
takes_time(arrange_pet_sitter_s1, 20, 30, minute).
step(arrange_pet_sitter, arrange_pet_sitter_s2).
action(arrange_pet_sitter_s2, meet).
object_of(arrange_pet_sitter_s2, pet_sitter).
takes_time(arrange_pet_sitter_s2, 20, 30, minute).
step(arrange_pet_sitter, arrange_pet_sitter_s3).
action(arrange_pet_sitter_s3, write).
object_of(arrange_pet_sitter_s3, instruction).
uses(arrange_pet_sitter_s3, paper).
uses(arrange_pet_sitter_s3, pen).
takes_time(arrange_pet_sitter_s3, 10, 15, minute).
step(arrange_pet_sitter, arrange_pet_sitter_s4).
action(arrange_pet_sitter_s4, buy).
object_of(arrange_pet_sitter_s4, pet_food).
takes_time(arrange_pet_sitter_s4, 10, 15, minute).
step(arrange_pet_sitter, arrange_pet_sitter_s5).
action(arrange_pet_sitter_s5, hand).
object_of(arrange_pet_sitter_s5, key).
takes_time(arrange_pet_sitter_s5, 5, 5, minute).
before(arrange_pet_sitter_s1, arrange_pet_sitter_s2).
before(arrange_pet_sitter_s2, arrange_pet_sitter_s3).
before(arrange_pet_sitter_s3, arrange_pet_sitter_s4).
before(arrange_pet_sitter_s4, arrange_pet_sitter_s5).
```

## teach
- To arrange a pet sitter you need pet food, a spare key, paper and a pen.
- Arranging a pet sitter takes 65 to 90 minutes.
- Step 1 of arranging a pet sitter: ask friends or neighbours to be the pet sitter, or find one on a sitting site, for 20 to 30 minutes.
- Step 2 of arranging a pet sitter: meet the pet sitter at home so they can meet your pet for 20 to 30 minutes.
- Step 3 of arranging a pet sitter: write down instructions on paper with the pen: feeding times, walks, the vet's number and your number, for 10 to 15 minutes.
- Step 4 of arranging a pet sitter: buy enough pet food for the whole trip for 10 to 15 minutes.
- Step 5 of arranging a pet sitter: hand over the spare key and the instructions for 5 minutes.

## checklist
- names what is needed: pet food, a spare key, paper and a pen
- gives the steps in order: find pet sitter, meet pet sitter, write instruction, buy pet food, hand key
- gives a total time of about 65 to 90 minutes
- closes politely
