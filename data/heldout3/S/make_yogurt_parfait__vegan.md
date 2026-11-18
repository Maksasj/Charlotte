---
id: make_yogurt_parfait__vegan
stage: procedures
task: make_yogurt_parfait
schema: v3
tier: S
base: make_yogurt_parfait
---
# Make a yogurt parfait

## requests
- My flatmate is vegan and I want to make her a layered breakfast glass. What should change?

## ground truth
```datalog
task(make_yogurt_parfait).
domain(make_yogurt_parfait, cooking).
goal(make_yogurt_parfait, yogurt_parfait).
goal(make_yogurt_parfait, breakfast).
needs(make_yogurt_parfait, yogurt).
needs(make_yogurt_parfait, granola).
needs(make_yogurt_parfait, berry).
needs(make_yogurt_parfait, honey).
needs(make_yogurt_parfait, glass).
takes_time(make_yogurt_parfait, 5, 7, minute).
step(make_yogurt_parfait, make_yogurt_parfait_s1).
action(make_yogurt_parfait_s1, rinse).
object_of(make_yogurt_parfait_s1, berry).
step(make_yogurt_parfait, make_yogurt_parfait_s2).
action(make_yogurt_parfait_s2, pat).
object_of(make_yogurt_parfait_s2, berry).
achieves(make_yogurt_parfait_s2, dry).
takes_time(make_yogurt_parfait_s2, 1, 2, minute).
step(make_yogurt_parfait, make_yogurt_parfait_s3).
action(make_yogurt_parfait_s3, spoon).
object_of(make_yogurt_parfait_s3, yogurt).
uses(make_yogurt_parfait_s3, glass).
role(make_yogurt_parfait_s3, glass, contain).
into(make_yogurt_parfait_s3, glass).
takes_time(make_yogurt_parfait_s3, 1, 1, minute).
step(make_yogurt_parfait, make_yogurt_parfait_s4).
action(make_yogurt_parfait_s4, layer).
object_of(make_yogurt_parfait_s4, granola).
uses(make_yogurt_parfait_s4, berry).
role(make_yogurt_parfait_s4, berry, flavour).
step(make_yogurt_parfait, make_yogurt_parfait_s5).
action(make_yogurt_parfait_s5, repeat).
object_of(make_yogurt_parfait_s5, layer).
takes_time(make_yogurt_parfait_s5, 2, 3, minute).
step(make_yogurt_parfait, make_yogurt_parfait_s6).
action(make_yogurt_parfait_s6, drizzle).
object_of(make_yogurt_parfait_s6, honey).
takes_time(make_yogurt_parfait_s6, 1, 1, minute).
before(make_yogurt_parfait_s1, make_yogurt_parfait_s2).
before(make_yogurt_parfait_s2, make_yogurt_parfait_s3).
before(make_yogurt_parfait_s3, make_yogurt_parfait_s4).
before(make_yogurt_parfait_s4, make_yogurt_parfait_s5).
before(make_yogurt_parfait_s5, make_yogurt_parfait_s6).
```

## aliases
```datalog
task_alias(make_yogurt_parfait, "yogurt parfait").
task_alias(make_yogurt_parfait, "making a yogurt parfait").
task_alias(make_yogurt_parfait, "breakfast in a glass").
alias(berry, "berries").
alias(berry, "fruit").
alias(glass, "tall glass").
alias(layer, "layers").
alias(layer, "add a layer").
alias(spoon, "spoon a layer").
alias(dry, "pat dry").
```

## teach
- To make a yogurt parfait you need yogurt, granola, berries, honey and a glass.
- Making a yogurt parfait takes 5 to 7 minutes.
- Step 1 of making a yogurt parfait: rinse the berries.
- Step 2 of making a yogurt parfait: pat the berries dry for 1 to 2 minutes.
- Step 3 of making a yogurt parfait: spoon a layer of yogurt into the glass for 1 minute.
- Step 4 of making a yogurt parfait: add a layer of granola and berries.
- Step 5 of making a yogurt parfait: repeat the layers, for 2 to 3 minutes.
- Step 6 of making a yogurt parfait: drizzle the honey over the top for 1 minute.

## variant
- change: vegan: use soy yogurt and maple syrup instead of yogurt and honey
- request: My flatmate is vegan and I want to make her a layered breakfast glass. What should change?
- situation: She eats no animal products. I have soy yogurt, granola, berries, maple syrup and a tall glass.
- expect: The reply layers soy yogurt instead of dairy yogurt and drizzles maple syrup instead of honey, keeping the other steps.

```datalog situation
has(user, soy_yogurt).
has(user, granola).
has(user, berry).
has(user, maple_syrup).
has(user, glass).
excludes(user, animal_product).
```

```datalog difference
change(make_yogurt_parfait_s3, object_of, yogurt, soy_yogurt).
change(make_yogurt_parfait_s6, object_of, honey, maple_syrup).
no_need(yogurt).
no_need(honey).
extra_need(soy_yogurt).
extra_need(maple_syrup).
```
