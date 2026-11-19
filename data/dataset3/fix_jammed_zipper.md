---
id: fix_jammed_zipper
stage: procedures
task: fix_jammed_zipper
schema: v3
---
# Fix a jammed zipper

## requests
- My jacket zip has got stuck halfway. How do I get it moving again?
- The slider on my bag won't go up or down. What's the best way to free it?
- Can you tell me how to unstick a zipper that keeps catching on the teeth?

## situation
- I have a jammed zipper, a cotton swab, a pencil and pliers.
- I've got about 10 minutes.

## ground truth
```datalog
task(fix_jammed_zipper).
domain(fix_jammed_zipper, repair).
goal(fix_jammed_zipper, unstick_zipper).
needs(fix_jammed_zipper, zipper).
needs(fix_jammed_zipper, cotton_swab).
needs(fix_jammed_zipper, pencil).
needs(fix_jammed_zipper, pliers).
takes_time(fix_jammed_zipper, 5, 10, minute).
step(fix_jammed_zipper, fix_jammed_zipper_s1).
action(fix_jammed_zipper_s1, wipe).
object_of(fix_jammed_zipper_s1, zipper).
uses(fix_jammed_zipper_s1, cotton_swab).
role(fix_jammed_zipper_s1, cotton_swab, wipe).
purpose(fix_jammed_zipper_s1, debris_removed).
takes_time(fix_jammed_zipper_s1, 2, 3, minute).
step(fix_jammed_zipper, fix_jammed_zipper_s2).
action(fix_jammed_zipper_s2, rub).
object_of(fix_jammed_zipper_s2, zipper).
uses(fix_jammed_zipper_s2, pencil).
role(fix_jammed_zipper_s2, pencil, coat).
purpose(fix_jammed_zipper_s2, slides_freely).
takes_time(fix_jammed_zipper_s2, 1, 2, minute).
step(fix_jammed_zipper, fix_jammed_zipper_s3).
action(fix_jammed_zipper_s3, grip).
object_of(fix_jammed_zipper_s3, slider).
uses(fix_jammed_zipper_s3, pliers).
role(fix_jammed_zipper_s3, pliers, hold).
warning(fix_jammed_zipper_s3, fabric_tears).
takes_time(fix_jammed_zipper_s3, 1, 1, minute).
step(fix_jammed_zipper, fix_jammed_zipper_s4).
action(fix_jammed_zipper_s4, move).
object_of(fix_jammed_zipper_s4, slider).
repeat(fix_jammed_zipper_s4, 5).
until(fix_jammed_zipper_s4, unstuck).
takes_time(fix_jammed_zipper_s4, 2, 4, minute).
before(fix_jammed_zipper_s1, fix_jammed_zipper_s2).
before(fix_jammed_zipper_s2, fix_jammed_zipper_s3).
before(fix_jammed_zipper_s3, fix_jammed_zipper_s4).
```

## aliases
```datalog
task_alias(fix_jammed_zipper, "stuck zip").
task_alias(fix_jammed_zipper, "jammed zipper").
task_alias(fix_jammed_zipper, "unstick a zipper").
alias(zipper, "zip").
alias(cotton_swab, "cotton bud").
alias(pencil, "graphite pencil").
alias(pliers, "pair of pliers").
alias(slider, "zip pull").
alias(debris_removed, "debris cleared").
alias(slides_freely, "slides smoothly").
alias(fabric_tears, "fabric rips").
alias(unstuck, "moves freely").
```

## teach
- To fix a jammed zipper you need a zipper, a cotton swab, a pencil and pliers.
- Fixing a jammed zipper takes 5 to 10 minutes.
- Step 1 of fixing a jammed zipper: wipe the teeth clean with the cotton swab, for 2 to 3 minutes, so debris is removed.
- Step 2 of fixing a jammed zipper: rub the teeth with the pencil so the slider slides freely, for 1 to 2 minutes.
- Step 3 of fixing a jammed zipper: grip the slider with the pliers for about 1 minute, taking care not to tear the fabric.
- Step 4 of fixing a jammed zipper: move the slider up and down 5 times, for 2 to 4 minutes, until it moves freely.

## checklist
- names what is needed, including a zipper, a cotton swab, a pencil and pliers
- gives the steps in order: wipe the teeth, rub with the pencil, grip the slider, move it back and forth
- warns not to tear the fabric when gripping the slider
- gives a total time of about 5 to 10 minutes

## variant
- change: no pencil: use a bar of soap on the teeth instead
- request: My zip is stuck and I have no pencil, only a bar of soap. Can I still fix it?
- situation: I have a jammed zipper, a cotton swab, a bar of soap and pliers.
- expect: The reply rubs the teeth with the bar of soap in step 2 instead of the pencil and keeps the other steps unchanged.

```datalog situation
lacks(user, pencil).
has(user, cotton_swab).
has(user, soap).
has(user, pliers).
```

```datalog difference
swap(fix_jammed_zipper_s2, pencil, soap).
extra_need(soap).
no_need(pencil).
```

- change: no pliers: grip the slider with tweezers instead
- request: The slider on my jacket is stuck and I only have tweezers, no pliers. How do I free it?
- situation: I have a jammed zipper, a cotton swab, a pencil and tweezers, and about 10 minutes.
- expect: The reply grips the slider with the tweezers in step 3 instead of pliers, keeps the same steps, and still warns not to tear the fabric.

```datalog situation
lacks(user, pliers).
has(user, tweezers).
has(user, cotton_swab).
has(user, pencil).
time_limit(user, 10, minute).
```

```datalog difference
swap(fix_jammed_zipper_s3, pliers, tweezers).
extra_need(tweezers).
no_need(pliers).
```
