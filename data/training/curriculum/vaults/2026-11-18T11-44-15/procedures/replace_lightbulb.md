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
needs(replace_lightbulb, new_lightbulb). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
needs(replace_lightbulb, step_ladder). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
takes_time(replace_lightbulb, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
step(replace_lightbulb, replace_lightbulb_s1). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
step(replace_lightbulb, replace_lightbulb_s2). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
action(replace_lightbulb_s2, wait). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
object_of(replace_lightbulb_s2, old_bulb). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
until(replace_lightbulb_s2, cool). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
takes_time(replace_lightbulb_s2, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
before(replace_lightbulb_s1, replace_lightbulb_s2). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
step(replace_lightbulb, replace_lightbulb_s3). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
action(replace_lightbulb_s3, climb). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
object_of(replace_lightbulb_s3, step_ladder). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
before(replace_lightbulb_s2, replace_lightbulb_s3). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
step(replace_lightbulb, replace_lightbulb_s4). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
action(replace_lightbulb_s4, unscrew). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
object_of(replace_lightbulb_s4, old_bulb). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
before(replace_lightbulb_s3, replace_lightbulb_s4). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
step(replace_lightbulb, replace_lightbulb_s5). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
before(replace_lightbulb_s4, replace_lightbulb_s5). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
step(replace_lightbulb, replace_lightbulb_s6). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
action(replace_lightbulb_s6, test). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
object_of(replace_lightbulb_s6, light). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
before(replace_lightbulb_s5, replace_lightbulb_s6). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
action(replace_lightbulb_s1, switch_off). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
object_of(replace_lightbulb_s1, light). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
action(replace_lightbulb_s5, screw). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
object_of(replace_lightbulb_s5, new_lightbulb). @conf 1 @source teacher:curriculum @conv s1795002267478-760026-65
```

## Taught wording

- replace_lightbulb_s2: wait for the old bulb until it is cool
- replace_lightbulb_s3: climb the step ladder
- replace_lightbulb_s4: unscrew the old bulb
- replace_lightbulb_s6: test the light by switching it back on
- replace_lightbulb_s1: switch off the light at the wall
- replace_lightbulb_s5: screw in the new lightbulb
- task: replace a lightbulb
