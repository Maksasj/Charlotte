---
id: make_salad
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make salad

Learned in conversation.

```datalog
task(make_salad). @conf 1 @source ""
needs(make_salad, lettuce). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
needs(make_salad, tomato). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
needs(make_salad, cucumber). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
needs(make_salad, olive_oil). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
needs(make_salad, lemon). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
needs(make_salad, knife). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
needs(make_salad, bowl). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
takes_time(make_salad, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
step(make_salad, make_salad_s1). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
action(make_salad_s1, wash). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
object_of(make_salad_s1, lettuce). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
takes_time(make_salad_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
step(make_salad, make_salad_s2). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
action(make_salad_s2, chop). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
object_of(make_salad_s2, tomato). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
uses(make_salad_s2, knife). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
before(make_salad_s1, make_salad_s2). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
step(make_salad, make_salad_s3). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
action(make_salad_s3, slice). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
object_of(make_salad_s3, cucumber). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
uses(make_salad_s3, knife). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
before(make_salad_s2, make_salad_s3). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
step(make_salad, make_salad_s4). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
action(make_salad_s4, mix). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
object_of(make_salad_s4, olive_oil). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
uses(make_salad_s4, lemon_juice). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
before(make_salad_s3, make_salad_s4). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
step(make_salad, make_salad_s5). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
action(make_salad_s5, toss). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
object_of(make_salad_s5, salad). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
uses(make_salad_s5, dressing). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
uses(make_salad_s5, bowl). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
before(make_salad_s4, make_salad_s5). @conf 1 @source teacher:curriculum @conv s1794998830802-558984-59
```
