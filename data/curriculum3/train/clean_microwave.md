---
id: clean_microwave
stage: household
task: clean_microwave
schema: v3
---
# Clean the microwave

## requests
- The inside of my microwave is splattered with old food. How do I clean it easily?
- Is there a trick for cleaning a microwave without scrubbing?
- Can you tell me how to clean the microwave with a lemon?

## situation
- I have a microwave-safe bowl, water, a lemon and a cloth.
- I have 15 minutes.

## ground truth
```datalog
task(clean_microwave).
domain(clean_microwave, cleaning).
goal(clean_microwave, spotless_microwave).
needs(clean_microwave, bowl).
needs(clean_microwave, water).
needs(clean_microwave, lemon).
needs(clean_microwave, cloth).
needs(clean_microwave, microwave).
takes_time(clean_microwave, 11, 14, minute).
step(clean_microwave, clean_microwave_s1).
action(clean_microwave_s1, fill).
object_of(clean_microwave_s1, bowl).
uses(clean_microwave_s1, water).
role(clean_microwave_s1, water, clean).
uses(clean_microwave_s1, lemon).
role(clean_microwave_s1, lemon, clean).
amount(clean_microwave_s1, lemon, 0.5, piece).
takes_time(clean_microwave_s1, 1, 1, minute).
step(clean_microwave, clean_microwave_s2).
action(clean_microwave_s2, heat).
object_of(clean_microwave_s2, bowl).
uses(clean_microwave_s2, microwave).
role(clean_microwave_s2, microwave, heat).
setting(clean_microwave_s2, power, high, level).
until(clean_microwave_s2, steamy).
takes_time(clean_microwave_s2, 3, 5, minute).
step(clean_microwave, clean_microwave_s3).
action(clean_microwave_s3, leave).
object_of(clean_microwave_s3, microwave).
with_part(clean_microwave_s3, door).
purpose(clean_microwave_s3, steam_loosens_food).
takes_time(clean_microwave_s3, 5, 5, minute).
step(clean_microwave, clean_microwave_s4).
action(clean_microwave_s4, wipe).
object_of(clean_microwave_s4, microwave).
uses(clean_microwave_s4, cloth).
role(clean_microwave_s4, cloth, wipe).
with_part(clean_microwave_s4, inside).
takes_time(clean_microwave_s4, 2, 3, minute).
before(clean_microwave_s1, clean_microwave_s2).
before(clean_microwave_s2, clean_microwave_s3).
before(clean_microwave_s3, clean_microwave_s4).
```

## aliases
```datalog
task_alias(clean_microwave, "cleaning the microwave").
task_alias(clean_microwave, "steam-clean my microwave").
alias(bowl, "microwave-safe bowl").
alias(lemon, "juice of half the lemon").
alias(lemon, "lemons").
alias(vinegar, "white vinegar").
alias(steamy, "the window steams up").
alias(leave, "leave the door shut").
alias(steam_loosens_food, "the steam loosens the food").
```

## teach
- To clean the microwave you need a microwave-safe bowl, water, a lemon and a cloth.
- Cleaning the microwave takes 11 to 14 minutes.
- Step 1 of cleaning the microwave: fill the bowl with water and the juice of half the lemon for 1 minute.
- Step 2 of cleaning the microwave: heat the bowl in the microwave on high until the window steams up, for 3 to 5 minutes.
- Step 3 of cleaning the microwave: leave the microwave door shut for 5 minutes so the steam loosens the food.
- Step 4 of cleaning the microwave: wipe the inside of the microwave with the cloth for 2 to 3 minutes.

## checklist
- names what is needed: a microwave-safe bowl, water, a lemon and a cloth
- gives the steps in order: fill bowl, heat bowl, leave microwave, wipe microwave
- gives a total time of about 11 to 14 minutes
- closes politely

## variant
- change: no lemon: use two spoons of white vinegar in the water
- request: I want to steam-clean my microwave but I have no lemons. What else works?
- situation: I have a microwave-safe bowl, water, white vinegar and a cloth.
- expect: The reply adds a couple of spoons of vinegar to the water in place of lemon and keeps the heat, wait and wipe steps.

```datalog situation
has(user, bowl).
has(user, water).
has(user, vinegar).
has(user, cloth).
lacks(user, lemon).
```

```datalog difference
swap(clean_microwave_s1, lemon, vinegar).
change(clean_microwave_s1, amount, lemon_half_piece, vinegar_2_tablespoon).
extra_need(vinegar).
no_need(lemon).
```
