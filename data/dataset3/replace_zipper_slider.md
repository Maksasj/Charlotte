---
id: replace_zipper_slider
stage: procedures
task: replace_zipper_slider
schema: v3
---
# Replace a zipper slider

## requests
- My zip pull fell off my jacket. How do I put a new slider on?
- Can you show me how to replace the slider on a zipper?
- What's the way to swap out a broken zip slider on a coat?

## situation
- I have a new slider, a pair of pliers and a soft cloth.
- I want to fix my jacket zip this afternoon.

## ground truth
```datalog
task(replace_zipper_slider).
domain(replace_zipper_slider, repair).
goal(replace_zipper_slider, fixed_zipper).
needs(replace_zipper_slider, new_slider).
needs(replace_zipper_slider, pliers).
needs(replace_zipper_slider, cloth).
takes_time(replace_zipper_slider, 20, 30, minute).
step(replace_zipper_slider, replace_zipper_slider_s1).
action(replace_zipper_slider_s1, remove).
object_of(replace_zipper_slider_s1, old_slider).
uses(replace_zipper_slider_s1, pliers).
role(replace_zipper_slider_s1, pliers, loosen).
warning(replace_zipper_slider_s1, tape_tears).
takes_time(replace_zipper_slider_s1, 5, 10, minute).
step(replace_zipper_slider, replace_zipper_slider_s2).
action(replace_zipper_slider_s2, wipe).
object_of(replace_zipper_slider_s2, zipper_teeth).
uses(replace_zipper_slider_s2, cloth).
role(replace_zipper_slider_s2, cloth, wipe).
optional(replace_zipper_slider_s2).
takes_time(replace_zipper_slider_s2, 2, 3, minute).
step(replace_zipper_slider, replace_zipper_slider_s3).
action(replace_zipper_slider_s3, slide).
object_of(replace_zipper_slider_s3, new_slider).
onto(replace_zipper_slider_s3, zipper_teeth).
takes_time(replace_zipper_slider_s3, 3, 5, minute).
step(replace_zipper_slider, replace_zipper_slider_s4).
action(replace_zipper_slider_s4, crimp).
object_of(replace_zipper_slider_s4, new_stop).
uses(replace_zipper_slider_s4, pliers).
role(replace_zipper_slider_s4, pliers, clamp).
purpose(replace_zipper_slider_s4, slider_stays_on).
takes_time(replace_zipper_slider_s4, 3, 5, minute).
step(replace_zipper_slider, replace_zipper_slider_s5).
action(replace_zipper_slider_s5, test).
object_of(replace_zipper_slider_s5, new_slider).
check(replace_zipper_slider_s5, runs_smoothly).
takes_time(replace_zipper_slider_s5, 2, 5, minute).
before(replace_zipper_slider_s1, replace_zipper_slider_s2).
before(replace_zipper_slider_s2, replace_zipper_slider_s3).
before(replace_zipper_slider_s3, replace_zipper_slider_s4).
before(replace_zipper_slider_s4, replace_zipper_slider_s5).
```

## aliases
```datalog
task_alias(replace_zipper_slider, "fix a zip").
task_alias(replace_zipper_slider, "replace zip slider").
task_alias(replace_zipper_slider, "change zipper pull").
alias(new_slider, "new zip slider").
alias(old_slider, "broken slider").
alias(zipper_teeth, "zip teeth").
alias(new_stop, "bottom stop").
alias(pliers, "pliers").
alias(slider_stays_on, "slider stays on").
alias(tape_tears, "tape tears").
alias(runs_smoothly, "runs smoothly").
```

## teach
- To replace a zipper slider you need a new slider, pliers and a cloth.
- Replacing a zipper slider takes 20 to 30 minutes.
- Step 1 of replacing a zipper slider: pull the old slider off with the pliers, gently so the tape does not tear, which takes 5 to 10 minutes.
- Step 2 of replacing a zipper slider: wipe the zipper teeth with the cloth for 2 to 3 minutes; you can skip this if the teeth are already clean.
- Step 3 of replacing a zipper slider: slide the new slider onto the zipper teeth, which takes 3 to 5 minutes.
- Step 4 of replacing a zipper slider: squeeze the new bottom stop shut with the pliers for 3 to 5 minutes so the slider stays on.
- Step 5 of replacing a zipper slider: test the slider for 2 to 5 minutes and check that it runs smoothly up and down.

## checklist
- names what is needed: a new slider, pliers and a cloth
- gives the steps in order: pull the old slider off, wipe the teeth, slide on the new slider, squeeze the stop shut, test
- says the wiping step can be skipped if the teeth are clean
- warns not to pull so hard that the tape tears
- gives a total time of about 20 to 30 minutes

## variant
- change: no pliers: use a hammer instead to pull the old slider off and squeeze the new stop shut
- request: I don't own pliers, but I have a hammer. Can I still swap the slider?
- situation: I have a new slider, a cloth and a hammer, but no pliers.
- expect: The reply uses the hammer in place of pliers for steps 1 and 4, keeps the same order and warns to tap gently so the tape does not tear.

```datalog situation
has(user, new_slider).
has(user, cloth).
has(user, hammer).
lacks(user, pliers).
```

```datalog difference
swap(replace_zipper_slider_s1, pliers, hammer).
swap(replace_zipper_slider_s4, pliers, hammer).
no_need(pliers).
extra_need(hammer).
```

- change: only 10 minutes: skip the wiping step because the teeth look clean
- request: The teeth look clean already and I only have ten minutes. What's the quick way to swap the slider?
- situation: I have a new slider and pliers, and I have 10 minutes.
- expect: The reply leaves out the wiping step, keeps the other four steps in order and gives a total time of about 10 to 15 minutes.

```datalog situation
has(user, new_slider).
has(user, pliers).
time_limit(user, 10, minute).
```

```datalog difference
drop(replace_zipper_slider_s2).
no_need(cloth).
change(replace_zipper_slider, takes_time, 20_30, 10_15).
```
