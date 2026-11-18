---
id: poach_egg
stage: procedures
task: poach_egg
schema: v3
tier: T
---
# Poach an egg

## requests
- How do I poach an egg without it falling apart in the water?
- I'd love an egg like the ones you get at brunch cafes, soft in the middle. How?
- Can you explain poaching eggs? Mine always turn into wispy clouds.

## situation
- I have eggs, a saucepan, water, white vinegar, a cup, a slotted spoon and paper towels.

## ground truth
```datalog
task(poach_egg).
domain(poach_egg, cooking).
goal(poach_egg, poached_egg).
goal(poach_egg, breakfast).
needs(poach_egg, egg).
needs(poach_egg, saucepan).
needs(poach_egg, water).
needs(poach_egg, vinegar).
needs(poach_egg, cup).
needs(poach_egg, slotted_spoon).
needs(poach_egg, paper_towel).
serves(poach_egg, 1).
takes_time(poach_egg, 8, 12, minute).
step(poach_egg, poach_egg_s1).
action(poach_egg_s1, heat).
object_of(poach_egg_s1, water).
uses(poach_egg_s1, saucepan).
role(poach_egg_s1, saucepan, contain).
until(poach_egg_s1, simmering).
takes_time(poach_egg_s1, 4, 5, minute).
step(poach_egg, poach_egg_s2).
action(poach_egg_s2, add).
object_of(poach_egg_s2, vinegar).
into(poach_egg_s2, water).
purpose(poach_egg_s2, white_holds_together).
step(poach_egg, poach_egg_s3).
action(poach_egg_s3, crack).
object_of(poach_egg_s3, egg).
into(poach_egg_s3, cup).
step(poach_egg, poach_egg_s4).
action(poach_egg_s4, slide).
object_of(poach_egg_s4, egg).
into(poach_egg_s4, water).
uses(poach_egg_s4, cup).
role(poach_egg_s4, cup, hold).
step(poach_egg, poach_egg_s5).
action(poach_egg_s5, cook).
object_of(poach_egg_s5, egg).
takes_time(poach_egg_s5, 3, 4, minute).
check(poach_egg_s5, white_set).
step(poach_egg, poach_egg_s6).
action(poach_egg_s6, lift).
object_of(poach_egg_s6, egg).
uses(poach_egg_s6, slotted_spoon).
role(poach_egg_s6, slotted_spoon, lift).
onto(poach_egg_s6, paper_towel).
purpose(poach_egg_s6, drained).
before(poach_egg_s1, poach_egg_s2).
before(poach_egg_s2, poach_egg_s3).
before(poach_egg_s3, poach_egg_s4).
before(poach_egg_s4, poach_egg_s5).
before(poach_egg_s5, poach_egg_s6).
```

## aliases
```datalog
task_alias(poach_egg, "poached egg").
task_alias(poach_egg, "poaching eggs").
task_alias(poach_egg, "poaching an egg").
alias(slotted_spoon, "slotted spoon").
alias(vinegar, "white vinegar").
alias(simmering, "a gentle simmer").
alias(white_set, "the white is set").
alias(white_holds_together, "keeps the white together").
alias(paper_towel, "kitchen paper").
```

## teach
- To poach an egg you need an egg, a saucepan of water, white vinegar, a cup, a slotted spoon and paper towel.
- Poaching an egg takes 8 to 12 minutes.
- Step 1 of poaching an egg: heat the water in the saucepan until it simmers gently, for 4 to 5 minutes.
- Step 2 of poaching an egg: add a splash of vinegar to the water to keep the white together.
- Step 3 of poaching an egg: crack the egg into a cup.
- Step 4 of poaching an egg: slide the egg from the cup into the water.
- Step 5 of poaching an egg: cook the egg for 3 to 4 minutes until the white is set.
- Step 6 of poaching an egg: lift the egg out with the slotted spoon onto paper towel to drain.

## checklist
- names what is needed, including an egg, a saucepan, vinegar and a slotted spoon
- gives the steps in order: heat to a simmer, add vinegar, crack into a cup, slide in, cook, lift out
- says the egg cooks for 3 to 4 minutes until the white is set

## variant
- change: no slotted spoon: lift the egg out with a ladle and tip the water off
- request: My slotted spoon has gone missing. How do I get a poached egg out of the pan?
- situation: I have eggs, a saucepan, water, vinegar, a cup, a soup ladle and paper towels.
- expect: The reply lifts the cooked egg out with the ladle, tipping off the water, then drains it on paper towel.

```datalog situation
has(user, egg).
has(user, saucepan).
has(user, water).
has(user, vinegar).
has(user, cup).
has(user, ladle).
has(user, paper_towel).
lacks(user, slotted_spoon).
```

```datalog difference
swap(poach_egg_s6, slotted_spoon, ladle).
extra_need(ladle).
no_need(slotted_spoon).
```

## variant
- change: wants toast: toast bread while the egg cooks
- request: I'd like my poached egg on toast. How do I time it so both are ready together?
- situation: I have eggs, a saucepan, water, vinegar, a cup, a slotted spoon, paper towels, bread and a toaster.
- expect: The reply toasts the bread while the egg is cooking so the egg goes straight onto the toast.

```datalog situation
has(user, egg).
has(user, saucepan).
has(user, water).
has(user, vinegar).
has(user, cup).
has(user, slotted_spoon).
has(user, paper_towel).
has(user, bread).
has(user, toaster).
wants(user, toast).
```

```datalog difference
extra_need(bread).
extra_need(toaster).
add_step(poach_egg_s9, poach_egg_s4).
step(poach_egg, poach_egg_s9).
action(poach_egg_s9, toast).
object_of(poach_egg_s9, bread).
uses(poach_egg_s9, toaster).
role(poach_egg_s9, toaster, heat).
overlaps(poach_egg_s9, poach_egg_s5).
```
