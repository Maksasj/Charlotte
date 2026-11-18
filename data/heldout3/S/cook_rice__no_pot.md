---
id: cook_rice__no_pot
stage: procedures
task: cook_rice
schema: v3
tier: S
base: cook_rice
---
# Cook rice

## requests
- I'm in a hostel kitchen with a bag of rice, but there are no pots or pans at all, not even a microwave. How do I cook it?

## ground truth
```datalog
task(cook_rice).
domain(cook_rice, cooking).
goal(cook_rice, cooked_rice).
needs(cook_rice, rice).
needs(cook_rice, water).
needs(cook_rice, pot).
needs(cook_rice, sieve).
optional_need(cook_rice, lid).
takes_time(cook_rice, 25, 30, minute).
step(cook_rice, cook_rice_s1).
action(cook_rice_s1, boil).
object_of(cook_rice_s1, water).
uses(cook_rice_s1, pot).
role(cook_rice_s1, pot, heat).
takes_time(cook_rice_s1, 5, 6, minute).
step(cook_rice, cook_rice_s2).
action(cook_rice_s2, rinse).
object_of(cook_rice_s2, rice).
uses(cook_rice_s2, sieve).
role(cook_rice_s2, sieve, hold).
until(cook_rice_s2, clear).
purpose(cook_rice_s2, fluffy_rice).
takes_time(cook_rice_s2, 1, 2, minute).
overlaps(cook_rice_s2, cook_rice_s1).
step(cook_rice, cook_rice_s3).
action(cook_rice_s3, add).
object_of(cook_rice_s3, rice).
into(cook_rice_s3, water).
requires_state(cook_rice_s3, boiling).
step(cook_rice, cook_rice_s4).
action(cook_rice_s4, simmer).
object_of(cook_rice_s4, rice).
uses(cook_rice_s4, lid).
role(cook_rice_s4, lid, cover).
takes_time(cook_rice_s4, 15, 18, minute).
step(cook_rice, cook_rice_s5).
action(cook_rice_s5, rest).
object_of(cook_rice_s5, rice).
requires_state(cook_rice_s5, off_heat).
takes_time(cook_rice_s5, 5, 6, minute).
before(cook_rice_s1, cook_rice_s3).
before(cook_rice_s2, cook_rice_s3).
before(cook_rice_s3, cook_rice_s4).
before(cook_rice_s4, cook_rice_s5).
```

## aliases
```datalog
task_alias(cook_rice, "cooking rice").
task_alias(cook_rice, "plain rice").
task_alias(cook_rice, "white rice").
alias(rice, "white rice").
alias(rice, "plain rice").
alias(water, "boiling water").
alias(clear, "water runs clear").
alias(fluffy_rice, "fluffy").
alias(fluffy_rice, "not mushy").
alias(off_heat, "off the heat").
alias(lid, "lid on").
```

## teach
- To cook rice you need rice, water, a pot and a sieve.
- Cooking rice takes 25 to 30 minutes.
- Step 1 of cooking rice: boil the water in the pot for 5 to 6 minutes.
- While step 1 runs, step 2 of cooking rice: rinse the rice in the sieve until the water runs clear, for 1 to 2 minutes, so the rice cooks fluffy.
- Step 3 of cooking rice: add the rice to the boiling water.
- Step 4 of cooking rice: simmer the rice with the lid on for 15 to 18 minutes.
- Step 5 of cooking rice: rest the rice off the heat for 5 to 6 minutes.

## variant
- change: no pot or other vessel: rice cannot be cooked
- request: I'm in a hostel kitchen with a bag of rice, but there are no pots or pans at all, not even a microwave. How do I cook it?
- situation: I have rice, water and a sieve, but no pot, no pan and no microwave.
- expect: The reply says the rice cannot be cooked without a pot or something to boil it in.

```datalog situation
has(user, rice).
has(user, water).
has(user, sieve).
lacks(user, pot).
```

```datalog difference
cannot(no_pot).
```
