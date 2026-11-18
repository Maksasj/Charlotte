---
id: clean_grout
stage: household
task: clean_grout
schema: v3
tier: T
---
# Clean tile grout

## requests
- The lines between my bathroom tiles have gone grey and grubby. How do I fix that?
- How do I clean grout?
- Any method for getting the grout in my shower white again?

## situation
- I have baking soda, water, a bowl, an old toothbrush, white vinegar in a spray bottle and a sponge.

## ground truth
```datalog
task(clean_grout).
domain(clean_grout, cleaning).
goal(clean_grout, clean_grout).
needs(clean_grout, baking_soda).
needs(clean_grout, water).
needs(clean_grout, bowl).
needs(clean_grout, old_toothbrush).
needs(clean_grout, white_vinegar).
needs(clean_grout, spray_bottle).
needs(clean_grout, sponge).
takes_time(clean_grout, 25, 35, minute).
step(clean_grout, clean_grout_s1).
action(clean_grout_s1, mix).
object_of(clean_grout_s1, baking_soda).
into(clean_grout_s1, bowl).
uses(clean_grout_s1, bowl).
role(clean_grout_s1, bowl, contain).
uses(clean_grout_s1, water).
role(clean_grout_s1, water, mix).
achieves(clean_grout_s1, paste).
step(clean_grout, clean_grout_s2).
action(clean_grout_s2, spread).
object_of(clean_grout_s2, paste).
onto(clean_grout_s2, grout).
uses(clean_grout_s2, old_toothbrush).
role(clean_grout_s2, old_toothbrush, spread).
step(clean_grout, clean_grout_s3).
action(clean_grout_s3, spray).
object_of(clean_grout_s3, paste).
uses(clean_grout_s3, spray_bottle).
role(clean_grout_s3, spray_bottle, spray).
uses(clean_grout_s3, white_vinegar).
role(clean_grout_s3, white_vinegar, clean).
purpose(clean_grout_s3, fizzes).
step(clean_grout, clean_grout_s4).
action(clean_grout_s4, scrub).
object_of(clean_grout_s4, grout).
uses(clean_grout_s4, old_toothbrush).
role(clean_grout_s4, old_toothbrush, scrub).
takes_time(clean_grout_s4, 5, 10, minute).
step(clean_grout, clean_grout_s5).
action(clean_grout_s5, rinse).
object_of(clean_grout_s5, grout).
uses(clean_grout_s5, sponge).
role(clean_grout_s5, sponge, wipe).
uses(clean_grout_s5, water).
role(clean_grout_s5, water, clean).
check(clean_grout_s5, grout_clean).
before(clean_grout_s1, clean_grout_s2).
before(clean_grout_s2, clean_grout_s3).
before(clean_grout_s3, clean_grout_s4).
before(clean_grout_s4, clean_grout_s5).
```

## aliases
```datalog
task_alias(clean_grout, "cleaning grout").
task_alias(clean_grout, "whitening grout").
task_alias(clean_grout, "scrubbing tile grout").
alias(grout, "the lines between the tiles").
alias(old_toothbrush, "toothbrush").
alias(white_vinegar, "vinegar").
alias(fizzes, "it fizzes and lifts the dirt").
alias(paste, "a thick paste").
```

## teach
- To clean tile grout you need baking soda, water, a bowl, an old toothbrush, white vinegar in a spray bottle and a sponge.
- Cleaning tile grout takes 25 to 35 minutes.
- Step 1 of cleaning tile grout: mix baking soda with water in the bowl into a thick paste.
- Step 2 of cleaning tile grout: spread the paste onto the grout with the old toothbrush.
- Step 3 of cleaning tile grout: spray the paste with white vinegar from the spray bottle so it fizzes.
- Step 4 of cleaning tile grout: scrub the grout with the toothbrush for 5 to 10 minutes.
- Step 5 of cleaning tile grout: rinse the grout with the sponge and clean water until it looks clean.

## checklist
- names what is needed, including baking soda, a toothbrush and vinegar
- gives the steps in order: make paste, spread on grout, spray vinegar, scrub, rinse
- gives a total time of about 25 to 35 minutes

## variant
- change: no vinegar: skip the spray and scrub the paste alone
- request: I'm all out of vinegar. Can I still clean the grout with baking soda?
- situation: I have baking soda, water, a bowl, an old toothbrush and a sponge.
- expect: The reply skips spraying with vinegar and scrubs the baking soda paste straight into the grout before rinsing.

```datalog situation
has(user, baking_soda).
has(user, water).
has(user, bowl).
has(user, old_toothbrush).
has(user, sponge).
lacks(user, white_vinegar).
```

```datalog difference
drop(clean_grout_s3).
no_need(white_vinegar).
no_need(spray_bottle).
```

## variant
- change: wants it to stay clean: seal the grout once dry
- request: I'd like the grout to stay clean longer once it's done. Is there something I can add?
- situation: I have everything for cleaning grout plus a bottle of grout sealer with a brush tip.
- expect: The reply adds a last step: once the grout is dry, coat it with grout sealer.

```datalog situation
has(user, baking_soda).
has(user, water).
has(user, bowl).
has(user, old_toothbrush).
has(user, white_vinegar).
has(user, spray_bottle).
has(user, sponge).
has(user, grout_sealer).
wants(user, stays_clean).
```

```datalog difference
extra_need(grout_sealer).
add_step(clean_grout_s9, clean_grout_s5).
step(clean_grout, clean_grout_s9).
action(clean_grout_s9, seal).
object_of(clean_grout_s9, grout).
uses(clean_grout_s9, grout_sealer).
role(clean_grout_s9, grout_sealer, coat).
requires_state(clean_grout_s9, grout_dry).
before(clean_grout_s5, clean_grout_s9).
```
