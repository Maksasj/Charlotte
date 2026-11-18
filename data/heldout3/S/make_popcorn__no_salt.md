---
id: make_popcorn__no_salt
stage: procedures
task: make_popcorn
schema: v3
tier: S
base: make_popcorn
---
# Make popcorn

## requests
- Movie's about to start and I want stovetop popcorn, but we're out of salt and there's nothing else to season it with.

## ground truth
```datalog
task(make_popcorn).
domain(make_popcorn, cooking).
goal(make_popcorn, popcorn).
needs(make_popcorn, popcorn_kernel).
needs(make_popcorn, oil).
needs(make_popcorn, salt).
needs(make_popcorn, pot).
needs(make_popcorn, bowl).
takes_time(make_popcorn, 6, 8, minute).
step(make_popcorn, make_popcorn_s1).
action(make_popcorn_s1, heat).
object_of(make_popcorn_s1, oil).
uses(make_popcorn_s1, pot).
role(make_popcorn_s1, pot, heat).
setting(make_popcorn_s1, heat, medium_high, level).
takes_time(make_popcorn_s1, 2, 3, minute).
step(make_popcorn, make_popcorn_s2).
action(make_popcorn_s2, add).
object_of(make_popcorn_s2, popcorn_kernel).
uses(make_popcorn_s2, pot).
role(make_popcorn_s2, pot, contain).
into(make_popcorn_s2, pot).
achieves(make_popcorn_s2, one_layer).
step(make_popcorn, make_popcorn_s3).
action(make_popcorn_s3, put).
object_of(make_popcorn_s3, lid).
onto(make_popcorn_s3, pot).
purpose(make_popcorn_s3, kernels_stay_in).
step(make_popcorn, make_popcorn_s4).
action(make_popcorn_s4, shake).
object_of(make_popcorn_s4, pot).
until(make_popcorn_s4, slow).
takes_time(make_popcorn_s4, 3, 4, minute).
step(make_popcorn, make_popcorn_s5).
action(make_popcorn_s5, pour).
object_of(make_popcorn_s5, popcorn).
uses(make_popcorn_s5, bowl).
role(make_popcorn_s5, bowl, contain).
into(make_popcorn_s5, bowl).
step(make_popcorn, make_popcorn_s6).
action(make_popcorn_s6, sprinkle).
object_of(make_popcorn_s6, popcorn).
uses(make_popcorn_s6, salt).
role(make_popcorn_s6, salt, season).
before(make_popcorn_s1, make_popcorn_s2).
before(make_popcorn_s2, make_popcorn_s3).
before(make_popcorn_s3, make_popcorn_s4).
before(make_popcorn_s4, make_popcorn_s5).
before(make_popcorn_s5, make_popcorn_s6).
```

## aliases
```datalog
task_alias(make_popcorn, "making popcorn").
task_alias(make_popcorn, "pop corn kernels").
task_alias(make_popcorn, "popcorn on the stove").
alias(popcorn_kernel, "popcorn kernels").
alias(popcorn_kernel, "kernels").
alias(pot, "large pot").
alias(medium_high, "medium-high heat").
alias(slow, "the popping slows").
alias(one_layer, "in one layer").
alias(kernels_stay_in, "the kernels cannot jump out").
```

## teach
- To make popcorn you need popcorn kernels, oil, salt, a large pot with a lid and a bowl.
- Making popcorn takes 6 to 8 minutes.
- Step 1 of making popcorn: heat the oil in the pot over medium-high heat for 2 to 3 minutes.
- Step 2 of making popcorn: add the popcorn kernels in one layer.
- Step 3 of making popcorn: put the lid on the pot so the kernels cannot jump out.
- Step 4 of making popcorn: shake the pot gently until the popping slows to a few seconds apart, for 3 to 4 minutes.
- Step 5 of making popcorn: pour the popcorn into the bowl.
- Step 6 of making popcorn: sprinkle the popcorn with salt.

## variant
- change: no salt: skip the seasoning
- request: Movie's about to start and I want stovetop popcorn, but we're out of salt and there's nothing else to season it with.
- situation: I have popcorn kernels, oil, a large pot with a lid and a bowl, but no salt.
- expect: The reply pops the corn as usual and leaves out the salting step.

```datalog situation
has(user, popcorn_kernel).
has(user, oil).
has(user, pot).
has(user, bowl).
lacks(user, salt).
```

```datalog difference
drop(make_popcorn_s6).
no_need(salt).
```
