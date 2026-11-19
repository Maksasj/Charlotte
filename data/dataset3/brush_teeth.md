---
id: brush_teeth
stage: procedures
task: brush_teeth
schema: v3
---
# Brush your teeth

## requests
- How do I brush my teeth properly?
- What's the right way to clean my teeth with a toothbrush?
- Can you walk me through brushing my teeth step by step?

## situation
- I have a toothbrush, toothpaste, a cup and tap water in the bathroom.
- I'm getting ready for bed.

## ground truth
```datalog
task(brush_teeth).
domain(brush_teeth, personal_care).
goal(brush_teeth, clean_teeth).
needs(brush_teeth, toothbrush).
needs(brush_teeth, toothpaste).
needs(brush_teeth, water).
needs(brush_teeth, cup).
takes_time(brush_teeth, 3, 5, minute).
step(brush_teeth, brush_teeth_s1).
action(brush_teeth_s1, wet).
object_of(brush_teeth_s1, toothbrush).
uses(brush_teeth_s1, water).
role(brush_teeth_s1, water, wet).
step(brush_teeth, brush_teeth_s2).
action(brush_teeth_s2, spread).
object_of(brush_teeth_s2, toothpaste).
onto(brush_teeth_s2, toothbrush).
uses(brush_teeth_s2, toothpaste).
role(brush_teeth_s2, toothpaste, spread).
amount(brush_teeth_s2, toothpaste, 1, squirt).
step(brush_teeth, brush_teeth_s3).
action(brush_teeth_s3, brush).
object_of(brush_teeth_s3, teeth).
uses(brush_teeth_s3, toothbrush).
role(brush_teeth_s3, toothbrush, scrub).
takes_time(brush_teeth_s3, 2, 3, minute).
purpose(brush_teeth_s3, remove_plaque).
warning(brush_teeth_s3, hard_scrub).
step(brush_teeth, brush_teeth_s4).
action(brush_teeth_s4, rinse).
object_of(brush_teeth_s4, mouth).
uses(brush_teeth_s4, cup).
role(brush_teeth_s4, cup, contain).
uses(brush_teeth_s4, water).
role(brush_teeth_s4, water, clean).
step(brush_teeth, brush_teeth_s5).
action(brush_teeth_s5, rinse).
object_of(brush_teeth_s5, toothbrush).
uses(brush_teeth_s5, water).
role(brush_teeth_s5, water, clean).
before(brush_teeth_s1, brush_teeth_s2).
before(brush_teeth_s2, brush_teeth_s3).
before(brush_teeth_s3, brush_teeth_s4).
before(brush_teeth_s4, brush_teeth_s5).
```

## aliases
```datalog
task_alias(brush_teeth, "brushing teeth").
task_alias(brush_teeth, "cleaning teeth").
alias(toothbrush, "toothbrush").
alias(toothpaste, "paste").
alias(water, "tap water").
alias(teeth, "teeth").
alias(mouth, "mouth").
alias(remove_plaque, "plaque").
alias(hard_scrub, "scrubbing too hard").
```

## teach
- To brush your teeth you need a toothbrush, toothpaste, water and a cup.
- Brushing your teeth takes 3 to 5 minutes.
- Step 1 of brushing your teeth: wet the toothbrush under the tap water.
- Step 2 of brushing your teeth: squeeze one squirt of toothpaste onto the toothbrush.
- Step 3 of brushing your teeth: brush your teeth gently for 2 to 3 minutes to remove plaque.
- Step 4 of brushing your teeth: rinse your mouth with water from the cup.
- Step 5 of brushing your teeth: rinse the toothbrush under the water.

## checklist
- names what is needed, including a toothbrush, toothpaste, water and a cup
- gives the steps in order: wet the brush, add paste, brush gently, rinse mouth, rinse brush
- says the brushing should be gentle, not hard
- gives a total time of about 3 to 5 minutes

## variant
- change: no toothpaste: brush with water only, leaving out the paste step
- request: I'm out of toothpaste tonight. Can I still brush my teeth?
- situation: I have a toothbrush, a cup and tap water, but no toothpaste.
- expect: The reply says yes: wet the brush, brush gently for 2 to 3 minutes with water only, then rinse the mouth and the brush, and it skips the paste step.

```datalog situation
lacks(user, toothpaste).
has(user, toothbrush).
has(user, cup).
has(user, water).
```

```datalog difference
no_need(toothpaste).
drop(brush_teeth_s2).
change(brush_teeth, takes_time, 3_5, 2_3).
```

## variant
- change: only 2 minutes: keep all steps but brush for 1 to 2 minutes
- request: I only have 2 minutes before bed. How should I brush my teeth?
- situation: I have a toothbrush, toothpaste, a cup and tap water, and 2 minutes.
- expect: The reply keeps all five steps but shortens the brushing to 1 to 2 minutes, so the whole routine takes about 2 to 3 minutes.

```datalog situation
has(user, toothbrush).
has(user, toothpaste).
has(user, cup).
has(user, water).
time_limit(user, 2, minute).
```

```datalog difference
change(brush_teeth, takes_time, 3_5, 2_3).
change(brush_teeth_s3, takes_time, 2_3, 1_2).
```
