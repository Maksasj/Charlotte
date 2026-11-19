---
id: clean_ceiling_fan
stage: procedures
task: clean_ceiling_fan
schema: v3
---
# Clean a ceiling fan

## requests
- My ceiling fan is really dusty. How should I clean it?
- What's the proper way to wash the blades on a ceiling fan?
- Can you tell me how to clean the fan above my bed without making a mess?

## situation
- I have a feather duster, a microfibre cloth, a bucket, dish soap and a ladder.
- I have about fifteen minutes free.

## ground truth
```datalog
task(clean_ceiling_fan).
domain(clean_ceiling_fan, cleaning).
goal(clean_ceiling_fan, clean_fan).
needs(clean_ceiling_fan, duster).
needs(clean_ceiling_fan, microfiber_cloth).
needs(clean_ceiling_fan, bucket).
needs(clean_ceiling_fan, dish_soap).
takes_time(clean_ceiling_fan, 10, 15, minute).
step(clean_ceiling_fan, clean_ceiling_fan_s1).
action(clean_ceiling_fan_s1, switch_off).
object_of(clean_ceiling_fan_s1, ceiling_fan).
check(clean_ceiling_fan_s1, blades_stopped).
step(clean_ceiling_fan, clean_ceiling_fan_s2).
action(clean_ceiling_fan_s2, dust).
object_of(clean_ceiling_fan_s2, fan_blade).
uses(clean_ceiling_fan_s2, duster).
role(clean_ceiling_fan_s2, duster, wipe).
step(clean_ceiling_fan, clean_ceiling_fan_s3).
action(clean_ceiling_fan_s3, mix).
object_of(clean_ceiling_fan_s3, dish_soap).
uses(clean_ceiling_fan_s3, dish_soap).
role(clean_ceiling_fan_s3, dish_soap, mix).
uses(clean_ceiling_fan_s3, bucket).
role(clean_ceiling_fan_s3, bucket, contain).
into(clean_ceiling_fan_s3, bucket).
overlaps(clean_ceiling_fan_s3, clean_ceiling_fan_s2).
step(clean_ceiling_fan, clean_ceiling_fan_s4).
action(clean_ceiling_fan_s4, wipe).
object_of(clean_ceiling_fan_s4, fan_blade).
uses(clean_ceiling_fan_s4, microfiber_cloth).
role(clean_ceiling_fan_s4, microfiber_cloth, wipe).
check(clean_ceiling_fan_s4, no_dust).
before(clean_ceiling_fan_s1, clean_ceiling_fan_s2).
before(clean_ceiling_fan_s2, clean_ceiling_fan_s4).
before(clean_ceiling_fan_s3, clean_ceiling_fan_s4).
```

## aliases
```datalog
task_alias(clean_ceiling_fan, "clean a ceiling fan").
task_alias(clean_ceiling_fan, "wash ceiling fan blades").
alias(ceiling_fan, "fan").
alias(fan_blade, "blade").
alias(duster, "feather duster").
alias(microfiber_cloth, "microfibre cloth").
alias(dish_soap, "washing up liquid").
alias(bucket, "bowl").
alias(switch_off, "turn off").
alias(dust, "dust off").
alias(no_dust, "no dust left").
alias(blades_stopped, "blades have stopped").
```

## teach
- To clean a ceiling fan you need a duster, a microfibre cloth, a bucket and dish soap.
- Cleaning a ceiling fan takes 10 to 15 minutes.
- Step 1 of cleaning a ceiling fan: switch off the fan and wait until the blades have stopped.
- Step 2 of cleaning a ceiling fan: dust the blades with the duster.
- While step 2 runs, step 3 of cleaning a ceiling fan: mix dish soap into a bucket of warm water.
- Step 4 of cleaning a ceiling fan: wipe each blade with the damp microfibre cloth until there is no dust left.

## checklist
- names what is needed, including a duster, a microfibre cloth, a bucket and dish soap
- gives the steps in order: switch off the fan, dust the blades, mix soapy water, wipe each blade with the cloth
- says the soapy water can be mixed while the blades are being dusted
- gives a total time of about 10 to 15 minutes

## variant
- change: no dish soap and only 10 minutes: skip the soapy water and dry-wipe the blades with the microfibre cloth
- request: I want to clean the fan but I have no washing up liquid and only ten minutes. What can I do?
- situation: I have a duster, a microfibre cloth and a ladder, and ten minutes.
- expect: The reply switches the fan off, dusts the blades with the duster, then dry-wipes each blade with the microfibre cloth, taking about 5 to 8 minutes without any soapy water.

```datalog situation
lacks(user, dish_soap).
time_limit(user, 10, minute).
```

```datalog difference
drop(clean_ceiling_fan_s3).
no_need(dish_soap).
no_need(bucket).
change(clean_ceiling_fan, takes_time, 10_15, 5_8).
```

## variant
- change: no duster, so use the microfibre cloth in its place to dust the blades first
- request: My feather duster has broken. Can I still dust the fan blades before washing them?
- situation: I have a microfibre cloth, a bucket, dish soap and a ladder, and no duster.
- expect: The reply switches the fan off, dusts the blades with the dry microfibre cloth in place of the duster, then wipes each blade with the damp cloth.

```datalog situation
lacks(user, duster).
has(user, microfiber_cloth).
```

```datalog difference
swap(clean_ceiling_fan_s2, duster, microfiber_cloth).
no_need(duster).
```
