---
id: jump_start_car
stage: tools
task: jump_start_car
schema: v3
tier: T
---
# Jump-start a car

## requests
- My car won't start, just clicks. A neighbour offered his car. How do we jump it?
- How do I jump-start a flat car battery?
- What's the right order for connecting jump leads?

## situation
- I have a set of jumper cables, and my neighbour's car is parked next to mine.

## ground truth
```datalog
task(jump_start_car).
domain(jump_start_car, repair).
goal(jump_start_car, car_starts).
needs(jump_start_car, jumper_cable).
needs(jump_start_car, second_car).
takes_time(jump_start_car, 15, 25, minute).
step(jump_start_car, jump_start_car_s1).
action(jump_start_car_s1, park).
object_of(jump_start_car_s1, second_car).
purpose(jump_start_car_s1, cables_reach).
step(jump_start_car, jump_start_car_s2).
action(jump_start_car_s2, connect).
object_of(jump_start_car_s2, jumper_cable).
onto(jump_start_car_s2, positive_terminal).
uses(jump_start_car_s2, jumper_cable).
role(jump_start_car_s2, jumper_cable, fasten).
step(jump_start_car, jump_start_car_s3).
action(jump_start_car_s3, clamp).
object_of(jump_start_car_s3, jumper_cable).
onto(jump_start_car_s3, engine_metal).
uses(jump_start_car_s3, jumper_cable).
role(jump_start_car_s3, jumper_cable, fasten).
warning(jump_start_car_s3, spark).
step(jump_start_car, jump_start_car_s4).
action(jump_start_car_s4, start).
object_of(jump_start_car_s4, second_car).
takes_time(jump_start_car_s4, 2, 3, minute).
step(jump_start_car, jump_start_car_s5).
action(jump_start_car_s5, start).
object_of(jump_start_car_s5, car).
check(jump_start_car_s5, engine_runs).
step(jump_start_car, jump_start_car_s6).
action(jump_start_car_s6, remove).
object_of(jump_start_car_s6, jumper_cable).
purpose(jump_start_car_s6, safe).
warning(jump_start_car_s6, spark).
before(jump_start_car_s1, jump_start_car_s2).
before(jump_start_car_s2, jump_start_car_s3).
before(jump_start_car_s3, jump_start_car_s4).
before(jump_start_car_s4, jump_start_car_s5).
before(jump_start_car_s5, jump_start_car_s6).
```

## aliases
```datalog
task_alias(jump_start_car, "jump-starting a car").
task_alias(jump_start_car, "jumping a flat battery").
task_alias(jump_start_car, "jump leads").
alias(jumper_cable, "jump leads").
alias(jumper_cable, "jumper cables").
alias(second_car, "the other car").
alias(positive_terminal, "the red terminals").
alias(engine_metal, "bare metal on the engine").
alias(car, "my car").
```

## teach
- To jump-start a car you need jumper cables and a second car with a working battery.
- Jump-starting a car takes 15 to 25 minutes.
- Step 1 of jump-starting a car: park the second car close enough for the cables to reach.
- Step 2 of jump-starting a car: connect the red cable to the positive terminals of both batteries.
- Step 3 of jump-starting a car: clamp the black cable to the good battery and to bare metal on the dead car's engine, away from the battery, to avoid a spark.
- Step 4 of jump-starting a car: start the second car and let it run for 2 to 3 minutes.
- Step 5 of jump-starting a car: start your car and check the engine runs.
- Step 6 of jump-starting a car: remove the cables in reverse order to be safe, avoiding sparks.

## checklist
- names what is needed: jumper cables and a second car
- gives the steps in order: park close, red to positives, black to good battery and bare metal, start helper car, start car, remove cables
- warns about sparks

## variant
- change: no second car and no booster: cannot jump-start
- request: My battery's dead, I've got jump leads but there's no other car around and nobody to help. Can I jump it?
- situation: I have jumper cables but no second car and no battery pack.
- expect: The reply says a jump-start is not possible without a second power source and suggests calling breakdown cover.

```datalog situation
has(user, jumper_cable).
lacks(user, second_car).
```

```datalog difference
cannot(no_power_source).
```

## variant
- change: no second car but a jump starter pack: clamp the pack straight on
- request: No one's around to give me a jump but I've got one of those portable battery boosters. How do I use it?
- situation: I have a charged jump starter pack; there is no other car.
- expect: The reply skips parking and starting a second car and clamps the jump starter pack to the battery instead of jumper cables to another car.

```datalog situation
has(user, jump_starter_pack).
lacks(user, second_car).
```

```datalog difference
drop(jump_start_car_s1).
drop(jump_start_car_s4).
swap(jump_start_car_s2, jumper_cable, jump_starter_pack).
swap(jump_start_car_s3, jumper_cable, jump_starter_pack).
extra_need(jump_starter_pack).
no_need(second_car).
no_need(jumper_cable).
```
