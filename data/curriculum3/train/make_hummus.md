---
id: make_hummus
stage: procedures
task: make_hummus
schema: v3
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
domain(make_hummus, cooking).
goal(make_hummus, hummus).
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
role(make_hummus_s1, sieve, hold).
step(make_hummus, make_hummus_s2).
action(make_hummus_s2, rinse).
object_of(make_hummus_s2, chickpea).
uses(make_hummus_s2, sieve).
role(make_hummus_s2, sieve, hold).
takes_time(make_hummus_s2, 1, 2, minute).
step(make_hummus, make_hummus_s3).
action(make_hummus_s3, squeeze).
object_of(make_hummus_s3, lemon).
takes_time(make_hummus_s3, 1, 2, minute).
step(make_hummus, make_hummus_s4).
action(make_hummus_s4, blend).
object_of(make_hummus_s4, chickpea).
uses(make_hummus_s4, blender).
role(make_hummus_s4, blender, blend).
uses(make_hummus_s4, tahini).
role(make_hummus_s4, tahini, flavour).
uses(make_hummus_s4, garlic).
role(make_hummus_s4, garlic, flavour).
uses(make_hummus_s4, lemon).
role(make_hummus_s4, lemon, flavour).
until(make_hummus_s4, smooth).
takes_time(make_hummus_s4, 3, 5, minute).
step(make_hummus, make_hummus_s5).
action(make_hummus_s5, drizzle).
object_of(make_hummus_s5, olive_oil).
onto(make_hummus_s5, hummus).
takes_time(make_hummus_s5, 1, 1, minute).
before(make_hummus_s1, make_hummus_s2).
before(make_hummus_s2, make_hummus_s3).
before(make_hummus_s3, make_hummus_s4).
before(make_hummus_s4, make_hummus_s5).
```

## aliases
```datalog
task_alias(make_hummus, "making hummus").
task_alias(make_hummus, "chickpea dip").
task_alias(make_hummus, "blending hummus").
alias(chickpea, "chickpeas").
alias(chickpea, "tin of chickpeas").
alias(lemon, "lemon juice").
alias(lemon, "juice").
alias(olive_oil, "oil").
```

## teach
- To make hummus you need a tin of chickpeas, tahini, a lemon, garlic, olive oil, a sieve and a blender.
- Making hummus takes 8 to 12 minutes.
- Step 1 of making hummus: drain the chickpeas in the sieve.
- Step 2 of making hummus: rinse the chickpeas in the sieve for 1 to 2 minutes.
- Step 3 of making hummus: squeeze the juice from the lemon for 1 to 2 minutes.
- Step 4 of making hummus: blend the chickpeas with the tahini, the garlic and the lemon juice until smooth, for 3 to 5 minutes.
- Step 5 of making hummus: drizzle the olive oil over the hummus for 1 minute.

## checklist
- names what is needed: a tin of chickpeas, tahini, a lemon, garlic, olive oil, a sieve and a blender
- gives the steps in order: drain chickpea, squeeze lemon, blend chickpea, drizzle olive oil
- gives a total time of about 8 to 12 minutes
- closes politely
