---
id: wax_eyebrows
stage: procedures
task: wax_eyebrows
schema: v3
---
# Wax eyebrows

## requests
- How do I wax my eyebrows at home without ruining the shape?
- Could you walk me through doing my own eyebrow waxing with strips?
- What's the right order for waxing eyebrows? I've never done it before.

## situation
- I have wax strips, a cleansing wipe, talc powder and tweezers, and about 20 minutes free.

## ground truth
```datalog
task(wax_eyebrows).
domain(wax_eyebrows, personal_care).
goal(wax_eyebrows, smooth_brows).
needs(wax_eyebrows, wax_strip).
needs(wax_eyebrows, cleansing_wipe).
needs(wax_eyebrows, talc_powder).
needs(wax_eyebrows, tweezers).
takes_time(wax_eyebrows, 15, 20, minute).
step(wax_eyebrows, wax_eyebrows_s1).
action(wax_eyebrows_s1, clean).
object_of(wax_eyebrows_s1, eyebrow).
uses(wax_eyebrows_s1, cleansing_wipe).
role(wax_eyebrows_s1, cleansing_wipe, clean).
step(wax_eyebrows, wax_eyebrows_s2).
action(wax_eyebrows_s2, dust).
object_of(wax_eyebrows_s2, skin).
uses(wax_eyebrows_s2, talc_powder).
role(wax_eyebrows_s2, talc_powder, spread).
step(wax_eyebrows, wax_eyebrows_s3).
action(wax_eyebrows_s3, apply).
object_of(wax_eyebrows_s3, wax_strip).
uses(wax_eyebrows_s3, wax_strip).
role(wax_eyebrows_s3, wax_strip, cover).
step(wax_eyebrows, wax_eyebrows_s4).
action(wax_eyebrows_s4, pull).
object_of(wax_eyebrows_s4, wax_strip).
uses(wax_eyebrows_s4, wax_strip).
role(wax_eyebrows_s4, wax_strip, lift).
step(wax_eyebrows, wax_eyebrows_s5).
action(wax_eyebrows_s5, pluck).
object_of(wax_eyebrows_s5, eyebrow).
uses(wax_eyebrows_s5, tweezers).
role(wax_eyebrows_s5, tweezers, hold).
before(wax_eyebrows_s1, wax_eyebrows_s2).
before(wax_eyebrows_s2, wax_eyebrows_s3).
before(wax_eyebrows_s3, wax_eyebrows_s4).
before(wax_eyebrows_s4, wax_eyebrows_s5).
```

## aliases
```datalog
task_alias(wax_eyebrows, "waxing eyebrows").
task_alias(wax_eyebrows, "eyebrow waxing").
alias(wax_strip, "wax strip").
alias(cleansing_wipe, "cleansing wipe").
alias(talc_powder, "talc powder").
alias(tweezers, "tweezers").
alias(smooth_brows, "neat brows").
alias(aloe_gel, "aloe gel").
```

## teach
- To wax eyebrows you need a wax strip, a cleansing wipe, talc powder and tweezers.
- Waxing eyebrows takes 15 to 20 minutes.
- Step 1 of waxing eyebrows: clean the eyebrows with a cleansing wipe.
- Step 2 of waxing eyebrows: dust the skin with talc powder.
- Step 3 of waxing eyebrows: press a wax strip onto the eyebrow hair.
- Step 4 of waxing eyebrows: pull the strip off against the direction of hair growth.
- Step 5 of waxing eyebrows: pluck any stray hairs with tweezers.

## checklist
- names what is needed, including a wax strip, a cleansing wipe, talc powder and tweezers
- gives the steps in order: clean, dust, apply strip, pull strip, pluck with tweezers
- says to pull the strip against the direction of hair growth
- gives a total time of about 15 to 20 minutes

## variant
- change: no talc powder at home: skip the dusting step and go straight from cleaning to applying the strip
- request: I don't have any talc powder. Can I still wax my eyebrows?
- situation: I have wax strips, a cleansing wipe and tweezers, but no talc powder.
- expect: The reply leaves out the dusting step and goes straight from cleaning the brows to pressing on the wax strip, then pulling and plucking as usual.

```datalog situation
lacks(user, talc_powder).
has(user, wax_strip).
has(user, cleansing_wipe).
has(user, tweezers).
```

```datalog difference
drop(wax_eyebrows_s2).
no_need(talc_powder).
```

## variant
- change: sensitive skin: add a soothing aloe step at the end, after plucking
- request: My skin gets irritated easily. Can you add something to calm it down after I've waxed?
- situation: I have wax strips, a cleansing wipe, talc powder, tweezers and aloe gel, and I want something to soothe my skin afterwards.
- expect: The reply keeps the usual five steps and adds a last step after plucking that spreads aloe gel over the brows to soothe the skin.

```datalog situation
has(user, wax_strip).
has(user, cleansing_wipe).
has(user, talc_powder).
has(user, tweezers).
has(user, aloe_gel).
wants(user, aftercare).
```

```datalog difference
extra_need(aloe_gel).
add_step(wax_eyebrows_s9, wax_eyebrows_s5).
step(wax_eyebrows, wax_eyebrows_s9).
action(wax_eyebrows_s9, apply).
object_of(wax_eyebrows_s9, skin).
uses(wax_eyebrows_s9, aloe_gel).
role(wax_eyebrows_s9, aloe_gel, cover).
before(wax_eyebrows_s5, wax_eyebrows_s9).
```
