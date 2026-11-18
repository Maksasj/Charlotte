---
id: fix_dripping_tap
stage: tools
task: fix_dripping_tap
schema: v3
---
# Fix a dripping tap

## requests
- My kitchen tap won't stop dripping. Can I fix it myself?
- How do I change the washer in a leaky tap?
- The bathroom tap drips all night. What's the repair?

## situation
- I have an adjustable spanner, a screwdriver and a new washer that fits the tap.
- I have about half an hour.

## ground truth
```datalog
task(fix_dripping_tap).
domain(fix_dripping_tap, repair).
goal(fix_dripping_tap, no_drip).
needs(fix_dripping_tap, spanner).
needs(fix_dripping_tap, screwdriver).
needs(fix_dripping_tap, washer).
takes_time(fix_dripping_tap, 15, 25, minute).
step(fix_dripping_tap, fix_dripping_tap_s1).
action(fix_dripping_tap_s1, shut).
object_of(fix_dripping_tap_s1, water_supply).
takes_time(fix_dripping_tap_s1, 1, 2, minute).
step(fix_dripping_tap, fix_dripping_tap_s2).
action(fix_dripping_tap_s2, remove).
object_of(fix_dripping_tap_s2, handle).
uses(fix_dripping_tap_s2, screwdriver).
role(fix_dripping_tap_s2, screwdriver, loosen).
takes_time(fix_dripping_tap_s2, 2, 3, minute).
step(fix_dripping_tap, fix_dripping_tap_s3).
action(fix_dripping_tap_s3, unscrew).
object_of(fix_dripping_tap_s3, valve).
uses(fix_dripping_tap_s3, spanner).
role(fix_dripping_tap_s3, spanner, loosen).
takes_time(fix_dripping_tap_s3, 2, 3, minute).
step(fix_dripping_tap, fix_dripping_tap_s4).
action(fix_dripping_tap_s4, replace).
object_of(fix_dripping_tap_s4, washer).
takes_time(fix_dripping_tap_s4, 2, 3, minute).
step(fix_dripping_tap, fix_dripping_tap_s5).
action(fix_dripping_tap_s5, reassemble).
object_of(fix_dripping_tap_s5, tap).
step(fix_dripping_tap, fix_dripping_tap_s6).
action(fix_dripping_tap_s6, tighten).
object_of(fix_dripping_tap_s6, tap).
uses(fix_dripping_tap_s6, spanner).
role(fix_dripping_tap_s6, spanner, fasten).
uses(fix_dripping_tap_s6, screwdriver).
role(fix_dripping_tap_s6, screwdriver, fasten).
takes_time(fix_dripping_tap_s6, 3, 5, minute).
step(fix_dripping_tap, fix_dripping_tap_s7).
action(fix_dripping_tap_s7, open).
object_of(fix_dripping_tap_s7, water_supply).
step(fix_dripping_tap, fix_dripping_tap_s8).
action(fix_dripping_tap_s8, check).
object_of(fix_dripping_tap_s8, tap).
check(fix_dripping_tap_s8, no_drip).
takes_time(fix_dripping_tap_s8, 1, 2, minute).
before(fix_dripping_tap_s1, fix_dripping_tap_s2).
before(fix_dripping_tap_s2, fix_dripping_tap_s3).
before(fix_dripping_tap_s3, fix_dripping_tap_s4).
before(fix_dripping_tap_s4, fix_dripping_tap_s5).
before(fix_dripping_tap_s5, fix_dripping_tap_s6).
before(fix_dripping_tap_s6, fix_dripping_tap_s7).
before(fix_dripping_tap_s7, fix_dripping_tap_s8).
```

## aliases
```datalog
task_alias(fix_dripping_tap, "fixing a dripping tap").
task_alias(fix_dripping_tap, "leaky tap").
task_alias(fix_dripping_tap, "change the washer").
alias(spanner, "adjustable spanner").
alias(washer, "new washer").
alias(washer, "old washer").
alias(handle, "tap handle").
alias(water_supply, "water").
alias(shut, "shut off").
alias(reassemble, "put back on").
alias(no_drip, "the tap no longer drips").
alias(cartridge, "ceramic cartridge").
```

## teach
- To fix a dripping tap you need an adjustable spanner, a screwdriver and a new washer.
- Fixing a dripping tap takes 15 to 25 minutes.
- Step 1 of fixing a dripping tap: shut off the water supply under the sink for 1 to 2 minutes.
- Step 2 of fixing a dripping tap: remove the tap handle with the screwdriver for 2 to 3 minutes.
- Step 3 of fixing a dripping tap: unscrew the valve with the spanner for 2 to 3 minutes.
- Step 4 of fixing a dripping tap: replace the old washer with the new one for 2 to 3 minutes.
- Step 5 of fixing a dripping tap: put the valve and the handle back on the tap.
- Step 6 of fixing a dripping tap: tighten them with the spanner and the screwdriver for 3 to 5 minutes.
- Step 7 of fixing a dripping tap: open the water supply.
- Step 8 of fixing a dripping tap: check that the tap no longer drips for 1 to 2 minutes.

## checklist
- names what is needed: an adjustable spanner, a screwdriver and a new washer
- gives the steps in order: shut water supply, remove handle, unscrew valve, replace washer, reassemble tap, open water supply
- gives a total time of about 15 to 25 minutes
- closes politely

## variant
- change: mixer tap with a ceramic cartridge: replace the cartridge instead of a washer
- request: My dripping tap is a single-lever mixer. Does it have a washer too?
- situation: I have an adjustable spanner, a screwdriver and a new cartridge that matches my mixer tap.
- expect: The reply keeps shutting off the water and taking off the handle, then swaps the old cartridge for the new one instead of a washer before reassembling and testing.

```datalog situation
has(user, spanner).
has(user, screwdriver).
has(user, cartridge).
```

```datalog difference
change(fix_dripping_tap_s4, object_of, washer, cartridge).
extra_need(cartridge).
no_need(washer).
```
