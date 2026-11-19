---
id: adjust_bicycle_brakes
stage: procedures
task: adjust_bicycle_brakes
schema: v3
---
# Adjust bicycle brakes

## requests
- My rear brake feels spongy and the pads rub. How do I adjust the brakes on my bike?
- What's the way to tune up bicycle brakes that have gone too loose?
- Can you talk me through adjusting the brake cable on my bike?

## situation
- I have a bicycle with rim brakes and an allen key.
- I've got about 20 minutes.

## ground truth
```datalog
task(adjust_bicycle_brakes).
domain(adjust_bicycle_brakes, repair).
goal(adjust_bicycle_brakes, working_brakes).
needs(adjust_bicycle_brakes, bicycle).
needs(adjust_bicycle_brakes, allen_key).
takes_time(adjust_bicycle_brakes, 15, 25, minute).
step(adjust_bicycle_brakes, adjust_bicycle_brakes_s1).
action(adjust_bicycle_brakes_s1, loosen).
object_of(adjust_bicycle_brakes_s1, brake_cable).
uses(adjust_bicycle_brakes_s1, allen_key).
role(adjust_bicycle_brakes_s1, allen_key, loosen).
step(adjust_bicycle_brakes, adjust_bicycle_brakes_s2).
action(adjust_bicycle_brakes_s2, pull).
object_of(adjust_bicycle_brakes_s2, brake_cable).
purpose(adjust_bicycle_brakes_s2, remove_slack).
check(adjust_bicycle_brakes_s2, cable_taut).
step(adjust_bicycle_brakes, adjust_bicycle_brakes_s3).
action(adjust_bicycle_brakes_s3, fasten).
object_of(adjust_bicycle_brakes_s3, brake_cable).
uses(adjust_bicycle_brakes_s3, allen_key).
role(adjust_bicycle_brakes_s3, allen_key, fasten).
warning(adjust_bicycle_brakes_s3, bolt_overtightened).
step(adjust_bicycle_brakes, adjust_bicycle_brakes_s4).
action(adjust_bicycle_brakes_s4, squeeze).
object_of(adjust_bicycle_brakes_s4, brake_lever).
check(adjust_bicycle_brakes_s4, pads_touch_rim).
step(adjust_bicycle_brakes, adjust_bicycle_brakes_s5).
action(adjust_bicycle_brakes_s5, spin).
object_of(adjust_bicycle_brakes_s5, wheel).
check(adjust_bicycle_brakes_s5, wheel_spins_freely).
before(adjust_bicycle_brakes_s1, adjust_bicycle_brakes_s2).
before(adjust_bicycle_brakes_s2, adjust_bicycle_brakes_s3).
before(adjust_bicycle_brakes_s3, adjust_bicycle_brakes_s4).
before(adjust_bicycle_brakes_s4, adjust_bicycle_brakes_s5).
```

## aliases
```datalog
task_alias(adjust_bicycle_brakes, "adjusting bicycle brakes").
task_alias(adjust_bicycle_brakes, "tuning up bicycle brakes").
task_alias(adjust_bicycle_brakes, "adjusting the brake cable").
alias(allen_key, "allen key").
alias(allen_key, "hex key").
alias(spanner, "wrench").
alias(brake_cable, "brake cable").
alias(brake_lever, "brake lever").
alias(loosen, "loosen").
alias(fasten, "tighten").
alias(cable_taut, "taut").
alias(pads_touch_rim, "pads touch the rim").
alias(wheel_spins_freely, "wheel spins freely").
alias(bolt_overtightened, "overtightened").
alias(remove_slack, "remove slack").
```

## teach
- To adjust bicycle brakes you need a bicycle and an allen key.
- Adjusting bicycle brakes takes 15 to 25 minutes.
- Step 1 of adjusting bicycle brakes: loosen the brake cable bolt with the allen key.
- Step 2 of adjusting bicycle brakes: pull the brake cable until it is taut, to remove the slack.
- Step 3 of adjusting bicycle brakes: fasten the brake cable bolt again with the allen key, without overtightening it.
- Step 4 of adjusting bicycle brakes: squeeze the brake lever to check the pads touch the rim.
- Step 5 of adjusting bicycle brakes: spin the wheel to check it turns freely.

## checklist
- names what is needed, including a bicycle and an allen key
- gives the steps in order: loosen the cable bolt, pull the cable taut, tighten the bolt without overtightening, squeeze the lever to check the pads, spin the wheel
- warns not to overtighten the cable bolt
- gives a total time of about 15 to 25 minutes

## variant
- change: no allen key: use a spanner on the cable bolt in steps 1 and 3
- request: I don't have an allen key, only a spanner. Can I still adjust my brakes?
- situation: I have a bicycle, a spanner and about 20 minutes.
- expect: The reply uses the spanner instead of the allen key to loosen and then tighten the cable bolt, and keeps the other steps the same.

```datalog situation
has(user, bicycle).
has(user, spanner).
lacks(user, allen_key).
```

```datalog difference
extra_need(spanner).
no_need(allen_key).
swap(adjust_bicycle_brakes_s1, allen_key, spanner).
swap(adjust_bicycle_brakes_s3, allen_key, spanner).
```

## variant
- change: only 10 minutes: skip the wheel spin check and give a quick version
- request: I've only got ten minutes. Can you give me a quick version of adjusting my bike brakes?
- situation: I have a bicycle, an allen key and 10 minutes.
- expect: The reply gives steps 1 to 4 only (loosen, pull taut, tighten, squeeze the lever) for about 6 to 8 minutes and leaves out the wheel spin check.

```datalog situation
has(user, bicycle).
has(user, allen_key).
time_limit(user, 10, minute).
```

```datalog difference
change(adjust_bicycle_brakes, takes_time, 15_25, 6_8).
drop(adjust_bicycle_brakes_s5).
```
