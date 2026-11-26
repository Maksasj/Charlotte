# Meetings

A workflow that asks who, which day and what time, then saves the meeting.

## Scheduling a meeting

```karin
goal_need(goal="meeting", slot="time", prompt="What time?")

goal_need(goal="meeting", slot="day", prompt="Which day?")

goal_need(goal="meeting", slot="who", prompt="Who is the meeting with?")

rule wf-meeting-start
when: "schedule a meeting", !dialogue_goal()
then: +dialogue_goal(goal="meeting")

rule wf-meeting-start-who
when: "schedule a meeting with $who", !dialogue_goal()
then:
  +dialogue_goal(goal="meeting")
  +dialogue_filled(goal="meeting", slot="who", value=$who)

rule wf-meeting-finish
when: dialogue_goal(goal="meeting"), dialogue_filled(goal="meeting", slot="who", value=$who), dialogue_filled(goal="meeting", slot="day", value=$day), dialogue_filled(goal="meeting", slot="time", value=$time)
then:
  -dialogue_goal(goal="meeting")
  -dialogue_filled(goal="meeting")
  ++meeting(who=$who, day=$day, time=$time)
  => "Meeting booked with $who on $day at $time."

rule wf-meeting-recall
when: "my meetings", meeting(who=$who, day=$day, time=$time)
then: => "Next meeting: $who on $day at $time."

rule wf-meeting-recall-empty
when: "my meetings", !meeting()
then: => "No meetings yet. Say \"schedule a meeting\" to book one."
```

## Listing meetings

```karin
rule opus2-list-meetings
when: "list all my meetings", !collected(tag="opus2_meet"), !collected_empty(tag="opus2_meet")
then: @collect(rel="meeting", field="who", tag="opus2_meet")

rule opus2-list-meetings-day
when: "meetings on $day", !collected(tag="opus2_meet_day"), !collected_empty(tag="opus2_meet_day")
then:
  +opus2_ctx(kind="meet_day", day=$day)
  @collect(rel="meeting", field="who", tag="opus2_meet_day", day=$day)

rule opus2-reply-meetings
when: collected(tag="opus2_meet", count=$n, list=$list)
then: => "You have $n meeting(s), with: $list."

rule opus2-reply-meetings-day
when: collected(tag="opus2_meet_day", list=$list), opus2_ctx(kind="meet_day", day=$day)
then: => "On $day you meet: $list."

rule opus2-reply-meetings-empty
when: collected_empty(tag="opus2_meet")
then: => "No meetings yet. Say \"schedule a meeting\" to book one."

rule opus2-reply-meetings-day-empty
when: collected_empty(tag="opus2_meet_day"), opus2_ctx(kind="meet_day", day=$day)
then: => "Nothing scheduled on $day."
```
