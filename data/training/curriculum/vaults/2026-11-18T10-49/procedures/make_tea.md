---
id: make_tea
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make tea

Learned in conversation.

```datalog
task(make_tea). @conf 1 @source ""
needs(make_tea, tea_bag). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
needs(make_tea, water). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
needs(make_tea, kettle). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
needs(make_tea, mug). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
takes_time(make_tea, 5, 7, minute). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
step(make_tea, make_tea_s1). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
action(make_tea_s1, boil). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
object_of(make_tea_s1, water). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
uses(make_tea_s1, kettle). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
takes_time(make_tea_s1, 3, 4, minute). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
step(make_tea, make_tea_s2). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
action(make_tea_s2, pour). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
object_of(make_tea_s2, water). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
uses(make_tea_s2, mug). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
uses(make_tea_s2, tea_bag). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
before(make_tea_s1, make_tea_s2). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
step(make_tea, make_tea_s3). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
action(make_tea_s3, steep). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
object_of(make_tea_s3, tea_bag). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
takes_time(make_tea_s3, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
before(make_tea_s2, make_tea_s3). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
step(make_tea, make_tea_s4). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
action(make_tea_s4, remove). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
object_of(make_tea_s4, tea_bag). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
before(make_tea_s3, make_tea_s4). @conf 1 @source teacher:curriculum @conv s1794998941785-568584-28
```
