---
id: apply_sunscreen
stage: procedures
task: apply_sunscreen
schema: v3
---
# Apply sunscreen

## requests
- How do I put sunscreen on properly before going out in the sun?
- What's the right way to apply sun cream to my face?
- Can you list the steps for applying sunscreen before a day outdoors?

## situation
- I have sunscreen, a towel and water, and a bathroom sink.
- I'm heading out in about half an hour.

## ground truth
```datalog
task(apply_sunscreen).
domain(apply_sunscreen, personal_care).
goal(apply_sunscreen, sun_protection).
needs(apply_sunscreen, sunscreen).
needs(apply_sunscreen, towel).
needs(apply_sunscreen, water).
takes_time(apply_sunscreen, 20, 25, minute).
step(apply_sunscreen, apply_sunscreen_s1).
action(apply_sunscreen_s1, wash).
object_of(apply_sunscreen_s1, face).
uses(apply_sunscreen_s1, water).
role(apply_sunscreen_s1, water, clean).
purpose(apply_sunscreen_s1, clean_skin).
step(apply_sunscreen, apply_sunscreen_s2).
action(apply_sunscreen_s2, dry).
object_of(apply_sunscreen_s2, face).
uses(apply_sunscreen_s2, towel).
role(apply_sunscreen_s2, towel, dry).
step(apply_sunscreen, apply_sunscreen_s3).
action(apply_sunscreen_s3, spread).
object_of(apply_sunscreen_s3, sunscreen).
onto(apply_sunscreen_s3, face).
step(apply_sunscreen, apply_sunscreen_s4).
action(apply_sunscreen_s4, wait).
object_of(apply_sunscreen_s4, face).
takes_time(apply_sunscreen_s4, 15, 15, minute).
check(apply_sunscreen_s4, absorbed).
before(apply_sunscreen_s1, apply_sunscreen_s2).
before(apply_sunscreen_s2, apply_sunscreen_s3).
before(apply_sunscreen_s3, apply_sunscreen_s4).
```

## aliases
```datalog
task_alias(apply_sunscreen, "apply sunscreen").
task_alias(apply_sunscreen, "put on sunscreen").
task_alias(apply_sunscreen, "apply sun cream").
alias(sunscreen, "sun cream").
alias(sunscreen, "sunblock").
alias(towel, "hand towel").
alias(clean_skin, "clean skin").
alias(absorbed, "sunk in").
```

## teach
- To apply sunscreen you need sunscreen, a towel and water.
- Applying sunscreen takes 20 to 25 minutes.
- Step 1 of applying sunscreen: wash your face with water to clean the skin.
- Step 2 of applying sunscreen: dry the face with the towel.
- Step 3 of applying sunscreen: spread the sunscreen onto the face.
- Step 4 of applying sunscreen: wait 15 minutes until the sunscreen has sunk in.

## checklist
- names what is needed, including sunscreen, a towel and water
- gives the steps in order: wash face, dry face, spread sunscreen, wait until it has sunk in
- gives a total time of about 20 to 25 minutes
- explains that washing is there to clean the skin before the sunscreen goes on

## variant
- change: no towel: dry the face with a paper towel instead
- request: I don't have a towel. Can I use something else to dry my face before putting sunscreen on?
- situation: I have sunscreen, water and a pack of paper towels, but no towel.
- expect: The reply says to dry the face with a paper towel instead of a towel, then carry on with the sunscreen steps as normal.

```datalog situation
has(user, sunscreen).
has(user, water).
has(user, paper_towel).
lacks(user, towel).
```

```datalog difference
swap(apply_sunscreen_s2, towel, paper_towel).
no_need(towel).
extra_need(paper_towel).
```

## variant
- change: only 10 minutes: skip the 15-minute wait
- request: I'm running late and have only 10 minutes. Can I still put sunscreen on quickly?
- situation: I have sunscreen, water and a towel, and only 10 minutes before I leave.
- expect: The reply drops the wait so the sunscreen goes on in about 5 to 10 minutes, and warns that it still needs a little time to sink in before heavy sweating.

```datalog situation
has(user, sunscreen).
has(user, water).
has(user, towel).
time_limit(user, 10, minute).
```

```datalog difference
drop(apply_sunscreen_s4).
change(apply_sunscreen, takes_time, 20_25, 5_10).
```
