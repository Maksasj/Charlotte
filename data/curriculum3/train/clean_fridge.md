---
id: clean_fridge
stage: household
task: clean_fridge
schema: v3
---
# Clean the fridge

## requests
- Something smells in my fridge. How do I give it a proper clean?
- Can you walk me through cleaning out the fridge, shelves and all?
- I haven't cleaned my fridge in months. Where do I start?

## situation
- I have a cool box, a bin bag, dish soap, warm water, baking soda, a sponge and a towel.
- I have about an hour.

## ground truth
```datalog
task(clean_fridge).
domain(clean_fridge, cleaning).
goal(clean_fridge, fresh_fridge).
needs(clean_fridge, cool_box).
needs(clean_fridge, bin_bag).
needs(clean_fridge, dish_soap).
needs(clean_fridge, water).
needs(clean_fridge, baking_soda).
needs(clean_fridge, sponge).
needs(clean_fridge, towel).
takes_time(clean_fridge, 40, 60, minute).
step(clean_fridge, clean_fridge_s1).
action(clean_fridge_s1, empty).
object_of(clean_fridge_s1, fridge).
step(clean_fridge, clean_fridge_s2).
action(clean_fridge_s2, put).
object_of(clean_fridge_s2, food).
uses(clean_fridge_s2, cool_box).
role(clean_fridge_s2, cool_box, contain).
into(clean_fridge_s2, cool_box).
takes_time(clean_fridge_s2, 5, 10, minute).
step(clean_fridge, clean_fridge_s3).
action(clean_fridge_s3, throw).
object_of(clean_fridge_s3, food).
uses(clean_fridge_s3, bin_bag).
role(clean_fridge_s3, bin_bag, contain).
into(clean_fridge_s3, bin_bag).
condition(clean_fridge_s3, past_date).
takes_time(clean_fridge_s3, 3, 5, minute).
step(clean_fridge, clean_fridge_s4).
action(clean_fridge_s4, remove).
object_of(clean_fridge_s4, shelf).
takes_time(clean_fridge_s4, 3, 5, minute).
step(clean_fridge, clean_fridge_s5).
action(clean_fridge_s5, wash).
object_of(clean_fridge_s5, shelf).
uses(clean_fridge_s5, sponge).
role(clean_fridge_s5, sponge, scrub).
uses(clean_fridge_s5, water).
role(clean_fridge_s5, water, clean).
uses(clean_fridge_s5, dish_soap).
role(clean_fridge_s5, dish_soap, clean).
takes_time(clean_fridge_s5, 10, 15, minute).
step(clean_fridge, clean_fridge_s6).
action(clean_fridge_s6, wipe).
object_of(clean_fridge_s6, fridge).
uses(clean_fridge_s6, sponge).
role(clean_fridge_s6, sponge, wipe).
uses(clean_fridge_s6, baking_soda).
role(clean_fridge_s6, baking_soda, clean).
uses(clean_fridge_s6, water).
role(clean_fridge_s6, water, clean).
with_part(clean_fridge_s6, inside).
takes_time(clean_fridge_s6, 10, 15, minute).
step(clean_fridge, clean_fridge_s7).
action(clean_fridge_s7, dry).
object_of(clean_fridge_s7, shelf).
uses(clean_fridge_s7, towel).
role(clean_fridge_s7, towel, dry).
step(clean_fridge, clean_fridge_s8).
action(clean_fridge_s8, put).
object_of(clean_fridge_s8, shelf).
into(clean_fridge_s8, fridge).
step(clean_fridge, clean_fridge_s9).
action(clean_fridge_s9, return).
object_of(clean_fridge_s9, food).
into(clean_fridge_s9, fridge).
takes_time(clean_fridge_s9, 5, 10, minute).
before(clean_fridge_s1, clean_fridge_s2).
before(clean_fridge_s2, clean_fridge_s3).
before(clean_fridge_s3, clean_fridge_s4).
before(clean_fridge_s4, clean_fridge_s5).
before(clean_fridge_s5, clean_fridge_s6).
before(clean_fridge_s6, clean_fridge_s7).
before(clean_fridge_s7, clean_fridge_s8).
before(clean_fridge_s8, clean_fridge_s9).
```

## aliases
```datalog
task_alias(clean_fridge, "cleaning the fridge").
task_alias(clean_fridge, "cleaning out the fridge").
alias(empty, "take everything out").
alias(food, "cold food").
alias(shelf, "shelves").
alias(shelf, "shelves and drawers").
alias(water, "warm water").
alias(bin_bag, "bin").
alias(past_date, "past its date").
alias(put, "put back").
alias(return, "put back").
```

## teach
- To clean the fridge you need a cool box, a bin bag, dish soap, warm water, baking soda, a sponge and a towel.
- Cleaning the fridge takes 40 to 60 minutes.
- Step 1 of cleaning the fridge: take everything out of the fridge.
- Step 2 of cleaning the fridge: put the cold food in the cool box for 5 to 10 minutes.
- Step 3 of cleaning the fridge: throw any food past its date into the bin bag for 3 to 5 minutes.
- Step 4 of cleaning the fridge: remove the shelves and drawers for 3 to 5 minutes.
- Step 5 of cleaning the fridge: wash the shelves and drawers with the sponge in warm water and dish soap for 10 to 15 minutes.
- Step 6 of cleaning the fridge: wipe the inside of the fridge with the sponge and a little baking soda in water for 10 to 15 minutes.
- Step 7 of cleaning the fridge: dry the shelves with the towel.
- Step 8 of cleaning the fridge: put the shelves back.
- Step 9 of cleaning the fridge: return the food, for 5 to 10 minutes.

## checklist
- names what is needed: a cool box, a bin bag, dish soap, warm water, baking soda, a sponge and a towel
- gives the steps in order: empty fridge, throw food, remove shelf, wash shelf, wipe fridge, dry shelf
- gives a total time of about 40 to 60 minutes
- closes politely
