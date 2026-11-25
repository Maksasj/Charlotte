# Dialogue Frames

Reusable multi-turn slot-filling protocol shared by all skills.
Any skill emits `+need_slot(goal=..., slot=..., prompt=...)`, and on the next turn this module captures the user's answer into `+slot_value(goal=..., slot=..., value=...)` or cancels cleanly on `"cancel"`.

```karin
rule dialogue-ask-slot
when: need_slot(goal=$goal, slot=$slot, prompt=$prompt), !dialogue_slot(goal=$goal, slot=$slot)
then:
  +dialogue_slot(goal=$goal, slot=$slot)
  => "$prompt"

rule dialogue-cancel-slot
when: dialogue_slot(goal=$goal, slot=$slot), "cancel"
then:
  -dialogue_slot(goal=$goal, slot=$slot)
  -dialogue_goal(goal=$goal)
  -dialogue_filled(goal=$goal)
  => "Okay, cancelled."

rule dialogue-capture-slot
when: dialogue_slot(goal=$goal, slot=$slot), "$value", $value != "cancel", !slot_value(goal=$goal, slot=$slot)
then:
  -dialogue_slot(goal=$goal, slot=$slot)
  +slot_value(goal=$goal, slot=$slot, value=$value)
```
