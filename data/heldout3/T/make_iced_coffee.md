---
id: make_iced_coffee
stage: procedures
task: make_iced_coffee
schema: v3
tier: T
---
# Make iced coffee

## requests
- How do I make an iced coffee at home?
- It's boiling hot today and I want a cold coffee. What's the easiest way?
- Can you show me how to make one of those chilled coffees with milk over ice?

## situation
- I have instant coffee, a kettle, milk, ice cubes, a tall glass and a spoon.

## ground truth
```datalog
task(make_iced_coffee).
domain(make_iced_coffee, cooking).
goal(make_iced_coffee, iced_coffee).
goal(make_iced_coffee, cold_drink).
needs(make_iced_coffee, instant_coffee).
needs(make_iced_coffee, hot_water).
needs(make_iced_coffee, milk).
needs(make_iced_coffee, ice).
needs(make_iced_coffee, glass).
needs(make_iced_coffee, spoon).
needs(make_iced_coffee, kettle).
serves(make_iced_coffee, 1).
takes_time(make_iced_coffee, 5, 7, minute).
step(make_iced_coffee, make_iced_coffee_s1).
action(make_iced_coffee_s1, boil).
object_of(make_iced_coffee_s1, water).
uses(make_iced_coffee_s1, kettle).
role(make_iced_coffee_s1, kettle, heat).
takes_time(make_iced_coffee_s1, 2, 3, minute).
step(make_iced_coffee, make_iced_coffee_s2).
action(make_iced_coffee_s2, dissolve).
object_of(make_iced_coffee_s2, instant_coffee).
uses(make_iced_coffee_s2, hot_water).
role(make_iced_coffee_s2, hot_water, mix).
uses(make_iced_coffee_s2, spoon).
role(make_iced_coffee_s2, spoon, stir).
amount(make_iced_coffee_s2, instant_coffee, 2, teaspoon).
amount(make_iced_coffee_s2, hot_water, 50, millilitre).
step(make_iced_coffee, make_iced_coffee_s3).
action(make_iced_coffee_s3, fill).
object_of(make_iced_coffee_s3, glass).
uses(make_iced_coffee_s3, ice).
role(make_iced_coffee_s3, ice, cool).
overlaps(make_iced_coffee_s3, make_iced_coffee_s1).
step(make_iced_coffee, make_iced_coffee_s4).
action(make_iced_coffee_s4, pour).
object_of(make_iced_coffee_s4, coffee).
into(make_iced_coffee_s4, glass).
step(make_iced_coffee, make_iced_coffee_s5).
action(make_iced_coffee_s5, top).
object_of(make_iced_coffee_s5, milk).
into(make_iced_coffee_s5, glass).
amount(make_iced_coffee_s5, milk, 150, millilitre).
before(make_iced_coffee_s1, make_iced_coffee_s2).
before(make_iced_coffee_s2, make_iced_coffee_s4).
before(make_iced_coffee_s3, make_iced_coffee_s4).
before(make_iced_coffee_s4, make_iced_coffee_s5).
```

## aliases
```datalog
task_alias(make_iced_coffee, "iced coffee").
task_alias(make_iced_coffee, "a cold coffee").
task_alias(make_iced_coffee, "chilled coffee").
alias(instant_coffee, "coffee").
alias(ice, "ice cubes").
alias(hot_water, "hot water").
alias(top, "top up").
```

## teach
- To make iced coffee you need instant coffee, hot water from a kettle, milk, ice, a glass and a spoon.
- Making iced coffee takes 5 to 7 minutes.
- Step 1 of making iced coffee: boil the water in the kettle for 2 to 3 minutes.
- Step 2 of making iced coffee: dissolve 2 teaspoons of instant coffee in 50 millilitres of hot water, stirring with the spoon.
- While step 1 runs, step 3 of making iced coffee: fill the glass with ice.
- Step 4 of making iced coffee: pour the coffee over the ice in the glass.
- Step 5 of making iced coffee: top up the glass with 150 millilitres of milk.

## checklist
- names what is needed, including instant coffee, milk, ice and a glass
- gives the steps in order: boil water, dissolve coffee, fill glass with ice, pour coffee, top up with milk
- says the glass can be filled with ice while the water boils

## variant
- change: no dairy: use oat milk
- request: I'm off dairy at the moment. Can I still make an iced coffee?
- situation: I have instant coffee, a kettle, oat milk, ice, a glass and a spoon. I avoid dairy.
- expect: The reply uses oat milk instead of cow's milk to top up the glass.

```datalog situation
has(user, instant_coffee).
has(user, kettle).
has(user, hot_water).
has(user, oat_milk).
has(user, ice).
has(user, glass).
has(user, spoon).
excludes(user, dairy).
```

```datalog difference
change(make_iced_coffee_s5, object_of, milk, oat_milk).
extra_need(oat_milk).
no_need(milk).
```

## variant
- change: no ice: chill the coffee in the fridge instead
- request: I want iced coffee but my freezer has no ice in it. What now?
- situation: I have instant coffee, a kettle, milk, a glass, a spoon and a fridge, but no ice.
- expect: The reply skips the ice, chills the coffee and milk in the fridge for about 20 minutes instead and then pours them into the glass.

```datalog situation
has(user, instant_coffee).
has(user, kettle).
has(user, hot_water).
has(user, milk).
has(user, glass).
has(user, spoon).
has(user, fridge).
lacks(user, ice).
```

```datalog difference
drop(make_iced_coffee_s3).
no_need(ice).
extra_need(fridge).
add_step(make_iced_coffee_s9, make_iced_coffee_s2).
step(make_iced_coffee, make_iced_coffee_s9).
action(make_iced_coffee_s9, chill).
object_of(make_iced_coffee_s9, coffee).
uses(make_iced_coffee_s9, fridge).
role(make_iced_coffee_s9, fridge, cool).
takes_time(make_iced_coffee_s9, 20, 20, minute).
before(make_iced_coffee_s2, make_iced_coffee_s9).
before(make_iced_coffee_s9, make_iced_coffee_s4).
change(make_iced_coffee, takes_time, 5_7, 25_30).
```
