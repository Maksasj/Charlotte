---
id: plan_catch_up
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# plan catch up

Learned in conversation.

```datalog
task(plan_catch_up). @conf 1 @source ""
takes_time(plan_catch_up, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
step(plan_catch_up, plan_catch_up_s1). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
action(plan_catch_up_s1, message). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
object_of(plan_catch_up_s1, friend). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
uses(plan_catch_up_s1, phone). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
takes_time(plan_catch_up_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
step(plan_catch_up, plan_catch_up_s2). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
action(plan_catch_up_s2, find). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
object_of(plan_catch_up_s2, time). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
uses(plan_catch_up_s2, calendar). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
takes_time(plan_catch_up_s2, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
before(plan_catch_up_s1, plan_catch_up_s2). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
step(plan_catch_up, plan_catch_up_s3). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
action(plan_catch_up_s3, choose). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
object_of(plan_catch_up_s3, cafe). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
takes_time(plan_catch_up_s3, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
before(plan_catch_up_s2, plan_catch_up_s3). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
step(plan_catch_up, plan_catch_up_s4). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
action(plan_catch_up_s4, add). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
object_of(plan_catch_up_s4, plan). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
uses(plan_catch_up_s4, calendar). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
takes_time(plan_catch_up_s4, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
before(plan_catch_up_s3, plan_catch_up_s4). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
step(plan_catch_up, plan_catch_up_s5). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
action(plan_catch_up_s5, confirm). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
object_of(plan_catch_up_s5, plan). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
uses(plan_catch_up_s5, phone). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
takes_time(plan_catch_up_s5, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
before(plan_catch_up_s4, plan_catch_up_s5). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
needs(plan_catch_up, phone). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
needs(plan_catch_up, calendar). @conf 1 @source teacher:curriculum @conv s1794978775941-85704-83
```
