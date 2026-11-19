---
id: prepare_job_interview
stage: procedures
task: prepare_job_interview
schema: v3
---
# Prepare for a job interview

## requests
- How should I get ready for a job interview the night before?
- What's the best way to prepare for an interview next week?
- Can you give me a plan for preparing for a job interview, including my answers and what to wear?

## situation
- I have a laptop, a notebook, a pen, an iron and a shirt.
- I have about 90 minutes free this evening.

## ground truth
```datalog
task(prepare_job_interview).
domain(prepare_job_interview, planning).
goal(prepare_job_interview, ready_for_interview).
needs(prepare_job_interview, laptop).
needs(prepare_job_interview, notebook).
needs(prepare_job_interview, pen).
needs(prepare_job_interview, iron).
needs(prepare_job_interview, shirt).
takes_time(prepare_job_interview, 90, 120, minute).
step(prepare_job_interview, prepare_job_interview_s1).
action(prepare_job_interview_s1, search).
object_of(prepare_job_interview_s1, company).
uses(prepare_job_interview_s1, laptop).
role(prepare_job_interview_s1, laptop, search).
takes_time(prepare_job_interview_s1, 30, 45, minute).
step(prepare_job_interview, prepare_job_interview_s2).
action(prepare_job_interview_s2, write).
object_of(prepare_job_interview_s2, answers).
uses(prepare_job_interview_s2, notebook).
role(prepare_job_interview_s2, notebook, hold).
uses(prepare_job_interview_s2, pen).
role(prepare_job_interview_s2, pen, mark).
takes_time(prepare_job_interview_s2, 30, 30, minute).
step(prepare_job_interview, prepare_job_interview_s3).
action(prepare_job_interview_s3, iron).
object_of(prepare_job_interview_s3, shirt).
uses(prepare_job_interview_s3, iron).
role(prepare_job_interview_s3, iron, heat).
setting(prepare_job_interview_s3, heat, medium, level).
takes_time(prepare_job_interview_s3, 15, 15, minute).
overlaps(prepare_job_interview_s3, prepare_job_interview_s1).
step(prepare_job_interview, prepare_job_interview_s4).
action(prepare_job_interview_s4, practise).
object_of(prepare_job_interview_s4, answers).
purpose(prepare_job_interview_s4, sound_natural).
takes_time(prepare_job_interview_s4, 20, 20, minute).
step(prepare_job_interview, prepare_job_interview_s5).
action(prepare_job_interview_s5, pack).
object_of(prepare_job_interview_s5, shirt).
before(prepare_job_interview_s1, prepare_job_interview_s2).
before(prepare_job_interview_s2, prepare_job_interview_s4).
before(prepare_job_interview_s4, prepare_job_interview_s5).
```

## aliases
```datalog
task_alias(prepare_job_interview, "interview prep").
task_alias(prepare_job_interview, "getting ready for an interview").
alias(laptop, "computer").
alias(shirt, "work shirt").
alias(iron, "clothes iron").
alias(search, "look up").
alias(practise, "rehearse").
alias(pack, "get ready").
alias(sound_natural, "sound natural").
```

## teach
- To prepare for a job interview you need a laptop, a notebook, a pen, an iron and a shirt.
- Preparing for a job interview takes 90 to 120 minutes.
- Step 1 of preparing for a job interview: research the company online with the laptop for 30 to 45 minutes.
- Step 2 of preparing for a job interview: write answers to likely questions in the notebook with a pen for 30 minutes.
- While step 1 runs, step 3 of preparing for a job interview: iron the shirt on medium heat for 15 minutes.
- Step 4 of preparing for a job interview: practise the answers aloud for 20 minutes so they sound natural.
- Step 5 of preparing for a job interview: pack the shirt and documents for the morning.

## checklist
- names what is needed, including a laptop, a notebook, a pen, an iron and a shirt
- gives the steps in order: research the company, write answers, practise them aloud, pack the shirt
- says the ironing can be done while the research runs
- gives a total time of about 90 to 120 minutes

## variant
- change: no iron: skip the ironing and fit the preparation into about 75 minutes
- request: I don't have an iron. Can I still get ready in about 75 minutes?
- situation: I have a laptop, a notebook, a pen and a shirt, but no iron, and about 75 minutes.
- expect: The reply skips the ironing and gives the research, answers, practice and packing steps in order, fitting them into about 70 to 90 minutes.

```datalog situation
has(user, laptop).
has(user, notebook).
has(user, pen).
has(user, shirt).
lacks(user, iron).
time_limit(user, 75, minute).
```

```datalog difference
drop(prepare_job_interview_s3).
no_need(iron).
change(prepare_job_interview, takes_time, 90_120, 70_90).
```

## variant
- change: add a mock interview with a friend asking the questions, after practising alone
- request: I'd like to rehearse with a friend too. Can you add that to the plan?
- situation: I have a laptop, a notebook, a pen, an iron and a shirt, a friend who can help, and two hours.
- expect: The reply adds a step where a friend asks you the interview questions after you practise alone, before packing, and allows about two hours in total.

```datalog situation
has(user, laptop).
has(user, notebook).
has(user, pen).
has(user, iron).
has(user, shirt).
wants(user, mock_interview).
time_limit(user, 120, minute).
```

```datalog difference
add_step(prepare_job_interview_s9, prepare_job_interview_s4).
step(prepare_job_interview, prepare_job_interview_s9).
action(prepare_job_interview_s9, ask).
object_of(prepare_job_interview_s9, friend).
takes_time(prepare_job_interview_s9, 20, 30, minute).
before(prepare_job_interview_s4, prepare_job_interview_s9).
before(prepare_job_interview_s9, prepare_job_interview_s5).
extra_need(friend).
```
