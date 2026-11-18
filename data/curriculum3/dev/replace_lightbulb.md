---
id: replace_lightbulb
stage: tools
task: replace_lightbulb
schema: v3
---
# Replace a lightbulb

## requests
- The ceiling light in my kitchen just went out. How do I change the bulb safely?
- Can you tell me the right way to swap a dead lightbulb for a new one?
- A bulb blew in my hallway. What steps should I follow to replace it?

## situation
- I have a new bulb of the same type and a step ladder.
- It should only take me a few minutes.

## ground truth
```datalog
task(replace_lightbulb).
domain(replace_lightbulb, repair).
goal(replace_lightbulb, working_light).
needs(replace_lightbulb, lightbulb).
needs(replace_lightbulb, step_ladder).
takes_time(replace_lightbulb, 5, 10, minute).
step(replace_lightbulb, replace_lightbulb_s1).
action(replace_lightbulb_s1, switch).
object_of(replace_lightbulb_s1, light).
achieves(replace_lightbulb_s1, off).
purpose(replace_lightbulb_s1, no_shock).
step(replace_lightbulb, replace_lightbulb_s2).
action(replace_lightbulb_s2, wait).
object_of(replace_lightbulb_s2, old_bulb).
until(replace_lightbulb_s2, cool).
purpose(replace_lightbulb_s2, no_burn).
takes_time(replace_lightbulb_s2, 3, 5, minute).
step(replace_lightbulb, replace_lightbulb_s3).
action(replace_lightbulb_s3, climb).
object_of(replace_lightbulb_s3, step_ladder).
step(replace_lightbulb, replace_lightbulb_s4).
action(replace_lightbulb_s4, unscrew).
object_of(replace_lightbulb_s4, old_bulb).
step(replace_lightbulb, replace_lightbulb_s5).
action(replace_lightbulb_s5, screw).
object_of(replace_lightbulb_s5, lightbulb).
step(replace_lightbulb, replace_lightbulb_s6).
action(replace_lightbulb_s6, test).
object_of(replace_lightbulb_s6, light).
before(replace_lightbulb_s1, replace_lightbulb_s2).
before(replace_lightbulb_s2, replace_lightbulb_s3).
before(replace_lightbulb_s3, replace_lightbulb_s4).
before(replace_lightbulb_s4, replace_lightbulb_s5).
before(replace_lightbulb_s5, replace_lightbulb_s6).
```

## aliases
```datalog
task_alias(replace_lightbulb, "replacing a lightbulb").
task_alias(replace_lightbulb, "change the bulb").
task_alias(replace_lightbulb, "swap a dead lightbulb").
alias(lightbulb, "new lightbulb").
alias(lightbulb, "new bulb").
alias(old_bulb, "old bulb").
alias(old_bulb, "dead lightbulb").
alias(light, "ceiling light").
alias(step_ladder, "ladder").
alias(switch, "switch off").
alias(screw, "screw in").
alias(no_shock, "do not get a shock").
alias(no_burn, "do not burn your fingers").
```

## teach
- To replace a lightbulb you need a new lightbulb and a step ladder.
- Replacing a lightbulb takes 5 to 10 minutes.
- Step 1 of replacing a lightbulb: switch off the light at the wall so you do not get a shock.
- Step 2 of replacing a lightbulb: wait for the old bulb until it is cool, for 3 to 5 minutes, so you do not burn your fingers.
- Step 3 of replacing a lightbulb: climb the step ladder.
- Step 4 of replacing a lightbulb: unscrew the old bulb.
- Step 5 of replacing a lightbulb: screw in the new lightbulb.
- Step 6 of replacing a lightbulb: test the light by switching it back on.

## checklist
- says to switch the light off and let the old bulb cool first
- gives the steps in order: switch off, wait, climb, unscrew, screw in, test
- gives a total time of about 5 to 10 minutes
- closes politely
