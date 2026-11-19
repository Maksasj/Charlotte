---
id: clean_washing_machine
stage: procedures
task: clean_washing_machine
schema: v3
---
# Clean a washing machine

## requests
- My washing machine smells musty. How do I clean it out properly?
- What's the method for cleaning a washer drum so it stops smelling?
- Can you give me the steps to clean my washer, including the door seal and the detergent drawer?

## situation
- I have white vinegar, a cloth, a brush and a washing machine.
- I've got a free afternoon, about two hours.

## ground truth
```datalog
task(clean_washing_machine).
domain(clean_washing_machine, cleaning).
goal(clean_washing_machine, clean_machine).
needs(clean_washing_machine, washing_machine).
needs(clean_washing_machine, white_vinegar).
needs(clean_washing_machine, cloth).
needs(clean_washing_machine, brush).
takes_time(clean_washing_machine, 90, 120, minute).
step(clean_washing_machine, clean_washing_machine_s1).
action(clean_washing_machine_s1, pour).
object_of(clean_washing_machine_s1, white_vinegar).
into(clean_washing_machine_s1, washing_machine).
uses(clean_washing_machine_s1, white_vinegar).
role(clean_washing_machine_s1, white_vinegar, clean).
amount(clean_washing_machine_s1, white_vinegar, 2, cup).
warning(clean_washing_machine_s1, mixes_with_bleach).
step(clean_washing_machine, clean_washing_machine_s2).
action(clean_washing_machine_s2, run).
object_of(clean_washing_machine_s2, washing_machine).
setting(clean_washing_machine_s2, temperature, 60, celsius).
takes_time(clean_washing_machine_s2, 60, 90, minute).
step(clean_washing_machine, clean_washing_machine_s3).
action(clean_washing_machine_s3, wipe).
object_of(clean_washing_machine_s3, washing_machine).
with_part(clean_washing_machine_s3, door_seal).
uses(clean_washing_machine_s3, cloth).
role(clean_washing_machine_s3, cloth, wipe).
purpose(clean_washing_machine_s3, removes_mould).
overlaps(clean_washing_machine_s3, clean_washing_machine_s2).
step(clean_washing_machine, clean_washing_machine_s4).
action(clean_washing_machine_s4, scrub).
object_of(clean_washing_machine_s4, washing_machine).
with_part(clean_washing_machine_s4, dispenser_drawer).
uses(clean_washing_machine_s4, brush).
role(clean_washing_machine_s4, brush, scrub).
overlaps(clean_washing_machine_s4, clean_washing_machine_s2).
step(clean_washing_machine, clean_washing_machine_s5).
action(clean_washing_machine_s5, leave_open).
object_of(clean_washing_machine_s5, washing_machine).
purpose(clean_washing_machine_s5, prevents_mould).
before(clean_washing_machine_s1, clean_washing_machine_s2).
before(clean_washing_machine_s2, clean_washing_machine_s5).
before(clean_washing_machine_s3, clean_washing_machine_s5).
before(clean_washing_machine_s4, clean_washing_machine_s5).
```

## aliases
```datalog
task_alias(clean_washing_machine, "washing machine cleaning").
task_alias(clean_washing_machine, "clean the washer").
task_alias(clean_washing_machine, "washer cleaning").
alias(washing_machine, "washer").
alias(white_vinegar, "vinegar").
alias(brush, "scrub brush").
alias(dispenser_drawer, "detergent drawer").
alias(door_seal, "rubber seal").
alias(baking_soda, "bicarbonate of soda").
alias(removes_mould, "removes mould").
alias(prevents_mould, "stops mould").
alias(mixes_with_bleach, "mixed with bleach").
```

## teach
- To clean a washing machine you need a washing machine, white vinegar, a cloth and a brush.
- Cleaning a washing machine takes 90 to 120 minutes.
- Step 1 of cleaning a washing machine: pour 2 cups of white vinegar into the drum, and never mix it with bleach.
- Step 2 of cleaning a washing machine: run an empty hot cycle at 60 degrees for 60 to 90 minutes.
- While step 2 runs, step 3 of cleaning a washing machine: wipe the rubber door seal with the cloth to remove mould.
- While step 2 runs, step 4 of cleaning a washing machine: scrub the detergent drawer with the brush.
- Step 5 of cleaning a washing machine: leave the door open so the drum dries, which stops mould.

## checklist
- names what is needed, including a washing machine, white vinegar, a cloth and a brush
- gives the steps in order: pour vinegar, run hot empty cycle, wipe door seal and scrub drawer during the cycle, leave door open
- warns not to mix the vinegar with bleach
- gives a total time of about 90 to 120 minutes

## variant
- change: no scrub brush: use an old toothbrush on the detergent drawer
- request: I don't have a scrub brush. Can I still clean the detergent drawer?
- situation: I have white vinegar, a cloth, an old toothbrush and a washing machine.
- expect: The reply uses the old toothbrush to scrub the detergent drawer in step 4 and keeps the rest of the procedure the same.

```datalog situation
lacks(user, brush).
has(user, old_toothbrush).
has(user, white_vinegar).
has(user, cloth).
has(user, washing_machine).
```

```datalog difference
swap(clean_washing_machine_s4, brush, old_toothbrush).
no_need(brush).
extra_need(old_toothbrush).
```

## variant
- change: no white vinegar: use baking soda in the drum instead
- request: I'm out of vinegar. Is baking soda a good substitute for cleaning the machine?
- situation: I have baking soda, a cloth, a brush and a washing machine.
- expect: The reply pours baking soda into the drum in place of vinegar in step 1 and keeps the hot cycle and the cleaning steps.

```datalog situation
lacks(user, white_vinegar).
has(user, baking_soda).
has(user, cloth).
has(user, brush).
has(user, washing_machine).
```

```datalog difference
swap(clean_washing_machine_s1, white_vinegar, baking_soda).
no_need(white_vinegar).
extra_need(baking_soda).
```
