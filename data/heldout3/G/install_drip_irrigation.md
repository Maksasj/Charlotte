---
id: install_drip_irrigation
stage: household
task: install_drip_irrigation
schema: v3
tier: G
---
# Install drip irrigation

## requests
- I'm tired of watering the veg patch by hand every evening. How do I set up a soaker hose?
- How do I lay a drip line along my vegetable rows?
- Could you explain how to install drip irrigation from the outside tap?

## situation
- I have a soaker hose, scissors, an end cap, a bag of garden pegs and a hose connector, and there's an outside tap.

## ground truth
```datalog
task(install_drip_irrigation).
domain(install_drip_irrigation, gardening).
goal(install_drip_irrigation, watered_garden).
needs(install_drip_irrigation, drip_hose).
needs(install_drip_irrigation, scissors).
needs(install_drip_irrigation, end_cap).
needs(install_drip_irrigation, garden_peg).
needs(install_drip_irrigation, hose_connector).
needs(install_drip_irrigation, tap).
takes_time(install_drip_irrigation, 30, 45, minute).
step(install_drip_irrigation, install_drip_irrigation_s1).
action(install_drip_irrigation_s1, lay).
object_of(install_drip_irrigation_s1, drip_hose).
onto(install_drip_irrigation_s1, bed).
takes_time(install_drip_irrigation_s1, 10, 15, minute).
step(install_drip_irrigation, install_drip_irrigation_s2).
action(install_drip_irrigation_s2, cut).
object_of(install_drip_irrigation_s2, drip_hose).
uses(install_drip_irrigation_s2, scissors).
role(install_drip_irrigation_s2, scissors, cut).
step(install_drip_irrigation, install_drip_irrigation_s3).
action(install_drip_irrigation_s3, fit).
object_of(install_drip_irrigation_s3, end_cap).
onto(install_drip_irrigation_s3, drip_hose).
step(install_drip_irrigation, install_drip_irrigation_s4).
action(install_drip_irrigation_s4, pin).
object_of(install_drip_irrigation_s4, drip_hose).
uses(install_drip_irrigation_s4, garden_peg).
role(install_drip_irrigation_s4, garden_peg, hold).
takes_time(install_drip_irrigation_s4, 5, 10, minute).
step(install_drip_irrigation, install_drip_irrigation_s5).
action(install_drip_irrigation_s5, attach).
object_of(install_drip_irrigation_s5, drip_hose).
onto(install_drip_irrigation_s5, tap).
uses(install_drip_irrigation_s5, hose_connector).
role(install_drip_irrigation_s5, hose_connector, fasten).
step(install_drip_irrigation, install_drip_irrigation_s6).
action(install_drip_irrigation_s6, turn_on).
object_of(install_drip_irrigation_s6, tap).
check(install_drip_irrigation_s6, water_drips_evenly).
before(install_drip_irrigation_s1, install_drip_irrigation_s2).
before(install_drip_irrigation_s2, install_drip_irrigation_s3).
before(install_drip_irrigation_s3, install_drip_irrigation_s4).
before(install_drip_irrigation_s4, install_drip_irrigation_s5).
before(install_drip_irrigation_s5, install_drip_irrigation_s6).
```

## aliases
```datalog
task_alias(install_drip_irrigation, "installing drip irrigation").
task_alias(install_drip_irrigation, "set up a soaker hose").
task_alias(install_drip_irrigation, "lay a drip line").
alias(drip_hose, "soaker hose").
alias(drip_hose, "drip line").
alias(garden_peg, "garden pegs").
alias(hose_connector, "connector").
alias(tap, "outside tap").
alias(bed, "vegetable rows").
alias(bed, "veg patch").
alias(water_timer, "tap timer").
alias(water_timer, "timer").
alias(tent_peg, "tent pegs").
alias(water_drips_evenly, "water seeps out evenly along the whole hose").
alias(automatic_watering, "water itself while I'm away").
```

## teach
- To install drip irrigation you need a soaker hose, scissors, an end cap, garden pegs, a hose connector and an outside tap.
- Installing drip irrigation takes 30 to 45 minutes.
- Step 1 of installing drip irrigation: lay the soaker hose along the bed, for 10 to 15 minutes.
- Step 2 of installing drip irrigation: cut the soaker hose to length with the scissors.
- Step 3 of installing drip irrigation: fit the end cap onto the far end of the hose.
- Step 4 of installing drip irrigation: pin the hose down with the garden pegs, for 5 to 10 minutes.
- Step 5 of installing drip irrigation: attach the hose to the tap with the hose connector.
- Step 6 of installing drip irrigation: turn on the tap and check the water seeps out evenly along the whole hose.

## checklist
- names what is needed: a soaker hose, scissors, an end cap, pegs, a connector and a tap
- gives the steps in order: lay, cut, cap, peg, connect, turn on
- says to check the water seeps out evenly
- gives a total time of about 30 to 45 minutes

## variant
- change: wants it automatic: fit a tap timer before connecting the hose
- request: We're going away for two weeks in July. Can I set up a drip line that will water itself while I'm away?
- situation: I have a soaker hose, scissors, an end cap, garden pegs, a hose connector, an outside tap and a battery tap timer.
- expect: The reply adds a step to fit the timer onto the tap before attaching the hose, and to set it to water once a day.

```datalog situation
has(user, drip_hose).
has(user, scissors).
has(user, end_cap).
has(user, garden_peg).
has(user, hose_connector).
has(user, tap).
has(user, water_timer).
wants(user, automatic_watering).
```

```datalog difference
extra_need(water_timer).
add_step(install_drip_irrigation_s9, install_drip_irrigation_s4).
step(install_drip_irrigation, install_drip_irrigation_s9).
action(install_drip_irrigation_s9, fit).
object_of(install_drip_irrigation_s9, water_timer).
onto(install_drip_irrigation_s9, tap).
setting(install_drip_irrigation_s9, interval, 1, day).
before(install_drip_irrigation_s4, install_drip_irrigation_s9).
before(install_drip_irrigation_s9, install_drip_irrigation_s5).
```

## variant
- change: no garden pegs: pin the hose down with tent pegs
- request: The soaker hose kit didn't come with any pegs to hold it down. I've got a bag of old tent pegs. Would those do the job?
- situation: I have a soaker hose, scissors, an end cap, a hose connector, an outside tap and some tent pegs, but no garden pegs.
- expect: The reply pins the hose down with the tent pegs instead of garden pegs and keeps the other steps.

```datalog situation
lacks(user, garden_peg).
has(user, tent_peg).
has(user, drip_hose).
has(user, scissors).
has(user, end_cap).
has(user, hose_connector).
has(user, tap).
```

```datalog difference
no_need(garden_peg).
extra_need(tent_peg).
swap(install_drip_irrigation_s4, garden_peg, tent_peg).
```
