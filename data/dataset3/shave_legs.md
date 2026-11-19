---
id: shave_legs
stage: procedures
task: shave_legs
schema: v3
---
# Shave your legs

## requests
- How do I shave my legs without getting nicks?
- What's the proper way to shave legs in the shower?
- Can you give me the steps for a smooth leg shave?

## situation
- I have shaving cream, a razor, a towel and moisturiser.
- I'm getting ready for bed and have a bit of time.

## ground truth
```datalog
task(shave_legs).
domain(shave_legs, personal_care).
goal(shave_legs, smooth_legs).
needs(shave_legs, shaving_cream).
needs(shave_legs, razor).
needs(shave_legs, towel).
needs(shave_legs, moisturiser).
takes_time(shave_legs, 10, 15, minute).
step(shave_legs, shave_legs_s1).
action(shave_legs_s1, apply).
object_of(shave_legs_s1, legs).
uses(shave_legs_s1, shaving_cream).
role(shave_legs_s1, shaving_cream, coat).
purpose(shave_legs_s1, razor_glides).
step(shave_legs, shave_legs_s2).
action(shave_legs_s2, shave).
object_of(shave_legs_s2, legs).
uses(shave_legs_s2, razor).
role(shave_legs_s2, razor, cut).
warning(shave_legs_s2, razor_nicks).
check(shave_legs_s2, skin_smooth).
step(shave_legs, shave_legs_s3).
action(shave_legs_s3, rinse).
object_of(shave_legs_s3, legs).
step(shave_legs, shave_legs_s4).
action(shave_legs_s4, dry).
object_of(shave_legs_s4, legs).
uses(shave_legs_s4, towel).
role(shave_legs_s4, towel, dry).
step(shave_legs, shave_legs_s5).
action(shave_legs_s5, moisturise).
object_of(shave_legs_s5, legs).
uses(shave_legs_s5, moisturiser).
role(shave_legs_s5, moisturiser, coat).
before(shave_legs_s1, shave_legs_s2).
before(shave_legs_s2, shave_legs_s3).
before(shave_legs_s3, shave_legs_s4).
before(shave_legs_s4, shave_legs_s5).
```

## aliases
```datalog
task_alias(shave_legs, "shaving legs").
task_alias(shave_legs, "shaving your legs").
alias(shaving_cream, "shaving foam").
alias(moisturiser, "body lotion").
alias(razor_glides, "razor glides").
alias(razor_nicks, "nicks the skin").
alias(skin_smooth, "smooth skin").
alias(shave, "shaving").
```

## teach
- To shave your legs you need shaving cream, a razor, a towel and moisturiser.
- Shaving your legs takes 10 to 15 minutes.
- Step 1 of shaving your legs: apply shaving cream to the legs so the razor glides.
- Step 2 of shaving your legs: shave the legs with the razor, carefully so the razor does not nick the skin, until the skin feels smooth.
- Step 3 of shaving your legs: rinse the legs.
- Step 4 of shaving your legs: dry the legs with the towel.
- Step 5 of shaving your legs: moisturise the legs with the moisturiser.

## checklist
- names what is needed: shaving cream, a razor, a towel and moisturiser
- gives the steps in order: apply cream, shave, rinse, dry, moisturise
- warns to shave carefully so the razor does not nick the skin
- gives a total time of about 10 to 15 minutes

## variant
- change: no shaving cream, so use bar soap instead
- request: I've run out of shaving cream. Can I use bar soap to shave my legs?
- situation: I have bar soap, a razor, a towel and moisturiser, but no shaving cream.
- expect: The reply uses bar soap in place of shaving cream and keeps the other steps in the same order.

```datalog situation
lacks(user, shaving_cream).
has(user, soap).
has(user, razor).
has(user, towel).
has(user, moisturiser).
```

```datalog difference
swap(shave_legs_s1, shaving_cream, soap).
extra_need(soap).
no_need(shaving_cream).
```

- change: only 8 minutes, so skip the moisturiser
- request: I'm in a hurry and only have about 8 minutes. What can I leave out when shaving my legs?
- situation: I have shaving cream, a razor, a towel and moisturiser, and about 8 minutes.
- expect: The reply cream, shaves, rinses and dries the legs, leaves out the moisturiser, and says it takes about 6 to 8 minutes.

```datalog situation
has(user, shaving_cream).
has(user, razor).
has(user, towel).
has(user, moisturiser).
time_limit(user, 8, minute).
```

```datalog difference
drop(shave_legs_s5).
no_need(moisturiser).
change(shave_legs, takes_time, 10_15, 6_8).
```
