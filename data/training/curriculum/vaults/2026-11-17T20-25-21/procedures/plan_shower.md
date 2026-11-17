---
id: plan_shower
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# plan shower

Learned in conversation.

```datalog
task(plan_shower). @conf 1 @source ""
needs(plan_shower, guest_list). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
needs(plan_shower, invitation). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
needs(plan_shower, cake). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
needs(plan_shower, game). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
needs(plan_shower, decoration). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
takes_time(plan_shower, 90, 130, minute). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
step(plan_shower, plan_shower_s2). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
action(plan_shower_s2, make). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
object_of(plan_shower_s2, guest_list). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
takes_time(plan_shower_s2, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
step(plan_shower, plan_shower_s3). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
action(plan_shower_s3, send). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
object_of(plan_shower_s3, invitation). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
takes_time(plan_shower_s3, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
before(plan_shower_s2, plan_shower_s3). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
step(plan_shower, plan_shower_s4). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
action(plan_shower_s4, order). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
object_of(plan_shower_s4, cake). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
takes_time(plan_shower_s4, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
before(plan_shower_s3, plan_shower_s4). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
step(plan_shower, plan_shower_s6). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
action(plan_shower_s6, buy). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
object_of(plan_shower_s6, decoration). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
takes_time(plan_shower_s6, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
before(plan_shower_s4, plan_shower_s6). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
step(plan_shower, plan_shower_s5). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
action(plan_shower_s5, plan). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
object_of(plan_shower_s5, game). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
takes_time(plan_shower_s5, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
before(plan_shower_s4, plan_shower_s5). @conf 1 @source teacher:curriculum @conv s1794946423920-131028-121
step(plan_shower, plan_shower_s1). @conf 1 @source teacher:curriculum @conv s1794946799735-131028-444
action(plan_shower_s1, ask). @conf 1 @source teacher:curriculum @conv s1794946799735-131028-444
object_of(plan_shower_s1, date). @conf 1 @source teacher:curriculum @conv s1794946799735-131028-444
takes_time(plan_shower_s1, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946799735-131028-444
before(plan_shower_s1, plan_shower_s2). @conf 1 @source teacher:curriculum @conv s1794946799735-131028-444
before(plan_shower_s5, plan_shower_s6). @conf 1 @source teacher:curriculum @conv s1794946799735-131028-444
```
