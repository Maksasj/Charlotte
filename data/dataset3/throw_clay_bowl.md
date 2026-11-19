---
id: throw_clay_bowl
stage: procedures
task: throw_clay_bowl
schema: v3
---
# Throw a clay bowl

## requests
- How do I throw a bowl on a potter's wheel?
- Can you explain the steps for making a small clay bowl at the wheel?
- What's the process for shaping a bowl out of clay on a wheel?

## situation
- I have a bag of clay, a potter's wheel, water and a cutting wire.
- I have about an hour in the studio.

## ground truth
```datalog
task(throw_clay_bowl).
domain(throw_clay_bowl, crafts).
goal(throw_clay_bowl, clay_bowl).
needs(throw_clay_bowl, clay).
needs(throw_clay_bowl, wheel).
needs(throw_clay_bowl, water).
needs(throw_clay_bowl, wire).
takes_time(throw_clay_bowl, 30, 45, minute).
step(throw_clay_bowl, throw_clay_bowl_s1).
action(throw_clay_bowl_s1, wedge).
object_of(throw_clay_bowl_s1, clay).
purpose(throw_clay_bowl_s1, air_bubbles_removed).
step(throw_clay_bowl, throw_clay_bowl_s2).
action(throw_clay_bowl_s2, centre).
object_of(throw_clay_bowl_s2, clay).
uses(throw_clay_bowl_s2, wheel).
role(throw_clay_bowl_s2, wheel, hold).
step(throw_clay_bowl, throw_clay_bowl_s3).
action(throw_clay_bowl_s3, open).
object_of(throw_clay_bowl_s3, clay).
uses(throw_clay_bowl_s3, water).
role(throw_clay_bowl_s3, water, wet).
warning(throw_clay_bowl_s3, too_much_water).
step(throw_clay_bowl, throw_clay_bowl_s4).
action(throw_clay_bowl_s4, pull).
object_of(throw_clay_bowl_s4, clay).
check(throw_clay_bowl_s4, walls_even).
step(throw_clay_bowl, throw_clay_bowl_s5).
action(throw_clay_bowl_s5, cut).
object_of(throw_clay_bowl_s5, bowl).
uses(throw_clay_bowl_s5, wire).
role(throw_clay_bowl_s5, wire, cut).
before(throw_clay_bowl_s1, throw_clay_bowl_s2).
before(throw_clay_bowl_s2, throw_clay_bowl_s3).
before(throw_clay_bowl_s3, throw_clay_bowl_s4).
before(throw_clay_bowl_s4, throw_clay_bowl_s5).
```

## aliases
```datalog
task_alias(throw_clay_bowl, "throwing a clay bowl").
task_alias(throw_clay_bowl, "making a clay bowl").
task_alias(throw_clay_bowl, "pottery bowl").
alias(clay_bowl, "clay bowl").
alias(wheel, "potter's wheel").
alias(wire, "cutting wire").
alias(wedge, "knead").
alias(centre, "center").
alias(open, "open the centre").
alias(air_bubbles_removed, "no air bubbles").
alias(walls_even, "even walls").
```

## teach
- To throw a clay bowl you need clay, a potter's wheel, water and a cutting wire.
- Throwing a clay bowl takes 30 to 45 minutes.
- Step 1 of throwing a clay bowl: wedge the clay to remove air bubbles.
- Step 2 of throwing a clay bowl: centre the clay on the potter's wheel.
- Step 3 of throwing a clay bowl: open the centre with water, but not too much water.
- Step 4 of throwing a clay bowl: pull the walls up until they are even.
- Step 5 of throwing a clay bowl: cut the bowl off the wheel with the cutting wire.

## checklist
- names what is needed, including clay, a potter's wheel, water and a cutting wire
- gives the steps in order: wedge, centre, open, pull the walls, cut off
- warns against using too much water when opening the centre
- gives a total time of about 30 to 45 minutes

## variant
- change: no cutting wire, use a kitchen knife to cut the bowl off the wheel
- request: I don't have a cutting wire. Can I use something else to take the bowl off the wheel?
- situation: I have clay, a potter's wheel, water and a kitchen knife.
- expect: The reply replaces the cutting wire with the kitchen knife in step 5 and keeps the other steps the same.

```datalog situation
lacks(user, wire).
has(user, knife).
```

```datalog difference
swap(throw_clay_bowl_s5, wire, knife).
extra_need(knife).
no_need(wire).
```

- change: only 20 minutes: keep all five steps but shorten the total time
- request: I only have about 20 minutes. Can I still throw a bowl?
- situation: I have clay, a potter's wheel, water and a cutting wire, and 20 minutes.
- expect: The reply keeps all five steps and says the total time is about 15 to 20 minutes, so each step has to go faster.

```datalog situation
has(user, clay).
has(user, wheel).
has(user, water).
has(user, wire).
time_limit(user, 20, minute).
```

```datalog difference
change(throw_clay_bowl, takes_time, 30_45, 15_20).
```
