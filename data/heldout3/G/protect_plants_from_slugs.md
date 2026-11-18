---
id: protect_plants_from_slugs
stage: household
task: protect_plants_from_slugs
schema: v3
tier: G
---
# Protect plants from slugs

## requests
- Something keeps munching holes in my lettuces overnight. I think it's slugs. What can I do?
- How do I keep slugs off my hostas without pellets?
- What's a good way to protect my plants from slugs and snails?

## situation
- I have copper tape, crushed eggshells, a clean jar, a trowel and a can of beer.

## ground truth
```datalog
task(protect_plants_from_slugs).
domain(protect_plants_from_slugs, gardening).
goal(protect_plants_from_slugs, slug_free_plant).
needs(protect_plants_from_slugs, copper_tape).
needs(protect_plants_from_slugs, eggshell).
needs(protect_plants_from_slugs, jar).
needs(protect_plants_from_slugs, trowel).
needs(protect_plants_from_slugs, beer).
takes_time(protect_plants_from_slugs, 15, 20, minute).
step(protect_plants_from_slugs, protect_plants_from_slugs_s1).
action(protect_plants_from_slugs_s1, stick).
object_of(protect_plants_from_slugs_s1, copper_tape).
onto(protect_plants_from_slugs_s1, pot).
with_part(protect_plants_from_slugs_s1, rim).
purpose(protect_plants_from_slugs_s1, slug_barrier).
step(protect_plants_from_slugs, protect_plants_from_slugs_s2).
action(protect_plants_from_slugs_s2, sprinkle).
object_of(protect_plants_from_slugs_s2, eggshell).
onto(protect_plants_from_slugs_s2, soil).
purpose(protect_plants_from_slugs_s2, slug_barrier).
step(protect_plants_from_slugs, protect_plants_from_slugs_s3).
action(protect_plants_from_slugs_s3, sink).
object_of(protect_plants_from_slugs_s3, jar).
into(protect_plants_from_slugs_s3, soil).
uses(protect_plants_from_slugs_s3, trowel).
role(protect_plants_from_slugs_s3, trowel, lift).
check(protect_plants_from_slugs_s3, rim_above_soil).
step(protect_plants_from_slugs, protect_plants_from_slugs_s4).
action(protect_plants_from_slugs_s4, pour).
object_of(protect_plants_from_slugs_s4, beer).
into(protect_plants_from_slugs_s4, jar).
purpose(protect_plants_from_slugs_s4, lure_slugs).
step(protect_plants_from_slugs, protect_plants_from_slugs_s5).
action(protect_plants_from_slugs_s5, empty).
object_of(protect_plants_from_slugs_s5, jar).
condition(protect_plants_from_slugs_s5, next_morning).
before(protect_plants_from_slugs_s1, protect_plants_from_slugs_s2).
before(protect_plants_from_slugs_s2, protect_plants_from_slugs_s3).
before(protect_plants_from_slugs_s3, protect_plants_from_slugs_s4).
before(protect_plants_from_slugs_s4, protect_plants_from_slugs_s5).
```

## aliases
```datalog
task_alias(protect_plants_from_slugs, "protecting plants from slugs").
task_alias(protect_plants_from_slugs, "keep slugs off").
task_alias(protect_plants_from_slugs, "stop slugs eating my plants").
alias(eggshell, "crushed eggshells").
alias(jar, "clean jar").
alias(jar, "beer trap").
alias(pot, "pots").
alias(rim_above_soil, "rim just above the soil").
alias(milk, "milk").
alias(alcohol, "beer").
alias(alcohol, "alcohol").
alias(slug_barrier, "barrier slugs won't cross").
```

## teach
- To protect plants from slugs you need copper tape, crushed eggshells, a jar, a trowel and some beer.
- Protecting plants from slugs takes 15 to 20 minutes.
- Step 1 of protecting plants from slugs: stick copper tape around the rim of each pot as a barrier slugs won't cross.
- Step 2 of protecting plants from slugs: sprinkle crushed eggshells on the soil around the plants as another barrier.
- Step 3 of protecting plants from slugs: sink the jar into the soil with the trowel, with the rim just above the soil.
- Step 4 of protecting plants from slugs: pour beer into the jar to lure the slugs.
- Step 5 of protecting plants from slugs: empty the jar the next morning.

## checklist
- names what is needed: copper tape, eggshells, a jar, a trowel and beer
- gives the steps in order: copper tape, eggshells, sink jar, beer, empty
- says to leave the jar rim just above the soil
- gives a total time of about 15 to 20 minutes

## variant
- change: no alcohol in the house: bait the trap with milk instead of beer
- request: We don't keep any alcohol at home. Is there another way to make a slug trap for my lettuces?
- situation: I have copper tape, eggshells, a jar, a trowel and some milk, and we don't have alcohol.
- expect: The reply fills the trap with milk instead of beer and keeps the other steps.

```datalog situation
excludes(user, alcohol).
has(user, milk).
has(user, copper_tape).
has(user, eggshell).
has(user, jar).
has(user, trowel).
```

```datalog difference
no_need(beer).
extra_need(milk).
change(protect_plants_from_slugs_s4, object_of, beer, milk).
```

## variant
- change: no copper tape: skip the tape and rely on eggshells and the trap
- request: I'm out of copper tape. How do I keep slugs away from the hostas without it?
- situation: I have eggshells, a jar, a trowel and beer, but no copper tape.
- expect: The reply leaves out the copper tape step and keeps the eggshells and beer trap.

```datalog situation
lacks(user, copper_tape).
has(user, eggshell).
has(user, jar).
has(user, trowel).
has(user, beer).
```

```datalog difference
no_need(copper_tape).
drop(protect_plants_from_slugs_s1).
```
