---
id: tighten_loose_screw
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Tighten a loose screw

Find the loose screw and turn it tight with the right screwdriver.

## ground truth
```datalog
task(tighten_loose_screw).
domain(tighten_loose_screw, repair).
goal(tighten_loose_screw, tightened_screw).
needs(tighten_loose_screw, screwdriver).
needs(tighten_loose_screw, flashlight).
takes_time(tighten_loose_screw, 2, 5, minute).
step(tighten_loose_screw, tighten_loose_screw_s1).
action(tighten_loose_screw_s1, find).
object_of(tighten_loose_screw_s1, screw).
uses(tighten_loose_screw_s1, flashlight).
role(tighten_loose_screw_s1, flashlight, light).
takes_time(tighten_loose_screw_s1, 1, 2, minute).
step(tighten_loose_screw, tighten_loose_screw_s2).
action(tighten_loose_screw_s2, choose).
object_of(tighten_loose_screw_s2, screwdriver).
check(tighten_loose_screw_s2, fits_screw).
step(tighten_loose_screw, tighten_loose_screw_s3).
action(tighten_loose_screw_s3, turn).
object_of(tighten_loose_screw_s3, screw).
uses(tighten_loose_screw_s3, screwdriver).
role(tighten_loose_screw_s3, screwdriver, fasten).
takes_time(tighten_loose_screw_s3, 1, 2, minute).
until(tighten_loose_screw_s3, tight).
step(tighten_loose_screw, tighten_loose_screw_s4).
action(tighten_loose_screw_s4, test).
object_of(tighten_loose_screw_s4, fitting).
before(tighten_loose_screw_s1, tighten_loose_screw_s2).
before(tighten_loose_screw_s2, tighten_loose_screw_s3).
before(tighten_loose_screw_s3, tighten_loose_screw_s4).
```

## aliases
```datalog
task_alias(tighten_loose_screw, "tightening a loose screw").
task_alias(tighten_loose_screw, "fix a loose screw").
task_alias(tighten_loose_screw, "tighten a screw").
alias(flashlight, "torch").
alias(screw, "loose screw").
alias(turn, "tighten").
alias(fits_screw, "right screwdriver").
alias(fitting, "handle").
```
