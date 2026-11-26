# Reminders

A workflow (see `core/dialogue/workflows.md`) that asks for the task and the time, then saves the reminder.

## Setting a reminder

```karin
goal_need(goal="reminder", slot="when", prompt="When should I remind you?")

goal_need(goal="reminder", slot="task", prompt="What should I remind you to do?")

rule workflow-start-reminder
when: "add a reminder", !dialogue_goal()
then: +dialogue_goal(goal="reminder")

rule workflow-start-reminder-task
when: "remind me to $task", !dialogue_goal()
then:
  +dialogue_goal(goal="reminder")
  +dialogue_filled(goal="reminder", slot="task", value=$task)

rule workflow-start-reminder-full
when: "remind me to $task at $when", !dialogue_goal()
then:
  +dialogue_goal(goal="reminder")
  +dialogue_filled(goal="reminder", slot="task", value=$task)
  +dialogue_filled(goal="reminder", slot="when", value=$when)

rule workflow-finish-reminder
when: dialogue_goal(goal="reminder"), dialogue_filled(goal="reminder", slot="task", value=$task), dialogue_filled(goal="reminder", slot="when", value=$when)
then:
  -dialogue_goal(goal="reminder")
  -dialogue_filled(goal="reminder")
  ++reminder(task=$task, when=$when)
  => "Reminder set: \"$task\" ($when)."

rule recall-latest-reminder
when: "my reminders", reminder(task=$task, when=$when)
then: => "Latest reminder: \"$task\" ($when)."

rule recall-reminders-empty
when: "my reminders", !reminder()
then: => "You don't have any reminders yet. Say \"remind me to ...\" to add one."
```

## Listing reminders

```karin
rule opus2-list-reminders
when: "list all my reminders", !collected(tag="opus2_rem"), !collected_empty(tag="opus2_rem")
then: @collect(rel="reminder", field="task", tag="opus2_rem")

rule opus2-reply-reminders
when: collected(tag="opus2_rem", count=$n, list=$list)
then: => "You have $n reminder(s): $list."

rule opus2-reply-reminders-empty
when: collected_empty(tag="opus2_rem")
then: => "No reminders saved. Say \"remind me to ...\" to add one."
```
