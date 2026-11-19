---
id: style_curly_hair
stage: procedures
task: style_curly_hair
schema: v3
---
# Style curly hair

## requests
- What's the best way to style curly hair so the curls stay defined?
- Can you take me through a routine for defining my curls after washing?
- Steps for styling curly hair without frizz, please?

## situation
- I have damp curly hair after a shower, a leave-in conditioner, a wide-tooth comb, a microfibre towel, a hair dryer and a diffuser.
- I'd like defined curls with no frizz.

## ground truth
```datalog
task(style_curly_hair).
domain(style_curly_hair, personal_care).
goal(style_curly_hair, defined_curls).
needs(style_curly_hair, wide_tooth_comb).
needs(style_curly_hair, leave_in_conditioner).
needs(style_curly_hair, microfibre_towel).
needs(style_curly_hair, hair_dryer).
needs(style_curly_hair, hair_diffuser).
takes_time(style_curly_hair, 20, 30, minute).
step(style_curly_hair, style_curly_hair_s1).
action(style_curly_hair_s1, detangle).
object_of(style_curly_hair_s1, hair).
uses(style_curly_hair_s1, wide_tooth_comb).
role(style_curly_hair_s1, wide_tooth_comb, loosen).
step(style_curly_hair, style_curly_hair_s2).
action(style_curly_hair_s2, apply).
object_of(style_curly_hair_s2, hair).
uses(style_curly_hair_s2, leave_in_conditioner).
role(style_curly_hair_s2, leave_in_conditioner, coat).
purpose(style_curly_hair_s2, curl_definition).
step(style_curly_hair, style_curly_hair_s3).
action(style_curly_hair_s3, squeeze).
object_of(style_curly_hair_s3, hair).
uses(style_curly_hair_s3, microfibre_towel).
role(style_curly_hair_s3, microfibre_towel, dry).
purpose(style_curly_hair_s3, less_frizz).
step(style_curly_hair, style_curly_hair_s4).
action(style_curly_hair_s4, dry).
object_of(style_curly_hair_s4, hair).
uses(style_curly_hair_s4, hair_dryer).
role(style_curly_hair_s4, hair_dryer, heat).
uses(style_curly_hair_s4, hair_diffuser).
role(style_curly_hair_s4, hair_diffuser, spread).
setting(style_curly_hair_s4, heat, low, level).
until(style_curly_hair_s4, dry).
before(style_curly_hair_s1, style_curly_hair_s2).
before(style_curly_hair_s2, style_curly_hair_s3).
before(style_curly_hair_s3, style_curly_hair_s4).
```

## aliases
```datalog
task_alias(style_curly_hair, "curly hair routine").
task_alias(style_curly_hair, "defining curls").
alias(wide_tooth_comb, "wide-tooth comb").
alias(leave_in_conditioner, "leave-in conditioner").
alias(microfibre_towel, "microfiber towel").
alias(hair_dryer, "hair dryer").
alias(hair_diffuser, "diffuser").
alias(hair_oil, "hair oil").
alias(air_dry, "air dry").
alias(squeeze, "scrunch").
alias(curl_definition, "defined curls").
alias(less_frizz, "no frizz").
```

## teach
- To style curly hair you need a wide-tooth comb, a leave-in conditioner, a microfibre towel, a hair dryer and a diffuser.
- Styling curly hair takes 20 to 30 minutes.
- Step 1 of styling curly hair: detangle the hair with a wide-tooth comb.
- Step 2 of styling curly hair: apply leave-in conditioner to the hair for curl definition.
- Step 3 of styling curly hair: squeeze the hair in a microfibre towel to remove water, without rubbing, to reduce frizz.
- Step 4 of styling curly hair: dry the hair with the hair dryer on low heat and the diffuser until it is dry.

## checklist
- names what is needed: a wide-tooth comb, leave-in conditioner, microfibre towel, hair dryer and diffuser
- gives the steps in order: detangle, apply leave-in conditioner, squeeze in the towel, dry with the diffuser
- says to squeeze rather than rub the hair to reduce frizz
- gives a total time of about 20 to 30 minutes

## variant
- change: no hair dryer: air dry the hair instead, which takes about an hour
- request: I don't have a hair dryer. How can I still get defined curls?
- situation: I have a wide-tooth comb, leave-in conditioner and a microfibre towel, but no dryer or diffuser.
- expect: The reply keeps the comb, conditioner and towel steps, drops the dryer and diffuser step, and has the hair air dry for about an hour.

```datalog situation
has(user, wide_tooth_comb).
has(user, leave_in_conditioner).
has(user, microfibre_towel).
lacks(user, hair_dryer).
lacks(user, hair_diffuser).
```

```datalog difference
drop(style_curly_hair_s4).
no_need(hair_dryer).
no_need(hair_diffuser).
add_step(style_curly_hair_s9, style_curly_hair_s3).
step(style_curly_hair, style_curly_hair_s9).
action(style_curly_hair_s9, air_dry).
object_of(style_curly_hair_s9, hair).
takes_time(style_curly_hair_s9, 60, 90, minute).
before(style_curly_hair_s3, style_curly_hair_s9).
change(style_curly_hair, takes_time, 20_30, 60_90).
```

## variant
- change: no leave-in conditioner: use hair oil instead
- request: I'm out of leave-in conditioner, but I have hair oil. Can I still do my curls?
- situation: I have a wide-tooth comb, hair oil, a microfibre towel, a hair dryer and a diffuser, but no leave-in conditioner.
- expect: The reply uses hair oil in place of leave-in conditioner at step 2 and keeps the rest of the routine as it is.

```datalog situation
has(user, wide_tooth_comb).
has(user, hair_oil).
has(user, microfibre_towel).
has(user, hair_dryer).
has(user, hair_diffuser).
lacks(user, leave_in_conditioner).
```

```datalog difference
swap(style_curly_hair_s2, leave_in_conditioner, hair_oil).
no_need(leave_in_conditioner).
extra_need(hair_oil).
```
