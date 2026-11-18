---
id: replace_battery
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# replace battery

Learned in conversation.

```datalog
task(replace_battery). @conf 1 @source ""
needs(replace_battery, battery). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
needs(replace_battery, step_ladder). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
takes_time(replace_battery, 5, 8, minute). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
step(replace_battery, replace_battery_s1). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
action(replace_battery_s1, place). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
object_of(replace_battery_s1, step_ladder). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
takes_time(replace_battery_s1, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
step(replace_battery, replace_battery_s2). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
action(replace_battery_s2, twist). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
object_of(replace_battery_s2, alarm). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
takes_time(replace_battery_s2, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
before(replace_battery_s1, replace_battery_s2). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
step(replace_battery, replace_battery_s3). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
action(replace_battery_s3, replace). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
object_of(replace_battery_s3, battery). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
takes_time(replace_battery_s3, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
before(replace_battery_s2, replace_battery_s3). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
step(replace_battery, replace_battery_s4). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
action(replace_battery_s4, twist). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
object_of(replace_battery_s4, alarm). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
takes_time(replace_battery_s4, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
before(replace_battery_s3, replace_battery_s4). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
step(replace_battery, replace_battery_s5). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
action(replace_battery_s5, press). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
object_of(replace_battery_s5, test_button). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
until(replace_battery_s5, beeps). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
takes_time(replace_battery_s5, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
before(replace_battery_s4, replace_battery_s5). @conf 1 @source teacher:curriculum @conv s1794982375933-336211-85
needs(replace_battery, new_lightbulb). @conf 1 @source teacher:curriculum @conv s1794982673597-336211-533
```
