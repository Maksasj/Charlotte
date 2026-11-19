---
id: fix_running_toilet
stage: procedures
task: fix_running_toilet
schema: v3
---
# Fix a running toilet

## requests
- The toilet keeps running after I flush it. How do I stop that?
- What's the fix for a toilet that won't stop filling?
- Could you walk me through repairing a running toilet step by step?

## situation
- I have a rag, a pair of pliers, and a toilet whose tank keeps refilling.
- I can reach the shut-off valve behind the toilet.

## ground truth
```datalog
task(fix_running_toilet).
domain(fix_running_toilet, repair).
goal(fix_running_toilet, stop_running_toilet).
needs(fix_running_toilet, cloth).
needs(fix_running_toilet, pliers).
takes_time(fix_running_toilet, 15, 30, minute).
step(fix_running_toilet, fix_running_toilet_s1).
action(fix_running_toilet_s1, close).
object_of(fix_running_toilet_s1, water_valve).
purpose(fix_running_toilet_s1, stop_water_flow).
takes_time(fix_running_toilet_s1, 1, 1, minute).
step(fix_running_toilet, fix_running_toilet_s2).
action(fix_running_toilet_s2, flush).
object_of(fix_running_toilet_s2, toilet).
purpose(fix_running_toilet_s2, empty_tank).
takes_time(fix_running_toilet_s2, 2, 5, minute).
step(fix_running_toilet, fix_running_toilet_s3).
action(fix_running_toilet_s3, wipe).
object_of(fix_running_toilet_s3, flapper_seat).
uses(fix_running_toilet_s3, cloth).
role(fix_running_toilet_s3, cloth, wipe).
check(fix_running_toilet_s3, seat_clean).
overlaps(fix_running_toilet_s3, fix_running_toilet_s2).
step(fix_running_toilet, fix_running_toilet_s4).
action(fix_running_toilet_s4, adjust).
object_of(fix_running_toilet_s4, flapper_chain).
uses(fix_running_toilet_s4, pliers).
role(fix_running_toilet_s4, pliers, hold).
warning(fix_running_toilet_s4, chain_too_tight).
takes_time(fix_running_toilet_s4, 3, 5, minute).
step(fix_running_toilet, fix_running_toilet_s5).
action(fix_running_toilet_s5, open).
object_of(fix_running_toilet_s5, water_valve).
check(fix_running_toilet_s5, tank_stops_refilling).
takes_time(fix_running_toilet_s5, 1, 1, minute).
before(fix_running_toilet_s1, fix_running_toilet_s2).
before(fix_running_toilet_s3, fix_running_toilet_s4).
before(fix_running_toilet_s2, fix_running_toilet_s4).
before(fix_running_toilet_s4, fix_running_toilet_s5).
```

## aliases
```datalog
task_alias(fix_running_toilet, "fix a running toilet").
task_alias(fix_running_toilet, "stop a toilet running").
task_alias(fix_running_toilet, "repair a running toilet").
alias(water_valve, "shut-off valve").
alias(flapper_seat, "flapper seat").
alias(flapper_chain, "flapper chain").
alias(cloth, "rag").
alias(pliers, "pair of pliers").
alias(stop_water_flow, "stops water coming in").
alias(empty_tank, "empties the tank").
alias(chain_too_tight, "chain too tight").
alias(tank_stops_refilling, "tank stops refilling").
```

## teach
- To fix a running toilet you need a cloth and pliers.
- Fixing a running toilet takes 15 to 30 minutes.
- Step 1 of fix a running toilet: shut off the water valve behind the toilet to stop water flow, for 1 minute.
- Step 2 of fix a running toilet: flush the toilet to empty the tank, for 2 to 5 minutes.
- While step 2 runs, step 3 of fix a running toilet: wipe the flapper seat clean with the cloth.
- Step 4 of fix a running toilet: adjust the flapper chain with the pliers so it has a little slack, for 3 to 5 minutes; a chain that is too tight keeps the flapper open.
- Step 5 of fix a running toilet: open the water valve again and wait until the tank stops refilling, for 1 minute.

## checklist
- names what is needed, including a cloth and pliers
- gives the steps in order: shut off the valve, flush, wipe the seat, adjust the chain, reopen the valve
- says the wiping can be done while the tank drains
- warns that a chain adjusted too tight keeps the toilet running
- gives a total time of about 15 to 30 minutes

## variant
- change: no cloth; use a paper towel to wipe the flapper seat
- request: My rag is missing. Can I wipe the flapper seat with a paper towel instead?
- situation: I have pliers, a paper towel and a shut-off valve, but no cloth.
- expect: The reply keeps the same five steps and replaces the cloth with a paper towel in the wiping step.

```datalog situation
lacks(user, cloth).
has(user, paper_towel).
has(user, pliers).
```

```datalog difference
no_need(cloth).
extra_need(paper_towel).
swap(fix_running_toilet_s3, cloth, paper_towel).
```

## variant
- change: no pliers; use an adjustable wrench to hold the chain while adjusting it
- request: I can't find my pliers. Will an adjustable wrench work for the chain?
- situation: I have an adjustable wrench and a rag, but no pliers.
- expect: The reply keeps the same five steps and uses the adjustable wrench in place of the pliers in the chain-adjusting step.

```datalog situation
lacks(user, pliers).
has(user, adjustable_wrench).
has(user, cloth).
```

```datalog difference
no_need(pliers).
extra_need(adjustable_wrench).
swap(fix_running_toilet_s4, pliers, adjustable_wrench).
```
