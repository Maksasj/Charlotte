---
id: shave_face_with_razor
stage: procedures
task: shave_face_with_razor
schema: v3
---
# Shave your face with a razor

## requests
- How do I shave my face with a safety razor without nicking myself?
- What's the proper way to give myself a close shave with a blade?
- Can you walk me through shaving my face with a razor, step by step?

## situation
- I have a razor, shaving cream, water and a towel.
- I'm getting ready for work and have about 15 minutes.

## ground truth
```datalog
task(shave_face_with_razor).
domain(shave_face_with_razor, personal_care).
goal(shave_face_with_razor, close_shave).
needs(shave_face_with_razor, razor).
needs(shave_face_with_razor, shaving_cream).
needs(shave_face_with_razor, water).
needs(shave_face_with_razor, towel).
takes_time(shave_face_with_razor, 10, 15, minute).
step(shave_face_with_razor, shave_face_with_razor_s1).
action(shave_face_with_razor_s1, wet).
object_of(shave_face_with_razor_s1, face).
uses(shave_face_with_razor_s1, water).
role(shave_face_with_razor_s1, water, wet).
step(shave_face_with_razor, shave_face_with_razor_s2).
action(shave_face_with_razor_s2, apply).
object_of(shave_face_with_razor_s2, face).
uses(shave_face_with_razor_s2, shaving_cream).
role(shave_face_with_razor_s2, shaving_cream, coat).
step(shave_face_with_razor, shave_face_with_razor_s3).
action(shave_face_with_razor_s3, shave).
object_of(shave_face_with_razor_s3, face).
uses(shave_face_with_razor_s3, razor).
role(shave_face_with_razor_s3, razor, cut).
takes_time(shave_face_with_razor_s3, 5, 8, minute).
warning(shave_face_with_razor_s3, skin_nicks).
purpose(shave_face_with_razor_s3, smooth_skin).
step(shave_face_with_razor, shave_face_with_razor_s4).
action(shave_face_with_razor_s4, rinse).
object_of(shave_face_with_razor_s4, face).
uses(shave_face_with_razor_s4, water).
role(shave_face_with_razor_s4, water, clean).
step(shave_face_with_razor, shave_face_with_razor_s5).
action(shave_face_with_razor_s5, dry).
object_of(shave_face_with_razor_s5, face).
uses(shave_face_with_razor_s5, towel).
role(shave_face_with_razor_s5, towel, dry).
before(shave_face_with_razor_s1, shave_face_with_razor_s2).
before(shave_face_with_razor_s2, shave_face_with_razor_s3).
before(shave_face_with_razor_s3, shave_face_with_razor_s4).
before(shave_face_with_razor_s4, shave_face_with_razor_s5).
```

## aliases
```datalog
task_alias(shave_face_with_razor, "shave with a razor").
task_alias(shave_face_with_razor, "close shave").
task_alias(shave_face_with_razor, "shaving my face").
alias(razor, "safety razor").
alias(razor, "blade").
alias(shaving_cream, "shaving foam").
alias(water, "warm water").
alias(towel, "hand towel").
alias(skin_nicks, "nicks").
alias(smooth_skin, "smooth skin").
```

## teach
- To shave your face with a razor you need a razor, shaving cream, water and a towel.
- Shaving your face with a razor takes 10 to 15 minutes.
- Step 1 of shaving your face with a razor: wet your face with warm water.
- Step 2 of shaving your face with a razor: apply shaving cream to your face to coat it.
- Step 3 of shaving your face with a razor: shave your face with the razor for 5 to 8 minutes, going with the grain so you don't get nicks and the skin ends up smooth.
- Step 4 of shaving your face with a razor: rinse your face with water to clean off the cream.
- Step 5 of shaving your face with a razor: dry your face with a towel.

## checklist
- names what is needed, including a razor, shaving cream, water and a towel
- gives the steps in order: wet face, apply shaving cream, shave with the grain, rinse, dry
- warns to shave with the grain to avoid nicks
- gives a total time of about 10 to 15 minutes

## variant
- change: no shaving cream, so use soap to lather the face instead
- request: I don't have shaving cream. Can I shave my face with a razor using soap?
- situation: I have a razor, soap, water and a towel, but no shaving cream.
- expect: The reply uses soap in place of shaving cream to lather the face, and keeps the other steps the same: wet, lather, shave with the grain, rinse, dry.

```datalog situation
lacks(user, shaving_cream).
has(user, soap).
has(user, razor).
has(user, water).
has(user, towel).
```

```datalog difference
swap(shave_face_with_razor_s2, shaving_cream, soap).
extra_need(soap).
no_need(shaving_cream).
```

## variant
- change: finish with aftershave after drying, to cool the skin
- request: I want to finish my shave with aftershave to cool my skin. Where does that go?
- situation: I have a razor, shaving cream, water, a towel and aftershave.
- expect: The reply keeps the five base steps and adds a final step after drying where you apply aftershave to the face to cool it.

```datalog situation
has(user, razor).
has(user, shaving_cream).
has(user, water).
has(user, towel).
has(user, aftershave).
```

```datalog difference
add_step(shave_face_with_razor_s9, shave_face_with_razor_s5).
step(shave_face_with_razor, shave_face_with_razor_s9).
action(shave_face_with_razor_s9, apply).
object_of(shave_face_with_razor_s9, face).
uses(shave_face_with_razor_s9, aftershave).
role(shave_face_with_razor_s9, aftershave, cool).
before(shave_face_with_razor_s5, shave_face_with_razor_s9).
extra_need(aftershave).
```
