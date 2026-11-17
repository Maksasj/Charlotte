---
id: wash_dishes
stage: household
task: wash_dishes
---
# Wash the dishes

## requests
- My dishwasher broke and there's a pile of dirty plates. How do I wash them by hand?
- What's the right way to do the washing up in the sink?
- Could you tell me how to hand-wash dishes so they actually come out clean?

## situation
- I have a sink, hot water, dish soap, a sponge and a drying rack.
- I'd like to be done in about 20 minutes.

## ground truth
```datalog
task(wash_dishes).
needs(wash_dishes, sink).
needs(wash_dishes, water).
needs(wash_dishes, dish_soap).
needs(wash_dishes, sponge).
needs(wash_dishes, drying_rack).
takes_time(wash_dishes, 15, 25, minute).
step(wash_dishes, wash_dishes_s1).
action(wash_dishes_s1, scrape).
object_of(wash_dishes_s1, plate).
takes_time(wash_dishes_s1, 2, 3, minute).
step(wash_dishes, wash_dishes_s2).
action(wash_dishes_s2, fill).
object_of(wash_dishes_s2, sink).
uses(wash_dishes_s2, water).
uses(wash_dishes_s2, dish_soap).
takes_time(wash_dishes_s2, 2, 3, minute).
step(wash_dishes, wash_dishes_s3).
action(wash_dishes_s3, wash).
object_of(wash_dishes_s3, dish).
uses(wash_dishes_s3, sponge).
takes_time(wash_dishes_s3, 8, 14, minute).
step(wash_dishes, wash_dishes_s4).
action(wash_dishes_s4, rinse).
object_of(wash_dishes_s4, dish).
uses(wash_dishes_s4, water).
takes_time(wash_dishes_s4, 2, 3, minute).
step(wash_dishes, wash_dishes_s5).
action(wash_dishes_s5, put).
object_of(wash_dishes_s5, dish).
uses(wash_dishes_s5, drying_rack).
until(wash_dishes_s5, dry).
before(wash_dishes_s1, wash_dishes_s2).
before(wash_dishes_s2, wash_dishes_s3).
before(wash_dishes_s3, wash_dishes_s4).
before(wash_dishes_s4, wash_dishes_s5).
```

## teach
- To wash the dishes you need a sink, water, dish soap, a sponge and a drying rack.
- Washing the dishes takes 15 to 25 minutes.
- Step 1 of washing the dishes: scrape the leftover food off each plate into the bin for 2 to 3 minutes.
- Step 2 of washing the dishes: fill the sink with hot water and a squirt of dish soap for 2 to 3 minutes.
- Step 3 of washing the dishes: wash each dish with the sponge, glasses first and pans last, for 8 to 14 minutes.
- Step 4 of washing the dishes: rinse each dish under clean water for 2 to 3 minutes.
- Step 5 of washing the dishes: put each dish on the drying rack until it is dry.

## checklist
- names what is needed, including hot water, dish soap and a sponge
- gives the steps in order: scrape, fill sink, wash, rinse, dry on the rack
- gives a total time of about 15 to 25 minutes
- closes politely
