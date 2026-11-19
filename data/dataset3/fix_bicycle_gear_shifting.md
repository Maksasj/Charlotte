---
id: fix_bicycle_gear_shifting
stage: procedures
task: fix_bicycle_gear_shifting
schema: v3
---
# Fix bicycle gear shifting

## requests
- My bike's gears skip and won't change smoothly. How do I fix the shifting myself?
- Could you tell me how to adjust the gear cable so my bicycle shifts properly again?
- The chain jumps between gears on my bike. What's the procedure to get the shifting working right?

## situation
- I have a bicycle with a rear derailleur that won't shift cleanly, a rag and a hex key.
- I have about half an hour to work on it.

## ground truth
```datalog
task(fix_bicycle_gear_shifting).
domain(fix_bicycle_gear_shifting, repair).
goal(fix_bicycle_gear_shifting, smooth_gear_shifting).
needs(fix_bicycle_gear_shifting, bicycle).
needs(fix_bicycle_gear_shifting, rag).
needs(fix_bicycle_gear_shifting, hex_key).
takes_time(fix_bicycle_gear_shifting, 30, 45, minute).
step(fix_bicycle_gear_shifting, fix_bicycle_gear_shifting_s1).
action(fix_bicycle_gear_shifting_s1, wipe).
object_of(fix_bicycle_gear_shifting_s1, chain).
uses(fix_bicycle_gear_shifting_s1, rag).
role(fix_bicycle_gear_shifting_s1, rag, wipe).
optional(fix_bicycle_gear_shifting_s1).
step(fix_bicycle_gear_shifting, fix_bicycle_gear_shifting_s2).
action(fix_bicycle_gear_shifting_s2, loosen).
object_of(fix_bicycle_gear_shifting_s2, cable_bolt).
uses(fix_bicycle_gear_shifting_s2, hex_key).
role(fix_bicycle_gear_shifting_s2, hex_key, loosen).
step(fix_bicycle_gear_shifting, fix_bicycle_gear_shifting_s3).
action(fix_bicycle_gear_shifting_s3, pull).
object_of(fix_bicycle_gear_shifting_s3, cable).
purpose(fix_bicycle_gear_shifting_s3, cable_taut).
step(fix_bicycle_gear_shifting, fix_bicycle_gear_shifting_s4).
action(fix_bicycle_gear_shifting_s4, tighten).
object_of(fix_bicycle_gear_shifting_s4, cable_bolt).
uses(fix_bicycle_gear_shifting_s4, hex_key).
role(fix_bicycle_gear_shifting_s4, hex_key, fasten).
warning(fix_bicycle_gear_shifting_s4, cable_too_loose).
step(fix_bicycle_gear_shifting, fix_bicycle_gear_shifting_s5).
action(fix_bicycle_gear_shifting_s5, test).
object_of(fix_bicycle_gear_shifting_s5, gears).
check(fix_bicycle_gear_shifting_s5, shifts_cleanly).
before(fix_bicycle_gear_shifting_s1, fix_bicycle_gear_shifting_s2).
before(fix_bicycle_gear_shifting_s2, fix_bicycle_gear_shifting_s3).
before(fix_bicycle_gear_shifting_s3, fix_bicycle_gear_shifting_s4).
before(fix_bicycle_gear_shifting_s4, fix_bicycle_gear_shifting_s5).
```

## aliases
```datalog
task_alias(fix_bicycle_gear_shifting, "fix bike gears").
task_alias(fix_bicycle_gear_shifting, "adjust bicycle gears").
alias(bicycle, "bike").
alias(hex_key, "allen key").
alias(cable_bolt, "cable clamp bolt").
alias(shifts_cleanly, "shifts smoothly").
alias(cable_taut, "cable tight").
```

## teach
- To fix bicycle gear shifting you need a bicycle, a rag and a hex key.
- Fixing bicycle gear shifting takes 30 to 45 minutes.
- Step 1 of fixing bicycle gear shifting: wipe the chain with the rag, which can be skipped if you are short on time.
- Step 2 of fixing bicycle gear shifting: loosen the cable bolt with the hex key (a multitool with the right bit also works).
- Step 3 of fixing bicycle gear shifting: pull the cable taut by hand so the derailleur sits in place.
- Step 4 of fixing bicycle gear shifting: tighten the cable bolt with the hex key, but not too tight.
- Step 5 of fixing bicycle gear shifting: test the gears and check that they shift smoothly.

## checklist
- names what is needed, including a bicycle, a rag and a hex key
- gives the steps in order: wipe chain, loosen cable bolt, pull cable taut, tighten cable bolt, test gears
- says the chain wiping is optional and can be skipped when short on time
- warns not to overtighten the cable bolt
- gives a total time of about 30 to 45 minutes

## variant
- change: no hex key, so use a multitool instead
- request: My bike needs its gears adjusted but I only have a multitool, no hex key. Can I still do it?
- situation: I have a bicycle, a rag and a multitool, but no hex key.
- expect: The reply uses the multitool with the right bit wherever the hex key was needed for loosening and tightening the cable bolt, and keeps the rest of the steps the same.

```datalog situation
lacks(user, hex_key).
has(user, multitool).
```

```datalog difference
swap(fix_bicycle_gear_shifting_s2, hex_key, multitool).
swap(fix_bicycle_gear_shifting_s4, hex_key, multitool).
no_need(hex_key).
extra_need(multitool).
```

## variant
- change: only 15 minutes and no rag, so skip the chain wiping
- request: I only have 15 minutes and no rag. Is there a quick way to get my bike shifting again?
- situation: I have a bicycle and a hex key, no rag, and 15 minutes.
- expect: The reply leaves out the chain wiping step, since it is optional, and gives the remaining steps to fit in about 10 to 15 minutes.

```datalog situation
lacks(user, rag).
time_limit(user, 15, minute).
```

```datalog difference
drop(fix_bicycle_gear_shifting_s1).
no_need(rag).
change(fix_bicycle_gear_shifting, takes_time, 30_45, 10_15).
```
