---
id: clean_mattress
stage: procedures
task: clean_mattress
schema: v3
---
# Clean a mattress

## requests
- How do I clean a stained mattress properly at home?
- What's the best way to deep clean a mattress?
- Can you walk me through getting stains out of my mattress?

## situation
- I have a stained mattress, a vacuum cleaner, mild detergent, a spray bottle, a soft brush and a clean cloth.
- I have a free afternoon and can leave the mattress to dry overnight.

## ground truth
```datalog
task(clean_mattress).
domain(clean_mattress, cleaning).
goal(clean_mattress, fresh_mattress).
needs(clean_mattress, vacuum_cleaner).
needs(clean_mattress, mild_detergent).
needs(clean_mattress, spray_bottle).
needs(clean_mattress, soft_brush).
needs(clean_mattress, clean_cloth).
takes_time(clean_mattress, 150, 210, minute).
step(clean_mattress, clean_mattress_s1).
action(clean_mattress_s1, vacuum).
object_of(clean_mattress_s1, mattress).
uses(clean_mattress_s1, vacuum_cleaner).
role(clean_mattress_s1, vacuum_cleaner, clean).
takes_time(clean_mattress_s1, 10, 15, minute).
step(clean_mattress, clean_mattress_s2).
action(clean_mattress_s2, mix).
object_of(clean_mattress_s2, mild_detergent).
uses(clean_mattress_s2, spray_bottle).
role(clean_mattress_s2, spray_bottle, contain).
into(clean_mattress_s2, spray_bottle).
overlaps(clean_mattress_s2, clean_mattress_s1).
step(clean_mattress, clean_mattress_s3).
action(clean_mattress_s3, spray).
object_of(clean_mattress_s3, mattress).
uses(clean_mattress_s3, spray_bottle).
role(clean_mattress_s3, spray_bottle, spread).
purpose(clean_mattress_s3, stains_lift).
step(clean_mattress, clean_mattress_s4).
action(clean_mattress_s4, scrub).
object_of(clean_mattress_s4, mattress).
uses(clean_mattress_s4, soft_brush).
role(clean_mattress_s4, soft_brush, scrub).
step(clean_mattress, clean_mattress_s5).
action(clean_mattress_s5, wipe).
object_of(clean_mattress_s5, mattress).
uses(clean_mattress_s5, clean_cloth).
role(clean_mattress_s5, clean_cloth, wipe).
step(clean_mattress, clean_mattress_s6).
action(clean_mattress_s6, dry).
object_of(clean_mattress_s6, mattress).
until(clean_mattress_s6, dry).
takes_time(clean_mattress_s6, 120, 180, minute).
before(clean_mattress_s1, clean_mattress_s3).
before(clean_mattress_s2, clean_mattress_s3).
before(clean_mattress_s3, clean_mattress_s4).
before(clean_mattress_s4, clean_mattress_s5).
before(clean_mattress_s5, clean_mattress_s6).
```

## aliases
```datalog
task_alias(clean_mattress, "deep clean a mattress").
task_alias(clean_mattress, "cleaning a mattress").
alias(vacuum_cleaner, "vacuum").
alias(mild_detergent, "mild soap").
alias(soft_brush, "soft-bristled brush").
alias(clean_cloth, "clean cloth").
alias(spray_bottle, "spray bottle").
alias(stains_lift, "stains lift").
alias(dry, "dries").
```

## teach
- To clean a mattress you need a vacuum cleaner, mild detergent, a spray bottle, a soft brush and a clean cloth.
- Cleaning a mattress takes 150 to 210 minutes.
- Step 1 of cleaning a mattress: vacuum the mattress with the vacuum cleaner for 10 to 15 minutes.
- While step 1 runs, step 2 of cleaning a mattress: mix the mild detergent into the spray bottle.
- Step 3 of cleaning a mattress: spray the mattress from the spray bottle so the stains lift.
- Step 4 of cleaning a mattress: scrub the stains with the soft brush.
- Step 5 of cleaning a mattress: wipe the mattress with the clean cloth.
- Step 6 of cleaning a mattress: leave the mattress to dry for 120 to 180 minutes until it is dry.

## checklist
- names what is needed: a vacuum cleaner, mild detergent, a spray bottle, a soft brush and a clean cloth
- gives the steps in order: vacuum, mix detergent, spray, scrub, wipe, dry
- says the detergent can be mixed while the mattress is being vacuumed
- gives a total time of about 150 to 210 minutes

## variant
- change: no soft brush: scrub the stains with the clean cloth instead
- request: I don't have a soft brush for the stains on my mattress. Can I still clean it?
- situation: I have no soft brush, but I have a vacuum cleaner, mild detergent, a spray bottle and a clean cloth.
- expect: The reply scrubs the stains with the clean cloth in place of the soft brush and keeps the other steps the same.

```datalog situation
has(user, vacuum_cleaner).
has(user, mild_detergent).
has(user, spray_bottle).
has(user, clean_cloth).
lacks(user, soft_brush).
```

```datalog difference
swap(clean_mattress_s4, soft_brush, clean_cloth).
no_need(soft_brush).
```

- change: add a final step to cover the dried mattress with a mattress protector
- request: After I clean my mattress I want to put a protector on it so the dog can't stain it again. What do I do?
- situation: I have a mattress protector, and I have all the usual cleaning supplies.
- expect: The reply keeps the cleaning steps and adds a last step, after drying, to cover the mattress with the mattress protector.

```datalog situation
has(user, mattress_protector).
has(user, vacuum_cleaner).
has(user, mild_detergent).
has(user, spray_bottle).
has(user, soft_brush).
has(user, clean_cloth).
```

```datalog difference
extra_need(mattress_protector).
add_step(clean_mattress_s9, clean_mattress_s6).
step(clean_mattress, clean_mattress_s9).
action(clean_mattress_s9, cover).
object_of(clean_mattress_s9, mattress).
uses(clean_mattress_s9, mattress_protector).
role(clean_mattress_s9, mattress_protector, cover).
before(clean_mattress_s6, clean_mattress_s9).
```
