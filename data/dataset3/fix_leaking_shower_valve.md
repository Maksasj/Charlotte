---
id: fix_leaking_shower_valve
stage: procedures
task: fix_leaking_shower_valve
schema: v3
---
# Fix a leaking shower valve

## requests
- My shower keeps dripping from the valve behind the handle. How do I fix a leaking shower valve?
- What's the job for replacing the cartridge in a dripping shower? I've never done one before.
- Can you talk me through stopping a shower that still drips when it's switched off? Which tools do I need?

## situation
- I have a screwdriver, pliers, grease and a replacement cartridge.
- I have a free afternoon and about an hour.

## ground truth
```datalog
task(fix_leaking_shower_valve).
domain(fix_leaking_shower_valve, repair).
goal(fix_leaking_shower_valve, stop_leak).
needs(fix_leaking_shower_valve, screwdriver).
needs(fix_leaking_shower_valve, pliers).
needs(fix_leaking_shower_valve, grease).
needs(fix_leaking_shower_valve, new_cartridge).
takes_time(fix_leaking_shower_valve, 45, 60, minute).
step(fix_leaking_shower_valve, fix_leaking_shower_valve_s1).
action(fix_leaking_shower_valve_s1, shut_off).
object_of(fix_leaking_shower_valve_s1, water_supply_valve).
step(fix_leaking_shower_valve, fix_leaking_shower_valve_s2).
action(fix_leaking_shower_valve_s2, remove).
object_of(fix_leaking_shower_valve_s2, handle).
uses(fix_leaking_shower_valve_s2, screwdriver).
role(fix_leaking_shower_valve_s2, screwdriver, loosen).
step(fix_leaking_shower_valve, fix_leaking_shower_valve_s3).
action(fix_leaking_shower_valve_s3, lift).
object_of(fix_leaking_shower_valve_s3, old_cartridge).
uses(fix_leaking_shower_valve_s3, pliers).
role(fix_leaking_shower_valve_s3, pliers, lift).
step(fix_leaking_shower_valve, fix_leaking_shower_valve_s4).
action(fix_leaking_shower_valve_s4, coat).
object_of(fix_leaking_shower_valve_s4, new_cartridge).
uses(fix_leaking_shower_valve_s4, grease).
role(fix_leaking_shower_valve_s4, grease, coat).
purpose(fix_leaking_shower_valve_s4, seal_slides).
step(fix_leaking_shower_valve, fix_leaking_shower_valve_s5).
action(fix_leaking_shower_valve_s5, fit).
object_of(fix_leaking_shower_valve_s5, new_cartridge).
into(fix_leaking_shower_valve_s5, valve_body).
step(fix_leaking_shower_valve, fix_leaking_shower_valve_s6).
action(fix_leaking_shower_valve_s6, fasten).
object_of(fix_leaking_shower_valve_s6, handle).
uses(fix_leaking_shower_valve_s6, screwdriver).
role(fix_leaking_shower_valve_s6, screwdriver, fasten).
step(fix_leaking_shower_valve, fix_leaking_shower_valve_s7).
action(fix_leaking_shower_valve_s7, turn_on).
object_of(fix_leaking_shower_valve_s7, water_supply_valve).
check(fix_leaking_shower_valve_s7, no_drip).
before(fix_leaking_shower_valve_s1, fix_leaking_shower_valve_s2).
before(fix_leaking_shower_valve_s2, fix_leaking_shower_valve_s3).
before(fix_leaking_shower_valve_s3, fix_leaking_shower_valve_s4).
before(fix_leaking_shower_valve_s4, fix_leaking_shower_valve_s5).
before(fix_leaking_shower_valve_s5, fix_leaking_shower_valve_s6).
before(fix_leaking_shower_valve_s6, fix_leaking_shower_valve_s7).
```

## aliases
```datalog
task_alias(fix_leaking_shower_valve, "fixing a leaking shower valve").
task_alias(fix_leaking_shower_valve, "fix a dripping shower").
task_alias(fix_leaking_shower_valve, "replacing the shower cartridge").
alias(water_supply_valve, "stop valve").
alias(new_cartridge, "replacement cartridge").
alias(grease, "silicone grease").
alias(valve_body, "valve body").
alias(cloth, "dry cloth").
alias(no_drip, "no drip").
alias(seal_slides, "seal slides smoothly").
alias(stop_leak, "stop the leak").
```

## teach
- To fix a leaking shower valve you need a screwdriver, pliers, grease and a replacement cartridge.
- Fixing a leaking shower valve takes 45 to 60 minutes.
- Step 1 of fixing a leaking shower valve: shut off the water supply at the stop valve.
- Step 2 of fixing a leaking shower valve: remove the handle with the screwdriver to loosen its screw.
- Step 3 of fixing a leaking shower valve: lift out the old cartridge with the pliers.
- Step 4 of fixing a leaking shower valve: coat the new cartridge with grease so the seal slides smoothly.
- Step 5 of fixing a leaking shower valve: fit the new cartridge into the valve body.
- Step 6 of fixing a leaking shower valve: fasten the handle back on with the screwdriver.
- Step 7 of fixing a leaking shower valve: turn the water supply back on and check there is no drip.

## checklist
- names the tools and parts needed: screwdriver, pliers, grease and a replacement cartridge
- gives the seven steps in order: shut off water, remove handle, lift out old cartridge, grease new cartridge, fit it, fasten handle, turn water on
- says the pliers lift out the old cartridge and the screwdriver loosens and fastens the handle
- says to check there is no drip after the water is back on
- gives a total time of about 45 to 60 minutes

## variant
- change: add a final step to wipe the valve body dry with a cloth and check for drips
- request: Once the valve is fixed, I want to dry it off and check it straight away. How do I fix my leaking shower valve?
- situation: I have the usual tools, a dry cloth, and I want to check the valve straight after.
- expect: The reply gives the seven steps in order, then adds a final step to wipe the valve body dry with the cloth and check for drips.

```datalog situation
has(user, cloth).
wants(user, dry_check).
```

```datalog difference
extra_need(cloth).
add_step(fix_leaking_shower_valve_s9, fix_leaking_shower_valve_s7).
step(fix_leaking_shower_valve, fix_leaking_shower_valve_s9).
action(fix_leaking_shower_valve_s9, wipe).
object_of(fix_leaking_shower_valve_s9, valve_body).
uses(fix_leaking_shower_valve_s9, cloth).
role(fix_leaking_shower_valve_s9, cloth, wipe).
takes_time(fix_leaking_shower_valve_s9, 2, 3, minute).
before(fix_leaking_shower_valve_s7, fix_leaking_shower_valve_s9).
```

- change: a shorter job: the whole fix must fit into 30 minutes, so the time estimate drops
- request: I'm short on time and need my shower valve fixed in half an hour. Can I do it?
- situation: I have the same tools and only about 30 minutes.
- expect: The reply gives the same seven steps in order and says the job should take about 30 to 40 minutes, so it fits the half hour if the parts are ready.

```datalog situation
has(user, screwdriver).
has(user, pliers).
has(user, grease).
has(user, new_cartridge).
time_limit(user, 30, minute).
```

```datalog difference
change(fix_leaking_shower_valve, takes_time, 45_60, 30_40).
```
