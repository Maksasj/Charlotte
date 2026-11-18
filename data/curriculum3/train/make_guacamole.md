---
id: make_guacamole
stage: procedures
task: make_guacamole
schema: v3
---
# Make guacamole

## requests
- Friends are coming over and I want to make guacamole. How do I do it?
- Can you give me a simple guacamole recipe with ripe avocados?
- What do I need to mash up a quick avocado dip?

## situation
- I have avocados, a lime, an onion, salt, a knife, a bowl and a fork.
- They arrive in 20 minutes.

## ground truth
```datalog
task(make_guacamole).
domain(make_guacamole, cooking).
goal(make_guacamole, guacamole).
needs(make_guacamole, avocado).
needs(make_guacamole, lime).
needs(make_guacamole, onion).
needs(make_guacamole, salt).
needs(make_guacamole, knife).
needs(make_guacamole, bowl).
needs(make_guacamole, fork).
takes_time(make_guacamole, 10, 14, minute).
step(make_guacamole, make_guacamole_s1).
action(make_guacamole_s1, chop).
object_of(make_guacamole_s1, onion).
uses(make_guacamole_s1, knife).
role(make_guacamole_s1, knife, cut).
takes_time(make_guacamole_s1, 3, 4, minute).
step(make_guacamole, make_guacamole_s2).
action(make_guacamole_s2, halve).
object_of(make_guacamole_s2, avocado).
uses(make_guacamole_s2, knife).
role(make_guacamole_s2, knife, cut).
step(make_guacamole, make_guacamole_s3).
action(make_guacamole_s3, scoop).
object_of(make_guacamole_s3, avocado).
uses(make_guacamole_s3, bowl).
role(make_guacamole_s3, bowl, contain).
into(make_guacamole_s3, bowl).
with_part(make_guacamole_s3, flesh).
takes_time(make_guacamole_s3, 2, 3, minute).
step(make_guacamole, make_guacamole_s4).
action(make_guacamole_s4, mash).
object_of(make_guacamole_s4, avocado).
uses(make_guacamole_s4, fork).
role(make_guacamole_s4, fork, press).
until(make_guacamole_s4, chunky).
takes_time(make_guacamole_s4, 2, 3, minute).
step(make_guacamole, make_guacamole_s5).
action(make_guacamole_s5, mix).
object_of(make_guacamole_s5, onion).
uses(make_guacamole_s5, lime).
role(make_guacamole_s5, lime, flavour).
uses(make_guacamole_s5, salt).
role(make_guacamole_s5, salt, season).
amount(make_guacamole_s5, salt, 1, pinch).
takes_time(make_guacamole_s5, 1, 2, minute).
before(make_guacamole_s1, make_guacamole_s2).
before(make_guacamole_s2, make_guacamole_s3).
before(make_guacamole_s3, make_guacamole_s4).
before(make_guacamole_s4, make_guacamole_s5).
```

## aliases
```datalog
task_alias(make_guacamole, "making guacamole").
task_alias(make_guacamole, "avocado dip").
task_alias(make_guacamole, "guac").
alias(avocado, "avocados").
alias(avocado, "ripe avocados").
alias(lime, "juice of the lime").
alias(lime, "lime juice").
alias(lemon, "lemon juice").
alias(salt, "pinch of salt").
alias(chunky, "only a little chunky").
alias(mix, "mix in").
```

## teach
- To make guacamole you need avocados, a lime, an onion, salt, a knife, a bowl and a fork.
- Making guacamole takes 10 to 14 minutes.
- Step 1 of making guacamole: chop the onion finely with the knife for 3 to 4 minutes.
- Step 2 of making guacamole: halve the avocados with the knife.
- Step 3 of making guacamole: scoop the flesh into the bowl for 2 to 3 minutes.
- Step 4 of making guacamole: mash the avocado with the fork until it is only a little chunky, for 2 to 3 minutes.
- Step 5 of making guacamole: mix in the onion, the juice of the lime and a pinch of salt, for 1 to 2 minutes.

## checklist
- names what is needed: avocados, a lime, an onion, salt, a knife, a bowl and a fork
- gives the steps in order: chop onion, halve avocado, mash avocado, mix onion
- gives a total time of about 10 to 14 minutes
- closes politely

## variant
- change: no lime: use lemon juice instead
- request: I want to make guacamole but there's no lime in the house, only a lemon. Will that work?
- situation: I have avocados, a lemon, an onion, salt, a knife, a bowl and a fork.
- expect: The reply uses lemon juice in place of lime juice and keeps the chop, halve, mash and mix steps.

```datalog situation
has(user, avocado).
has(user, lemon).
has(user, onion).
has(user, salt).
has(user, knife).
has(user, bowl).
has(user, fork).
lacks(user, lime).
```

```datalog difference
swap(make_guacamole_s5, lime, lemon).
extra_need(lemon).
no_need(lime).
```
