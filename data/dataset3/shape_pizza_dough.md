---
id: shape_pizza_dough
stage: procedures
task: shape_pizza_dough
schema: v3
---
# Shape pizza dough

## requests
- I want to make a pizza base from scratch. How do I shape the dough?
- What's the method for making pizza dough and getting it ready to top?
- Can you take me through preparing pizza dough, from mixing to putting it on the tray?

## situation
- I have flour, water, a bowl, a spoon, a towel, olive oil, a rolling pin and a baking tray.
- I have about an hour to spare.

## ground truth
```datalog
task(shape_pizza_dough).
domain(shape_pizza_dough, cooking).
goal(shape_pizza_dough, pizza_base).
needs(shape_pizza_dough, flour).
needs(shape_pizza_dough, water).
needs(shape_pizza_dough, bowl).
needs(shape_pizza_dough, spoon).
needs(shape_pizza_dough, towel).
needs(shape_pizza_dough, olive_oil).
needs(shape_pizza_dough, rolling_pin).
needs(shape_pizza_dough, baking_tray).
takes_time(shape_pizza_dough, 45, 60, minute).
step(shape_pizza_dough, shape_pizza_dough_s1).
action(shape_pizza_dough_s1, mix).
object_of(shape_pizza_dough_s1, flour).
uses(shape_pizza_dough_s1, water).
role(shape_pizza_dough_s1, water, mix).
uses(shape_pizza_dough_s1, bowl).
role(shape_pizza_dough_s1, bowl, contain).
uses(shape_pizza_dough_s1, spoon).
role(shape_pizza_dough_s1, spoon, stir).
takes_time(shape_pizza_dough_s1, 5, 5, minute).
step(shape_pizza_dough, shape_pizza_dough_s2).
action(shape_pizza_dough_s2, cover).
object_of(shape_pizza_dough_s2, dough).
uses(shape_pizza_dough_s2, towel).
role(shape_pizza_dough_s2, towel, cover).
until(shape_pizza_dough_s2, risen).
takes_time(shape_pizza_dough_s2, 30, 30, minute).
step(shape_pizza_dough, shape_pizza_dough_s3).
action(shape_pizza_dough_s3, knead).
object_of(shape_pizza_dough_s3, dough).
uses(shape_pizza_dough_s3, olive_oil).
role(shape_pizza_dough_s3, olive_oil, coat).
purpose(shape_pizza_dough_s3, elastic_dough).
takes_time(shape_pizza_dough_s3, 10, 10, minute).
step(shape_pizza_dough, shape_pizza_dough_s4).
action(shape_pizza_dough_s4, roll).
object_of(shape_pizza_dough_s4, dough).
uses(shape_pizza_dough_s4, rolling_pin).
role(shape_pizza_dough_s4, rolling_pin, spread).
takes_time(shape_pizza_dough_s4, 5, 5, minute).
step(shape_pizza_dough, shape_pizza_dough_s5).
action(shape_pizza_dough_s5, place).
object_of(shape_pizza_dough_s5, dough).
onto(shape_pizza_dough_s5, baking_tray).
uses(shape_pizza_dough_s5, baking_tray).
role(shape_pizza_dough_s5, baking_tray, hold).
takes_time(shape_pizza_dough_s5, 2, 2, minute).
before(shape_pizza_dough_s1, shape_pizza_dough_s2).
before(shape_pizza_dough_s2, shape_pizza_dough_s3).
before(shape_pizza_dough_s3, shape_pizza_dough_s4).
before(shape_pizza_dough_s4, shape_pizza_dough_s5).
```

## aliases
```datalog
task_alias(shape_pizza_dough, "pizza base").
task_alias(shape_pizza_dough, "pizza dough").
alias(dough, "pizza dough").
alias(rolling_pin, "rolling pin").
alias(baking_tray, "baking sheet").
alias(olive_oil, "oil").
alias(towel, "tea towel").
alias(risen, "has risen").
alias(elastic_dough, "elastic").
```

## teach
- To shape pizza dough you need flour, water, a bowl, a spoon, a towel, olive oil, a rolling pin and a baking tray.
- Shaping pizza dough takes 45 to 60 minutes.
- Step 1 of shape pizza dough: mix the flour and water in the bowl with the spoon, which takes about 5 minutes.
- Step 2 of shape pizza dough: cover the dough with the towel and leave it until it has risen, about 30 minutes.
- Step 3 of shape pizza dough: knead the dough with a little olive oil for 10 minutes so it becomes elastic.
- Step 4 of shape pizza dough: roll the dough flat with the rolling pin for about 5 minutes.
- Step 5 of shape pizza dough: place the dough onto the baking tray, which takes about 2 minutes.

## checklist
- names what is needed, including flour, water, a bowl, a rolling pin and a baking tray
- gives the steps in order: mix, cover and rise, knead with oil, roll out, place on the tray
- says the dough has to rise under a towel before it is kneaded and rolled
- gives a total time of about 45 to 60 minutes

## variant
- change: no rolling pin: press and stretch the dough by hand into a round
- request: I don't own a rolling pin. Can I still shape the pizza dough?
- situation: I have flour, water, a bowl, a spoon, a towel, olive oil and a baking tray, but no rolling pin.
- expect: The reply says to press and stretch the dough by hand into a round in place of rolling it with a rolling pin.

```datalog situation
lacks(user, rolling_pin).
has(user, flour).
```

```datalog difference
swap(shape_pizza_dough_s4, rolling_pin, hands).
extra_need(hands).
no_need(rolling_pin).
```

- change: no tea towel: cover the dough with plastic wrap while it rises
- request: I've run out of towels. What can I cover the dough with while it rises?
- situation: I have flour, water, a bowl, a spoon, olive oil, a rolling pin, a baking tray and plastic wrap, but no towel.
- expect: The reply covers the dough with plastic wrap instead of the towel while it rises, then continues as usual.

```datalog situation
lacks(user, towel).
has(user, plastic_wrap).
```

```datalog difference
swap(shape_pizza_dough_s2, towel, plastic_wrap).
extra_need(plastic_wrap).
no_need(towel).
```
