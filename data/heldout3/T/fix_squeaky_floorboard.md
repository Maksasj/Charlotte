---
id: fix_squeaky_floorboard
stage: tools
task: fix_squeaky_floorboard
schema: v3
tier: T
---
# Fix a squeaky floorboard

## requests
- There's a floorboard in the hallway that creaks every time I step on it. How do I stop it?
- How do you fix a squeaky floorboard?
- My bedroom floor squeaks at night and wakes the baby. What can I do about the board?

## situation
- I have a stud finder, a cordless drill and some wood screws. The floorboards are bare.

## ground truth
```datalog
task(fix_squeaky_floorboard).
domain(fix_squeaky_floorboard, repair).
goal(fix_squeaky_floorboard, no_squeak).
needs(fix_squeaky_floorboard, stud_finder).
needs(fix_squeaky_floorboard, drill).
needs(fix_squeaky_floorboard, screw).
takes_time(fix_squeaky_floorboard, 20, 30, minute).
step(fix_squeaky_floorboard, fix_squeaky_floorboard_s1).
action(fix_squeaky_floorboard_s1, locate).
object_of(fix_squeaky_floorboard_s1, squeak).
purpose(fix_squeaky_floorboard_s1, board_found).
step(fix_squeaky_floorboard, fix_squeaky_floorboard_s2).
action(fix_squeaky_floorboard_s2, find).
object_of(fix_squeaky_floorboard_s2, joist).
uses(fix_squeaky_floorboard_s2, stud_finder).
role(fix_squeaky_floorboard_s2, stud_finder, measure).
step(fix_squeaky_floorboard, fix_squeaky_floorboard_s3).
action(fix_squeaky_floorboard_s3, drill).
object_of(fix_squeaky_floorboard_s3, pilot_hole).
uses(fix_squeaky_floorboard_s3, drill).
role(fix_squeaky_floorboard_s3, drill, pierce).
into(fix_squeaky_floorboard_s3, joist).
step(fix_squeaky_floorboard, fix_squeaky_floorboard_s4).
action(fix_squeaky_floorboard_s4, drive).
object_of(fix_squeaky_floorboard_s4, screw).
into(fix_squeaky_floorboard_s4, joist).
uses(fix_squeaky_floorboard_s4, drill).
role(fix_squeaky_floorboard_s4, drill, fasten).
step(fix_squeaky_floorboard, fix_squeaky_floorboard_s5).
action(fix_squeaky_floorboard_s5, check).
object_of(fix_squeaky_floorboard_s5, floorboard).
check(fix_squeaky_floorboard_s5, no_squeak).
before(fix_squeaky_floorboard_s1, fix_squeaky_floorboard_s2).
before(fix_squeaky_floorboard_s2, fix_squeaky_floorboard_s3).
before(fix_squeaky_floorboard_s3, fix_squeaky_floorboard_s4).
before(fix_squeaky_floorboard_s4, fix_squeaky_floorboard_s5).
```

## aliases
```datalog
task_alias(fix_squeaky_floorboard, "fixing a squeaky floorboard").
task_alias(fix_squeaky_floorboard, "stopping a creaky floor").
task_alias(fix_squeaky_floorboard, "silencing a creaking board").
alias(squeak, "the creaky spot").
alias(joist, "the joist underneath").
alias(drill, "cordless drill").
alias(screw, "wood screws").
alias(pilot_hole, "a small hole").
alias(no_squeak, "it no longer squeaks").
```

## teach
- To fix a squeaky floorboard you need a stud finder, a drill and wood screws.
- Fixing a squeaky floorboard takes 20 to 30 minutes.
- Step 1 of fixing a squeaky floorboard: walk over the floor to locate the squeak and find the board.
- Step 2 of fixing a squeaky floorboard: find the joist underneath with the stud finder.
- Step 3 of fixing a squeaky floorboard: drill a small pilot hole through the board into the joist.
- Step 4 of fixing a squeaky floorboard: drive a screw into the joist with the drill.
- Step 5 of fixing a squeaky floorboard: walk on it again and check it no longer squeaks.

## checklist
- names what is needed: a stud finder, a drill and screws
- gives the steps in order: locate squeak, find joist, pilot hole, screw down, check

## variant
- change: no drill: knock a nail through for the pilot hole and drive the screw with a screwdriver
- request: My drill's battery is dead and I've no charger. Can I still screw down the creaky board?
- situation: I have a stud finder, wood screws, a hammer, a nail and a screwdriver.
- expect: The reply makes the pilot hole by tapping in a nail with the hammer and drives the screw in with a screwdriver instead of the drill.

```datalog situation
has(user, stud_finder).
has(user, screw).
has(user, hammer).
has(user, nail).
has(user, screwdriver).
lacks(user, drill).
```

```datalog difference
swap(fix_squeaky_floorboard_s3, drill, nail).
swap(fix_squeaky_floorboard_s4, drill, screwdriver).
extra_need(hammer).
extra_need(nail).
extra_need(screwdriver).
no_need(drill).
```

## variant
- change: renting, no holes: dust talcum powder into the joints instead
- request: I rent my flat so I can't put screws in the floor. Any way to quiet a squeaky board?
- situation: I have talcum powder and a soft brush. I can't make holes in the floor.
- expect: The reply leaves out the joist, drilling and screwing steps and works talcum powder into the gaps around the board with the brush.

```datalog situation
has(user, talcum_powder).
has(user, brush).
excludes(user, hole).
```

```datalog difference
drop(fix_squeaky_floorboard_s2).
drop(fix_squeaky_floorboard_s3).
drop(fix_squeaky_floorboard_s4).
no_need(stud_finder).
no_need(drill).
no_need(screw).
extra_need(talcum_powder).
extra_need(brush).
add_step(fix_squeaky_floorboard_s9, fix_squeaky_floorboard_s1).
step(fix_squeaky_floorboard, fix_squeaky_floorboard_s9).
action(fix_squeaky_floorboard_s9, sprinkle).
object_of(fix_squeaky_floorboard_s9, talcum_powder).
into(fix_squeaky_floorboard_s9, gap).
uses(fix_squeaky_floorboard_s9, brush).
role(fix_squeaky_floorboard_s9, brush, spread).
purpose(fix_squeaky_floorboard_s9, less_friction).
before(fix_squeaky_floorboard_s1, fix_squeaky_floorboard_s9).
before(fix_squeaky_floorboard_s9, fix_squeaky_floorboard_s5).
```
