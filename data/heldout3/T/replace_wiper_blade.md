---
id: replace_wiper_blade
stage: tools
task: replace_wiper_blade
schema: v3
tier: T
---
# Replace a windscreen wiper blade

## requests
- My wipers are smearing the windscreen. How do I put new blades on?
- How do I change a windscreen wiper blade?
- The rubber on my driver's side wiper is split and squeaks across the glass. What's the fix?

## situation
- I have a tape measure and a new wiper blade.

## ground truth
```datalog
task(replace_wiper_blade).
domain(replace_wiper_blade, repair).
goal(replace_wiper_blade, clear_windscreen).
needs(replace_wiper_blade, tape_measure).
needs(replace_wiper_blade, new_wiper_blade).
takes_time(replace_wiper_blade, 10, 15, minute).
step(replace_wiper_blade, replace_wiper_blade_s1).
action(replace_wiper_blade_s1, measure).
object_of(replace_wiper_blade_s1, old_blade).
uses(replace_wiper_blade_s1, tape_measure).
role(replace_wiper_blade_s1, tape_measure, measure).
purpose(replace_wiper_blade_s1, right_size).
step(replace_wiper_blade, replace_wiper_blade_s2).
action(replace_wiper_blade_s2, lift).
object_of(replace_wiper_blade_s2, wiper_arm).
warning(replace_wiper_blade_s2, arm_snaps_back).
step(replace_wiper_blade, replace_wiper_blade_s3).
action(replace_wiper_blade_s3, press).
object_of(replace_wiper_blade_s3, release_tab).
step(replace_wiper_blade, replace_wiper_blade_s4).
action(replace_wiper_blade_s4, slide).
object_of(replace_wiper_blade_s4, old_blade).
from(replace_wiper_blade_s4, wiper_arm).
step(replace_wiper_blade, replace_wiper_blade_s5).
action(replace_wiper_blade_s5, attach).
object_of(replace_wiper_blade_s5, new_wiper_blade).
onto(replace_wiper_blade_s5, wiper_arm).
check(replace_wiper_blade_s5, clicks).
step(replace_wiper_blade, replace_wiper_blade_s6).
action(replace_wiper_blade_s6, lower).
object_of(replace_wiper_blade_s6, wiper_arm).
before(replace_wiper_blade_s1, replace_wiper_blade_s2).
before(replace_wiper_blade_s2, replace_wiper_blade_s3).
before(replace_wiper_blade_s3, replace_wiper_blade_s4).
before(replace_wiper_blade_s4, replace_wiper_blade_s5).
before(replace_wiper_blade_s5, replace_wiper_blade_s6).
```

## aliases
```datalog
task_alias(replace_wiper_blade, "changing a wiper blade").
task_alias(replace_wiper_blade, "new wiper blades").
task_alias(replace_wiper_blade, "fitting windscreen wipers").
alias(old_blade, "the old blade").
alias(new_wiper_blade, "new blade").
alias(wiper_arm, "the wiper arm").
alias(release_tab, "the little tab").
alias(arm_snaps_back, "the arm can snap back and crack the glass").
alias(clicks, "it clicks into place").
```

## teach
- To replace a windscreen wiper blade you need a tape measure and a new wiper blade.
- Replacing a wiper blade takes 10 to 15 minutes.
- Step 1 of replacing a wiper blade: measure the old blade with the tape measure to get the right size.
- Step 2 of replacing a wiper blade: lift the wiper arm off the glass; hold it, because the arm can snap back and crack the windscreen.
- Step 3 of replacing a wiper blade: press the release tab.
- Step 4 of replacing a wiper blade: slide the old blade off the wiper arm.
- Step 5 of replacing a wiper blade: attach the new blade onto the arm until it clicks.
- Step 6 of replacing a wiper blade: lower the wiper arm back onto the glass.

## checklist
- names what is needed: a tape measure and a new blade
- gives the steps in order: measure, lift arm, press tab, slide off, click new one on, lower arm
- warns the arm can snap back onto the glass

## variant
- change: no new blade: it cannot be replaced until one is bought
- request: My wiper rubber's torn. Can I replace the blade with what I have at home? I've no spare blade.
- situation: I have a tape measure but no replacement wiper blade.
- expect: The reply says the blade cannot be replaced without a new one, and suggests measuring the old one so the right size can be bought.

```datalog situation
has(user, tape_measure).
lacks(user, new_wiper_blade).
```

```datalog difference
cannot(no_replacement_blade).
```

## variant
- change: no tape measure: measure the old blade with a piece of string
- request: I need to know what size wiper blade to buy but I don't have a tape measure. Any trick?
- situation: I have a piece of string and can see the shop's size chart online.
- expect: The reply marks the old blade's length on a piece of string and compares it with the size chart instead of using a tape measure.

```datalog situation
has(user, string).
lacks(user, tape_measure).
```

```datalog difference
swap(replace_wiper_blade_s1, tape_measure, string).
extra_need(string).
no_need(tape_measure).
```
