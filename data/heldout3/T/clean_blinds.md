---
id: clean_blinds
stage: household
task: clean_blinds
schema: v3
tier: T
---
# Clean venetian blinds

## requests
- How do you clean venetian blinds without taking them down?
- The slats on my blinds are thick with dust. What's the best way to get them clean?
- What's a good way to wash horizontal window blinds?

## situation
- I have a duster, a cloth, washing-up liquid, a bowl of warm water and a towel.

## ground truth
```datalog
task(clean_blinds).
domain(clean_blinds, cleaning).
goal(clean_blinds, clean_blind).
needs(clean_blinds, duster).
needs(clean_blinds, cloth).
needs(clean_blinds, dish_soap).
needs(clean_blinds, warm_water).
needs(clean_blinds, towel).
takes_time(clean_blinds, 20, 30, minute).
step(clean_blinds, clean_blinds_s1).
action(clean_blinds_s1, close).
object_of(clean_blinds_s1, blind).
step(clean_blinds, clean_blinds_s2).
action(clean_blinds_s2, dust).
object_of(clean_blinds_s2, blind).
with_part(clean_blinds_s2, slat).
uses(clean_blinds_s2, duster).
role(clean_blinds_s2, duster, clean).
step(clean_blinds, clean_blinds_s3).
action(clean_blinds_s3, wipe).
object_of(clean_blinds_s3, blind).
with_part(clean_blinds_s3, slat).
uses(clean_blinds_s3, cloth).
role(clean_blinds_s3, cloth, wipe).
uses(clean_blinds_s3, dish_soap).
role(clean_blinds_s3, dish_soap, clean).
uses(clean_blinds_s3, warm_water).
role(clean_blinds_s3, warm_water, wet).
step(clean_blinds, clean_blinds_s4).
action(clean_blinds_s4, tilt).
object_of(clean_blinds_s4, blind).
purpose(clean_blinds_s4, other_side_shown).
step(clean_blinds, clean_blinds_s5).
action(clean_blinds_s5, wipe).
object_of(clean_blinds_s5, blind).
with_part(clean_blinds_s5, back_of_slat).
uses(clean_blinds_s5, cloth).
role(clean_blinds_s5, cloth, wipe).
step(clean_blinds, clean_blinds_s6).
action(clean_blinds_s6, dry).
object_of(clean_blinds_s6, blind).
uses(clean_blinds_s6, towel).
role(clean_blinds_s6, towel, dry).
warning(clean_blinds_s6, bent_slat).
before(clean_blinds_s1, clean_blinds_s2).
before(clean_blinds_s2, clean_blinds_s3).
before(clean_blinds_s3, clean_blinds_s4).
before(clean_blinds_s4, clean_blinds_s5).
before(clean_blinds_s5, clean_blinds_s6).
```

## aliases
```datalog
task_alias(clean_blinds, "cleaning blinds").
task_alias(clean_blinds, "washing window blinds").
task_alias(clean_blinds, "dusting the blinds").
alias(blind, "blinds").
alias(slat, "slats").
alias(dish_soap, "washing-up liquid").
alias(back_of_slat, "the other side of the slats").
alias(tilt, "turn the slats").
```

## teach
- To clean venetian blinds you need a duster, a cloth, washing-up liquid, warm water and a towel.
- Cleaning venetian blinds takes 20 to 30 minutes.
- Step 1 of cleaning venetian blinds: close the blinds.
- Step 2 of cleaning venetian blinds: dust the slats with the duster.
- Step 3 of cleaning venetian blinds: wipe the slats with a cloth dipped in warm soapy water.
- Step 4 of cleaning venetian blinds: tilt the blinds the other way to show the other side.
- Step 5 of cleaning venetian blinds: wipe the other side of the slats with the cloth.
- Step 6 of cleaning venetian blinds: dry the blinds with the towel; press lightly so you do not bend a slat.

## checklist
- names what is needed, including a duster, cloth, soapy water and a towel
- gives the steps in order: close, dust, wipe, tilt, wipe other side, dry
- warns against bending the slats

## variant
- change: no duster: slip an old sock over your hand
- request: I don't have a duster. What can I use on the blinds instead?
- situation: I have an old sock, a cloth, washing-up liquid, warm water and a towel.
- expect: The reply dusts each slat with an old sock worn over the hand instead of a duster.

```datalog situation
has(user, old_sock).
has(user, cloth).
has(user, dish_soap).
has(user, warm_water).
has(user, towel).
lacks(user, duster).
```

```datalog difference
swap(clean_blinds_s2, duster, old_sock).
extra_need(old_sock).
no_need(duster).
```

## variant
- change: five blinds: same steps for each, about two hours in all
- request: There are five sets of blinds in the flat. How long will it take to clean them all?
- situation: I have a duster, a cloth, washing-up liquid, warm water and a towel, and five blinds.
- expect: The reply repeats the routine for each of the five blinds and estimates about 100 to 150 minutes in total.

```datalog situation
has(user, duster).
has(user, cloth).
has(user, dish_soap).
has(user, warm_water).
has(user, towel).
count(user, blind, 5).
```

```datalog difference
change(clean_blinds, takes_time, 20_30, 100_150).
```
