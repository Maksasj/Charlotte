---
id: make_hummus
stage: procedures
task: make_hummus
---
# Make hummus

## requests
- I'd like to make my own hummus instead of buying it. How?
- Can you walk me through blending hummus from a tin of chickpeas?
- What's the easiest way to make a smooth chickpea dip at home?

## situation
- I have a tin of chickpeas, tahini, a lemon, garlic, olive oil, a sieve and a blender.
- I have about 15 minutes.

## ground truth
```datalog
task(make_hummus).
needs(make_hummus, chickpea).
needs(make_hummus, tahini).
needs(make_hummus, lemon).
needs(make_hummus, garlic).
needs(make_hummus, olive_oil).
needs(make_hummus, sieve).
needs(make_hummus, blender).
takes_time(make_hummus, 8, 12, minute).
step(make_hummus, make_hummus_s1).
action(make_hummus_s1, drain).
object_of(make_hummus_s1, chickpea).
uses(make_hummus_s1, sieve).
takes_time(make_hummus_s1, 1, 2, minute).
step(make_hummus, make_hummus_s2).
action(make_hummus_s2, squeeze).
object_of(make_hummus_s2, lemon).
takes_time(make_hummus_s2, 1, 2, minute).
step(make_hummus, make_hummus_s3).
action(make_hummus_s3, blend).
object_of(make_hummus_s3, chickpea).
uses(make_hummus_s3, blender).
uses(make_hummus_s3, tahini).
uses(make_hummus_s3, garlic).
uses(make_hummus_s3, lemon).
until(make_hummus_s3, smooth).
takes_time(make_hummus_s3, 3, 5, minute).
step(make_hummus, make_hummus_s4).
action(make_hummus_s4, drizzle).
object_of(make_hummus_s4, olive_oil).
takes_time(make_hummus_s4, 1, 1, minute).
before(make_hummus_s1, make_hummus_s2).
before(make_hummus_s2, make_hummus_s3).
before(make_hummus_s3, make_hummus_s4).
```

## teach
- To make hummus you need a tin of chickpeas, tahini, a lemon, garlic, olive oil, a sieve and a blender.
- Making hummus takes 8 to 12 minutes.
- Step 1 of making hummus: drain and rinse the chickpeas in the sieve for 1 to 2 minutes.
- Step 2 of making hummus: squeeze the juice from the lemon for 1 to 2 minutes.
- Step 3 of making hummus: blend the chickpeas with the tahini, the garlic and the lemon juice until smooth, for 3 to 5 minutes.
- Step 4 of making hummus: drizzle the olive oil over the hummus for 1 minute.

## checklist
- names what is needed: a tin of chickpeas, tahini, a lemon, garlic, olive oil, a sieve and a blender
- gives the steps in order: drain chickpea, squeeze lemon, blend chickpea, drizzle olive oil
- gives a total time of about 8 to 12 minutes
- closes politely
