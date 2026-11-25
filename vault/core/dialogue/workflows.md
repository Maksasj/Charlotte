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

## Example Workflow: Trip Planner

```karin
goal_need(goal="plan_trip", slot="days", prompt="How many days will you stay?")
goal_need(goal="plan_trip", slot="city", prompt="Which city are you visiting?")

rule workflow-start-plan-trip
when: "plan a trip", !dialogue_goal()
then: +dialogue_goal(goal="plan_trip")

rule workflow-start-plan-trip-city
when: "plan a trip to $city", !dialogue_goal()
then:
  +dialogue_goal(goal="plan_trip")
  +dialogue_filled(goal="plan_trip", slot="city", value=$city)

rule workflow-finish-plan-trip-weather
when: dialogue_goal(goal="plan_trip"), dialogue_filled(goal="plan_trip", slot="city", value=$city), dialogue_filled(goal="plan_trip", slot="days", value=$days), weather_city(city=$city, condition=$cond, temp=$temp)
then:
  -dialogue_goal(goal="plan_trip")
  -dialogue_filled(goal="plan_trip")
  => "Trip plan ready: $days days in $city (expect $cond, $temp)."

rule workflow-finish-plan-trip-plain
when: dialogue_goal(goal="plan_trip"), dialogue_filled(goal="plan_trip", slot="city", value=$city), dialogue_filled(goal="plan_trip", slot="days", value=$days), !weather_city(city=$city)
then:
  -dialogue_goal(goal="plan_trip")
  -dialogue_filled(goal="plan_trip")
  => "Trip plan ready: $days days in $city."
```
