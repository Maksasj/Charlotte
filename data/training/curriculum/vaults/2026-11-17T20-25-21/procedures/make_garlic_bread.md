---
id: make_garlic_bread
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make garlic bread

Learned in conversation.

```datalog
task(make_garlic_bread). @conf 1 @source ""
needs(make_garlic_bread, baguette). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
needs(make_garlic_bread, butter). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
needs(make_garlic_bread, garlic). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
needs(make_garlic_bread, knife). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
needs(make_garlic_bread, foil). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
needs(make_garlic_bread, oven). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
takes_time(make_garlic_bread, 23, 28, minute). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
step(make_garlic_bread, make_garlic_bread_s1). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
action(make_garlic_bread_s1, preheat). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
object_of(make_garlic_bread_s1, oven). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
takes_time(make_garlic_bread_s1, 10, 12, minute). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
step(make_garlic_bread, make_garlic_bread_s2). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
action(make_garlic_bread_s2, mix). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
object_of(make_garlic_bread_s2, butter). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
uses(make_garlic_bread_s2, garlic). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
takes_time(make_garlic_bread_s2, 3, 4, minute). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
overlaps(make_garlic_bread_s2, make_garlic_bread_s1). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
step(make_garlic_bread, make_garlic_bread_s3). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
action(make_garlic_bread_s3, cut). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
object_of(make_garlic_bread_s3, baguette). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
uses(make_garlic_bread_s3, knife). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
takes_time(make_garlic_bread_s3, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
step(make_garlic_bread, make_garlic_bread_s4). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
action(make_garlic_bread_s4, spread). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
object_of(make_garlic_bread_s4, garlic_butter). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
before(make_garlic_bread_s3, make_garlic_bread_s4). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
before(make_garlic_bread_s2, make_garlic_bread_s3). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
before(make_garlic_bread_s1, make_garlic_bread_s3). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
action(make_garlic_bread_s4, wrap). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
object_of(make_garlic_bread_s4, baguette). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
uses(make_garlic_bread_s4, foil). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
takes_time(make_garlic_bread_s4, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
step(make_garlic_bread, make_garlic_bread_s5). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
action(make_garlic_bread_s5, bake). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
object_of(make_garlic_bread_s5, baguette). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
uses(make_garlic_bread_s5, oven). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
until(make_garlic_bread_s5, crisp). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
takes_time(make_garlic_bread_s5, 10, 12, minute). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
before(make_garlic_bread_s4, make_garlic_bread_s5). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
takes_time(make_garlic_bread_s4, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946311410-131028-57
action(make_garlic_bread_s3, spread). @conf 1 @source teacher:curriculum @conv s1794946664488-131028-319
object_of(make_garlic_bread_s3, garlic_butter). @conf 1 @source teacher:curriculum @conv s1794946664488-131028-319
```
