---
id: make_fruit_salad
stage: procedures
task: make_fruit_salad
schema: v3
tier: T
---
# Make a fruit salad

## requests
- How do I put together a nice fruit salad?
- I've got a fruit bowl full of stuff going soft. What fresh dessert can I make with it?
- Could you help me make a bowl of mixed chopped fruit for a brunch?

## situation
- I have two apples, two bananas, an orange, a bunch of grapes, a lemon, a knife, a chopping board and a big bowl.

## ground truth
```datalog
task(make_fruit_salad).
domain(make_fruit_salad, cooking).
goal(make_fruit_salad, fruit_salad).
goal(make_fruit_salad, dessert).
needs(make_fruit_salad, apple).
needs(make_fruit_salad, banana).
needs(make_fruit_salad, orange).
needs(make_fruit_salad, grape).
needs(make_fruit_salad, lemon).
needs(make_fruit_salad, knife).
needs(make_fruit_salad, chopping_board).
needs(make_fruit_salad, bowl).
serves(make_fruit_salad, 4).
takes_time(make_fruit_salad, 10, 15, minute).
step(make_fruit_salad, make_fruit_salad_s1).
action(make_fruit_salad_s1, cut).
object_of(make_fruit_salad_s1, apple).
uses(make_fruit_salad_s1, knife).
role(make_fruit_salad_s1, knife, cut).
uses(make_fruit_salad_s1, chopping_board).
role(make_fruit_salad_s1, chopping_board, hold).
step(make_fruit_salad, make_fruit_salad_s2).
action(make_fruit_salad_s2, slice).
object_of(make_fruit_salad_s2, banana).
uses(make_fruit_salad_s2, knife).
role(make_fruit_salad_s2, knife, cut).
step(make_fruit_salad, make_fruit_salad_s3).
action(make_fruit_salad_s3, peel).
object_of(make_fruit_salad_s3, orange).
step(make_fruit_salad, make_fruit_salad_s4).
action(make_fruit_salad_s4, halve).
object_of(make_fruit_salad_s4, grape).
uses(make_fruit_salad_s4, knife).
role(make_fruit_salad_s4, knife, cut).
step(make_fruit_salad, make_fruit_salad_s5).
action(make_fruit_salad_s5, toss).
object_of(make_fruit_salad_s5, fruit).
into(make_fruit_salad_s5, bowl).
uses(make_fruit_salad_s5, bowl).
role(make_fruit_salad_s5, bowl, contain).
uses(make_fruit_salad_s5, lemon).
role(make_fruit_salad_s5, lemon, flavour).
purpose(make_fruit_salad_s5, no_browning).
before(make_fruit_salad_s1, make_fruit_salad_s2).
before(make_fruit_salad_s2, make_fruit_salad_s3).
before(make_fruit_salad_s3, make_fruit_salad_s4).
before(make_fruit_salad_s1, make_fruit_salad_s5).
before(make_fruit_salad_s2, make_fruit_salad_s5).
before(make_fruit_salad_s3, make_fruit_salad_s5).
before(make_fruit_salad_s4, make_fruit_salad_s5).
```

## aliases
```datalog
task_alias(make_fruit_salad, "fruit salad").
task_alias(make_fruit_salad, "a bowl of chopped fruit").
task_alias(make_fruit_salad, "mixed fruit").
alias(grape, "grapes").
alias(apple, "apples").
alias(banana, "bananas").
alias(lemon, "lemon juice").
alias(no_browning, "stops it going brown").
alias(chopping_board, "cutting board").
```

## teach
- To make a fruit salad you need apples, bananas, an orange, grapes, a lemon, a knife, a chopping board and a bowl.
- Making a fruit salad takes 10 to 15 minutes and serves 4.
- Step 1 of making a fruit salad: cut the apples into chunks with the knife on the chopping board.
- Step 2 of making a fruit salad: slice the bananas with the knife.
- Step 3 of making a fruit salad: peel the orange and break it into segments.
- Step 4 of making a fruit salad: halve the grapes with the knife.
- Step 5 of making a fruit salad: toss all the fruit in the bowl with lemon juice so it does not go brown.

## checklist
- names what is needed, including the fruit, a lemon, a knife and a bowl
- gives the steps: cut apple, slice banana, peel orange, halve grapes, toss with lemon in the bowl
- says the lemon juice stops the fruit going brown

## variant
- change: no lemon: use a splash of orange juice from a carton
- request: I want to make fruit salad but I'm out of lemons. Will the apple go brown?
- situation: I have apples, bananas, an orange, grapes, a carton of orange juice, a knife, a board and a bowl.
- expect: The reply tosses the fruit in a splash of orange juice instead of lemon juice to slow browning.

```datalog situation
has(user, apple).
has(user, banana).
has(user, orange).
has(user, grape).
has(user, orange_juice).
has(user, knife).
has(user, chopping_board).
has(user, bowl).
lacks(user, lemon).
```

```datalog difference
swap(make_fruit_salad_s5, lemon, orange_juice).
extra_need(orange_juice).
no_need(lemon).
```

## variant
- change: 12 people: triple the fruit
- request: I'm bringing fruit salad to a brunch for twelve. How much should I make?
- situation: I have plenty of fruit, lemons, a knife, a board and a large bowl. There will be 12 people.
- expect: The reply triples the fruit for 12 people and allows more time to cut it, about 30 to 40 minutes.

```datalog situation
has(user, apple).
has(user, banana).
has(user, orange).
has(user, grape).
has(user, lemon).
has(user, knife).
has(user, chopping_board).
has(user, bowl).
count(user, guest, 12).
```

```datalog difference
change(make_fruit_salad, serves, 4, 12).
change(make_fruit_salad, takes_time, 10_15, 30_40).
```
