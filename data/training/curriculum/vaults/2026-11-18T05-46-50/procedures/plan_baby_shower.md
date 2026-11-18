---
id: plan_baby_shower
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# plan baby shower

Learned in conversation.

```datalog
task(plan_baby_shower). @conf 1 @source ""
needs(plan_baby_shower, guest_list). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
needs(plan_baby_shower, invitation). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
needs(plan_baby_shower, cake). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
needs(plan_baby_shower, game). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
needs(plan_baby_shower, decoration). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
takes_time(plan_baby_shower, 90, 130, minute). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
step(plan_baby_shower, plan_baby_shower_s2). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
action(plan_baby_shower_s2, make). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
object_of(plan_baby_shower_s2, guest_list). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
takes_time(plan_baby_shower_s2, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
step(plan_baby_shower, plan_baby_shower_s3). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
action(plan_baby_shower_s3, send). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
object_of(plan_baby_shower_s3, invitation). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
takes_time(plan_baby_shower_s3, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
before(plan_baby_shower_s2, plan_baby_shower_s3). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
step(plan_baby_shower, plan_baby_shower_s4). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
action(plan_baby_shower_s4, order). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
object_of(plan_baby_shower_s4, cake). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
takes_time(plan_baby_shower_s4, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
before(plan_baby_shower_s3, plan_baby_shower_s4). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
step(plan_baby_shower, plan_baby_shower_s5). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
action(plan_baby_shower_s5, plan). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
object_of(plan_baby_shower_s5, game). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
takes_time(plan_baby_shower_s5, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
before(plan_baby_shower_s4, plan_baby_shower_s5). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
step(plan_baby_shower, plan_baby_shower_s6). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
action(plan_baby_shower_s6, buy). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
object_of(plan_baby_shower_s6, decoration). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
takes_time(plan_baby_shower_s6, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
before(plan_baby_shower_s5, plan_baby_shower_s6). @conf 1 @source teacher:curriculum @conv s1794978812323-85704-117
needs(plan_baby_shower, balloon). @conf 1 @source teacher:curriculum @conv s1794979235064-85704-468
```
