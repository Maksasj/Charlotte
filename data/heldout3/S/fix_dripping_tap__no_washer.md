---
id: fix_dripping_tap__no_washer
stage: tools
task: fix_dripping_tap
schema: v3
tier: S
base: fix_dripping_tap
---
# Fix a dripping tap

## requests
- My bathroom tap won't stop dripping. I've got a spanner and screwdriver, but no new washer and the hardware shop is shut till Monday. Can I sort it tonight?

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

## variant
- change: no spare washer: the repair cannot be done tonight
- request: My bathroom tap won't stop dripping. I've got a spanner and screwdriver, but no new washer and the hardware shop is shut till Monday. Can I sort it tonight?
- situation: I have an adjustable spanner and a screwdriver, but no new washer and no shop open.
- expect: The reply says the tap cannot be properly fixed without a new washer and suggests getting one when the shop opens.

```datalog situation
has(user, spanner).
has(user, screwdriver).
lacks(user, washer).
```

```datalog difference
cannot(no_spare_washer).
```
