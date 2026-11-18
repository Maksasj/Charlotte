---
id: clean_toilet
stage: household
task: clean_toilet
schema: v3
tier: T
---
# Clean a toilet

## requests
- What's the right order for cleaning a toilet properly?
- The loo is looking grim. How do I give it a proper clean?
- How should I scrub a toilet bowl and seat so it's actually hygienic?

## situation
- I have rubber gloves, toilet cleaner, a toilet brush, disinfectant spray and a cloth.

## ground truth
```datalog
task(clean_toilet).
domain(clean_toilet, cleaning).
goal(clean_toilet, clean_toilet).
needs(clean_toilet, glove).
needs(clean_toilet, toilet_cleaner).
needs(clean_toilet, toilet_brush).
needs(clean_toilet, disinfectant_spray).
needs(clean_toilet, cloth).
takes_time(clean_toilet, 15, 20, minute).
step(clean_toilet, clean_toilet_s1).
action(clean_toilet_s1, squirt).
object_of(clean_toilet_s1, toilet_cleaner).
into(clean_toilet_s1, toilet).
with_part(clean_toilet_s1, rim).
uses(clean_toilet_s1, glove).
role(clean_toilet_s1, glove, protect).
step(clean_toilet, clean_toilet_s2).
action(clean_toilet_s2, leave).
object_of(clean_toilet_s2, toilet_cleaner).
takes_time(clean_toilet_s2, 10, 10, minute).
step(clean_toilet, clean_toilet_s3).
action(clean_toilet_s3, scrub).
object_of(clean_toilet_s3, toilet).
with_part(clean_toilet_s3, bowl).
uses(clean_toilet_s3, toilet_brush).
role(clean_toilet_s3, toilet_brush, scrub).
step(clean_toilet, clean_toilet_s4).
action(clean_toilet_s4, flush).
object_of(clean_toilet_s4, toilet).
step(clean_toilet, clean_toilet_s5).
action(clean_toilet_s5, spray).
object_of(clean_toilet_s5, toilet).
with_part(clean_toilet_s5, seat).
uses(clean_toilet_s5, disinfectant_spray).
role(clean_toilet_s5, disinfectant_spray, clean).
step(clean_toilet, clean_toilet_s6).
action(clean_toilet_s6, wipe).
object_of(clean_toilet_s6, toilet).
uses(clean_toilet_s6, cloth).
role(clean_toilet_s6, cloth, wipe).
with_part(clean_toilet_s6, outside).
before(clean_toilet_s1, clean_toilet_s2).
before(clean_toilet_s2, clean_toilet_s3).
before(clean_toilet_s3, clean_toilet_s4).
before(clean_toilet_s4, clean_toilet_s5).
before(clean_toilet_s5, clean_toilet_s6).
```

## aliases
```datalog
task_alias(clean_toilet, "cleaning a toilet").
task_alias(clean_toilet, "cleaning the loo").
task_alias(clean_toilet, "scrubbing the toilet").
alias(glove, "rubber gloves").
alias(toilet, "loo").
alias(rim, "under the rim").
alias(toilet_cleaner, "toilet gel").
```

## teach
- To clean a toilet you need rubber gloves, toilet cleaner, a toilet brush, disinfectant spray and a cloth.
- Cleaning a toilet takes 15 to 20 minutes.
- Step 1 of cleaning a toilet: wearing the gloves, squirt toilet cleaner under the rim.
- Step 2 of cleaning a toilet: leave the cleaner for 10 minutes.
- Step 3 of cleaning a toilet: scrub the bowl with the toilet brush.
- Step 4 of cleaning a toilet: flush the toilet.
- Step 5 of cleaning a toilet: spray the seat with disinfectant spray.
- Step 6 of cleaning a toilet: wipe the outside of the toilet with the cloth.

## checklist
- names what is needed, including gloves, toilet cleaner, a brush and a cloth
- gives the steps in order: cleaner under rim, leave, scrub bowl, flush, spray seat, wipe outside
- says to leave the cleaner about 10 minutes

## variant
- change: toilet brush broken: scrub with an old sponge kept for the job
- request: The head came off my toilet brush. How do I scrub the bowl now?
- situation: I have gloves, toilet cleaner, disinfectant spray, a cloth and an old sponge; the toilet brush is broken.
- expect: The reply scrubs the bowl with the old sponge, wearing gloves, and throws the sponge away afterwards.

```datalog situation
has(user, glove).
has(user, toilet_cleaner).
has(user, disinfectant_spray).
has(user, cloth).
has(user, sponge).
lacks(user, toilet_brush).
```

```datalog difference
swap(clean_toilet_s3, toilet_brush, sponge).
extra_need(sponge).
no_need(toilet_brush).
```

## variant
- change: no harsh chemicals: baking soda in the bowl and vinegar spray on the seat
- request: I'd rather not use bleach-type cleaners with a baby in the house. How can I clean the toilet?
- situation: I have gloves, baking soda, white vinegar, a toilet brush and a cloth. I want to avoid harsh chemicals.
- expect: The reply sprinkles baking soda into the bowl instead of toilet cleaner and uses white vinegar on the seat instead of disinfectant spray.

```datalog situation
has(user, glove).
has(user, baking_soda).
has(user, white_vinegar).
has(user, toilet_brush).
has(user, cloth).
excludes(user, harsh_chemical).
```

```datalog difference
change(clean_toilet_s1, object_of, toilet_cleaner, baking_soda).
swap(clean_toilet_s5, disinfectant_spray, white_vinegar).
extra_need(baking_soda).
extra_need(white_vinegar).
no_need(toilet_cleaner).
no_need(disinfectant_spray).
```
