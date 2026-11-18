---
id: harden_off_seedlings
stage: household
task: harden_off_seedlings
schema: v3
tier: G
---
# Harden off seedlings

## requests
- My courgette seedlings have been on the windowsill for weeks. Can I just put them straight outside?
- How do I get indoor-grown seedlings used to the outdoors before planting them out?
- What's the process for hardening off young plants?

## situation
- I have a tray of seedlings grown indoors, a sheet of garden fleece and a prepared bed.
- It's late spring and the frosts have mostly finished.

## ground truth
```datalog
task(harden_off_seedlings).
domain(harden_off_seedlings, gardening).
goal(harden_off_seedlings, hardy_seedlings).
needs(harden_off_seedlings, seedling_tray).
needs(harden_off_seedlings, fleece).
takes_time(harden_off_seedlings, 7, 14, day).
step(harden_off_seedlings, harden_off_seedlings_s1).
action(harden_off_seedlings_s1, carry).
object_of(harden_off_seedlings_s1, seedling_tray).
onto(harden_off_seedlings_s1, sheltered_spot).
takes_time(harden_off_seedlings_s1, 1, 2, hour).
condition(harden_off_seedlings_s1, no_frost).
step(harden_off_seedlings, harden_off_seedlings_s2).
action(harden_off_seedlings_s2, bring).
object_of(harden_off_seedlings_s2, seedling_tray).
condition(harden_off_seedlings_s2, evening).
repeat(harden_off_seedlings_s2, 7).
purpose(harden_off_seedlings_s2, avoid_cold_night).
step(harden_off_seedlings, harden_off_seedlings_s3).
action(harden_off_seedlings_s3, cover).
object_of(harden_off_seedlings_s3, seedling_tray).
uses(harden_off_seedlings_s3, fleece).
role(harden_off_seedlings_s3, fleece, cover).
condition(harden_off_seedlings_s3, left_out_overnight).
takes_time(harden_off_seedlings_s3, 3, 7, day).
step(harden_off_seedlings, harden_off_seedlings_s4).
action(harden_off_seedlings_s4, plant).
object_of(harden_off_seedlings_s4, seedling).
into(harden_off_seedlings_s4, bed).
before(harden_off_seedlings_s1, harden_off_seedlings_s2).
before(harden_off_seedlings_s2, harden_off_seedlings_s3).
before(harden_off_seedlings_s3, harden_off_seedlings_s4).
```

## aliases
```datalog
task_alias(harden_off_seedlings, "hardening off seedlings").
task_alias(harden_off_seedlings, "get seedlings used to the outdoors").
task_alias(harden_off_seedlings, "harden off young plants").
alias(seedling, "seedlings").
alias(seedling, "courgette seedlings").
alias(seedling_tray, "tray of seedlings").
alias(fleece, "garden fleece").
alias(sheltered_spot, "a sheltered, shady spot").
alias(net_curtain, "old net curtain").
alias(left_out_overnight, "once they stay out at night").
```

## teach
- To harden off seedlings you need the tray of seedlings and some garden fleece.
- Hardening off seedlings takes 7 to 14 days.
- Step 1 of hardening off seedlings: if there is no frost, carry the tray outside to a sheltered, shady spot for 1 to 2 hours, a little longer each day.
- Step 2 of hardening off seedlings: bring the tray back in each evening to avoid the cold night, for about 7 days.
- Step 3 of hardening off seedlings: once they stay out at night, cover the tray with fleece, for 3 to 7 days.
- Step 4 of hardening off seedlings: plant the seedlings into the bed.

## checklist
- names what is needed: the seedlings and some fleece
- gives the steps in order: out by day, in at night, out overnight under fleece, plant out
- says to increase the time outside gradually
- gives a total time of about 7 to 14 days

## variant
- change: only 3 days before going away: a shortened hardening off under fleece
- request: I'm off on holiday in three days and want the courgettes planted before I go. Can I harden them off that quickly?
- situation: I have a tray of seedlings, garden fleece and a prepared bed, and only 3 days.
- expect: The reply squeezes the process into about 3 days, with a day or two of days outside and nights in, then a night under fleece before planting, and warns that it is a bit rushed.

```datalog situation
has(user, seedling_tray).
has(user, fleece).
time_limit(user, 3, day).
```

```datalog difference
change(harden_off_seedlings_s2, repeat, 7, 2).
change(harden_off_seedlings_s3, takes_time, 3_7, 1_1).
change(harden_off_seedlings, takes_time, 7_14, 3_3).
```

## variant
- change: fleece torn: cover the seedlings with an old net curtain
- request: The fox ripped my garden fleece to shreds. Can I use an old net curtain to cover the seedlings at night instead?
- situation: My fleece is torn and useless. I have a tray of seedlings, a prepared bed and an old net curtain.
- expect: The reply covers the tray with the net curtain at night instead of fleece and keeps the other steps.

```datalog situation
lacks(user, fleece).
has(user, net_curtain).
has(user, seedling_tray).
```

```datalog difference
no_need(fleece).
extra_need(net_curtain).
swap(harden_off_seedlings_s3, fleece, net_curtain).
```
