---
id: tweeze_eyebrows
stage: procedures
task: tweeze_eyebrows
schema: v3
---
# Tweeze your eyebrows

## requests
- How do I tweeze my eyebrows without it hurting too much?
- What's the right way to shape my brows with tweezers at home?
- Can you tell me the steps for plucking my eyebrows?

## situation
- I have tweezers, a mirror and alcohol wipes at home.
- I have about 15 minutes before I go out.

## ground truth
```datalog
task(tweeze_eyebrows).
domain(tweeze_eyebrows, personal_care).
goal(tweeze_eyebrows, shaped_brows).
needs(tweeze_eyebrows, tweezers).
needs(tweeze_eyebrows, alcohol_wipe).
needs(tweeze_eyebrows, mirror).
needs(tweeze_eyebrows, ice_pack).
takes_time(tweeze_eyebrows, 10, 15, minute).
step(tweeze_eyebrows, tweeze_eyebrows_s1).
action(tweeze_eyebrows_s1, clean).
object_of(tweeze_eyebrows_s1, tweezers).
uses(tweeze_eyebrows_s1, alcohol_wipe).
role(tweeze_eyebrows_s1, alcohol_wipe, wipe).
purpose(tweeze_eyebrows_s1, germs_removed).
takes_time(tweeze_eyebrows_s1, 1, 2, minute).
step(tweeze_eyebrows, tweeze_eyebrows_s2).
action(tweeze_eyebrows_s2, stretch).
object_of(tweeze_eyebrows_s2, skin).
purpose(tweeze_eyebrows_s2, easier_to_grip).
overlaps(tweeze_eyebrows_s2, tweeze_eyebrows_s1).
takes_time(tweeze_eyebrows_s2, 1, 1, minute).
step(tweeze_eyebrows, tweeze_eyebrows_s3).
action(tweeze_eyebrows_s3, pluck).
object_of(tweeze_eyebrows_s3, hair).
uses(tweeze_eyebrows_s3, tweezers).
role(tweeze_eyebrows_s3, tweezers, lift).
until(tweeze_eyebrows_s3, brow_shaped).
takes_time(tweeze_eyebrows_s3, 5, 10, minute).
step(tweeze_eyebrows, tweeze_eyebrows_s4).
action(tweeze_eyebrows_s4, cool).
object_of(tweeze_eyebrows_s4, skin).
uses(tweeze_eyebrows_s4, ice_pack).
role(tweeze_eyebrows_s4, ice_pack, cool).
purpose(tweeze_eyebrows_s4, calms_redness).
takes_time(tweeze_eyebrows_s4, 2, 3, minute).
before(tweeze_eyebrows_s1, tweeze_eyebrows_s3).
before(tweeze_eyebrows_s2, tweeze_eyebrows_s3).
before(tweeze_eyebrows_s3, tweeze_eyebrows_s4).
```

## aliases
```datalog
task_alias(tweeze_eyebrows, "tweezing eyebrows").
task_alias(tweeze_eyebrows, "plucking eyebrows").
alias(tweezers, "tweezer").
alias(alcohol_wipe, "alcohol pad").
alias(ice_pack, "ice cube").
alias(hair, "brow hair").
alias(brow_shaped, "shape you want").
alias(germs_removed, "germs removed").
alias(calms_redness, "calms redness").
```

## teach
- To tweeze your eyebrows you need tweezers, an alcohol wipe, a mirror and an ice pack.
- Tweezing your eyebrows takes 10 to 15 minutes.
- Step 1 of tweezing your eyebrows: clean the tweezers with an alcohol wipe for 1 to 2 minutes, so germs are removed.
- While step 1 runs, step 2 of tweezing your eyebrows: stretch the skin under the brow for 1 minute so the hairs are easier to grip.
- Step 3 of tweezing your eyebrows: pluck the hair with the tweezers for 5 to 10 minutes, until the brow is the shape you want.
- Step 4 of tweezing your eyebrows: cool the skin with the ice pack for 2 to 3 minutes to calm redness.

## checklist
- names what is needed: tweezers, an alcohol wipe, a mirror and an ice pack
- gives the steps in order: clean the tweezers, stretch the skin, pluck the hair, cool the skin
- says stretching the skin can be done while the tweezers are being cleaned
- gives a total time of about 10 to 15 minutes

## variant
- change: no alcohol wipes: wash the tweezers with soap and water instead
- request: I don't have alcohol wipes. Can I still tweeze my eyebrows?
- situation: I have tweezers, a mirror, soap, an ice pack and a sink, but no alcohol wipes.
- expect: The reply uses soap and water to clean the tweezers in step 1 and gives the other steps unchanged.

```datalog situation
has(user, tweezers).
has(user, mirror).
has(user, soap).
has(user, ice_pack).
lacks(user, alcohol_wipe).
```

```datalog difference
no_need(alcohol_wipe).
extra_need(soap).
swap(tweeze_eyebrows_s1, alcohol_wipe, soap).
```

- change: only 5 minutes and no ice pack: keep the first three steps and skip the cooling
- request: I've got five minutes before I leave. How quickly can I tweeze my brows?
- situation: I have tweezers, alcohol wipes and a mirror, and five minutes, with no ice pack.
- expect: The reply gives the clean, stretch and pluck steps, leaves out the ice pack, and says it takes about 5 to 8 minutes.

```datalog situation
has(user, tweezers).
has(user, alcohol_wipe).
has(user, mirror).
lacks(user, ice_pack).
time_limit(user, 5, minute).
```

```datalog difference
drop(tweeze_eyebrows_s4).
no_need(ice_pack).
change(tweeze_eyebrows, takes_time, 10_15, 5_8).
change(tweeze_eyebrows_s3, takes_time, 5_10, 3_5).
```
