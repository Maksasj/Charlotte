---
id: bake_potato
status: hypothesis
source: user
uses: 0
successes: 0
---
# bake potato

Learned in conversation.

```datalog
task(bake_potato). @conf 1 @source ""
step(bake_potato, bake_potato_s1). @conf 1 @source user @conv s1794998488868-527283-3
action(bake_potato_s1, bake). @conf 1 @source user @conv s1794998488868-527283-3
object_of(bake_potato_s1, potato). @conf 1 @source user @conv s1794998488868-527283-3
recommended_for(bake_potato, dinner). @conf 1 @source user @conv s1794998488868-527283-3
needs(bake_potato, potato). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
needs(bake_potato, olive_oil). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
needs(bake_potato, salt). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
needs(bake_potato, fork). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
needs(bake_potato, baking_tray). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
needs(bake_potato, oven). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
takes_time(bake_potato, 60, 70, minute). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
action(bake_potato_s1, heat). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
object_of(bake_potato_s1, oven). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
takes_time(bake_potato_s1, 10, 10, minute). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
action(bake_potato_s1, wash). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
overlaps(bake_potato_s1, bake_potato_s0). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
step(bake_potato, bake_potato_s3). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
action(bake_potato_s3, prick). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
object_of(bake_potato_s3, potato). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
uses(bake_potato_s3, fork). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
overlaps(bake_potato_s3, bake_potato_s1). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
step(bake_potato, bake_potato_s4). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
action(bake_potato_s4, rub). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
object_of(bake_potato_s4, potato). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
uses(bake_potato_s4, olive_oil). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
uses(bake_potato_s4, salt). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
overlaps(bake_potato_s4, bake_potato_s1). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
step(bake_potato, bake_potato_s5). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
action(bake_potato_s5, bake). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
object_of(bake_potato_s5, potato). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
uses(bake_potato_s5, baking_tray). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
until(bake_potato_s5, soft). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
takes_time(bake_potato_s5, 50, 60, minute). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
before(bake_potato_s4, bake_potato_s5). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
before(bake_potato_s1, bake_potato_s5). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
uses(bake_potato_s1, oven). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
overlaps(bake_potato_s1, bake_potato_s2). @conf 1 @source teacher:curriculum @conv s1794998497016-527283-44
```
