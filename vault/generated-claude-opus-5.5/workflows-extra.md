# Extra Multi-Step Workflows (`claude-opus-5.5`)

Decision helper (composed with knowledge-graph `prop` facts) and meeting scheduler built on the shared `goal_need` workflow engine.

```karin
goal_need(goal="decide", slot="criterion", prompt="What matters most (e.g. safety, purpose, price)?")
goal_need(goal="decide", slot="option_b", prompt="What's the second option?")
goal_need(goal="decide", slot="option_a", prompt="What's the first option?")

rule wf-decide-start
when: "help me decide", !dialogue_goal()
then: +dialogue_goal(goal="decide")

rule wf-decide-start-pair
when: "help me choose between $a and $b", !dialogue_goal(), $a != $b
then:
  +dialogue_goal(goal="decide")
  +dialogue_filled(goal="decide", slot="option_a", value=$a)
  +dialogue_filled(goal="decide", slot="option_b", value=$b)

rule wf-decide-finish-compare
when: dialogue_goal(goal="decide"), dialogue_filled(goal="decide", slot="option_a", value=$a), dialogue_filled(goal="decide", slot="option_b", value=$b), dialogue_filled(goal="decide", slot="criterion", value=$c), prop(entity=$a, attr=$c, value=$va), prop(entity=$b, attr=$c, value=$vb)
then:
  -dialogue_goal(goal="decide")
  -dialogue_filled(goal="decide")
  => "On $c: $a → $va; $b → $vb. Pick the one that fits better."

rule wf-decide-finish-unknown-a
when: dialogue_goal(goal="decide"), dialogue_filled(goal="decide", slot="option_a", value=$a), dialogue_filled(goal="decide", slot="option_b", value=$b), dialogue_filled(goal="decide", slot="criterion", value=$c), !prop(entity=$a, attr=$c)
then:
  -dialogue_goal(goal="decide")
  -dialogue_filled(goal="decide")
  => "I don't know the $c of $a yet. Teach me with \"the $c of $a is ...\" and ask again."

rule wf-decide-finish-unknown-b
when: dialogue_goal(goal="decide"), dialogue_filled(goal="decide", slot="option_a", value=$a), dialogue_filled(goal="decide", slot="option_b", value=$b), dialogue_filled(goal="decide", slot="criterion", value=$c), prop(entity=$a, attr=$c), !prop(entity=$b, attr=$c)
then:
  -dialogue_goal(goal="decide")
  -dialogue_filled(goal="decide")
  => "I don't know the $c of $b yet. Teach me with \"the $c of $b is ...\" and ask again."

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
