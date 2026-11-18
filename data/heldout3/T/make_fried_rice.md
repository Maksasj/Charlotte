---
id: make_fried_rice
stage: procedures
task: make_fried_rice
schema: v3
tier: T
---
# Make egg fried rice

## requests
- What do I do with yesterday's leftover rice? I'm thinking fried rice.
- How do you make egg fried rice like the takeaway does?
- Quick dinner idea: can you take me through frying up some rice with egg and peas?

## situation
- I have cold cooked rice, two eggs, frozen peas, soy sauce, oil, a wok and a spatula.

## ground truth
```datalog
task(make_fried_rice).
domain(make_fried_rice, cooking).
goal(make_fried_rice, fried_rice).
goal(make_fried_rice, quick_meal).
needs(make_fried_rice, cooked_rice).
needs(make_fried_rice, egg).
needs(make_fried_rice, frozen_peas).
needs(make_fried_rice, soy_sauce).
needs(make_fried_rice, oil).
needs(make_fried_rice, wok).
needs(make_fried_rice, spatula).
takes_time(make_fried_rice, 12, 15, minute).
step(make_fried_rice, make_fried_rice_s1).
action(make_fried_rice_s1, heat).
object_of(make_fried_rice_s1, oil).
uses(make_fried_rice_s1, wok).
role(make_fried_rice_s1, wok, heat).
setting(make_fried_rice_s1, heat, high, level).
takes_time(make_fried_rice_s1, 1, 2, minute).
step(make_fried_rice, make_fried_rice_s2).
action(make_fried_rice_s2, scramble).
object_of(make_fried_rice_s2, egg).
uses(make_fried_rice_s2, spatula).
role(make_fried_rice_s2, spatula, stir).
takes_time(make_fried_rice_s2, 1, 2, minute).
step(make_fried_rice, make_fried_rice_s3).
action(make_fried_rice_s3, add).
object_of(make_fried_rice_s3, frozen_peas).
into(make_fried_rice_s3, wok).
takes_time(make_fried_rice_s3, 2, 3, minute).
step(make_fried_rice, make_fried_rice_s4).
action(make_fried_rice_s4, add).
object_of(make_fried_rice_s4, cooked_rice).
into(make_fried_rice_s4, wok).
requires_state(make_fried_rice_s4, rice_cold).
step(make_fried_rice, make_fried_rice_s5).
action(make_fried_rice_s5, fry).
object_of(make_fried_rice_s5, cooked_rice).
uses(make_fried_rice_s5, spatula).
role(make_fried_rice_s5, spatula, stir).
until(make_fried_rice_s5, hot).
takes_time(make_fried_rice_s5, 4, 5, minute).
step(make_fried_rice, make_fried_rice_s6).
action(make_fried_rice_s6, season).
object_of(make_fried_rice_s6, cooked_rice).
uses(make_fried_rice_s6, soy_sauce).
role(make_fried_rice_s6, soy_sauce, season).
before(make_fried_rice_s1, make_fried_rice_s2).
before(make_fried_rice_s2, make_fried_rice_s3).
before(make_fried_rice_s3, make_fried_rice_s4).
before(make_fried_rice_s4, make_fried_rice_s5).
before(make_fried_rice_s5, make_fried_rice_s6).
```

## aliases
```datalog
task_alias(make_fried_rice, "fried rice").
task_alias(make_fried_rice, "egg fried rice").
task_alias(make_fried_rice, "frying rice").
alias(cooked_rice, "leftover rice").
alias(cooked_rice, "cold rice").
alias(frozen_peas, "peas").
alias(soy_sauce, "soy").
alias(rice_cold, "rice from the day before").
alias(hot, "piping hot").
```

## teach
- To make egg fried rice you need cold cooked rice, eggs, frozen peas, soy sauce, oil, a wok and a spatula.
- Making egg fried rice takes 12 to 15 minutes.
- Step 1 of making egg fried rice: heat the oil in the wok on high heat for 1 to 2 minutes.
- Step 2 of making egg fried rice: scramble the eggs in the oil with the spatula for 1 to 2 minutes.
- Step 3 of making egg fried rice: add the frozen peas to the wok and cook them for 2 to 3 minutes.
- Step 4 of making egg fried rice: add the cooked rice to the wok; it works best when the rice is cold from the day before.
- Step 5 of making egg fried rice: fry the rice, stirring with the spatula, until it is piping hot, for 4 to 5 minutes.
- Step 6 of making egg fried rice: season the rice with soy sauce.

## checklist
- names what is needed, including cold cooked rice, eggs, peas, soy sauce and a wok
- gives the steps in order: heat oil, scramble eggs, add peas, add rice, fry until hot, season
- says day-old cold rice works best
- gives a total time of about 12 to 15 minutes

## variant
- change: no wok: use a large frying pan
- request: I don't have a wok, just a big frying pan. Will fried rice still work?
- situation: I have cold cooked rice, eggs, frozen peas, soy sauce, oil, a large frying pan and a spatula.
- expect: The reply does every step in the frying pan instead of the wok, on high heat.

```datalog situation
has(user, cooked_rice).
has(user, egg).
has(user, frozen_peas).
has(user, soy_sauce).
has(user, oil).
has(user, frying_pan).
has(user, spatula).
lacks(user, wok).
```

```datalog difference
swap(make_fried_rice_s1, wok, frying_pan).
change(make_fried_rice_s3, into, wok, frying_pan).
change(make_fried_rice_s4, into, wok, frying_pan).
extra_need(frying_pan).
no_need(wok).
```

## variant
- change: egg allergy: leave out the egg
- request: My son is allergic to eggs. Can I make fried rice for him?
- situation: I have cold cooked rice, frozen peas, soy sauce, oil, a wok and a spatula. My son can't eat egg.
- expect: The reply skips the scrambled egg step and fries the peas and rice in the oil, then seasons with soy sauce.

```datalog situation
has(user, cooked_rice).
has(user, frozen_peas).
has(user, soy_sauce).
has(user, oil).
has(user, wok).
has(user, spatula).
excludes(user, egg).
```

```datalog difference
drop(make_fried_rice_s2).
no_need(egg).
```
