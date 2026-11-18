---
id: change_bike_tire
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Change a bike tire

Take off the wheel, swap the inner tube and pump the tire up.

## ground truth
```datalog
task(change_bike_tire).
domain(change_bike_tire, repair).
goal(change_bike_tire, repaired_tire).
needs(change_bike_tire, wrench).
needs(change_bike_tire, tire_lever).
needs(change_bike_tire, inner_tube).
needs(change_bike_tire, pump).
takes_time(change_bike_tire, 20, 40, minute).
step(change_bike_tire, change_bike_tire_s1).
action(change_bike_tire_s1, remove).
object_of(change_bike_tire_s1, wheel).
uses(change_bike_tire_s1, wrench).
role(change_bike_tire_s1, wrench, loosen).
takes_time(change_bike_tire_s1, 3, 5, minute).
from(change_bike_tire_s1, bike).
step(change_bike_tire, change_bike_tire_s2).
action(change_bike_tire_s2, remove).
object_of(change_bike_tire_s2, tire).
uses(change_bike_tire_s2, tire_lever).
role(change_bike_tire_s2, tire_lever, lift).
takes_time(change_bike_tire_s2, 3, 5, minute).
from(change_bike_tire_s2, wheel).
step(change_bike_tire, change_bike_tire_s3).
action(change_bike_tire_s3, replace).
object_of(change_bike_tire_s3, inner_tube).
takes_time(change_bike_tire_s3, 3, 5, minute).
step(change_bike_tire, change_bike_tire_s4).
action(change_bike_tire_s4, fit).
object_of(change_bike_tire_s4, tire).
uses(change_bike_tire_s4, tire_lever).
role(change_bike_tire_s4, tire_lever, press).
takes_time(change_bike_tire_s4, 3, 5, minute).
onto(change_bike_tire_s4, wheel).
step(change_bike_tire, change_bike_tire_s5).
action(change_bike_tire_s5, inflate).
object_of(change_bike_tire_s5, tire).
uses(change_bike_tire_s5, pump).
role(change_bike_tire_s5, pump, inflate).
takes_time(change_bike_tire_s5, 3, 5, minute).
until(change_bike_tire_s5, firm).
step(change_bike_tire, change_bike_tire_s6).
action(change_bike_tire_s6, mount).
object_of(change_bike_tire_s6, wheel).
uses(change_bike_tire_s6, wrench).
role(change_bike_tire_s6, wrench, fasten).
takes_time(change_bike_tire_s6, 3, 5, minute).
onto(change_bike_tire_s6, bike).
before(change_bike_tire_s1, change_bike_tire_s2).
before(change_bike_tire_s2, change_bike_tire_s3).
before(change_bike_tire_s3, change_bike_tire_s4).
before(change_bike_tire_s4, change_bike_tire_s5).
before(change_bike_tire_s5, change_bike_tire_s6).
```

## aliases
```datalog
task_alias(change_bike_tire, "changing a bike tire").
task_alias(change_bike_tire, "change a bike tyre").
task_alias(change_bike_tire, "fix a flat bike tire").
task_alias(change_bike_tire, "swap the inner tube").
alias(tire, "tyre").
alias(inner_tube, "inner tube").
alias(inner_tube, "tube").
alias(tire_lever, "tire levers").
alias(tire_lever, "tyre lever").
alias(wrench, "spanner").
alias(pump, "bike pump").
alias(inflate, "pump up").
alias(remove, "take off").
alias(replace, "swap").
alias(mount, "put back on").
```
