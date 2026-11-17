---
id: bake_banana_bread
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# bake banana bread

Learned in conversation.

```datalog
task(bake_banana_bread). @conf 1 @source ""
needs(bake_banana_bread, ripe_banana). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
needs(bake_banana_bread, flour). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
needs(bake_banana_bread, sugar). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
needs(bake_banana_bread, egg). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
needs(bake_banana_bread, melted_butter). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
needs(bake_banana_bread, baking_soda). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
needs(bake_banana_bread, bowl). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
needs(bake_banana_bread, fork). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
needs(bake_banana_bread, loaf_tin). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
needs(bake_banana_bread, oven). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
takes_time(bake_banana_bread, 65, 80, minute). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
step(bake_banana_bread, bake_banana_bread_s1). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
action(bake_banana_bread_s1, preheat). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
object_of(bake_banana_bread_s1, oven). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
takes_time(bake_banana_bread_s1, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
step(bake_banana_bread, bake_banana_bread_s3). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
action(bake_banana_bread_s3, mix). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
object_of(bake_banana_bread_s3, melted_butter). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
object_of(bake_banana_bread_s3, sugar). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
object_of(bake_banana_bread_s3, egg). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
object_of(bake_banana_bread_s3, flour). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
object_of(bake_banana_bread_s3, baking_soda). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
takes_time(bake_banana_bread_s3, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
before(bake_banana_bread_s1, bake_banana_bread_s3). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
step(bake_banana_bread, bake_banana_bread_s4). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
action(bake_banana_bread_s4, pour). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
object_of(bake_banana_bread_s4, batter). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
uses(bake_banana_bread_s4, loaf_tin). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
takes_time(bake_banana_bread_s4, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
before(bake_banana_bread_s3, bake_banana_bread_s4). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
step(bake_banana_bread, bake_banana_bread_s5). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
action(bake_banana_bread_s5, bake). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
object_of(bake_banana_bread_s5, banana_bread). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
uses(bake_banana_bread_s5, oven). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
until(bake_banana_bread_s5, skewer_clean). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
takes_time(bake_banana_bread_s5, 50, 60, minute). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
before(bake_banana_bread_s4, bake_banana_bread_s5). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
step(bake_banana_bread, bake_banana_bread_s2). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
action(bake_banana_bread_s2, mash). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
object_of(bake_banana_bread_s2, banana). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
uses(bake_banana_bread_s2, bowl). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
uses(bake_banana_bread_s2, fork). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
takes_time(bake_banana_bread_s2, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
overlaps(bake_banana_bread_s2, bake_banana_bread_s1). @conf 1 @source teacher:curriculum @conv s1794946297457-131028-40
uses(bake_banana_bread_s5, square_cake_tin). @conf 1 @source teacher:curriculum @conv s1794946670228-131028-324
until(bake_banana_bread_s5, clean). @conf 1 @source teacher:curriculum @conv s1794946670228-131028-324
takes_time(bake_banana_bread_s5, 35, 40, minute). @conf 1 @source teacher:curriculum @conv s1794946670228-131028-324
before(bake_banana_bread_s2, bake_banana_bread_s3). @conf 1 @source teacher:curriculum @conv s1794946807886-131028-447
```
