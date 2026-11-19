---
id: replace_kitchen_faucet_cartridge
stage: procedures
task: replace_kitchen_faucet_cartridge
schema: v3
---
# Replace a kitchen faucet cartridge

## requests
- My kitchen faucet keeps dripping. How do I replace the cartridge myself?
- What's the procedure for swapping out a kitchen faucet cartridge?
- Can you talk me through fixing a dripping tap by changing its cartridge?

## situation
- I have a dripping kitchen faucet, a replacement cartridge, a screwdriver, an adjustable wrench and some plumber's grease.
- I have a free afternoon to do the job.

## ground truth
```datalog
task(replace_kitchen_faucet_cartridge).
domain(replace_kitchen_faucet_cartridge, repair).
goal(replace_kitchen_faucet_cartridge, fix_dripping_faucet).
needs(replace_kitchen_faucet_cartridge, replacement_cartridge).
needs(replace_kitchen_faucet_cartridge, screwdriver).
needs(replace_kitchen_faucet_cartridge, adjustable_wrench).
needs(replace_kitchen_faucet_cartridge, grease).
takes_time(replace_kitchen_faucet_cartridge, 30, 45, minute).
step(replace_kitchen_faucet_cartridge, replace_kitchen_faucet_cartridge_s1).
action(replace_kitchen_faucet_cartridge_s1, shut_off).
object_of(replace_kitchen_faucet_cartridge_s1, water_valve).
step(replace_kitchen_faucet_cartridge, replace_kitchen_faucet_cartridge_s2).
action(replace_kitchen_faucet_cartridge_s2, remove).
object_of(replace_kitchen_faucet_cartridge_s2, handle).
uses(replace_kitchen_faucet_cartridge_s2, screwdriver).
role(replace_kitchen_faucet_cartridge_s2, screwdriver, loosen).
step(replace_kitchen_faucet_cartridge, replace_kitchen_faucet_cartridge_s3).
action(replace_kitchen_faucet_cartridge_s3, unscrew).
object_of(replace_kitchen_faucet_cartridge_s3, old_cartridge).
uses(replace_kitchen_faucet_cartridge_s3, adjustable_wrench).
role(replace_kitchen_faucet_cartridge_s3, adjustable_wrench, loosen).
step(replace_kitchen_faucet_cartridge, replace_kitchen_faucet_cartridge_s4).
action(replace_kitchen_faucet_cartridge_s4, coat).
object_of(replace_kitchen_faucet_cartridge_s4, replacement_cartridge).
uses(replace_kitchen_faucet_cartridge_s4, grease).
role(replace_kitchen_faucet_cartridge_s4, grease, coat).
step(replace_kitchen_faucet_cartridge, replace_kitchen_faucet_cartridge_s5).
action(replace_kitchen_faucet_cartridge_s5, install).
object_of(replace_kitchen_faucet_cartridge_s5, replacement_cartridge).
step(replace_kitchen_faucet_cartridge, replace_kitchen_faucet_cartridge_s6).
action(replace_kitchen_faucet_cartridge_s6, fasten).
object_of(replace_kitchen_faucet_cartridge_s6, handle).
uses(replace_kitchen_faucet_cartridge_s6, screwdriver).
role(replace_kitchen_faucet_cartridge_s6, screwdriver, fasten).
step(replace_kitchen_faucet_cartridge, replace_kitchen_faucet_cartridge_s7).
action(replace_kitchen_faucet_cartridge_s7, open).
object_of(replace_kitchen_faucet_cartridge_s7, water_valve).
check(replace_kitchen_faucet_cartridge_s7, no_drip).
before(replace_kitchen_faucet_cartridge_s1, replace_kitchen_faucet_cartridge_s2).
before(replace_kitchen_faucet_cartridge_s2, replace_kitchen_faucet_cartridge_s3).
before(replace_kitchen_faucet_cartridge_s3, replace_kitchen_faucet_cartridge_s4).
before(replace_kitchen_faucet_cartridge_s4, replace_kitchen_faucet_cartridge_s5).
before(replace_kitchen_faucet_cartridge_s5, replace_kitchen_faucet_cartridge_s6).
before(replace_kitchen_faucet_cartridge_s6, replace_kitchen_faucet_cartridge_s7).
```

