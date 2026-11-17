---
id: roast_vegetables
stage: procedures
task: roast_vegetables
---
# Roast vegetables

## requests
- I have a pile of vegetables to use up. How do I roast them in the oven?
- What's the trick to getting roast veg nice and golden?
- Can you explain how to make a tray of roasted vegetables for dinner?

## situation
- I have mixed vegetables, olive oil, salt, a knife, a baking tray and an oven.
- Dinner is in about an hour.

## ground truth
```datalog
task(roast_vegetables).
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
takes_time(roast_vegetables_s1, 10, 15, minute).
step(roast_vegetables, roast_vegetables_s2).
action(roast_vegetables_s2, chop).
object_of(roast_vegetables_s2, vegetable).
uses(roast_vegetables_s2, knife).
takes_time(roast_vegetables_s2, 8, 10, minute).
overlaps(roast_vegetables_s2, roast_vegetables_s1).
step(roast_vegetables, roast_vegetables_s3).
action(roast_vegetables_s3, toss).
object_of(roast_vegetables_s3, vegetable).
uses(roast_vegetables_s3, olive_oil).
uses(roast_vegetables_s3, salt).
uses(roast_vegetables_s3, baking_tray).
takes_time(roast_vegetables_s3, 2, 3, minute).
step(roast_vegetables, roast_vegetables_s4).
action(roast_vegetables_s4, roast).
object_of(roast_vegetables_s4, vegetable).
uses(roast_vegetables_s4, oven).
until(roast_vegetables_s4, golden).
takes_time(roast_vegetables_s4, 25, 30, minute).
before(roast_vegetables_s1, roast_vegetables_s3).
before(roast_vegetables_s2, roast_vegetables_s3).
before(roast_vegetables_s3, roast_vegetables_s4).
```

## teach
- To roast vegetables you need vegetables, olive oil, salt, a knife, a baking tray and an oven.
- Roasting vegetables takes 40 to 50 minutes.
- Step 1 of roasting vegetables: preheat the oven to 200 degrees for 10 to 15 minutes.
- While step 1 runs, step 2 of roasting vegetables: chop the vegetables into even chunks with the knife for 8 to 10 minutes.
- Step 3 of roasting vegetables: toss the vegetables with olive oil and salt on the baking tray for 2 to 3 minutes.
- Step 4 of roasting vegetables: roast the vegetables in the oven until golden at the edges, for 25 to 30 minutes.

## checklist
- names what is needed: vegetables, olive oil, salt, a knife, a baking tray and an oven
- gives the steps in order: preheat oven, chop vegetable (while preheat oven), toss vegetable, roast vegetable
- gives a total time of about 40 to 50 minutes
- closes politely

## variant
- change: only 30 minutes: cut the vegetables smaller and roast them for about 15 minutes
- request: I only have half an hour before dinner. Can I still roast some vegetables?
- situation: I have vegetables, olive oil, salt, a knife, a baking tray and an oven, and about 30 minutes.
- expect: The reply cuts the vegetables smaller, chops them while the oven heats and shortens the roast to about 15 minutes so the whole thing fits in about 30 minutes.
