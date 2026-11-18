---
id: plan_exam_revision
stage: social
task: plan_exam_revision
schema: v3
tier: T
---
# Plan exam revision

## requests
- My finals start in a month and I haven't started revising. How do I make a plan?
- What's a good way to schedule my studying before exams?
- Can you help me set up a revision timetable?

## situation
- I have a notebook, a wall calendar, past papers and a kitchen timer.

## ground truth
```datalog
task(plan_exam_revision).
domain(plan_exam_revision, planning).
goal(plan_exam_revision, revision_plan).
goal(plan_exam_revision, exam_ready).
needs(plan_exam_revision, notebook).
needs(plan_exam_revision, calendar).
needs(plan_exam_revision, past_paper).
needs(plan_exam_revision, timer).
takes_time(plan_exam_revision, 3, 4, week).
step(plan_exam_revision, plan_exam_revision_s1).
action(plan_exam_revision_s1, list).
object_of(plan_exam_revision_s1, topic).
uses(plan_exam_revision_s1, notebook).
role(plan_exam_revision_s1, notebook, record).
step(plan_exam_revision, plan_exam_revision_s2).
action(plan_exam_revision_s2, split).
object_of(plan_exam_revision_s2, topic).
uses(plan_exam_revision_s2, calendar).
role(plan_exam_revision_s2, calendar, record).
purpose(plan_exam_revision_s2, spread_out).
step(plan_exam_revision, plan_exam_revision_s3).
action(plan_exam_revision_s3, schedule).
object_of(plan_exam_revision_s3, break).
purpose(plan_exam_revision_s3, stay_focused).
step(plan_exam_revision, plan_exam_revision_s4).
action(plan_exam_revision_s4, study).
object_of(plan_exam_revision_s4, topic).
uses(plan_exam_revision_s4, timer).
role(plan_exam_revision_s4, timer, measure).
takes_time(plan_exam_revision_s4, 25, 25, minute).
repeat(plan_exam_revision_s4, 4).
step(plan_exam_revision, plan_exam_revision_s5).
action(plan_exam_revision_s5, review).
object_of(plan_exam_revision_s5, topic).
condition(plan_exam_revision_s5, end_of_week).
step(plan_exam_revision, plan_exam_revision_s6).
action(plan_exam_revision_s6, practise).
object_of(plan_exam_revision_s6, past_paper).
purpose(plan_exam_revision_s6, exam_ready).
before(plan_exam_revision_s1, plan_exam_revision_s2).
before(plan_exam_revision_s2, plan_exam_revision_s3).
before(plan_exam_revision_s3, plan_exam_revision_s4).
before(plan_exam_revision_s4, plan_exam_revision_s5).
before(plan_exam_revision_s5, plan_exam_revision_s6).
```

## aliases
```datalog
task_alias(plan_exam_revision, "revision timetable").
task_alias(plan_exam_revision, "revising for exams").
task_alias(plan_exam_revision, "a study schedule").
alias(topic, "topics").
alias(past_paper, "past papers").
alias(timer, "kitchen timer").
alias(split, "spread").
alias(stay_focused, "keep focused").
```

## teach
- To plan exam revision you need a notebook, a calendar, past papers and a timer.
- Planning exam revision covers 3 to 4 weeks.
- Step 1 of planning exam revision: list every topic in the notebook.
- Step 2 of planning exam revision: split the topics across the days on the calendar to spread them out.
- Step 3 of planning exam revision: schedule breaks to stay focused.
- Step 4 of planning exam revision: study each topic in 25-minute blocks with the timer, 4 blocks a session.
- Step 5 of planning exam revision: review the topics at the end of each week.
- Step 6 of planning exam revision: practise past papers so you are exam ready.

## checklist
- names what is needed: a notebook, a calendar, past papers and a timer
- gives the steps in order: list topics, spread over days, breaks, timed study, weekly review, past papers
- mentions 25-minute study blocks

## variant
- change: only 3 days: focus on the weakest topics and past papers
- request: My exam is in three days and I've barely revised. What's the best plan now?
- situation: I have a notebook, a calendar, past papers and a timer, and 3 days.
- expect: The reply skips the weekly review, picks the weakest topics to study first and does past papers each day, all within 3 days.

```datalog situation
has(user, notebook).
has(user, calendar).
has(user, past_paper).
has(user, timer).
time_limit(user, 3, day).
```

```datalog difference
drop(plan_exam_revision_s5).
add_step(plan_exam_revision_s9, plan_exam_revision_s1).
step(plan_exam_revision, plan_exam_revision_s9).
action(plan_exam_revision_s9, rank).
object_of(plan_exam_revision_s9, topic).
purpose(plan_exam_revision_s9, weakest_first).
before(plan_exam_revision_s1, plan_exam_revision_s9).
before(plan_exam_revision_s9, plan_exam_revision_s2).
change(plan_exam_revision, takes_time, 3_4_week, 3_3_day).
```

## variant
- change: no notebook or calendar: plan it all on a whiteboard
- request: I don't have a planner or calendar, but there's a whiteboard on my wall. Can I plan revision on that?
- situation: I have a whiteboard and pens, past papers and a timer.
- expect: The reply lists the topics and spreads them across the days on the whiteboard instead of a notebook and calendar.

```datalog situation
has(user, whiteboard).
has(user, past_paper).
has(user, timer).
lacks(user, notebook).
lacks(user, calendar).
```

```datalog difference
swap(plan_exam_revision_s1, notebook, whiteboard).
swap(plan_exam_revision_s2, calendar, whiteboard).
extra_need(whiteboard).
no_need(notebook).
no_need(calendar).
```
