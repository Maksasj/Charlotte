---
id: roast_vegetables__wants_rosemary
stage: procedures
task: roast_vegetables
schema: v3
tier: S
base: roast_vegetables
---
# Roast vegetables

## requests
- Doing a tray of roast veg for Sunday lunch and I've got fresh rosemary from the garden. Where does it fit in?

## ground truth
```datalog
task(roast_vegetables).
domain(roast_vegetables, cooking).
goal(roast_vegetables, roasted_vegetable).
needs(roast_vegetables, vegetable).
needs(roast_vegetables, olive_oil).
needs(roast_vegetables, salt).
needs(roast_vegetables, knife).
needs(roast_vegetables, baking_tray).
needs(roast_vegetables, oven).
takes_time(roast_vegetables, 40, 50, minute).
step(roast_vegetables, roast_vegetables_s1).
action(roast_vegetables_s1, preheat).
object_of(roast_vegetables_s1, oven).
setting(roast_vegetables_s1, temperature, 200, celsius).
takes_time(roast_vegetables_s1, 10, 15, minute).
step(roast_vegetables, roast_vegetables_s2).
action(roast_vegetables_s2, chop).
object_of(roast_vegetables_s2, vegetable).
uses(roast_vegetables_s2, knife).
role(roast_vegetables_s2, knife, cut).
purpose(roast_vegetables_s2, even_cooking).
takes_time(roast_vegetables_s2, 8, 10, minute).
overlaps(roast_vegetables_s2, roast_vegetables_s1).
step(roast_vegetables, roast_vegetables_s3).
action(roast_vegetables_s3, toss).
object_of(roast_vegetables_s3, vegetable).
uses(roast_vegetables_s3, olive_oil).
role(roast_vegetables_s3, olive_oil, coat).
uses(roast_vegetables_s3, salt).
role(roast_vegetables_s3, salt, season).
uses(roast_vegetables_s3, baking_tray).
role(roast_vegetables_s3, baking_tray, hold).
onto(roast_vegetables_s3, baking_tray).
takes_time(roast_vegetables_s3, 2, 3, minute).
step(roast_vegetables, roast_vegetables_s4).
action(roast_vegetables_s4, roast).
object_of(roast_vegetables_s4, vegetable).
uses(roast_vegetables_s4, oven).
role(roast_vegetables_s4, oven, heat).
until(roast_vegetables_s4, golden).
check(roast_vegetables_s4, golden_edges).
takes_time(roast_vegetables_s4, 25, 30, minute).
before(roast_vegetables_s1, roast_vegetables_s3).
before(roast_vegetables_s2, roast_vegetables_s3).
before(roast_vegetables_s3, roast_vegetables_s4).
```

## aliases
```datalog
task_alias(roast_vegetables, "roasting vegetables").
task_alias(roast_vegetables, "roast veg").
task_alias(roast_vegetables, "roasted vegetables").
task_alias(roast_vegetables, "a tray of roasted vegetables").
alias(vegetable, "vegetables").
alias(vegetable, "mixed vegetables").
alias(vegetable, "veg").
alias(baking_tray, "tray").
alias(baking_tray, "baking sheet").
alias(preheat, "heat").
alias(chop, "cut").
alias(golden_edges, "golden at the edges").
alias(even_cooking, "they cook evenly").
```

## teach
- To roast vegetables you need vegetables, olive oil, salt, a knife, a baking tray and an oven.
- Roasting vegetables takes 40 to 50 minutes.
- Step 1 of roasting vegetables: preheat the oven to 200 degrees for 10 to 15 minutes.
- While step 1 runs, step 2 of roasting vegetables: chop the vegetables into even chunks with the knife so they cook evenly, for 8 to 10 minutes.
- Step 3 of roasting vegetables: toss the vegetables with olive oil and salt on the baking tray for 2 to 3 minutes.
- Step 4 of roasting vegetables: roast the vegetables in the oven until golden at the edges, for 25 to 30 minutes.

## variant
- change: add rosemary: scatter it over the oiled vegetables before roasting
- request: Doing a tray of roast veg for Sunday lunch and I've got fresh rosemary from the garden. Where does it fit in?
- situation: I have mixed vegetables, olive oil, salt, a knife, a baking tray, an oven and a few sprigs of rosemary.
- expect: The reply scatters the rosemary over the vegetables on the tray after tossing them in oil and salt, then roasts as usual.

```datalog situation
has(user, vegetable).
has(user, olive_oil).
has(user, salt).
has(user, knife).
has(user, baking_tray).
has(user, oven).
has(user, rosemary).
wants(user, rosemary).
```

```datalog difference
extra_need(rosemary).
add_step(roast_vegetables_s9, roast_vegetables_s3).
step(roast_vegetables, roast_vegetables_s9).
action(roast_vegetables_s9, scatter).
object_of(roast_vegetables_s9, rosemary).
onto(roast_vegetables_s9, vegetable).
before(roast_vegetables_s3, roast_vegetables_s9).
before(roast_vegetables_s9, roast_vegetables_s4).
```
