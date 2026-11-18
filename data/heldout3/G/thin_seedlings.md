---
id: thin_seedlings
stage: household
task: thin_seedlings
schema: v3
tier: G
---
# Thin seedlings

## requests
- My row of carrots has come up really thick. Do I need to pull some out?
- How do I thin out seedlings without disturbing the ones I keep?
- The lettuces I sowed are all crammed together. What should I do with them?

## situation
- I have a watering can, water, a small pair of scissors and a compost bin.

## ground truth
```datalog
task(thin_seedlings).
domain(thin_seedlings, gardening).
goal(thin_seedlings, spaced_seedlings).
needs(thin_seedlings, watering_can).
needs(thin_seedlings, water).
needs(thin_seedlings, scissors).
needs(thin_seedlings, compost_bin).
takes_time(thin_seedlings, 10, 20, minute).
step(thin_seedlings, thin_seedlings_s1).
action(thin_seedlings_s1, water).
object_of(thin_seedlings_s1, row).
uses(thin_seedlings_s1, watering_can).
role(thin_seedlings_s1, watering_can, pour).
uses(thin_seedlings_s1, water).
role(thin_seedlings_s1, water, wet).
purpose(thin_seedlings_s1, soil_soft).
step(thin_seedlings, thin_seedlings_s2).
action(thin_seedlings_s2, cut).
object_of(thin_seedlings_s2, extra_seedling).
with_part(thin_seedlings_s2, stem).
uses(thin_seedlings_s2, scissors).
role(thin_seedlings_s2, scissors, cut).
setting(thin_seedlings_s2, spacing, 5, centimetre).
purpose(thin_seedlings_s2, roots_undisturbed).
takes_time(thin_seedlings_s2, 5, 15, minute).
step(thin_seedlings, thin_seedlings_s3).
action(thin_seedlings_s3, press).
object_of(thin_seedlings_s3, soil).
step(thin_seedlings, thin_seedlings_s4).
action(thin_seedlings_s4, put).
object_of(thin_seedlings_s4, extra_seedling).
into(thin_seedlings_s4, compost_bin).
uses(thin_seedlings_s4, compost_bin).
role(thin_seedlings_s4, compost_bin, contain).
purpose(thin_seedlings_s4, no_pest_smell).
before(thin_seedlings_s1, thin_seedlings_s2).
before(thin_seedlings_s2, thin_seedlings_s3).
before(thin_seedlings_s3, thin_seedlings_s4).
```

## aliases
```datalog
task_alias(thin_seedlings, "thinning seedlings").
task_alias(thin_seedlings, "thin out the seedlings").
task_alias(thin_seedlings, "pull some out").
alias(row, "row of seedlings").
alias(extra_seedling, "extra seedlings").
alias(extra_seedling, "thinnings").
alias(scissors, "small scissors").
alias(colander, "sieve").
alias(salad_leaf, "eat the thinnings").
alias(no_pest_smell, "so the smell doesn't draw pests").
alias(roots_undisturbed, "so the roots of the ones you keep aren't disturbed").
```

## teach
- To thin seedlings you need a watering can, water, small scissors and a compost bin.
- Thinning seedlings takes 10 to 20 minutes.
- Step 1 of thinning seedlings: water the row first with the watering can so the soil is soft.
- Step 2 of thinning seedlings: cut the extra seedlings off at the stem with the scissors, leaving one every 5 centimetres, so the roots of the ones you keep aren't disturbed, for 5 to 15 minutes.
- Step 3 of thinning seedlings: press the soil gently around the ones that are left.
- Step 4 of thinning seedlings: put the thinnings into the compost bin so the smell doesn't draw pests.

## checklist
- names what is needed: a watering can, water, scissors and a compost bin
- gives the steps in order: water, cut extras, firm soil, compost thinnings
- says to leave one every 5 centimetres
- gives a total time of about 10 to 20 minutes

## variant
- change: wants to eat the thinnings: rinse them in a colander instead of composting
- request: Seems a shame to throw away the baby lettuce leaves when I thin the row. Can I eat them, and how do I do it?
- situation: I have a watering can, water, small scissors, a compost bin and a colander, and I'd like to eat the thinnings.
- expect: The reply adds a step to rinse the thinnings in a colander for a salad, after cutting them.

```datalog situation
has(user, watering_can).
has(user, water).
has(user, scissors).
has(user, compost_bin).
has(user, colander).
wants(user, salad_leaf).
```

```datalog difference
extra_need(colander).
add_step(thin_seedlings_s9, thin_seedlings_s3).
step(thin_seedlings, thin_seedlings_s9).
action(thin_seedlings_s9, rinse).
object_of(thin_seedlings_s9, extra_seedling).
uses(thin_seedlings_s9, colander).
role(thin_seedlings_s9, colander, strain).
before(thin_seedlings_s3, thin_seedlings_s9).
drop(thin_seedlings_s4).
```

## variant
- change: no compost bin: leave out the composting step
- request: I don't have a compost heap. How do I thin my carrot row?
- situation: I have a watering can, water and small scissors, but no compost bin.
- expect: The reply leaves out putting the thinnings in a compost bin and keeps the other steps.

```datalog situation
has(user, watering_can).
has(user, water).
has(user, scissors).
lacks(user, compost_bin).
```

```datalog difference
no_need(compost_bin).
drop(thin_seedlings_s4).
```
