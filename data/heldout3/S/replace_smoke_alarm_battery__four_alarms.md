---
id: replace_smoke_alarm_battery__four_alarms
stage: tools
task: replace_smoke_alarm_battery
schema: v3
tier: S
base: replace_smoke_alarm_battery
---
# Replace a smoke alarm battery

## requests
- We've got four smoke detectors in the house and I want to put fresh batteries in all of them today. How?

## ground truth
```datalog
task(replace_smoke_alarm_battery).
domain(replace_smoke_alarm_battery, repair).
goal(replace_smoke_alarm_battery, working_smoke_alarm).
needs(replace_smoke_alarm_battery, battery).
needs(replace_smoke_alarm_battery, step_ladder).
takes_time(replace_smoke_alarm_battery, 5, 8, minute).
step(replace_smoke_alarm_battery, replace_smoke_alarm_battery_s1).
action(replace_smoke_alarm_battery_s1, place).
object_of(replace_smoke_alarm_battery_s1, step_ladder).
takes_time(replace_smoke_alarm_battery_s1, 1, 1, minute).
step(replace_smoke_alarm_battery, replace_smoke_alarm_battery_s2).
action(replace_smoke_alarm_battery_s2, twist).
object_of(replace_smoke_alarm_battery_s2, alarm).
from(replace_smoke_alarm_battery_s2, base).
takes_time(replace_smoke_alarm_battery_s2, 1, 1, minute).
step(replace_smoke_alarm_battery, replace_smoke_alarm_battery_s3).
action(replace_smoke_alarm_battery_s3, replace).
object_of(replace_smoke_alarm_battery_s3, battery).
check(replace_smoke_alarm_battery_s3, ends_matched).
takes_time(replace_smoke_alarm_battery_s3, 1, 2, minute).
step(replace_smoke_alarm_battery, replace_smoke_alarm_battery_s4).
action(replace_smoke_alarm_battery_s4, attach).
object_of(replace_smoke_alarm_battery_s4, alarm).
onto(replace_smoke_alarm_battery_s4, base).
takes_time(replace_smoke_alarm_battery_s4, 1, 1, minute).
step(replace_smoke_alarm_battery, replace_smoke_alarm_battery_s5).
action(replace_smoke_alarm_battery_s5, test).
object_of(replace_smoke_alarm_battery_s5, alarm).
with_part(replace_smoke_alarm_battery_s5, test_button).
until(replace_smoke_alarm_battery_s5, beeping).
takes_time(replace_smoke_alarm_battery_s5, 1, 1, minute).
before(replace_smoke_alarm_battery_s1, replace_smoke_alarm_battery_s2).
before(replace_smoke_alarm_battery_s2, replace_smoke_alarm_battery_s3).
before(replace_smoke_alarm_battery_s3, replace_smoke_alarm_battery_s4).
before(replace_smoke_alarm_battery_s4, replace_smoke_alarm_battery_s5).
```

## aliases
```datalog
task_alias(replace_smoke_alarm_battery, "replacing a smoke alarm battery").
task_alias(replace_smoke_alarm_battery, "replace a smoke alarm battery").
task_alias(replace_smoke_alarm_battery, "change its battery").
task_alias(replace_smoke_alarm_battery, "swapping the battery in a smoke detector").
alias(alarm, "smoke alarm").
alias(alarm, "smoke detector").
alias(battery, "new battery").
alias(battery, "9-volt battery").
alias(step_ladder, "step ladder").
alias(attach, "twist back onto").
alias(test_button, "test button").
alias(ends_matched, "matching the plus and minus ends").
alias(beeping, "the alarm beeps").
```

## teach
- To replace a smoke alarm battery you need a new battery and a step ladder.
- Replacing a smoke alarm battery takes 5 to 8 minutes.
- Step 1 of replacing a smoke alarm battery: place the step ladder firmly under the alarm for 1 minute.
- Step 2 of replacing a smoke alarm battery: twist the alarm off its base for 1 minute.
- Step 3 of replacing a smoke alarm battery: replace the old battery with the new one, matching the plus and minus ends, for 1 to 2 minutes.
- Step 4 of replacing a smoke alarm battery: twist the alarm back onto its base for 1 minute.
- Step 5 of replacing a smoke alarm battery: press the test button until the alarm beeps, for 1 minute.

## variant
- change: four alarms: repeat the battery change for each alarm
- request: We've got four smoke detectors in the house and I want to put fresh batteries in all of them today. How?
- situation: There are four alarms. I have four new 9-volt batteries and a step ladder.
- expect: The reply repeats placing the ladder, twisting off, replacing, reattaching and testing for each of the four alarms, about 20 to 30 minutes in total.

```datalog situation
has(user, battery).
has(user, step_ladder).
count(user, alarm, 4).
```

```datalog difference
change(replace_smoke_alarm_battery_s1, repeat, 1, 4).
change(replace_smoke_alarm_battery_s2, repeat, 1, 4).
change(replace_smoke_alarm_battery_s3, repeat, 1, 4).
change(replace_smoke_alarm_battery_s4, repeat, 1, 4).
change(replace_smoke_alarm_battery_s5, repeat, 1, 4).
change(replace_smoke_alarm_battery, takes_time, 5_8, 20_30).
```