## aliases
```datalog
task_alias(replace_kitchen_faucet_cartridge, "replace faucet cartridge").
task_alias(replace_kitchen_faucet_cartridge, "fix dripping faucet").
alias(replacement_cartridge, "new cartridge").
alias(adjustable_wrench, "adjustable spanner").
alias(grease, "plumber's grease").
alias(plumber_tape, "plumber's tape").
alias(water_valve, "shut-off valve").
alias(no_drip, "no longer drips").
```

## teach
- To replace a kitchen faucet cartridge you need a replacement cartridge, a screwdriver, an adjustable wrench and plumber's grease.
- Replacing a kitchen faucet cartridge takes 30 to 45 minutes.
- Step 1 of replacing a kitchen faucet cartridge: shut off the water valve under the sink.
- Step 2 of replacing a kitchen faucet cartridge: remove the handle with the screwdriver, loosening its screw.
- Step 3 of replacing a kitchen faucet cartridge: unscrew the old cartridge with the adjustable wrench.
- Step 4 of replacing a kitchen faucet cartridge: coat the seal of the replacement cartridge with plumber's grease.
- Step 5 of replacing a kitchen faucet cartridge: install the replacement cartridge.
- Step 6 of replacing a kitchen faucet cartridge: fasten the handle back on with the screwdriver.
- Step 7 of replacing a kitchen faucet cartridge: open the water valve slowly and check that the faucet no longer drips.

## checklist
- names what is needed, including a replacement cartridge, a screwdriver, an adjustable wrench and plumber's grease
- gives the steps in order: shut off the water, remove the handle, unscrew the old cartridge, grease the new one, install it, fasten the handle, open the valve and check
- says the water must be shut off before the handle is removed
- gives a total time of about 30 to 45 minutes

## variant
- change: wrap the faucet threads with plumber's tape before installing the cartridge
- request: While I'm in there, I'd like to tape the threads too. Where does that fit?
- situation: I have the cartridge, a screwdriver, an adjustable wrench and grease, and I want to add plumber's tape.
- expect: The reply adds a step to wrap the faucet threads with plumber's tape after greasing the seal and before installing the cartridge.

```datalog situation
has(user, replacement_cartridge).
has(user, screwdriver).
has(user, adjustable_wrench).
has(user, grease).
wants(user, plumber_tape).
```

```datalog difference
extra_need(plumber_tape).
add_step(replace_kitchen_faucet_cartridge_s9, replace_kitchen_faucet_cartridge_s4).
step(replace_kitchen_faucet_cartridge, replace_kitchen_faucet_cartridge_s9).
action(replace_kitchen_faucet_cartridge_s9, wrap).
object_of(replace_kitchen_faucet_cartridge_s9, faucet_threads).
uses(replace_kitchen_faucet_cartridge_s9, plumber_tape).
role(replace_kitchen_faucet_cartridge_s9, plumber_tape, protect).
before(replace_kitchen_faucet_cartridge_s4, replace_kitchen_faucet_cartridge_s9).
before(replace_kitchen_faucet_cartridge_s9, replace_kitchen_faucet_cartridge_s5).
```

## variant
- change: no adjustable wrench, so use pliers to unscrew the old cartridge
- request: I don't have an adjustable wrench. Can I use pliers instead for the old cartridge?
- situation: I have a replacement cartridge, a screwdriver, pliers and grease, but no adjustable wrench.
- expect: The reply uses pliers in place of the adjustable wrench to unscrew the old cartridge and leaves the rest of the procedure unchanged.

```datalog situation
has(user, replacement_cartridge).
has(user, screwdriver).
lacks(user, adjustable_wrench).
has(user, pliers).
has(user, grease).
```

```datalog difference
swap(replace_kitchen_faucet_cartridge_s3, adjustable_wrench, pliers).
no_need(adjustable_wrench).
extra_need(pliers).
```
