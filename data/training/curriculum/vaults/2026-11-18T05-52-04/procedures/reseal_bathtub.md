---
id: reseal_bathtub
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# reseal bathtub

Learned in conversation.

```datalog
task(reseal_bathtub). @conf 1 @source ""
needs(reseal_bathtub, silicone_sealant). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
needs(reseal_bathtub, caulk_gun). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
needs(reseal_bathtub, utility_knife). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
needs(reseal_bathtub, rubbing_alcohol). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
needs(reseal_bathtub, masking_tape). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
needs(reseal_bathtub, cloth). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
takes_time(reseal_bathtub, 45, 60, minute). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
step(reseal_bathtub, reseal_bathtub_s1). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
action(reseal_bathtub_s1, cut). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
object_of(reseal_bathtub_s1, sealant). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
uses(reseal_bathtub_s1, utility_knife). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
takes_time(reseal_bathtub_s1, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
step(reseal_bathtub, reseal_bathtub_s2). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
action(reseal_bathtub_s2, clean). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
object_of(reseal_bathtub_s2, joint). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
uses(reseal_bathtub_s2, rubbing_alcohol). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
uses(reseal_bathtub_s2, cloth). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
step(reseal_bathtub, reseal_bathtub_s3). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
action(reseal_bathtub_s3, dry). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
object_of(reseal_bathtub_s3, joint). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
takes_time(reseal_bathtub_s3, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
before(reseal_bathtub_s2, reseal_bathtub_s3). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
before(reseal_bathtub_s1, reseal_bathtub_s2). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
action(reseal_bathtub_s3, put). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
object_of(reseal_bathtub_s3, masking_tape). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
uses(reseal_bathtub_s3, masking_tape). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
step(reseal_bathtub, reseal_bathtub_s4). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
action(reseal_bathtub_s4, apply). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
object_of(reseal_bathtub_s4, sealant). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
uses(reseal_bathtub_s4, caulk_gun). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
takes_time(reseal_bathtub_s4, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
before(reseal_bathtub_s3, reseal_bathtub_s4). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
step(reseal_bathtub, reseal_bathtub_s5). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
action(reseal_bathtub_s5, smooth). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
object_of(reseal_bathtub_s5, bead). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
uses(reseal_bathtub_s5, finger). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
step(reseal_bathtub, reseal_bathtub_s6). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
action(reseal_bathtub_s6, peel). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
object_of(reseal_bathtub_s6, tape). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
takes_time(reseal_bathtub_s6, 5, 5, minute). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
before(reseal_bathtub_s5, reseal_bathtub_s6). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
before(reseal_bathtub_s4, reseal_bathtub_s5). @conf 1 @source teacher:curriculum @conv s1794981172448-218805-124
```
