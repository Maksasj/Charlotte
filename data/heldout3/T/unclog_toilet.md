---
id: unclog_toilet
stage: tools
task: unclog_toilet
schema: v3
tier: T
---
# Unclog a toilet

## requests
- The toilet's blocked and the water is rising. What do I do?
- How do I use a plunger on a clogged toilet?
- My loo won't flush properly, the bowl just fills up. Can I sort it myself?

## situation
- I have rubber gloves and a flange plunger.

## ground truth
```datalog
task(unclog_toilet).
domain(unclog_toilet, repair).
goal(unclog_toilet, toilet_drains).
needs(unclog_toilet, glove).
needs(unclog_toilet, plunger).
takes_time(unclog_toilet, 10, 20, minute).
step(unclog_toilet, unclog_toilet_s1).
action(unclog_toilet_s1, shut).
object_of(unclog_toilet_s1, water_valve).
purpose(unclog_toilet_s1, no_overflow).
step(unclog_toilet, unclog_toilet_s2).
action(unclog_toilet_s2, put_on).
object_of(unclog_toilet_s2, glove).
step(unclog_toilet, unclog_toilet_s3).
action(unclog_toilet_s3, place).
object_of(unclog_toilet_s3, plunger).
into(unclog_toilet_s3, drain).
purpose(unclog_toilet_s3, tight_seal).
step(unclog_toilet, unclog_toilet_s4).
action(unclog_toilet_s4, plunge).
object_of(unclog_toilet_s4, drain).
uses(unclog_toilet_s4, plunger).
role(unclog_toilet_s4, plunger, press).
repeat(unclog_toilet_s4, 10).
takes_time(unclog_toilet_s4, 1, 2, minute).
step(unclog_toilet, unclog_toilet_s5).
action(unclog_toilet_s5, flush).
object_of(unclog_toilet_s5, toilet).
check(unclog_toilet_s5, water_drains).
before(unclog_toilet_s1, unclog_toilet_s2).
before(unclog_toilet_s2, unclog_toilet_s3).
before(unclog_toilet_s3, unclog_toilet_s4).
before(unclog_toilet_s4, unclog_toilet_s5).
```

## aliases
```datalog
task_alias(unclog_toilet, "unblocking a toilet").
task_alias(unclog_toilet, "clearing a blocked loo").
task_alias(unclog_toilet, "plunging a toilet").
alias(water_valve, "the valve behind the toilet").
alias(plunger, "flange plunger").
alias(glove, "rubber gloves").
alias(put_on, "put on").
alias(tight_seal, "a tight seal").
alias(water_drains, "the water drains away").
```

## teach
- To unclog a toilet you need rubber gloves and a flange plunger.
- Unclogging a toilet takes 10 to 20 minutes.
- Step 1 of unclogging a toilet: shut the water valve behind the toilet so it cannot overflow.
- Step 2 of unclogging a toilet: put on the rubber gloves.
- Step 3 of unclogging a toilet: place the plunger over the drain hole to make a tight seal.
- Step 4 of unclogging a toilet: plunge the drain about 10 times, for 1 to 2 minutes.
- Step 5 of unclogging a toilet: flush the toilet to check the water drains away.

## checklist
- names what is needed: gloves and a plunger
- gives the steps in order: shut valve, gloves on, seal plunger, plunge, flush to test
- says to shut the water valve first so it does not overflow

## variant
- change: no plunger: hook the blockage out with a straightened wire coat hanger
- request: Toilet's blocked and I don't own a plunger. Anything around the house that might work?
- situation: I have rubber gloves and a wire coat hanger.
- expect: The reply straightens the wire coat hanger and works it into the drain to break up and hook out the blockage in place of plunging.

```datalog situation
has(user, glove).
has(user, coat_hanger).
lacks(user, plunger).
```

```datalog difference
drop(unclog_toilet_s3).
swap(unclog_toilet_s4, plunger, coat_hanger).
change(unclog_toilet_s4, action, plunge, hook).
extra_need(coat_hanger).
no_need(plunger).
```

## variant
- change: wants it hygienic: disinfect the plunger afterwards
- request: Once the toilet's clear, how do I clean up so nothing nasty is left on the plunger?
- situation: I have rubber gloves, a plunger and disinfectant spray.
- expect: The reply adds a last step of spraying the plunger with disinfectant after flushing.

```datalog situation
has(user, glove).
has(user, plunger).
has(user, disinfectant_spray).
wants(user, hygienic).
```

```datalog difference
extra_need(disinfectant_spray).
add_step(unclog_toilet_s9, unclog_toilet_s5).
step(unclog_toilet, unclog_toilet_s9).
action(unclog_toilet_s9, spray).
object_of(unclog_toilet_s9, plunger).
uses(unclog_toilet_s9, disinfectant_spray).
role(unclog_toilet_s9, disinfectant_spray, clean).
before(unclog_toilet_s5, unclog_toilet_s9).
```
