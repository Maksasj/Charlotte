---
id: replace_lightbulb
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# replace lightbulb

Learned in conversation.

```datalog
task(replace_lightbulb). @conf 1 @source ""
needs(replace_lightbulb, new_lightbulb). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
needs(replace_lightbulb, step_ladder). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
takes_time(replace_lightbulb, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
step(replace_lightbulb, replace_lightbulb_s1). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
action(replace_lightbulb_s1, switch_off). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
object_of(replace_lightbulb_s1, light). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
step(replace_lightbulb, replace_lightbulb_s2). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
action(replace_lightbulb_s2, wait). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
object_of(replace_lightbulb_s2, old_bulb). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
until(replace_lightbulb_s2, cool). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
takes_time(replace_lightbulb_s2, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
before(replace_lightbulb_s1, replace_lightbulb_s2). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
step(replace_lightbulb, replace_lightbulb_s3). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
action(replace_lightbulb_s3, climb). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
object_of(replace_lightbulb_s3, step_ladder). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
before(replace_lightbulb_s2, replace_lightbulb_s3). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
step(replace_lightbulb, replace_lightbulb_s4). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
action(replace_lightbulb_s4, unscrew). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
object_of(replace_lightbulb_s4, old_bulb). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
before(replace_lightbulb_s3, replace_lightbulb_s4). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
step(replace_lightbulb, replace_lightbulb_s5). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
action(replace_lightbulb_s5, screw). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
object_of(replace_lightbulb_s5, new_lightbulb). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
before(replace_lightbulb_s4, replace_lightbulb_s5). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
step(replace_lightbulb, replace_lightbulb_s6). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
action(replace_lightbulb_s6, test). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
object_of(replace_lightbulb_s6, light). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
before(replace_lightbulb_s5, replace_lightbulb_s6). @conf 1 @source teacher:curriculum @conv s1794998645628-540404-68
```
