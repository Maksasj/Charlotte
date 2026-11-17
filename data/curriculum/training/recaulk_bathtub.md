---
id: recaulk_bathtub
stage: tools
task: recaulk_bathtub
---
# Reseal a bathtub

## requests
- The sealant around my bath is mouldy and peeling. How do I redo it?
- Can you walk me through replacing the silicone around the tub?
- Water is getting behind my bath. How do I reseal the edge?

## situation
- I have silicone sealant, a caulk gun, a utility knife, rubbing alcohol, masking tape and a cloth.
- I can do it on Sunday morning.

## ground truth
```datalog
task(recaulk_bathtub).
needs(recaulk_bathtub, sealant).
needs(recaulk_bathtub, caulk_gun).
needs(recaulk_bathtub, utility_knife).
needs(recaulk_bathtub, rubbing_alcohol).
needs(recaulk_bathtub, masking_tape).
needs(recaulk_bathtub, cloth).
takes_time(recaulk_bathtub, 45, 60, minute).
step(recaulk_bathtub, recaulk_bathtub_s1).
action(recaulk_bathtub_s1, cut).
object_of(recaulk_bathtub_s1, old_sealant).
uses(recaulk_bathtub_s1, utility_knife).
takes_time(recaulk_bathtub_s1, 15, 20, minute).
step(recaulk_bathtub, recaulk_bathtub_s2).
action(recaulk_bathtub_s2, clean).
object_of(recaulk_bathtub_s2, joint).
uses(recaulk_bathtub_s2, rubbing_alcohol).
uses(recaulk_bathtub_s2, cloth).
takes_time(recaulk_bathtub_s2, 5, 10, minute).
step(recaulk_bathtub, recaulk_bathtub_s3).
action(recaulk_bathtub_s3, tape).
object_of(recaulk_bathtub_s3, joint).
uses(recaulk_bathtub_s3, masking_tape).
takes_time(recaulk_bathtub_s3, 5, 10, minute).
step(recaulk_bathtub, recaulk_bathtub_s4).
action(recaulk_bathtub_s4, apply).
object_of(recaulk_bathtub_s4, sealant).
uses(recaulk_bathtub_s4, caulk_gun).
takes_time(recaulk_bathtub_s4, 10, 15, minute).
step(recaulk_bathtub, recaulk_bathtub_s5).
action(recaulk_bathtub_s5, smooth).
object_of(recaulk_bathtub_s5, sealant).
uses(recaulk_bathtub_s5, masking_tape).
takes_time(recaulk_bathtub_s5, 5, 5, minute).
before(recaulk_bathtub_s1, recaulk_bathtub_s2).
before(recaulk_bathtub_s2, recaulk_bathtub_s3).
before(recaulk_bathtub_s3, recaulk_bathtub_s4).
before(recaulk_bathtub_s4, recaulk_bathtub_s5).
```

## teach
- To reseal a bathtub you need silicone sealant, a caulk gun, a utility knife, rubbing alcohol, masking tape and a cloth.
- Resealing a bathtub takes 45 to 60 minutes.
- Step 1 of resealing a bathtub: cut out the old sealant with the utility knife for 15 to 20 minutes.
- Step 2 of resealing a bathtub: clean the joint with rubbing alcohol on the cloth and let it dry for 5 to 10 minutes.
- Step 3 of resealing a bathtub: put masking tape along both sides of the joint for 5 to 10 minutes.
- Step 4 of resealing a bathtub: apply a steady bead of sealant with the caulk gun for 10 to 15 minutes.
- Step 5 of resealing a bathtub: smooth the bead with a wet finger and peel off the tape for 5 minutes.

## checklist
- names what is needed: silicone sealant, a caulk gun, a utility knife, rubbing alcohol, masking tape and a cloth
- gives the steps in order: cut old sealant, clean joint, tape joint, apply sealant, smooth sealant
- gives a total time of about 45 to 60 minutes
- closes politely

## variant
- change: no masking tape: run the bead carefully and wipe the edges clean with a damp cloth
- request: I want to reseal the bath but I've got no masking tape. Can I do without?
- situation: I have silicone sealant, a caulk gun, a utility knife, rubbing alcohol and two cloths, but no tape.
- expect: The reply skips the taping step, runs the bead slowly and wipes any sealant off the edges with a damp cloth before smoothing, keeping the cut, clean and apply steps.
