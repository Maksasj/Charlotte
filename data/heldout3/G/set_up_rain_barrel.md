---
id: set_up_rain_barrel
stage: household
task: set_up_rain_barrel
schema: v3
tier: G
---
# Set up a rain barrel

## requests
- I want to catch the rain off the shed roof for watering. How do I hook up a water butt to the downpipe?
- How do I fit a rain diverter and a water butt?
- Could you take me through setting up a rain barrel by the house?

## situation
- I have a water butt and stand, a diverter kit, a spirit level, a tape measure, a pencil and a hacksaw.

## ground truth
```datalog
task(set_up_rain_barrel).
domain(set_up_rain_barrel, gardening).
goal(set_up_rain_barrel, collected_rainwater).
needs(set_up_rain_barrel, water_butt_stand).
needs(set_up_rain_barrel, spirit_level).
needs(set_up_rain_barrel, water_butt).
needs(set_up_rain_barrel, tape_measure).
needs(set_up_rain_barrel, pencil).
needs(set_up_rain_barrel, hacksaw).
needs(set_up_rain_barrel, diverter).
takes_time(set_up_rain_barrel, 45, 60, minute).
step(set_up_rain_barrel, set_up_rain_barrel_s1).
action(set_up_rain_barrel_s1, level).
object_of(set_up_rain_barrel_s1, water_butt_stand).
uses(set_up_rain_barrel_s1, spirit_level).
role(set_up_rain_barrel_s1, spirit_level, measure).
check(set_up_rain_barrel_s1, stand_level).
step(set_up_rain_barrel, set_up_rain_barrel_s2).
action(set_up_rain_barrel_s2, place).
object_of(set_up_rain_barrel_s2, water_butt).
onto(set_up_rain_barrel_s2, water_butt_stand).
step(set_up_rain_barrel, set_up_rain_barrel_s3).
action(set_up_rain_barrel_s3, mark).
object_of(set_up_rain_barrel_s3, downpipe).
uses(set_up_rain_barrel_s3, tape_measure).
role(set_up_rain_barrel_s3, tape_measure, measure).
uses(set_up_rain_barrel_s3, pencil).
role(set_up_rain_barrel_s3, pencil, mark).
condition(set_up_rain_barrel_s3, mark_level_with_butt_top).
step(set_up_rain_barrel, set_up_rain_barrel_s4).
action(set_up_rain_barrel_s4, cut).
object_of(set_up_rain_barrel_s4, downpipe).
uses(set_up_rain_barrel_s4, hacksaw).
role(set_up_rain_barrel_s4, hacksaw, cut).
takes_time(set_up_rain_barrel_s4, 10, 15, minute).
step(set_up_rain_barrel, set_up_rain_barrel_s5).
action(set_up_rain_barrel_s5, fit).
object_of(set_up_rain_barrel_s5, diverter).
into(set_up_rain_barrel_s5, downpipe).
step(set_up_rain_barrel, set_up_rain_barrel_s6).
action(set_up_rain_barrel_s6, connect).
object_of(set_up_rain_barrel_s6, diverter).
onto(set_up_rain_barrel_s6, water_butt).
before(set_up_rain_barrel_s1, set_up_rain_barrel_s2).
before(set_up_rain_barrel_s2, set_up_rain_barrel_s3).
before(set_up_rain_barrel_s3, set_up_rain_barrel_s4).
before(set_up_rain_barrel_s4, set_up_rain_barrel_s5).
before(set_up_rain_barrel_s5, set_up_rain_barrel_s6).
```

## aliases
```datalog
task_alias(set_up_rain_barrel, "setting up a rain barrel").
task_alias(set_up_rain_barrel, "hook up a water butt").
task_alias(set_up_rain_barrel, "catch the rain off the roof").
alias(water_butt, "rain barrel").
alias(water_butt_stand, "stand").
alias(diverter, "rain diverter").
alias(diverter, "diverter kit").
alias(downpipe, "drainpipe").
alias(phone, "spirit level app on my phone").
alias(phone, "smartphone").
alias(stand_level, "the stand sits level").
alias(mark_level_with_butt_top, "level with the top of the butt").
alias(no_saw, "nothing to cut the downpipe").
```

## teach
- To set up a rain barrel you need a water butt and its stand, a spirit level, a tape measure, a pencil, a hacksaw and a diverter kit.
- Setting up a rain barrel takes 45 to 60 minutes.
- Step 1 of setting up a rain barrel: level the stand on firm ground, using the spirit level to check the stand sits level.
- Step 2 of setting up a rain barrel: place the water butt on the stand.
- Step 3 of setting up a rain barrel: mark the downpipe level with the top of the butt using the tape measure and pencil.
- Step 4 of setting up a rain barrel: cut the downpipe at the mark with the hacksaw, for 10 to 15 minutes.
- Step 5 of setting up a rain barrel: fit the diverter into the cut downpipe.
- Step 6 of setting up a rain barrel: connect the diverter to the water butt with its short hose.

## checklist
- names what is needed: a water butt and stand, a spirit level, a tape measure, a pencil, a hacksaw and a diverter
- gives the steps in order: level stand, place butt, mark pipe, cut pipe, fit diverter, connect
- says the mark goes level with the top of the butt
- gives a total time of about 45 to 60 minutes

## variant
- change: hacksaw broken and no other saw: the downpipe cannot be cut, so it cannot be done today
- request: Halfway through fitting my water butt the hacksaw blade snapped, and I've no spare or other saw. Can I still finish today?
- situation: My hacksaw is broken. I have the water butt and stand, a diverter kit, a spirit level, a tape measure and a pencil, and nothing else to cut the downpipe.
- expect: The reply says the diverter cannot be fitted without cutting the downpipe, so it cannot be finished until a new blade or saw is found; the stand and butt can be set up now.

```datalog situation
lacks(user, hacksaw).
has(user, water_butt).
has(user, water_butt_stand).
has(user, diverter).
has(user, spirit_level).
has(user, tape_measure).
has(user, pencil).
```

```datalog difference
cannot(no_saw).
```

## variant
- change: no spirit level: use a level app on a phone
- request: I've no spirit level to check the water butt stand. Is a level app on my phone good enough?
- situation: I have the water butt and stand, a diverter kit, a tape measure, a pencil, a hacksaw and a smartphone, but no spirit level.
- expect: The reply checks the stand with the phone's level app instead of a spirit level and keeps the other steps.

```datalog situation
lacks(user, spirit_level).
has(user, phone).
has(user, water_butt).
has(user, water_butt_stand).
has(user, diverter).
has(user, tape_measure).
has(user, pencil).
has(user, hacksaw).
```

```datalog difference
no_need(spirit_level).
extra_need(phone).
swap(set_up_rain_barrel_s1, spirit_level, phone).
```
