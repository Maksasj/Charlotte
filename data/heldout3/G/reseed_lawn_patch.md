---
id: reseed_lawn_patch
stage: household
task: reseed_lawn_patch
schema: v3
tier: G
---
# Reseed a bare lawn patch

## requests
- The dog has worn a bald patch in the middle of the grass. How do I get it to grow back?
- What's the way to reseed a bare bit of lawn?
- How do I patch up a dead spot in my lawn with grass seed?

## situation
- I have a rake, a garden fork, a bag of topsoil, grass seed, a watering can and water.

## ground truth
```datalog
task(reseed_lawn_patch).
domain(reseed_lawn_patch, gardening).
goal(reseed_lawn_patch, repaired_lawn).
needs(reseed_lawn_patch, rake).
needs(reseed_lawn_patch, garden_fork).
needs(reseed_lawn_patch, topsoil).
needs(reseed_lawn_patch, grass_seed).
needs(reseed_lawn_patch, watering_can).
needs(reseed_lawn_patch, water).
takes_time(reseed_lawn_patch, 25, 35, minute).
step(reseed_lawn_patch, reseed_lawn_patch_s1).
action(reseed_lawn_patch_s1, rake).
object_of(reseed_lawn_patch_s1, patch).
uses(reseed_lawn_patch_s1, rake).
role(reseed_lawn_patch_s1, rake, gather).
purpose(reseed_lawn_patch_s1, remove_dead_grass).
step(reseed_lawn_patch, reseed_lawn_patch_s2).
action(reseed_lawn_patch_s2, loosen).
object_of(reseed_lawn_patch_s2, soil).
uses(reseed_lawn_patch_s2, garden_fork).
role(reseed_lawn_patch_s2, garden_fork, loosen).
step(reseed_lawn_patch, reseed_lawn_patch_s3).
action(reseed_lawn_patch_s3, spread).
object_of(reseed_lawn_patch_s3, topsoil).
onto(reseed_lawn_patch_s3, patch).
setting(reseed_lawn_patch_s3, depth, 2, centimetre).
step(reseed_lawn_patch, reseed_lawn_patch_s4).
action(reseed_lawn_patch_s4, sow).
object_of(reseed_lawn_patch_s4, grass_seed).
onto(reseed_lawn_patch_s4, patch).
amount(reseed_lawn_patch_s4, grass_seed, 35, gram).
step(reseed_lawn_patch, reseed_lawn_patch_s5).
action(reseed_lawn_patch_s5, rake).
object_of(reseed_lawn_patch_s5, grass_seed).
uses(reseed_lawn_patch_s5, rake).
role(reseed_lawn_patch_s5, rake, cover).
step(reseed_lawn_patch, reseed_lawn_patch_s6).
action(reseed_lawn_patch_s6, water).
object_of(reseed_lawn_patch_s6, patch).
uses(reseed_lawn_patch_s6, watering_can).
role(reseed_lawn_patch_s6, watering_can, pour).
uses(reseed_lawn_patch_s6, water).
role(reseed_lawn_patch_s6, water, wet).
check(reseed_lawn_patch_s6, soil_moist).
before(reseed_lawn_patch_s1, reseed_lawn_patch_s2).
before(reseed_lawn_patch_s2, reseed_lawn_patch_s3).
before(reseed_lawn_patch_s3, reseed_lawn_patch_s4).
before(reseed_lawn_patch_s4, reseed_lawn_patch_s5).
before(reseed_lawn_patch_s5, reseed_lawn_patch_s6).
```

## aliases
```datalog
task_alias(reseed_lawn_patch, "reseeding a bare lawn patch").
task_alias(reseed_lawn_patch, "patch up the lawn").
task_alias(reseed_lawn_patch, "reseed a bare bit of lawn").
alias(patch, "bald patch").
alias(patch, "bare patch").
alias(patch, "dead spot").
alias(grass_seed, "seed").
alias(garden_fork, "fork").
alias(remove_dead_grass, "clears the dead grass").
alias(netting, "bird netting").
alias(netting, "net").
alias(bird_protection, "keep the birds off").
```

## teach
- To reseed a bare lawn patch you need a rake, a garden fork, topsoil, grass seed, a watering can and water.
- Reseeding a bare lawn patch takes 25 to 35 minutes.
- Step 1 of reseeding a bare lawn patch: rake the patch to clear the dead grass.
- Step 2 of reseeding a bare lawn patch: loosen the soil with the garden fork.
- Step 3 of reseeding a bare lawn patch: spread about 2 centimetres of topsoil over the patch.
- Step 4 of reseeding a bare lawn patch: sow about 35 grams of grass seed per square metre over the patch.
- Step 5 of reseeding a bare lawn patch: rake the seed in lightly so it is just covered.
- Step 6 of reseeding a bare lawn patch: water the patch with the watering can until the soil is moist.

## checklist
- names what is needed: a rake, a fork, topsoil, grass seed and water
- gives the steps in order: rake, loosen, topsoil, sow, rake in, water
- gives the seed rate of about 35 grams per square metre
- gives a total time of about 25 to 35 minutes

## variant
- change: wants to stop birds eating the seed: cover the patch with netting after watering
- request: Every time I sow grass seed the pigeons eat it all. How do I fix the bare patch and keep the birds off this time?
- situation: I have a rake, a garden fork, topsoil, grass seed, a watering can, water and some bird netting.
- expect: The reply adds a step after watering to stretch the netting over the patch so the birds cannot reach the seed.

```datalog situation
has(user, rake).
has(user, garden_fork).
has(user, topsoil).
has(user, grass_seed).
has(user, watering_can).
has(user, water).
has(user, netting).
wants(user, bird_protection).
```

```datalog difference
extra_need(netting).
add_step(reseed_lawn_patch_s9, reseed_lawn_patch_s6).
step(reseed_lawn_patch, reseed_lawn_patch_s9).
action(reseed_lawn_patch_s9, cover).
object_of(reseed_lawn_patch_s9, patch).
uses(reseed_lawn_patch_s9, netting).
role(reseed_lawn_patch_s9, netting, cover).
purpose(reseed_lawn_patch_s9, bird_protection).
before(reseed_lawn_patch_s6, reseed_lawn_patch_s9).
```

## variant
- change: no topsoil: sow straight onto the loosened soil
- request: I'd like to sort out the bare spot on the lawn but I've got no topsoil. Can I just sow onto what's there?
- situation: I have a rake, a garden fork, grass seed, a watering can and water, but no topsoil.
- expect: The reply leaves out the topsoil step and sows straight onto the loosened soil, keeping the other steps.

```datalog situation
has(user, rake).
has(user, garden_fork).
has(user, grass_seed).
has(user, watering_can).
has(user, water).
lacks(user, topsoil).
```

```datalog difference
no_need(topsoil).
drop(reseed_lawn_patch_s3).
```
