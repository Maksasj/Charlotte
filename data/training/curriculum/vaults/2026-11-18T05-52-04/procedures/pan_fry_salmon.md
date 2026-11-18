---
id: pan_fry_salmon
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# pan fry salmon

Learned in conversation.

```datalog
task(pan_fry_salmon). @conf 1 @source ""
needs(pan_fry_salmon, salmon_fillet). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
needs(pan_fry_salmon, oil). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
needs(pan_fry_salmon, salt). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
needs(pan_fry_salmon, pepper). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
needs(pan_fry_salmon, lemon). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
needs(pan_fry_salmon, paper_towel). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
needs(pan_fry_salmon, frying_pan). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
needs(pan_fry_salmon, spatula). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
takes_time(pan_fry_salmon, 12, 16, minute). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
step(pan_fry_salmon, pan_fry_salmon_s1). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
action(pan_fry_salmon_s1, pat). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
object_of(pan_fry_salmon_s1, salmon_fillet). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
uses(pan_fry_salmon_s1, paper_towel). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
action(pan_fry_salmon_s1, season). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
uses(pan_fry_salmon_s1, salt). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
uses(pan_fry_salmon_s1, pepper). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
takes_time(pan_fry_salmon_s1, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
step(pan_fry_salmon, pan_fry_salmon_s2). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
action(pan_fry_salmon_s2, heat). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
object_of(pan_fry_salmon_s2, oil). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
uses(pan_fry_salmon_s2, frying_pan). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
takes_time(pan_fry_salmon_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
before(pan_fry_salmon_s1, pan_fry_salmon_s2). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
step(pan_fry_salmon, pan_fry_salmon_s3). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
action(pan_fry_salmon_s3, fry). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
object_of(pan_fry_salmon_s3, salmon_fillet). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
takes_time(pan_fry_salmon_s3, 4, 5, minute). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
until(pan_fry_salmon_s3, crisp). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
before(pan_fry_salmon_s2, pan_fry_salmon_s3). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
step(pan_fry_salmon, pan_fry_salmon_s4). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
action(pan_fry_salmon_s4, flip). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
object_of(pan_fry_salmon_s4, salmon_fillet). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
uses(pan_fry_salmon_s4, spatula). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
step(pan_fry_salmon, pan_fry_salmon_s5). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
action(pan_fry_salmon_s5, cook). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
object_of(pan_fry_salmon_s5, salmon_fillet). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
takes_time(pan_fry_salmon_s5, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
before(pan_fry_salmon_s4, pan_fry_salmon_s5). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
before(pan_fry_salmon_s3, pan_fry_salmon_s4). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
action(pan_fry_salmon_s5, rest). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
action(pan_fry_salmon_s5, serve). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
uses(pan_fry_salmon_s5, lemon). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
takes_time(pan_fry_salmon_s5, 2, 2, minute). @conf 1 @source teacher:curriculum @conv s1794981178147-218805-135
```
