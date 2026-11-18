---
id: fix_flat_tire
stage: tools
task: fix_flat_tire
schema: v3
---
# Fix a flat bike tire

## requests
- My bike has a flat tire and I need it for work tomorrow. How do I fix it myself?
- I got a puncture on my bicycle. Can you walk me through patching the tube?
- How do I repair a flat on my bike? I've got a patch kit but no idea how to use it.

## situation
- I have tire levers, a patch kit and a bike pump.
- I've got about half an hour this evening.

## ground truth
```datalog
task(fix_flat_tire).
domain(fix_flat_tire, repair).
goal(fix_flat_tire, repaired_tire).
needs(fix_flat_tire, tire_lever).
needs(fix_flat_tire, patch_kit).
needs(fix_flat_tire, pump).
takes_time(fix_flat_tire, 20, 30, minute).
step(fix_flat_tire, fix_flat_tire_s1).
action(fix_flat_tire_s1, remove).
object_of(fix_flat_tire_s1, wheel).
from(fix_flat_tire_s1, bike).
takes_time(fix_flat_tire_s1, 2, 3, minute).
step(fix_flat_tire, fix_flat_tire_s2).
action(fix_flat_tire_s2, remove).
object_of(fix_flat_tire_s2, tube).
from(fix_flat_tire_s2, tire).
uses(fix_flat_tire_s2, tire_lever).
role(fix_flat_tire_s2, tire_lever, lift).
takes_time(fix_flat_tire_s2, 3, 5, minute).
step(fix_flat_tire, fix_flat_tire_s3).
action(fix_flat_tire_s3, pump).
object_of(fix_flat_tire_s3, tube).
uses(fix_flat_tire_s3, pump).
role(fix_flat_tire_s3, pump, inflate).
step(fix_flat_tire, fix_flat_tire_s4).
action(fix_flat_tire_s4, find).
object_of(fix_flat_tire_s4, hole).
takes_time(fix_flat_tire_s4, 2, 4, minute).
step(fix_flat_tire, fix_flat_tire_s5).
action(fix_flat_tire_s5, patch).
object_of(fix_flat_tire_s5, hole).
uses(fix_flat_tire_s5, patch_kit).
role(fix_flat_tire_s5, patch_kit, cover).
takes_time(fix_flat_tire_s5, 5, 7, minute).
step(fix_flat_tire, fix_flat_tire_s6).
action(fix_flat_tire_s6, fit).
object_of(fix_flat_tire_s6, tube).
into(fix_flat_tire_s6, tire).
uses(fix_flat_tire_s6, tire_lever).
role(fix_flat_tire_s6, tire_lever, lift).
takes_time(fix_flat_tire_s6, 4, 6, minute).
step(fix_flat_tire, fix_flat_tire_s7).
action(fix_flat_tire_s7, pump).
object_of(fix_flat_tire_s7, tire).
uses(fix_flat_tire_s7, pump).
role(fix_flat_tire_s7, pump, inflate).
until(fix_flat_tire_s7, firm).
takes_time(fix_flat_tire_s7, 2, 3, minute).
before(fix_flat_tire_s1, fix_flat_tire_s2).
before(fix_flat_tire_s2, fix_flat_tire_s3).
before(fix_flat_tire_s3, fix_flat_tire_s4).
before(fix_flat_tire_s4, fix_flat_tire_s5).
before(fix_flat_tire_s5, fix_flat_tire_s6).
before(fix_flat_tire_s6, fix_flat_tire_s7).
```

## aliases
```datalog
task_alias(fix_flat_tire, "fixing a flat tire").
task_alias(fix_flat_tire, "fix a flat bike tire").
task_alias(fix_flat_tire, "patching the tube").
task_alias(fix_flat_tire, "repair a flat").
alias(tire_lever, "tire levers").
alias(pump, "bike pump").
alias(bike, "bicycle").
alias(tube, "inner tube").
alias(hole, "puncture").
alias(pump, "pumping").
alias(fit, "fit back").
alias(spare_tube, "spare inner tube").
alias(spare_tube, "new tube").
```

## teach
- To fix a flat tire you need tire levers, a patch kit and a pump.
- Fixing a flat tire takes 20 to 30 minutes.
- Step 1 of fixing a flat tire: remove the wheel from the bike for 2 to 3 minutes.
- Step 2 of fixing a flat tire: remove the tube from the tire with the tire levers for 3 to 5 minutes.
- Step 3 of fixing a flat tire: pump a little air into the tube with the pump.
- Step 4 of fixing a flat tire: find the hole by listening, for 2 to 4 minutes.
- Step 5 of fixing a flat tire: patch the hole with the patch kit for 5 to 7 minutes.
- Step 6 of fixing a flat tire: fit the tube back inside the tire with the tire levers for 4 to 6 minutes.
- Step 7 of fixing a flat tire: pump the tire with the pump until it is firm, for 2 to 3 minutes.

## checklist
- names what is needed: tire levers, a patch kit and a pump
- gives the steps in order: remove wheel, remove tube, find hole, patch, refit tube, pump up
- gives a total time of about 20 to 30 minutes
- closes politely

## variant
- change: a spare inner tube: swap in the new tube instead of finding and patching the hole
- request: My bike tire went flat, but luckily I bought a spare inner tube last month. How do I put it in?
- situation: I have tire levers, a pump and a new spare inner tube, but no patch kit.
- expect: The reply skips finding and patching the hole and fits the spare tube instead, so the job takes about 12 to 20 minutes.

```datalog situation
has(user, tire_lever).
has(user, pump).
has(user, spare_tube).
lacks(user, patch_kit).
```

```datalog difference
drop(fix_flat_tire_s3).
drop(fix_flat_tire_s4).
drop(fix_flat_tire_s5).
change(fix_flat_tire_s6, object_of, tube, spare_tube).
no_need(patch_kit).
extra_need(spare_tube).
```
