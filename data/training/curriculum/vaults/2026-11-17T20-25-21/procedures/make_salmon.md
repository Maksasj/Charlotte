---
id: make_salmon
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make salmon

Learned in conversation.

```datalog
task(make_salmon). @conf 1 @source ""
needs(make_salmon, salmon_fillet). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
needs(make_salmon, oil). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
needs(make_salmon, salt). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
needs(make_salmon, pepper). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
needs(make_salmon, lemon). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
needs(make_salmon, paper_towel). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
needs(make_salmon, frying_pan). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
needs(make_salmon, spatula). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
takes_time(make_salmon, 12, 16, minute). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
action(make_salmon_s1, pan_fry). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
object_of(make_salmon_s1, salmon). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
takes_time(make_salmon_s1, 12, 16, minute). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
step(make_salmon, make_salmon_s1). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
action(make_salmon_s1, pat_dry). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
object_of(make_salmon_s1, salmon_fillet). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
uses(make_salmon_s1, paper_towel). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
action(make_salmon_s1, season). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
uses(make_salmon_s1, salt). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
uses(make_salmon_s1, pepper). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
takes_time(make_salmon_s1, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
step(make_salmon, make_salmon_s2). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
action(make_salmon_s2, heat). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
object_of(make_salmon_s2, oil). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
uses(make_salmon_s2, frying_pan). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
takes_time(make_salmon_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
before(make_salmon_s1, make_salmon_s2). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
step(make_salmon, make_salmon_s3). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
action(make_salmon_s3, fry). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
object_of(make_salmon_s3, salmon_fillet). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
uses(make_salmon_s3, frying_pan). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
until(make_salmon_s3, crisp). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
takes_time(make_salmon_s3, 4, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
before(make_salmon_s2, make_salmon_s3). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
step(make_salmon, make_salmon_s4). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
action(make_salmon_s4, flip). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
object_of(make_salmon_s4, salmon_fillet). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
uses(make_salmon_s4, spatula). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
step(make_salmon, make_salmon_s5). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
action(make_salmon_s5, cook). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
object_of(make_salmon_s5, salmon_fillet). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
takes_time(make_salmon_s5, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
before(make_salmon_s4, make_salmon_s5). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
before(make_salmon_s3, make_salmon_s4). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
action(make_salmon_s5, rest). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
takes_time(make_salmon_s5, 2, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
step(make_salmon, make_salmon_s6). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
action(make_salmon_s6, serve). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
object_of(make_salmon_s6, salmon_fillet). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
uses(make_salmon_s6, lemon). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
before(make_salmon_s5, make_salmon_s6). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
action(make_salmon_s2, season). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
object_of(make_salmon_s2, salmon_fillet). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
uses(make_salmon_s2, salt). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
uses(make_salmon_s2, pepper). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
takes_time(make_salmon_s2, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946389013-131028-101
action(make_salmon_s4, cook). @conf 1 @source teacher:curriculum @conv s1794946664488-131028-319
object_of(make_salmon_s4, other_side). @conf 1 @source teacher:curriculum @conv s1794946664488-131028-319
takes_time(make_salmon_s4, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946664488-131028-319
action(make_salmon_s5, serve). @conf 1 @source teacher:curriculum @conv s1794946664488-131028-319
uses(make_salmon_s5, lemon). @conf 1 @source teacher:curriculum @conv s1794946664488-131028-319
```
