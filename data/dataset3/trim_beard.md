---
id: trim_beard
stage: procedures
task: trim_beard
schema: v3
---
# Trim a beard

## requests
- I want to trim my beard neatly at home. What should I do?
- What's the right order for trimming a beard with a trimmer and scissors?
- Can you talk me through a full beard trim from start to finish?

## situation
- I have a beard trimmer, scissors, a comb, a towel and beard oil.
- I have about 15 minutes before I go out.

## ground truth
```datalog
task(trim_beard).
domain(trim_beard, personal_care).
goal(trim_beard, trimmed_beard).
needs(trim_beard, comb).
needs(trim_beard, scissors).
needs(trim_beard, trimmer).
needs(trim_beard, towel).
needs(trim_beard, beard_oil).
takes_time(trim_beard, 10, 15, minute).
step(trim_beard, trim_beard_s1).
action(trim_beard_s1, comb).
object_of(trim_beard_s1, beard).
uses(trim_beard_s1, comb).
role(trim_beard_s1, comb, spread).
step(trim_beard, trim_beard_s2).
action(trim_beard_s2, cut).
object_of(trim_beard_s2, beard).
uses(trim_beard_s2, scissors).
role(trim_beard_s2, scissors, cut).
check(trim_beard_s2, even_length).
step(trim_beard, trim_beard_s3).
action(trim_beard_s3, trim).
object_of(trim_beard_s3, beard).
uses(trim_beard_s3, trimmer).
role(trim_beard_s3, trimmer, cut).
takes_time(trim_beard_s3, 5, 8, minute).
warning(trim_beard_s3, nick_skin).
step(trim_beard, trim_beard_s4).
action(trim_beard_s4, wipe).
object_of(trim_beard_s4, face).
uses(trim_beard_s4, towel).
role(trim_beard_s4, towel, wipe).
step(trim_beard, trim_beard_s5).
action(trim_beard_s5, rub).
object_of(trim_beard_s5, beard).
uses(trim_beard_s5, beard_oil).
role(trim_beard_s5, beard_oil, coat).
purpose(trim_beard_s5, soften_hair).
before(trim_beard_s1, trim_beard_s2).
before(trim_beard_s2, trim_beard_s3).
before(trim_beard_s3, trim_beard_s4).
before(trim_beard_s4, trim_beard_s5).
```

## aliases
```datalog
task_alias(trim_beard, "trim a beard").
task_alias(trim_beard, "beard trim").
task_alias(trim_beard, "trimming my beard").
alias(trimmer, "beard trimmer").
alias(beard_oil, "beard oil").
alias(towel, "hand towel").
alias(even_length, "even length").
alias(nick_skin, "nicks the skin").
alias(soften_hair, "softens the hair").
```

## teach
- To trim a beard you need a comb, scissors, a beard trimmer, a towel and beard oil.
- Trimming a beard takes 10 to 15 minutes.
- Step 1 of trimming a beard: comb the beard flat.
- Step 2 of trimming a beard: cut the longer hairs with the scissors until the beard is an even length.
- Step 3 of trimming a beard: trim the sides with the trimmer for 5 to 8 minutes, going slowly so you do not nick the skin.
- Step 4 of trimming a beard: wipe the face with the towel to clear the loose hair.
- Step 5 of trimming a beard: rub beard oil into the beard to soften the hair.

## checklist
- names what is needed: a comb, scissors, a beard trimmer, a towel and beard oil
- gives the steps in order: comb, cut the longer hairs, trim the sides, wipe the face, oil the beard
- says to trim slowly to avoid nicking the skin
- gives a total time of about 10 to 15 minutes

## variant
- change: no beard oil, so use olive oil instead for the last step
- request: I'm out of beard oil but I have olive oil. How should I finish the trim?
- situation: I have a comb, scissors, a beard trimmer, a towel and olive oil, and about 15 minutes.
- expect: The reply gives the same five steps and uses olive oil in place of beard oil in the final step.

```datalog situation
lacks(user, beard_oil).
has(user, olive_oil).
has(user, comb).
has(user, scissors).
has(user, trimmer).
has(user, towel).
```

```datalog difference
swap(trim_beard_s5, beard_oil, olive_oil).
no_need(beard_oil).
extra_need(olive_oil).
```

## variant
- change: no beard trimmer, so use scissors for the sides as well
- request: I can't find my trimmer. Can I trim the sides with scissors instead, and how long will it take?
- situation: I have a comb, scissors, a towel and beard oil, but no trimmer, and about 20 minutes.
- expect: The reply uses scissors for the sides in place of the trimmer and says the trim takes a little longer, about 15 to 20 minutes.

```datalog situation
lacks(user, trimmer).
has(user, scissors).
has(user, comb).
has(user, towel).
has(user, beard_oil).
time_limit(user, 20, minute).
```

```datalog difference
swap(trim_beard_s3, trimmer, scissors).
no_need(trimmer).
change(trim_beard, takes_time, 10_15, 15_20).
```
