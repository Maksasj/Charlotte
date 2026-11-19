---
id: trim_toenails
stage: procedures
task: trim_toenails
schema: v3
---
# Trim toenails

## requests
- How do I trim my toenails at home without hurting myself?
- What's the right way to cut toenails? I want to do it safely.
- Could you tell me the steps to clip and file my toenails?

## situation
- I have a basin, a towel, nail clippers, a nail file and some lotion.
- I have about 20 minutes free this evening.

## ground truth
```datalog
task(trim_toenails).
domain(trim_toenails, personal_care).
goal(trim_toenails, trimmed_nails).
needs(trim_toenails, basin).
needs(trim_toenails, towel).
needs(trim_toenails, nail_clippers).
needs(trim_toenails, nail_file).
needs(trim_toenails, lotion).
takes_time(trim_toenails, 20, 25, minute).
step(trim_toenails, trim_toenails_s1).
action(trim_toenails_s1, soak).
object_of(trim_toenails_s1, foot).
uses(trim_toenails_s1, basin).
role(trim_toenails_s1, basin, contain).
setting(trim_toenails_s1, water_temperature, warm, level).
takes_time(trim_toenails_s1, 10, 10, minute).
step(trim_toenails, trim_toenails_s2).
action(trim_toenails_s2, dry).
object_of(trim_toenails_s2, foot).
uses(trim_toenails_s2, towel).
role(trim_toenails_s2, towel, dry).
step(trim_toenails, trim_toenails_s3).
action(trim_toenails_s3, clip).
object_of(trim_toenails_s3, toenail).
uses(trim_toenails_s3, nail_clippers).
role(trim_toenails_s3, nail_clippers, cut).
purpose(trim_toenails_s3, avoid_ingrown_nail).
step(trim_toenails, trim_toenails_s4).
action(trim_toenails_s4, file).
object_of(trim_toenails_s4, toenail).
uses(trim_toenails_s4, nail_file).
role(trim_toenails_s4, nail_file, smooth).
step(trim_toenails, trim_toenails_s5).
action(trim_toenails_s5, rub).
object_of(trim_toenails_s5, foot).
uses(trim_toenails_s5, lotion).
role(trim_toenails_s5, lotion, coat).
optional(trim_toenails_s5).
before(trim_toenails_s1, trim_toenails_s2).
before(trim_toenails_s2, trim_toenails_s3).
before(trim_toenails_s3, trim_toenails_s4).
before(trim_toenails_s4, trim_toenails_s5).
```

## aliases
```datalog
task_alias(trim_toenails, "trimming toenails").
task_alias(trim_toenails, "cutting toenails").
task_alias(trim_toenails, "clipping toenails").
alias(foot, "feet").
alias(toenail, "toenails").
alias(basin, "bowl").
alias(nail_clippers, "nail clippers").
alias(nail_file, "nail file").
alias(lotion, "moisturiser").
alias(avoid_ingrown_nail, "stops ingrown nails").
```

## teach
- To trim toenails you need a basin, a towel, nail clippers, a nail file and lotion.
- Trimming toenails takes 20 to 25 minutes.
- Step 1 of trimming toenails: soak the feet in warm water in a basin for 10 minutes.
- Step 2 of trimming toenails: dry the feet with a towel.
- Step 3 of trimming toenails: clip the toenails straight across with nail clippers so they do not grow into the skin and cause ingrown nails.
- Step 4 of trimming toenails: file the edges of the toenails smooth with a nail file.
- Step 5 of trimming toenails: rub lotion into the feet, which is optional.

## checklist
- names what is needed, including a basin, a towel, nail clippers, a nail file and lotion
- gives the steps in order: soak feet, dry, clip toenails straight across, file edges, rub in lotion
- says the lotion step is optional
- gives a total time of about 20 to 25 minutes

## variant
- change: no nail clippers: use scissors to cut the toenails instead
- request: I don't have nail clippers. Can I use scissors to trim my toenails?
- situation: I have a basin, a towel, scissors, a nail file and lotion, but no nail clippers.
- expect: The reply swaps the nail clippers for scissors in the clipping step and keeps the other steps the same.

```datalog situation
has(user, basin).
has(user, towel).
has(user, scissors).
has(user, nail_file).
has(user, lotion).
lacks(user, nail_clippers).
```

```datalog difference
swap(trim_toenails_s3, nail_clippers, scissors).
no_need(nail_clippers).
extra_need(scissors).
```

## variant
- change: only 15 minutes: skip the lotion step
- request: I'm short on time and only have 15 minutes. Can I leave out the lotion?
- situation: I have a basin, a towel, nail clippers and a nail file, no lotion, and about 15 minutes.
- expect: The reply gives the soak, dry, clip and file steps in order without the lotion, in about 15 to 20 minutes.

```datalog situation
has(user, basin).
has(user, towel).
has(user, nail_clippers).
has(user, nail_file).
lacks(user, lotion).
time_limit(user, 15, minute).
```

```datalog difference
drop(trim_toenails_s5).
no_need(lotion).
change(trim_toenails, takes_time, 20_25, 15_20).
```
