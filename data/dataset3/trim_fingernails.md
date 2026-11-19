---
id: trim_fingernails
stage: procedures
task: trim_fingernails
schema: v3
---
# Trim fingernails

## requests
- What's the best way to trim my fingernails at home?
- Can you tell me the steps for clipping my nails neatly?
- I want to cut my nails without hurting my fingers. What should I do, in order?

## situation
- I have nail clippers, a nail file, a towel and a bowl.
- I have about fifteen minutes free.

## ground truth
```datalog
task(trim_fingernails).
domain(trim_fingernails, personal_care).
goal(trim_fingernails, neat_nails).
needs(trim_fingernails, bowl).
needs(trim_fingernails, nail_clippers).
needs(trim_fingernails, nail_file).
needs(trim_fingernails, towel).
takes_time(trim_fingernails, 10, 15, minute).
step(trim_fingernails, trim_fingernails_s1).
action(trim_fingernails_s1, soak).
object_of(trim_fingernails_s1, hands).
uses(trim_fingernails_s1, bowl).
role(trim_fingernails_s1, bowl, contain).
takes_time(trim_fingernails_s1, 5, 5, minute).
purpose(trim_fingernails_s1, nails_soften).
step(trim_fingernails, trim_fingernails_s2).
action(trim_fingernails_s2, cut).
object_of(trim_fingernails_s2, fingernails).
uses(trim_fingernails_s2, nail_clippers).
role(trim_fingernails_s2, nail_clippers, cut).
step(trim_fingernails, trim_fingernails_s3).
action(trim_fingernails_s3, file).
object_of(trim_fingernails_s3, fingernails).
uses(trim_fingernails_s3, nail_file).
role(trim_fingernails_s3, nail_file, smooth).
until(trim_fingernails_s3, smooth).
step(trim_fingernails, trim_fingernails_s4).
action(trim_fingernails_s4, wipe).
object_of(trim_fingernails_s4, hands).
uses(trim_fingernails_s4, towel).
role(trim_fingernails_s4, towel, wipe).
before(trim_fingernails_s1, trim_fingernails_s2).
before(trim_fingernails_s2, trim_fingernails_s3).
before(trim_fingernails_s3, trim_fingernails_s4).
```

## aliases
```datalog
task_alias(trim_fingernails, "trim nails").
task_alias(trim_fingernails, "cut nails").
task_alias(trim_fingernails, "clip nails").
alias(nail_clippers, "nail clippers").
alias(nail_file, "emery board").
alias(hands, "fingers").
alias(fingernails, "nails").
alias(nails_soften, "nails soften").
alias(smooth, "smooth").
```

## teach
- To trim fingernails you need a bowl, nail clippers, a nail file and a towel.
- Trimming fingernails takes 10 to 15 minutes.
- Step 1 of trimming fingernails: soak your hands in a bowl of warm water for 5 minutes so the nails soften.
- Step 2 of trimming fingernails: cut the fingernails with the nail clippers.
- Step 3 of trimming fingernails: file the fingernails with the nail file until they are smooth.
- Step 4 of trimming fingernails: wipe your hands with the towel.

## checklist
- names what is needed, including a bowl, nail clippers, a nail file and a towel
- gives the steps in order: soak hands, cut nails, file until smooth, wipe hands
- says the soaking is for 5 minutes so the nails soften
- gives a total time of about 10 to 15 minutes

## variant
- change: no nail clippers: cut the nails with scissors instead
- request: I don't have nail clippers. Can I trim my nails with scissors?
- situation: I have a bowl, scissors, a nail file and a towel, and about fifteen minutes.
- expect: The reply keeps the soaking, filing and wiping steps the same and says to cut the nails with scissors in step 2 instead of nail clippers.

```datalog situation
lacks(user, nail_clippers).
has(user, scissors).
has(user, bowl).
has(user, nail_file).
has(user, towel).
time_limit(user, 15, minute).
```

```datalog difference
swap(trim_fingernails_s2, nail_clippers, scissors).
no_need(nail_clippers).
extra_need(scissors).
```

- change: only 5 minutes: skip the soaking and go straight to cutting and filing
- request: I'm in a hurry and only have 5 minutes to trim my nails. What can I skip?
- situation: I have nail clippers, a nail file and a towel, and only 5 minutes.
- expect: The reply drops the soaking step, so the bowl is not needed, and says to cut, file and wipe in about 5 to 10 minutes.

```datalog situation
has(user, nail_clippers).
has(user, nail_file).
has(user, towel).
time_limit(user, 5, minute).
```

```datalog difference
drop(trim_fingernails_s1).
no_need(bowl).
change(trim_fingernails, takes_time, 10_15, 5_10).
```
