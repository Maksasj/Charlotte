---
id: braise_beef_short_ribs
stage: procedures
task: braise_beef_short_ribs
schema: v3
---
# Braise beef short ribs

## requests
- I want really tender beef short ribs for dinner. How do I braise them?
- Can you walk me through making short ribs in the oven with stock?
- What's the method for slow-cooked beef short ribs? I have most afternoon.

## situation
- I have short ribs, salt, a dutch oven with a lid, beef stock and an oven.
- I have about three hours free this afternoon.

## ground truth
```datalog
task(braise_beef_short_ribs).
domain(braise_beef_short_ribs, cooking).
goal(braise_beef_short_ribs, braised_short_ribs).
needs(braise_beef_short_ribs, short_ribs).
needs(braise_beef_short_ribs, salt).
needs(braise_beef_short_ribs, dutch_oven).
needs(braise_beef_short_ribs, beef_stock).
needs(braise_beef_short_ribs, lid).
needs(braise_beef_short_ribs, oven).
takes_time(braise_beef_short_ribs, 180, 210, minute).
step(braise_beef_short_ribs, braise_beef_short_ribs_s1).
action(braise_beef_short_ribs_s1, heat).
object_of(braise_beef_short_ribs_s1, oven).
setting(braise_beef_short_ribs_s1, temperature, 160, celsius).
takes_time(braise_beef_short_ribs_s1, 10, 10, minute).
step(braise_beef_short_ribs, braise_beef_short_ribs_s2).
action(braise_beef_short_ribs_s2, season).
object_of(braise_beef_short_ribs_s2, short_ribs).
uses(braise_beef_short_ribs_s2, salt).
role(braise_beef_short_ribs_s2, salt, season).
overlaps(braise_beef_short_ribs_s2, braise_beef_short_ribs_s1).
step(braise_beef_short_ribs, braise_beef_short_ribs_s3).
action(braise_beef_short_ribs_s3, sear).
object_of(braise_beef_short_ribs_s3, short_ribs).
uses(braise_beef_short_ribs_s3, dutch_oven).
role(braise_beef_short_ribs_s3, dutch_oven, heat).
check(braise_beef_short_ribs_s3, browned).
step(braise_beef_short_ribs, braise_beef_short_ribs_s4).
action(braise_beef_short_ribs_s4, pour).
object_of(braise_beef_short_ribs_s4, beef_stock).
uses(braise_beef_short_ribs_s4, beef_stock).
role(braise_beef_short_ribs_s4, beef_stock, pour).
into(braise_beef_short_ribs_s4, dutch_oven).
step(braise_beef_short_ribs, braise_beef_short_ribs_s5).
action(braise_beef_short_ribs_s5, braise).
object_of(braise_beef_short_ribs_s5, short_ribs).
uses(braise_beef_short_ribs_s5, dutch_oven).
role(braise_beef_short_ribs_s5, dutch_oven, contain).
uses(braise_beef_short_ribs_s5, lid).
role(braise_beef_short_ribs_s5, lid, cover).
uses(braise_beef_short_ribs_s5, oven).
role(braise_beef_short_ribs_s5, oven, heat).
until(braise_beef_short_ribs_s5, tender).
check(braise_beef_short_ribs_s5, tender).
takes_time(braise_beef_short_ribs_s5, 150, 180, minute).
before(braise_beef_short_ribs_s1, braise_beef_short_ribs_s5).
before(braise_beef_short_ribs_s2, braise_beef_short_ribs_s3).
before(braise_beef_short_ribs_s3, braise_beef_short_ribs_s4).
before(braise_beef_short_ribs_s4, braise_beef_short_ribs_s5).
```

## aliases
```datalog
task_alias(braise_beef_short_ribs, "braised short ribs").
task_alias(braise_beef_short_ribs, "beef short ribs").
alias(short_ribs, "beef short ribs").
alias(dutch_oven, "casserole pot").
alias(beef_stock, "stock").
alias(lid, "pot lid").
alias(heat, "preheat").
alias(tender, "fall-apart tender").
alias(stovetop, "hob").
alias(foil, "aluminium foil").
```

## teach
- To braise beef short ribs you need short ribs, salt, a dutch oven, beef stock, a lid and an oven.
- Braising beef short ribs takes 180 to 210 minutes.
- Step 1 of braise beef short ribs: heat the oven to 160 degrees for 10 minutes.
- While step 1 runs, step 2 of braise beef short ribs: season the short ribs with salt.
- Step 3 of braise beef short ribs: sear the short ribs in the dutch oven until browned.
- Step 4 of braise beef short ribs: pour the beef stock into the dutch oven.
- Step 5 of braise beef short ribs: cover the dutch oven with the lid and braise the short ribs in the oven for 150 to 180 minutes until tender.

## checklist
- names what is needed, including short ribs, salt, a dutch oven, beef stock, a lid and an oven
- gives the steps in order: heat oven, season, sear, pour stock, braise covered until tender
- says the seasoning can be done while the oven heats
- gives a total time of about 180 to 210 minutes

## variant
- change: no oven, so braise on the hob in the covered dutch oven instead
- request: I want braised short ribs but I don't have an oven, only a hob. Can I still do it?
- situation: I have short ribs, salt, a dutch oven with a lid, beef stock and a hob, and about three hours.
- expect: The reply skips the oven preheat and braises the seared short ribs covered on the hob over low heat until tender, allowing about 200 to 240 minutes.

```datalog situation
has(user, short_ribs).
has(user, salt).
has(user, dutch_oven).
has(user, lid).
has(user, beef_stock).
has(user, stovetop).
lacks(user, oven).
time_limit(user, 180, minute).
```

```datalog difference
extra_need(stovetop).
no_need(oven).
drop(braise_beef_short_ribs_s1).
swap(braise_beef_short_ribs_s5, oven, stovetop).
change(braise_beef_short_ribs, takes_time, 180_210, 200_240).
```

## variant
- change: the lid is cracked, so cover the dutch oven with aluminium foil instead
- request: My dutch oven lid cracked but I have aluminium foil. Can I still braise these short ribs?
- situation: I have short ribs, salt, a dutch oven, beef stock, an oven, aluminium foil and about three hours.
- expect: The reply covers the dutch oven tightly with aluminium foil in place of the lid and braises as normal in the oven.

```datalog situation
has(user, short_ribs).
has(user, salt).
has(user, dutch_oven).
has(user, beef_stock).
has(user, oven).
has(user, foil).
lacks(user, lid).
time_limit(user, 180, minute).
```

```datalog difference
no_need(lid).
extra_need(foil).
swap(braise_beef_short_ribs_s5, lid, foil).
```
