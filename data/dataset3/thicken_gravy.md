---
id: thicken_gravy
stage: procedures
task: thicken_gravy
schema: v3
---
# Thicken gravy

## requests
- How do I thicken gravy with flour and butter?
- What's the best way to make my gravy thicker before serving?
- Can you walk me through getting a smooth, thick gravy using flour?

## situation
- I have butter, flour, stock, a saucepan, a hob and a whisk.
- I have about 15 minutes before the meal.

## ground truth
```datalog
task(thicken_gravy).
domain(thicken_gravy, cooking).
goal(thicken_gravy, thick_gravy).
needs(thicken_gravy, butter).
needs(thicken_gravy, flour).
needs(thicken_gravy, stock).
needs(thicken_gravy, saucepan).
needs(thicken_gravy, hob).
needs(thicken_gravy, whisk).
takes_time(thicken_gravy, 8, 12, minute).
step(thicken_gravy, thicken_gravy_s1).
action(thicken_gravy_s1, heat).
object_of(thicken_gravy_s1, butter).
uses(thicken_gravy_s1, saucepan).
role(thicken_gravy_s1, saucepan, contain).
uses(thicken_gravy_s1, hob).
role(thicken_gravy_s1, hob, heat).
setting(thicken_gravy_s1, heat, medium, level).
takes_time(thicken_gravy_s1, 2, 3, minute).
step(thicken_gravy, thicken_gravy_s2).
action(thicken_gravy_s2, stir).
object_of(thicken_gravy_s2, flour).
into(thicken_gravy_s2, butter).
uses(thicken_gravy_s2, whisk).
role(thicken_gravy_s2, whisk, stir).
takes_time(thicken_gravy_s2, 1, 1, minute).
step(thicken_gravy, thicken_gravy_s3).
action(thicken_gravy_s3, cook).
object_of(thicken_gravy_s3, flour).
uses(thicken_gravy_s3, hob).
role(thicken_gravy_s3, hob, heat).
purpose(thicken_gravy_s3, avoid_floury_taste).
check(thicken_gravy_s3, smells_nutty).
takes_time(thicken_gravy_s3, 1, 2, minute).
step(thicken_gravy, thicken_gravy_s4).
action(thicken_gravy_s4, pour).
object_of(thicken_gravy_s4, stock).
into(thicken_gravy_s4, saucepan).
takes_time(thicken_gravy_s4, 1, 1, minute).
step(thicken_gravy, thicken_gravy_s5).
action(thicken_gravy_s5, simmer).
object_of(thicken_gravy_s5, gravy).
uses(thicken_gravy_s5, hob).
role(thicken_gravy_s5, hob, heat).
until(thicken_gravy_s5, thick).
check(thicken_gravy_s5, glossy).
takes_time(thicken_gravy_s5, 3, 5, minute).
before(thicken_gravy_s1, thicken_gravy_s2).
before(thicken_gravy_s2, thicken_gravy_s3).
before(thicken_gravy_s3, thicken_gravy_s4).
before(thicken_gravy_s4, thicken_gravy_s5).
```

## aliases
```datalog
task_alias(thicken_gravy, "thicken gravy").
task_alias(thicken_gravy, "thickening gravy").
alias(hob, "stove").
alias(saucepan, "pan").
alias(stock, "broth").
alias(thick, "thick and glossy").
alias(glossy, "glossy").
alias(avoid_floury_taste, "no floury taste").
```

## teach
- To thicken gravy you need butter, flour, stock, a saucepan, a hob and a whisk.
- Thickening gravy takes 8 to 12 minutes.
- Step 1 of thickening gravy: heat the butter in the saucepan on the hob over a medium heat for 2 to 3 minutes.
- Step 2 of thickening gravy: stir the flour into the butter with the whisk for 1 minute.
- Step 3 of thickening gravy: cook the flour on the hob for 1 to 2 minutes until it smells nutty, so it does not taste floury.
- Step 4 of thickening gravy: pour the stock into the saucepan slowly for 1 minute.
- Step 5 of thickening gravy: simmer the gravy on the hob for 3 to 5 minutes until it is thick and glossy.

## checklist
- names what is needed, including butter, flour, stock, a saucepan, a hob and a whisk
- gives the steps in order: heat butter, stir in flour, cook the flour, pour in stock, simmer until thick
- says the flour must be cooked so it does not taste floury
- gives a total time of about 8 to 12 minutes

## variant
- change: no whisk: stir the flour into the butter with a wooden spoon instead
- request: I don't own a whisk. Can I still thicken the gravy with flour and butter?
- situation: I have butter, flour, stock, a saucepan, a hob and a wooden spoon, but no whisk.
- expect: The reply uses a wooden spoon in place of the whisk for stirring in the flour and gives the same five steps in the same order.

```datalog situation
lacks(user, whisk).
has(user, wooden_spoon).
```

```datalog difference
swap(thicken_gravy_s2, whisk, wooden_spoon).
no_need(whisk).
extra_need(wooden_spoon).
```

- change: longer simmer for extra thick gravy: simmer for 8 to 10 minutes instead of 3 to 5
- request: I want really thick gravy and I have about 20 minutes. How long should I simmer it?
- situation: I want extra thick gravy and I have 20 minutes.
- expect: The reply keeps the same steps but simmers the gravy for 8 to 10 minutes, giving a total of about 13 to 17 minutes, which fits the 20 minutes.

```datalog situation
wants(user, extra_thick_gravy).
time_limit(user, 20, minute).
```

```datalog difference
change(thicken_gravy_s5, takes_time, 3_5, 8_10).
change(thicken_gravy, takes_time, 8_12, 13_17).
```
