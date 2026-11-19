---
id: clean_coffee_maker
stage: procedures
task: clean_coffee_maker
schema: v3
---
# Clean a coffee maker

## requests
- How do I descale my coffee maker with vinegar?
- What's the best way to clean out my coffee machine?
- Can you walk me through cleaning a coffee maker, inside and out?

## situation
- I have a coffee maker, white vinegar, a cloth and tap water.
- I have about 40 minutes free.

## ground truth
```datalog
task(clean_coffee_maker).
domain(clean_coffee_maker, cleaning).
goal(clean_coffee_maker, clean_appliance).
needs(clean_coffee_maker, coffee_maker).
needs(clean_coffee_maker, vinegar).
needs(clean_coffee_maker, water).
needs(clean_coffee_maker, cloth).
takes_time(clean_coffee_maker, 30, 40, minute).
step(clean_coffee_maker, clean_coffee_maker_s1).
action(clean_coffee_maker_s1, pour).
object_of(clean_coffee_maker_s1, vinegar).
into(clean_coffee_maker_s1, coffee_maker).
uses(clean_coffee_maker_s1, vinegar).
role(clean_coffee_maker_s1, vinegar, clean).
amount(clean_coffee_maker_s1, vinegar, 1, cup).
step(clean_coffee_maker, clean_coffee_maker_s2).
action(clean_coffee_maker_s2, brew).
object_of(clean_coffee_maker_s2, coffee_maker).
takes_time(clean_coffee_maker_s2, 15, 20, minute).
step(clean_coffee_maker, clean_coffee_maker_s3).
action(clean_coffee_maker_s3, rinse).
object_of(clean_coffee_maker_s3, coffee_maker).
uses(clean_coffee_maker_s3, water).
role(clean_coffee_maker_s3, water, rinse).
takes_time(clean_coffee_maker_s3, 15, 20, minute).
step(clean_coffee_maker, clean_coffee_maker_s4).
action(clean_coffee_maker_s4, wipe).
object_of(clean_coffee_maker_s4, coffee_maker).
uses(clean_coffee_maker_s4, cloth).
role(clean_coffee_maker_s4, cloth, wipe).
overlaps(clean_coffee_maker_s4, clean_coffee_maker_s2).
before(clean_coffee_maker_s1, clean_coffee_maker_s2).
before(clean_coffee_maker_s2, clean_coffee_maker_s3).
```

## aliases
```datalog
task_alias(clean_coffee_maker, "descale coffee maker").
task_alias(clean_coffee_maker, "clean coffee machine").
alias(coffee_maker, "coffee machine").
alias(vinegar, "white vinegar").
alias(cloth, "damp cloth").
alias(brew, "run a cycle").
alias(rinse, "rinse out").
alias(wipe, "wipe down").
```

## teach
- To clean a coffee maker you need white vinegar, water, a cloth and a coffee maker.
- Cleaning a coffee maker takes 30 to 40 minutes.
- Step 1 of cleaning a coffee maker: pour 1 cup of white vinegar into the coffee maker.
- Step 2 of cleaning a coffee maker: brew a full cycle and let it run for 15 to 20 minutes.
- Step 3 of cleaning a coffee maker: rinse the coffee maker by brewing plain water through it for 15 to 20 minutes.
- While step 2 runs, step 4 of cleaning a coffee maker: wipe the outside of the coffee maker with a damp cloth.

## checklist
- names what is needed, including white vinegar, water, a cloth and a coffee maker
- gives the steps in order: pour vinegar, brew, rinse with water, wipe
- says the outside wiping can be done while the brew cycle runs
- gives a total time of about 30 to 40 minutes

## variant
- change: no vinegar: use lemon juice instead of vinegar for the descaling step
- request: I'm out of vinegar. Can I clean the coffee maker with something else?
- situation: I have lemons, a coffee maker, a cloth and water, but no vinegar.
- expect: The reply uses lemon juice in place of vinegar for the first step and keeps the same brew, rinse and wipe steps.

```datalog situation
lacks(user, vinegar).
has(user, lemon_juice).
has(user, coffee_maker).
has(user, water).
has(user, cloth).
```

```datalog difference
swap(clean_coffee_maker_s1, vinegar, lemon_juice).
no_need(vinegar).
extra_need(lemon_juice).
```

- change: quick clean in 20 minutes: shorten the brew and rinse cycles to 8 to 10 minutes each
- request: I only have 20 minutes. How do I give the coffee maker a quick clean?
- situation: I have a coffee maker, white vinegar, a cloth and water, and 20 minutes.
- expect: The reply keeps the same steps but shortens the brew and rinse to about 8 to 10 minutes each, for about 16 to 20 minutes in total.

```datalog situation
has(user, coffee_maker).
has(user, vinegar).
has(user, water).
has(user, cloth).
time_limit(user, 20, minute).
```

```datalog difference
change(clean_coffee_maker_s2, takes_time, 15_20, 8_10).
change(clean_coffee_maker_s3, takes_time, 15_20, 8_10).
change(clean_coffee_maker, takes_time, 30_40, 16_20).
```
