---
id: make_pancake
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make pancake

Learned in conversation.

```datalog
task(make_pancake). @conf 1 @source ""
needs(make_pancake, flour). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
needs(make_pancake, egg). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
needs(make_pancake, milk). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
needs(make_pancake, butter). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
needs(make_pancake, bowl). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
needs(make_pancake, whisk). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
needs(make_pancake, frying_pan). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
needs(make_pancake, spatula). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
takes_time(make_pancake, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
step(make_pancake, make_pancake_s1). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
action(make_pancake_s1, mix). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
object_of(make_pancake_s1, batter). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
uses(make_pancake_s1, flour). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
uses(make_pancake_s1, egg). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
uses(make_pancake_s1, milk). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
uses(make_pancake_s1, bowl). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
uses(make_pancake_s1, whisk). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
takes_time(make_pancake_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
step(make_pancake, make_pancake_s2). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
action(make_pancake_s2, melt). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
object_of(make_pancake_s2, butter). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
uses(make_pancake_s2, pan). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
takes_time(make_pancake_s2, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
before(make_pancake_s1, make_pancake_s2). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
step(make_pancake, make_pancake_s3). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
action(make_pancake_s3, pour). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
object_of(make_pancake_s3, batter). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
uses(make_pancake_s3, pan). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
before(make_pancake_s2, make_pancake_s3). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
step(make_pancake, make_pancake_s4). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
action(make_pancake_s4, cook). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
object_of(make_pancake_s4, batter). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
takes_time(make_pancake_s4, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
before(make_pancake_s3, make_pancake_s4). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
step(make_pancake, make_pancake_s5). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
action(make_pancake_s5, flip). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
object_of(make_pancake_s5, pancake). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
uses(make_pancake_s5, spatula). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
step(make_pancake, make_pancake_s6). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
action(make_pancake_s6, cook). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
object_of(make_pancake_s6, pancake). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
takes_time(make_pancake_s6, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
before(make_pancake_s4, make_pancake_s5). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
before(make_pancake_s5, make_pancake_s6). @conf 1 @source teacher:curriculum @conv s1794979208546-85704-454
```
