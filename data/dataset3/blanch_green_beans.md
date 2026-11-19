---
id: blanch_green_beans
stage: procedures
task: blanch_green_beans
schema: v3
---
# Blanch green beans

## requests
- Can you tell me how to blanch green beans so they stay bright green?
- What's the way to parboil green beans before I stir-fry them?
- Walk me through cooking green beans in boiling water and then cooling them down.

## situation
- I have green beans, a pot, a stove, a knife, a colander, a bowl and some ice.
- They're a side dish for dinner and I have about 20 minutes.

## ground truth
```datalog
task(blanch_green_beans).
domain(blanch_green_beans, cooking).
goal(blanch_green_beans, cooked_green_beans).
needs(blanch_green_beans, green_beans).
needs(blanch_green_beans, water).
needs(blanch_green_beans, pot).
needs(blanch_green_beans, stove).
needs(blanch_green_beans, knife).
needs(blanch_green_beans, colander).
needs(blanch_green_beans, bowl).
needs(blanch_green_beans, ice).
takes_time(blanch_green_beans, 12, 16, minute).
step(blanch_green_beans, blanch_green_beans_s1).
action(blanch_green_beans_s1, heat).
object_of(blanch_green_beans_s1, water).
uses(blanch_green_beans_s1, pot).
role(blanch_green_beans_s1, pot, contain).
uses(blanch_green_beans_s1, stove).
role(blanch_green_beans_s1, stove, heat).
setting(blanch_green_beans_s1, temperature, 100, celsius).
until(blanch_green_beans_s1, boiling).
takes_time(blanch_green_beans_s1, 5, 8, minute).
step(blanch_green_beans, blanch_green_beans_s2).
action(blanch_green_beans_s2, trim).
object_of(blanch_green_beans_s2, green_beans).
uses(blanch_green_beans_s2, knife).
role(blanch_green_beans_s2, knife, cut).
overlaps(blanch_green_beans_s2, blanch_green_beans_s1).
takes_time(blanch_green_beans_s2, 3, 5, minute).
step(blanch_green_beans, blanch_green_beans_s3).
action(blanch_green_beans_s3, add).
object_of(blanch_green_beans_s3, green_beans).
into(blanch_green_beans_s3, pot).
until(blanch_green_beans_s3, tender_crisp).
check(blanch_green_beans_s3, bright_green).
takes_time(blanch_green_beans_s3, 2, 3, minute).
step(blanch_green_beans, blanch_green_beans_s4).
action(blanch_green_beans_s4, drain).
object_of(blanch_green_beans_s4, green_beans).
uses(blanch_green_beans_s4, colander).
role(blanch_green_beans_s4, colander, hold).
takes_time(blanch_green_beans_s4, 1, 1, minute).
step(blanch_green_beans, blanch_green_beans_s5).
action(blanch_green_beans_s5, chill).
object_of(blanch_green_beans_s5, green_beans).
uses(blanch_green_beans_s5, bowl).
role(blanch_green_beans_s5, bowl, contain).
uses(blanch_green_beans_s5, ice).
role(blanch_green_beans_s5, ice, cool).
until(blanch_green_beans_s5, cold).
purpose(blanch_green_beans_s5, stop_cooking).
takes_time(blanch_green_beans_s5, 3, 3, minute).
before(blanch_green_beans_s2, blanch_green_beans_s3).
before(blanch_green_beans_s1, blanch_green_beans_s3).
before(blanch_green_beans_s3, blanch_green_beans_s4).
before(blanch_green_beans_s4, blanch_green_beans_s5).
```

## aliases
```datalog
task_alias(blanch_green_beans, "blanched green beans").
task_alias(blanch_green_beans, "parboil green beans").
alias(green_beans, "string beans").
alias(stove, "hob").
alias(colander, "strainer").
alias(ice, "ice cubes").
alias(tender_crisp, "tender-crisp").
alias(bright_green, "bright green").
alias(stop_cooking, "stop cooking").
alias(chill, "cool").
alias(cold_tap_water, "cold tap water").
```

## teach
- To blanch green beans you need green beans, water, a pot, a stove, a knife, a colander, a bowl and ice.
- Blanching green beans takes 12 to 16 minutes.
- Step 1 of blanching green beans: heat the water in the pot on the stove to a boil at 100 degrees for 5 to 8 minutes.
- While step 1 runs, step 2 of blanching green beans: trim the green beans with the knife for 3 to 5 minutes.
- Step 3 of blanching green beans: add the green beans to the boiling water in the pot for 2 to 3 minutes, until tender-crisp and bright green.
- Step 4 of blanching green beans: drain the green beans in the colander for 1 minute.
- Step 5 of blanching green beans: chill the green beans in a bowl of ice until cold, for 3 minutes, so they stop cooking.

## checklist
- names what is needed, including green beans, water, a pot, a stove, a colander, a bowl and ice
- gives the steps in order: heat water to a boil, trim beans, add beans to boiling water, drain, chill in ice water
- says trimming can be done while the water heats
- gives a total time of about 12 to 16 minutes

## variant
- change: no ice: chill the beans under cold running tap water instead of in ice water
- request: I'm out of ice. Can I still blanch the green beans?
- situation: I have green beans, a pot, a stove, a knife, a colander, a bowl and cold tap water, but no ice.
- expect: The reply keeps the same steps but chills the drained beans under cold running tap water in the bowl instead of ice water, and notes that the beans cool more slowly without ice.

```datalog situation
has(user, green_beans).
has(user, water).
has(user, pot).
has(user, stove).
has(user, knife).
has(user, colander).
has(user, bowl).
lacks(user, ice).
```

```datalog difference
swap(blanch_green_beans_s5, ice, cold_tap_water).
extra_need(cold_tap_water).
no_need(ice).
```

## variant
- change: add salt: season the drained beans with salt after chilling them
- request: Could you add a pinch of salt to the beans after they've cooled?
- situation: I have everything from the original list and some salt, and I want the beans seasoned.
- expect: The reply keeps all five steps and adds a new final step after chilling, where the cooled beans are seasoned with salt.

```datalog situation
has(user, salt).
wants(user, salt).
```

```datalog difference
add_step(blanch_green_beans_s9, blanch_green_beans_s5).
step(blanch_green_beans, blanch_green_beans_s9).
action(blanch_green_beans_s9, season).
object_of(blanch_green_beans_s9, green_beans).
uses(blanch_green_beans_s9, salt).
role(blanch_green_beans_s9, salt, season).
takes_time(blanch_green_beans_s9, 1, 1, minute).
before(blanch_green_beans_s5, blanch_green_beans_s9).
extra_need(salt).
```
