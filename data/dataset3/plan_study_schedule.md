---
id: plan_study_schedule
stage: procedures
task: plan_study_schedule
schema: v3
---
# Plan a study schedule

## requests
- How do I make a study schedule that covers everything before my exam?
- Can you help me plan my revision timetable for the next few weeks?
- What's a good way to split up exam prep across the days I have left?

## situation
- I have my course syllabus, a paper planner and about three weeks until the exam.
- I want to start this evening.

## ground truth
```datalog
task(plan_study_schedule).
domain(plan_study_schedule, planning).
goal(plan_study_schedule, study_plan).
needs(plan_study_schedule, syllabus).
needs(plan_study_schedule, planner).
takes_time(plan_study_schedule, 40, 60, minute).
step(plan_study_schedule, plan_study_schedule_s1).
action(plan_study_schedule_s1, list).
object_of(plan_study_schedule_s1, topic).
uses(plan_study_schedule_s1, syllabus).
role(plan_study_schedule_s1, syllabus, hold).
takes_time(plan_study_schedule_s1, 10, 15, minute).
step(plan_study_schedule, plan_study_schedule_s2).
action(plan_study_schedule_s2, estimate).
object_of(plan_study_schedule_s2, topic).
takes_time(plan_study_schedule_s2, 10, 20, minute).
step(plan_study_schedule, plan_study_schedule_s3).
action(plan_study_schedule_s3, write).
object_of(plan_study_schedule_s3, study_session).
uses(plan_study_schedule_s3, planner).
role(plan_study_schedule_s3, planner, contain).
warning(plan_study_schedule_s3, overbooked_day).
takes_time(plan_study_schedule_s3, 10, 15, minute).
step(plan_study_schedule, plan_study_schedule_s4).
action(plan_study_schedule_s4, spread).
object_of(plan_study_schedule_s4, review_session).
uses(plan_study_schedule_s4, planner).
role(plan_study_schedule_s4, planner, contain).
purpose(plan_study_schedule_s4, memory_lasts).
takes_time(plan_study_schedule_s4, 5, 10, minute).
step(plan_study_schedule, plan_study_schedule_s5).
action(plan_study_schedule_s5, check).
object_of(plan_study_schedule_s5, plan).
check(plan_study_schedule_s5, fits_deadline).
takes_time(plan_study_schedule_s5, 5, 10, minute).
before(plan_study_schedule_s1, plan_study_schedule_s2).
before(plan_study_schedule_s2, plan_study_schedule_s3).
before(plan_study_schedule_s3, plan_study_schedule_s4).
before(plan_study_schedule_s4, plan_study_schedule_s5).
```

## aliases
```datalog
task_alias(plan_study_schedule, "study timetable").
task_alias(plan_study_schedule, "revision plan").
alias(syllabus, "course outline").
alias(planner, "paper planner").
alias(topic, "exam topic").
alias(study_session, "study block").
alias(review_session, "revision block").
alias(overbooked_day, "overbooked day").
alias(memory_lasts, "still remembered").
alias(fits_deadline, "fits before the exam").
alias(calendar, "phone calendar").
```

## teach
- To plan a study schedule you need a syllabus and a paper planner.
- Planning a study schedule takes 40 to 60 minutes.
- Step 1 of planning a study schedule: list every exam topic from the syllabus, for 10 to 15 minutes.
- Step 2 of planning a study schedule: estimate the hours each topic needs, for 10 to 20 minutes.
- Step 3 of planning a study schedule: write study blocks into the planner, for 10 to 15 minutes, and avoid overbooking any single day.
- Step 4 of planning a study schedule: spread revision blocks across the weeks, for 5 to 10 minutes, so the material is still remembered on exam day.
- Step 5 of planning a study schedule: check that the plan fits before the exam, for 5 to 10 minutes.

## checklist
- names what is needed, including a syllabus and a planner
- gives the steps in order: list topics, estimate hours, write study blocks, spread revision, check the deadline
- warns against overbooking a single day
- gives a total time of about 40 to 60 minutes

## variant
- change: no paper planner, so the study and revision blocks go into the phone calendar instead
- request: I don't have a paper planner, only my phone's calendar. How should I plan the study schedule?
- situation: I have the syllabus, a phone calendar and about three weeks until the exam.
- expect: The reply uses the phone calendar in place of the paper planner for the study and revision blocks, keeping the same five steps.

```datalog situation
has(user, syllabus).
has(user, calendar).
lacks(user, planner).
```

```datalog difference
no_need(planner).
extra_need(calendar).
swap(plan_study_schedule_s3, planner, calendar).
swap(plan_study_schedule_s4, planner, calendar).
```

## variant
- change: only 20 minutes: skip spreading the revision blocks and give a quick version of the plan
- request: I only have 20 minutes this evening to plan. Can you give me a quick version?
- situation: I have the syllabus and a paper planner, and 20 minutes now.
- expect: The reply gives the topic list, hour estimates, study blocks and deadline check in about 15 minutes, and leaves out the revision spreading step.

```datalog situation
has(user, syllabus).
has(user, planner).
time_limit(user, 20, minute).
```

```datalog difference
drop(plan_study_schedule_s4).
change(plan_study_schedule, takes_time, 40_60, 15_20).
```
