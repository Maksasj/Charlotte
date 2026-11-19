---
id: replace_toilet_flapper
stage: procedures
task: replace_toilet_flapper
schema: v3
---
# Replace a toilet flapper

## requests
- How do I replace the flapper in my toilet tank? The toilet keeps running.
- My toilet won't stop running and I think the flapper is worn out. What's the procedure to swap it?
- Walk me through changing a toilet flapper, step by step.

## situation
- I have a new flapper, a pair of pliers and a sponge.
- The toilet is running constantly and I want it fixed today.

## ground truth
```datalog
task(replace_toilet_flapper).
domain(replace_toilet_flapper, repair).
goal(replace_toilet_flapper, stop_running_toilet).
needs(replace_toilet_flapper, flapper).
needs(replace_toilet_flapper, pliers).
needs(replace_toilet_flapper, sponge).
takes_time(replace_toilet_flapper, 20, 30, minute).
step(replace_toilet_flapper, replace_toilet_flapper_s1).
action(replace_toilet_flapper_s1, shut).
object_of(replace_toilet_flapper_s1, water_valve).
setting(replace_toilet_flapper_s1, direction, clockwise, level).
step(replace_toilet_flapper, replace_toilet_flapper_s2).
action(replace_toilet_flapper_s2, flush).
object_of(replace_toilet_flapper_s2, toilet).
purpose(replace_toilet_flapper_s2, tank_empties).
step(replace_toilet_flapper, replace_toilet_flapper_s3).
action(replace_toilet_flapper_s3, wipe).
object_of(replace_toilet_flapper_s3, tank).
uses(replace_toilet_flapper_s3, sponge).
role(replace_toilet_flapper_s3, sponge, wipe).
step(replace_toilet_flapper, replace_toilet_flapper_s4).
action(replace_toilet_flapper_s4, loosen).
object_of(replace_toilet_flapper_s4, old_flapper).
uses(replace_toilet_flapper_s4, pliers).
role(replace_toilet_flapper_s4, pliers, loosen).
from(replace_toilet_flapper_s4, flush_valve).
step(replace_toilet_flapper, replace_toilet_flapper_s5).
action(replace_toilet_flapper_s5, fasten).
object_of(replace_toilet_flapper_s5, flapper).
uses(replace_toilet_flapper_s5, flapper).
role(replace_toilet_flapper_s5, flapper, seal).
onto(replace_toilet_flapper_s5, flush_valve).
step(replace_toilet_flapper, replace_toilet_flapper_s6).
action(replace_toilet_flapper_s6, open).
object_of(replace_toilet_flapper_s6, water_valve).
until(replace_toilet_flapper_s6, tank_fills).
check(replace_toilet_flapper_s6, tank_fills).
before(replace_toilet_flapper_s1, replace_toilet_flapper_s2).
before(replace_toilet_flapper_s2, replace_toilet_flapper_s3).
before(replace_toilet_flapper_s3, replace_toilet_flapper_s4).
before(replace_toilet_flapper_s4, replace_toilet_flapper_s5).
before(replace_toilet_flapper_s5, replace_toilet_flapper_s6).
```

## aliases
```datalog
task_alias(replace_toilet_flapper, "fix a running toilet").
task_alias(replace_toilet_flapper, "change a toilet flapper").
alias(flapper, "flapper valve").
alias(flush_valve, "flush valve").
alias(water_valve, "shut-off valve").
alias(tank, "toilet tank").
alias(old_flapper, "old flapper").
alias(tank_fills, "tank fills with water").
alias(tank_empties, "tank empties").
alias(stop_running_toilet, "stops running").
```

## teach
- To replace a toilet flapper you need a flapper, pliers and a sponge.
- Replacing a toilet flapper takes 20 to 30 minutes.
- Replacing a toilet flapper stops a toilet that keeps running.
- Step 1 of replacing a toilet flapper: shut the water valve clockwise.
- Step 2 of replacing a toilet flapper: flush the toilet to empty the tank.
- Step 3 of replacing a toilet flapper: wipe the tank with the sponge.
- Step 4 of replacing a toilet flapper: loosen the old flapper from the flush valve with the pliers.
- Step 5 of replacing a toilet flapper: fasten the new flapper onto the flush valve.
- Step 6 of replacing a toilet flapper: open the water valve until the tank fills.

## checklist
- names what is needed: a flapper, pliers and a sponge
- gives the steps in order: shut the water, flush, wipe the tank, loosen the old flapper, fasten the new one, open the water
- says the water must be shut off before flushing and the new flapper goes onto the flush valve
- gives a total time of about 20 to 30 minutes

## variant
- change: lacks pliers, so use a screwdriver to loosen the old flapper instead
- request: My toilet keeps running. I don't have pliers, but I do have a screwdriver. Can I still change the flapper?
- situation: I have a flapper, a screwdriver and a sponge, but no pliers.
- expect: The reply uses the screwdriver in place of pliers to loosen the old flapper and keeps all the other steps in order.

```datalog situation
has(user, flapper).
has(user, screwdriver).
has(user, sponge).
lacks(user, pliers).
```

```datalog difference
swap(replace_toilet_flapper_s4, pliers, screwdriver).
extra_need(screwdriver).
no_need(pliers).
```

## variant
- change: only 15 minutes, so skip wiping the tank since it is already clean
- request: I need to fix this toilet in 15 minutes. Can I skip anything?
- situation: I have a flapper, pliers and a sponge, the tank is already clean, and I have 15 minutes.
- expect: The reply drops the tank-wiping step, leaves the other steps unchanged, and gives a total time of about 10 to 15 minutes.

```datalog situation
has(user, flapper).
has(user, pliers).
has(user, sponge).
time_limit(user, 15, minute).
wants(user, quick_fix).
```

```datalog difference
drop(replace_toilet_flapper_s3).
no_need(sponge).
change(replace_toilet_flapper, takes_time, 20_30, 10_15).
```
