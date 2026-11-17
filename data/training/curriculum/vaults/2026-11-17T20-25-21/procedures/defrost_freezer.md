---
id: defrost_freezer
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Defrost the freezer

Empty and unplug the freezer, let the ice melt and wipe it dry.

```datalog
task(defrost_freezer).
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
takes_time(defrost_freezer_s2, 10, 15, minute).
step(defrost_freezer, defrost_freezer_s3).
action(defrost_freezer_s3, place).
object_of(defrost_freezer_s3, bowl).
uses(defrost_freezer_s3, towel).
takes_time(defrost_freezer_s3, 5, 5, minute).
step(defrost_freezer, defrost_freezer_s4).
action(defrost_freezer_s4, thaw).
object_of(defrost_freezer_s4, ice).
takes_time(defrost_freezer_s4, 1, 3, hour).
until(defrost_freezer_s4, melted).
step(defrost_freezer, defrost_freezer_s5).
action(defrost_freezer_s5, wipe).
object_of(defrost_freezer_s5, freezer).
uses(defrost_freezer_s5, cloth).
takes_time(defrost_freezer_s5, 10, 15, minute).
until(defrost_freezer_s5, dry).
step(defrost_freezer, defrost_freezer_s6).
action(defrost_freezer_s6, return).
object_of(defrost_freezer_s6, food).
takes_time(defrost_freezer_s6, 5, 10, minute).
before(defrost_freezer_s1, defrost_freezer_s2).
before(defrost_freezer_s2, defrost_freezer_s3).
before(defrost_freezer_s3, defrost_freezer_s4).
before(defrost_freezer_s4, defrost_freezer_s5).
before(defrost_freezer_s5, defrost_freezer_s6).
```
