---
id: fix_hinge
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# fix hinge

Learned in conversation.

```datalog
task(fix_hinge). @conf 1 @source ""
takes_time(fix_hinge, 4, 6, minute). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
step(fix_hinge, fix_hinge_s1). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
action(fix_hinge_s1, find). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
object_of(fix_hinge_s1, squeaky_hinge). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
takes_time(fix_hinge_s1, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
uses(fix_hinge_s1, door). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
step(fix_hinge, fix_hinge_s2). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
action(fix_hinge_s2, spray). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
object_of(fix_hinge_s2, squeaky_hinge). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
uses(fix_hinge_s2, spray_lubricant). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
takes_time(fix_hinge_s2, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
before(fix_hinge_s1, fix_hinge_s2). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
step(fix_hinge, fix_hinge_s3). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
action(fix_hinge_s3, swing). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
object_of(fix_hinge_s3, door). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
until(fix_hinge_s3, squeak_stop). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
takes_time(fix_hinge_s3, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
before(fix_hinge_s2, fix_hinge_s3). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
step(fix_hinge, fix_hinge_s4). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
action(fix_hinge_s4, wipe). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
object_of(fix_hinge_s4, lubricant). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
uses(fix_hinge_s4, cloth). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
takes_time(fix_hinge_s4, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
before(fix_hinge_s3, fix_hinge_s4). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
needs(fix_hinge, spray_lubricant). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
needs(fix_hinge, cloth). @conf 1 @source teacher:curriculum @conv s1794978739479-85704-48
```
