---
id: scrambled_eggs
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Scrambled eggs

Whisk eggs and cook them gently in butter until just set.

```datalog
task(scrambled_eggs).
solves(scrambled_eggs, plan_meal).
needs(scrambled_eggs, egg).
needs(scrambled_eggs, butter).
needs(scrambled_eggs, salt).
needs(scrambled_eggs, pan).
needs(scrambled_eggs, bowl).
needs(scrambled_eggs, fork).
takes_time(scrambled_eggs, 6, 10, minute).
step(scrambled_eggs, scrambled_eggs_s1).
action(scrambled_eggs_s1, crack).
object_of(scrambled_eggs_s1, egg).
uses(scrambled_eggs_s1, bowl).
takes_time(scrambled_eggs_s1, 1, 1, minute).
step(scrambled_eggs, scrambled_eggs_s2).
action(scrambled_eggs_s2, whisk).
object_of(scrambled_eggs_s2, egg).
uses(scrambled_eggs_s2, fork).
uses(scrambled_eggs_s2, salt).
takes_time(scrambled_eggs_s2, 1, 1, minute).
step(scrambled_eggs, scrambled_eggs_s3).
action(scrambled_eggs_s3, melt).
object_of(scrambled_eggs_s3, butter).
uses(scrambled_eggs_s3, pan).
takes_time(scrambled_eggs_s3, 1, 2, minute).
step(scrambled_eggs, scrambled_eggs_s4).
action(scrambled_eggs_s4, cook).
object_of(scrambled_eggs_s4, egg).
uses(scrambled_eggs_s4, pan).
takes_time(scrambled_eggs_s4, 3, 4, minute).
until(scrambled_eggs_s4, set).
step(scrambled_eggs, scrambled_eggs_s5).
action(scrambled_eggs_s5, serve).
object_of(scrambled_eggs_s5, egg).
before(scrambled_eggs_s1, scrambled_eggs_s2).
overlaps(scrambled_eggs_s3, scrambled_eggs_s2).
before(scrambled_eggs_s2, scrambled_eggs_s4).
before(scrambled_eggs_s3, scrambled_eggs_s4).
before(scrambled_eggs_s4, scrambled_eggs_s5).
```
