---
id: make_banana_bread
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make banana bread

Learned in conversation.

```datalog
task(make_banana_bread). @conf 1 @source ""
needs(make_banana_bread, ripe_banana). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
needs(make_banana_bread, flour). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
needs(make_banana_bread, sugar). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
needs(make_banana_bread, egg). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
needs(make_banana_bread, butter). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
needs(make_banana_bread, baking_soda). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
needs(make_banana_bread, bowl). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
needs(make_banana_bread, fork). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
needs(make_banana_bread, loaf_tin). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
needs(make_banana_bread, oven). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
takes_time(make_banana_bread, 65, 80, minute). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
step(make_banana_bread, make_banana_bread_s1). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
action(make_banana_bread_s1, preheat). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
object_of(make_banana_bread_s1, oven). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
takes_time(make_banana_bread_s1, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
step(make_banana_bread, make_banana_bread_s3). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
action(make_banana_bread_s3, mix). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
object_of(make_banana_bread_s3, butter). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
object_of(make_banana_bread_s3, sugar). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
object_of(make_banana_bread_s3, egg). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
object_of(make_banana_bread_s3, flour). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
object_of(make_banana_bread_s3, baking_soda). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
takes_time(make_banana_bread_s3, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
before(make_banana_bread_s1, make_banana_bread_s3). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
step(make_banana_bread, make_banana_bread_s4). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
action(make_banana_bread_s4, pour). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
object_of(make_banana_bread_s4, batter). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
uses(make_banana_bread_s4, loaf_tin). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
takes_time(make_banana_bread_s4, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
before(make_banana_bread_s3, make_banana_bread_s4). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
step(make_banana_bread, make_banana_bread_s5). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
action(make_banana_bread_s5, bake). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
object_of(make_banana_bread_s5, banana_bread). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
uses(make_banana_bread_s5, oven). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
until(make_banana_bread_s5, clean). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
takes_time(make_banana_bread_s5, 50, 60, minute). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
before(make_banana_bread_s4, make_banana_bread_s5). @conf 1 @source teacher:curriculum @conv s1794982364879-336211-57
```
