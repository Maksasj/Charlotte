---
id: clean_stovetop
stage: household
task: clean_stovetop
schema: v3
tier: T
---
# Clean a glass stovetop

## requests
- How do I get burnt-on gunk off a glass hob without scratching it?
- My ceramic cooktop has brown rings baked onto it. How can I clean it?
- What's the trick to cleaning a glass stovetop?

## situation
- I have baking soda, a towel, washing-up liquid, warm water, a plastic scraper and a cloth.

## ground truth
```datalog
task(clean_stovetop).
domain(clean_stovetop, cleaning).
goal(clean_stovetop, clean_hob).
needs(clean_stovetop, baking_soda).
needs(clean_stovetop, towel).
needs(clean_stovetop, dish_soap).
needs(clean_stovetop, warm_water).
needs(clean_stovetop, scraper).
needs(clean_stovetop, cloth).
takes_time(clean_stovetop, 25, 30, minute).
step(clean_stovetop, clean_stovetop_s1).
action(clean_stovetop_s1, check).
object_of(clean_stovetop_s1, hob).
check(clean_stovetop_s1, cool).
purpose(clean_stovetop_s1, no_burn).
step(clean_stovetop, clean_stovetop_s2).
action(clean_stovetop_s2, sprinkle).
object_of(clean_stovetop_s2, baking_soda).
onto(clean_stovetop_s2, hob).
step(clean_stovetop, clean_stovetop_s3).
action(clean_stovetop_s3, cover).
object_of(clean_stovetop_s3, baking_soda).
uses(clean_stovetop_s3, towel).
role(clean_stovetop_s3, towel, cover).
uses(clean_stovetop_s3, dish_soap).
role(clean_stovetop_s3, dish_soap, clean).
uses(clean_stovetop_s3, warm_water).
role(clean_stovetop_s3, warm_water, wet).
takes_time(clean_stovetop_s3, 15, 15, minute).
step(clean_stovetop, clean_stovetop_s4).
action(clean_stovetop_s4, remove).
object_of(clean_stovetop_s4, towel).
step(clean_stovetop, clean_stovetop_s5).
action(clean_stovetop_s5, scrape).
object_of(clean_stovetop_s5, hob).
uses(clean_stovetop_s5, scraper).
role(clean_stovetop_s5, scraper, lift).
warning(clean_stovetop_s5, scratch).
step(clean_stovetop, clean_stovetop_s6).
action(clean_stovetop_s6, wipe).
object_of(clean_stovetop_s6, hob).
uses(clean_stovetop_s6, cloth).
role(clean_stovetop_s6, cloth, wipe).
check(clean_stovetop_s6, streak_free).
before(clean_stovetop_s1, clean_stovetop_s2).
before(clean_stovetop_s2, clean_stovetop_s3).
before(clean_stovetop_s3, clean_stovetop_s4).
before(clean_stovetop_s4, clean_stovetop_s5).
before(clean_stovetop_s5, clean_stovetop_s6).
```

## aliases
```datalog
task_alias(clean_stovetop, "cleaning a glass hob").
task_alias(clean_stovetop, "cleaning the stovetop").
task_alias(clean_stovetop, "getting burnt marks off a cooktop").
alias(hob, "stovetop").
alias(hob, "cooktop").
alias(dish_soap, "washing-up liquid").
alias(scraper, "plastic scraper").
alias(cool, "completely cool").
```

## teach
- To clean a glass stovetop you need baking soda, a towel, washing-up liquid, warm water, a plastic scraper and a cloth.
- Cleaning a glass stovetop takes 25 to 30 minutes.
- Step 1 of cleaning a glass stovetop: check the hob is completely cool so you do not get burnt.
- Step 2 of cleaning a glass stovetop: sprinkle baking soda onto the hob.
- Step 3 of cleaning a glass stovetop: cover the baking soda with a towel soaked in warm soapy water for 15 minutes.
- Step 4 of cleaning a glass stovetop: remove the towel.
- Step 5 of cleaning a glass stovetop: scrape the hob gently with the plastic scraper; be careful not to scratch the glass.
- Step 6 of cleaning a glass stovetop: wipe the hob with the cloth until it is streak free.

## checklist
- names what is needed, including baking soda, a towel, soapy water and a scraper
- gives the steps in order: check cool, sprinkle soda, cover with wet towel, remove, scrape, wipe
- warns not to scratch the glass

## variant
- change: no scraper: use a plastic spatula
- request: I've got no scraper for the hob. Can I use something from the kitchen drawer?
- situation: I have baking soda, a towel, washing-up liquid, warm water, a plastic spatula and a cloth.
- expect: The reply lifts the loosened residue with the plastic spatula instead of a scraper, gently so as not to scratch.

```datalog situation
has(user, baking_soda).
has(user, towel).
has(user, dish_soap).
has(user, warm_water).
has(user, spatula).
has(user, cloth).
lacks(user, scraper).
```

```datalog difference
swap(clean_stovetop_s5, scraper, spatula).
extra_need(spatula).
no_need(scraper).
```

## variant
- change: only 10 minutes: skip the towel soak
- request: I've got ten minutes before people arrive and the hob looks awful. Quick fix?
- situation: I have baking soda, a towel, washing-up liquid, warm water, a scraper and a cloth, and only 10 minutes.
- expect: The reply skips the 15-minute towel soak, rubs the soda with the damp cloth, scrapes and wipes.

```datalog situation
has(user, baking_soda).
has(user, towel).
has(user, dish_soap).
has(user, warm_water).
has(user, scraper).
has(user, cloth).
time_limit(user, 10, minute).
```

```datalog difference
drop(clean_stovetop_s3).
drop(clean_stovetop_s4).
no_need(towel).
change(clean_stovetop, takes_time, 25_30, 8_10).
```
