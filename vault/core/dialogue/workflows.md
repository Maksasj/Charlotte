# Generic Multi-Step Workflows

Any multi-step goal declares its prerequisites as `goal_need(goal, slot, prompt)` facts.
Three reusable rules auto-fill known user properties, prompt for missing slots one at a time via `dialogue/frames.md`, and store answers until the goal completes.

## Reusable Workflow Engine

```karin
rule workflow-autofill-user-prop
when: dialogue_goal(goal=$goal), goal_need(goal=$goal, slot=$slot), !dialogue_filled(goal=$goal, slot=$slot), prop(entity="user", attr=$slot, value=$val)
then: +dialogue_filled(goal=$goal, slot=$slot, value=$val)

rule workflow-store-slot
when: dialogue_goal(goal=$goal), slot_value(goal=$goal, slot=$slot, value=$val), !dialogue_filled(goal=$goal, slot=$slot)
then:
  -slot_value(goal=$goal, slot=$slot)
  +dialogue_filled(goal=$goal, slot=$slot, value=$val)

rule workflow-request-slot
when: dialogue_goal(goal=$goal), goal_need(goal=$goal, slot=$slot, prompt=$prompt), !dialogue_filled(goal=$goal, slot=$slot), !need_slot(goal=$goal), !slot_value(goal=$goal)
then: +need_slot(goal=$goal, slot=$slot, prompt=$prompt)
```

