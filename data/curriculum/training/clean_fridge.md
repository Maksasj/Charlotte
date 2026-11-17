---
id: clean_fridge
stage: household
task: clean_fridge
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
uses(clean_fridge_s1, cool_box).
takes_time(clean_fridge_s1, 5, 10, minute).
step(clean_fridge, clean_fridge_s2).
action(clean_fridge_s2, throw).
object_of(clean_fridge_s2, food).
uses(clean_fridge_s2, bin_bag).
takes_time(clean_fridge_s2, 3, 5, minute).
step(clean_fridge, clean_fridge_s3).
action(clean_fridge_s3, remove).
object_of(clean_fridge_s3, shelf).
takes_time(clean_fridge_s3, 3, 5, minute).
step(clean_fridge, clean_fridge_s4).
action(clean_fridge_s4, wash).
object_of(clean_fridge_s4, shelf).
uses(clean_fridge_s4, water).
uses(clean_fridge_s4, dish_soap).
uses(clean_fridge_s4, sponge).
takes_time(clean_fridge_s4, 10, 15, minute).
step(clean_fridge, clean_fridge_s5).
action(clean_fridge_s5, wipe).
object_of(clean_fridge_s5, fridge).
uses(clean_fridge_s5, sponge).
uses(clean_fridge_s5, baking_soda).
takes_time(clean_fridge_s5, 10, 15, minute).
step(clean_fridge, clean_fridge_s6).
action(clean_fridge_s6, dry).
object_of(clean_fridge_s6, shelf).
uses(clean_fridge_s6, towel).
takes_time(clean_fridge_s6, 5, 10, minute).
before(clean_fridge_s1, clean_fridge_s2).
before(clean_fridge_s2, clean_fridge_s3).
before(clean_fridge_s3, clean_fridge_s4).
before(clean_fridge_s4, clean_fridge_s5).
before(clean_fridge_s5, clean_fridge_s6).
```

## teach
- To clean the fridge you need a cool box, a bin bag, dish soap, warm water, baking soda, a sponge and a towel.
- Cleaning the fridge takes 40 to 60 minutes.
- Step 1 of cleaning the fridge: take everything out of the fridge and put the cold food in the cool box for 5 to 10 minutes.
- Step 2 of cleaning the fridge: throw any food past its date into the bin bag for 3 to 5 minutes.
- Step 3 of cleaning the fridge: remove the shelves and drawers for 3 to 5 minutes.
- Step 4 of cleaning the fridge: wash the shelves and drawers with the sponge in warm water and dish soap for 10 to 15 minutes.
- Step 5 of cleaning the fridge: wipe the inside of the fridge with the sponge and a little baking soda in water for 10 to 15 minutes.
- Step 6 of cleaning the fridge: dry the shelves with the towel, put them back and return the food, for 5 to 10 minutes.

## checklist
- names what is needed: a cool box, a bin bag, dish soap, warm water, baking soda, a sponge and a towel
- gives the steps in order: empty fridge, throw food, remove shelf, wash shelf, wipe fridge, dry shelf
- gives a total time of about 40 to 60 minutes
- closes politely
