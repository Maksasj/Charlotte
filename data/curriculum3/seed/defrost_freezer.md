---
id: defrost_freezer
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Defrost the freezer

Empty and unplug the freezer, let the ice melt and wipe it dry.

## ground truth
```datalog
task(defrost_freezer).
domain(defrost_freezer, cleaning).
goal(defrost_freezer, defrosted_freezer).
needs(defrost_freezer, cooler_box).
needs(defrost_freezer, towel).
needs(defrost_freezer, bowl).
needs(defrost_freezer, cloth).
takes_time(defrost_freezer, 2, 4, hour).
step(defrost_freezer, defrost_freezer_s1).
action(defrost_freezer_s1, unplug).
object_of(defrost_freezer_s1, freezer).
step(defrost_freezer, defrost_freezer_s2).
action(defrost_freezer_s2, move).
object_of(defrost_freezer_s2, food).
uses(defrost_freezer_s2, cooler_box).
role(defrost_freezer_s2, cooler_box, contain).
takes_time(defrost_freezer_s2, 10, 15, minute).
from(defrost_freezer_s2, freezer).
into(defrost_freezer_s2, cooler_box).
step(defrost_freezer, defrost_freezer_s3).
action(defrost_freezer_s3, place).
object_of(defrost_freezer_s3, bowl).
uses(defrost_freezer_s3, towel).
role(defrost_freezer_s3, towel, protect).
takes_time(defrost_freezer_s3, 5, 5, minute).
purpose(defrost_freezer_s3, catch_meltwater).
step(defrost_freezer, defrost_freezer_s4).
action(defrost_freezer_s4, thaw).
object_of(defrost_freezer_s4, ice).
takes_time(defrost_freezer_s4, 1, 3, hour).
until(defrost_freezer_s4, melted).
step(defrost_freezer, defrost_freezer_s5).
action(defrost_freezer_s5, wipe).
object_of(defrost_freezer_s5, freezer).
uses(defrost_freezer_s5, cloth).
role(defrost_freezer_s5, cloth, wipe).
takes_time(defrost_freezer_s5, 10, 15, minute).
until(defrost_freezer_s5, dry).
step(defrost_freezer, defrost_freezer_s6).
action(defrost_freezer_s6, return).
object_of(defrost_freezer_s6, food).
takes_time(defrost_freezer_s6, 5, 10, minute).
from(defrost_freezer_s6, cooler_box).
into(defrost_freezer_s6, freezer).
before(defrost_freezer_s1, defrost_freezer_s2).
before(defrost_freezer_s2, defrost_freezer_s3).
before(defrost_freezer_s3, defrost_freezer_s4).
before(defrost_freezer_s4, defrost_freezer_s5).
before(defrost_freezer_s5, defrost_freezer_s6).
```

## aliases
```datalog
task_alias(defrost_freezer, "defrosting the freezer").
task_alias(defrost_freezer, "defrost a freezer").
task_alias(defrost_freezer, "defrost the freezer").
alias(cooler_box, "cool box").
alias(cooler_box, "cooler").
alias(food, "frozen food").
alias(ice, "frost").
alias(thaw, "melt").
alias(towel, "towels").
alias(return, "put back").
alias(move, "empty").
alias(catch_meltwater, "catch the water").
```
