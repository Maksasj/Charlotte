---
id: floss_teeth
stage: procedures
task: floss_teeth
schema: v3
---
# Floss your teeth

## requests
- How do I floss my teeth properly?
- What's the right way to use dental floss?
- Can you walk me through flossing between my teeth?

## situation
- I have dental floss, scissors, water and a mirror.
- I'm getting ready for bed.

## ground truth
```datalog
task(floss_teeth).
domain(floss_teeth, personal_care).
goal(floss_teeth, clean_teeth).
needs(floss_teeth, dental_floss).
needs(floss_teeth, scissors).
needs(floss_teeth, water).
optional_need(floss_teeth, mirror).
takes_time(floss_teeth, 5, 10, minute).
step(floss_teeth, floss_teeth_s1).
action(floss_teeth_s1, cut).
object_of(floss_teeth_s1, dental_floss).
uses(floss_teeth_s1, scissors).
role(floss_teeth_s1, scissors, cut).
step(floss_teeth, floss_teeth_s2).
action(floss_teeth_s2, guide).
object_of(floss_teeth_s2, dental_floss).
into(floss_teeth_s2, gumline).
purpose(floss_teeth_s2, reach_gumline).
step(floss_teeth, floss_teeth_s3).
action(floss_teeth_s3, slide).
object_of(floss_teeth_s3, dental_floss).
purpose(floss_teeth_s3, remove_plaque).
warning(floss_teeth_s3, gum_damage).
step(floss_teeth, floss_teeth_s4).
action(floss_teeth_s4, rinse).
object_of(floss_teeth_s4, mouth).
uses(floss_teeth_s4, water).
role(floss_teeth_s4, water, clean).
before(floss_teeth_s1, floss_teeth_s2).
before(floss_teeth_s2, floss_teeth_s3).
before(floss_teeth_s3, floss_teeth_s4).
```

## aliases
```datalog
task_alias(floss_teeth, "flossing").
task_alias(floss_teeth, "flossing your teeth").
alias(dental_floss, "floss").
alias(gum_damage, "snapping into gums").
alias(remove_plaque, "remove plaque").
alias(reach_gumline, "reach the gumline").
```

## teach
- To floss your teeth you need dental floss, scissors and water.
- Flossing your teeth takes 5 to 10 minutes.
- Step 1 of flossing your teeth: cut a length of dental floss with the scissors.
- Step 2 of flossing your teeth: guide the floss between two teeth down to the gumline.
- Step 3 of flossing your teeth: slide the floss up and down each side of the tooth to remove plaque, without snapping it into the gums.
- Step 4 of flossing your teeth: rinse your mouth with water.

## checklist
- names what is needed, including dental floss, scissors and water
- gives the steps in order: cut the floss, guide it between the teeth, slide it along each side, rinse
- warns not to snap the floss into the gums
- gives a total time of about 5 to 10 minutes

## variant
- change: no scissors, so use nail clippers to cut the floss
- request: I want to floss but I don't have scissors. Can I use something else?
- situation: I have dental floss, nail clippers, water and a mirror, but no scissors.
- expect: The reply has the user cut the floss with nail clippers instead of scissors, then follows the same flossing steps.

```datalog situation
lacks(user, scissors).
has(user, nail_clippers).
has(user, dental_floss).
has(user, water).
has(user, mirror).
```

```datalog difference
swap(floss_teeth_s1, scissors, nail_clippers).
no_need(scissors).
extra_need(nail_clippers).
```

## variant
- change: add a mouthwash rinse after the water rinse
- request: I'd like to floss and then use mouthwash. Walk me through the whole thing.
- situation: I have dental floss, scissors, water, a mirror and mouthwash.
- expect: The reply gives the usual flossing steps and adds a mouthwash rinse after the water rinse.

```datalog situation
has(user, dental_floss).
has(user, scissors).
has(user, water).
has(user, mirror).
has(user, mouthwash).
wants(user, mouthwash).
```

```datalog difference
extra_need(mouthwash).
add_step(floss_teeth_s9, floss_teeth_s4).
step(floss_teeth, floss_teeth_s9).
action(floss_teeth_s9, rinse).
object_of(floss_teeth_s9, mouth).
uses(floss_teeth_s9, mouthwash).
role(floss_teeth_s9, mouthwash, clean).
before(floss_teeth_s4, floss_teeth_s9).
```
