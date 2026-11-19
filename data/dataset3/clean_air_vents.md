---
id: clean_air_vents
stage: procedures
task: clean_air_vents
schema: v3
---
# Clean air vents

## requests
- How do I clean the air vents in my house without taking the whole wall apart?
- What's the right way to get dust out of ventilation grilles?
- I want to freshen up the vents in the bedroom. Can you give me the steps?

## situation
- I have a screwdriver, a vacuum cleaner and a damp cloth.
- The vent covers are screwed onto the wall.

## ground truth
```datalog
task(clean_air_vents).
domain(clean_air_vents, cleaning).
goal(clean_air_vents, clean_vents).
needs(clean_air_vents, screwdriver).
needs(clean_air_vents, vacuum_cleaner).
needs(clean_air_vents, damp_cloth).
takes_time(clean_air_vents, 30, 40, minute).
step(clean_air_vents, clean_air_vents_s1).
action(clean_air_vents_s1, unscrew).
object_of(clean_air_vents_s1, vent_cover).
uses(clean_air_vents_s1, screwdriver).
role(clean_air_vents_s1, screwdriver, loosen).
step(clean_air_vents, clean_air_vents_s2).
action(clean_air_vents_s2, vacuum).
object_of(clean_air_vents_s2, vent_grille).
uses(clean_air_vents_s2, vacuum_cleaner).
role(clean_air_vents_s2, vacuum_cleaner, clean).
purpose(clean_air_vents_s2, dust_removed).
step(clean_air_vents, clean_air_vents_s3).
action(clean_air_vents_s3, wipe).
object_of(clean_air_vents_s3, vent_cover).
uses(clean_air_vents_s3, damp_cloth).
role(clean_air_vents_s3, damp_cloth, wipe).
overlaps(clean_air_vents_s3, clean_air_vents_s2).
step(clean_air_vents, clean_air_vents_s4).
action(clean_air_vents_s4, dry).
object_of(clean_air_vents_s4, vent_cover).
achieves(clean_air_vents_s4, dry).
step(clean_air_vents, clean_air_vents_s5).
action(clean_air_vents_s5, fasten).
object_of(clean_air_vents_s5, vent_cover).
uses(clean_air_vents_s5, screwdriver).
role(clean_air_vents_s5, screwdriver, fasten).
before(clean_air_vents_s1, clean_air_vents_s2).
before(clean_air_vents_s1, clean_air_vents_s3).
before(clean_air_vents_s3, clean_air_vents_s4).
before(clean_air_vents_s4, clean_air_vents_s5).
before(clean_air_vents_s2, clean_air_vents_s5).
```

## aliases
```datalog
task_alias(clean_air_vents, "clean air vents").
task_alias(clean_air_vents, "clean vents").
alias(vent_cover, "vent cover").
alias(vent_grille, "grille").
alias(vacuum_cleaner, "hoover").
alias(damp_cloth, "damp cloth").
alias(fasten, "screw back on").
alias(dry, "dries").
alias(dust_removed, "dust gone").
```

## teach
- To clean air vents you need a screwdriver, a vacuum cleaner and a damp cloth.
- Cleaning air vents takes 30 to 40 minutes.
- Step 1 of clean air vents: unscrew the vent cover with the screwdriver.
- Step 2 of clean air vents: vacuum the vent grille to remove the dust.
- While step 2 runs, step 3 of clean air vents: wipe the vent cover with the damp cloth.
- Step 4 of clean air vents: let the vent cover dry completely.
- Step 5 of clean air vents: screw the vent cover back on with the screwdriver.

## checklist
- names what is needed: a screwdriver, a vacuum cleaner and a damp cloth
- gives the steps in order: unscrew the cover, vacuum the grille, wipe the cover, let it dry, screw it back on
- says the vacuuming and wiping of the cover can be done at the same time
- gives a total time of about 30 to 40 minutes

## variant
- change: the user wants the covers disinfected as well, so a spray step is added after wiping
- request: Can you also disinfect the vent covers while I clean them?
- situation: I have a screwdriver, a vacuum cleaner, a damp cloth and a disinfectant spray.
- expect: The reply adds a step after wiping where the cover is sprayed with disinfectant, and says to let it dry before screwing it back on.

```datalog situation
has(user, screwdriver).
has(user, vacuum_cleaner).
has(user, damp_cloth).
has(user, disinfectant_spray).
```

```datalog difference
add_step(clean_air_vents_s9, clean_air_vents_s3).
step(clean_air_vents, clean_air_vents_s9).
action(clean_air_vents_s9, spray).
object_of(clean_air_vents_s9, vent_cover).
uses(clean_air_vents_s9, disinfectant_spray).
role(clean_air_vents_s9, disinfectant_spray, clean).
before(clean_air_vents_s3, clean_air_vents_s9).
before(clean_air_vents_s9, clean_air_vents_s4).
extra_need(disinfectant_spray).
```

## variant
- change: only 20 minutes, so the drying wait is dropped and the cover is wiped dry before it is screwed back on
- request: I only have 20 minutes to clean the vents. Is that enough?
- situation: I have a screwdriver, a vacuum cleaner, a damp cloth and 20 minutes.
- expect: The reply says the job fits in about 15 to 20 minutes, skips the drying wait, and tells the user to wipe the cover dry before screwing it back on.

```datalog situation
has(user, screwdriver).
has(user, vacuum_cleaner).
has(user, damp_cloth).
time_limit(user, 20, minute).
```

```datalog difference
change(clean_air_vents, takes_time, 30_40, 15_20).
drop(clean_air_vents_s4).
```
