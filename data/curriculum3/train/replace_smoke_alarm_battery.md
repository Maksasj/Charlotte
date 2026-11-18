---
id: replace_smoke_alarm_battery
stage: tools
task: replace_smoke_alarm_battery
schema: v3
---
# Replace a smoke alarm battery

## requests
- My smoke alarm keeps chirping. How do I change its battery?
- Can you walk me through swapping the battery in a ceiling smoke detector?
- What's the safe way to put a new battery in the smoke alarm?

## situation
- I have a new 9-volt battery and a step ladder.
- It's chirping right now.

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

## checklist
- names what is needed: a new battery and a step ladder
- gives the steps in order: place step ladder, twist alarm, replace battery, attach alarm, test alarm
- gives a total time of about 5 to 8 minutes
- closes politely
