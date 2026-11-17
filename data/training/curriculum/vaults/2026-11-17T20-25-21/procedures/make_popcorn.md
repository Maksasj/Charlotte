---
id: make_popcorn
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make popcorn

Learned in conversation.

```datalog
task(make_popcorn). @conf 1 @source ""
needs(make_popcorn, popcorn_kernel). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
needs(make_popcorn, oil). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
needs(make_popcorn, salt). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
needs(make_popcorn, pot). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
needs(make_popcorn, lid). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
needs(make_popcorn, bowl). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
takes_time(make_popcorn, 6, 8, minute). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
step(make_popcorn, make_popcorn_s1). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
action(make_popcorn_s1, heat). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
object_of(make_popcorn_s1, oil). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
uses(make_popcorn_s1, pot). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
takes_time(make_popcorn_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
step(make_popcorn, make_popcorn_s2). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
action(make_popcorn_s2, add). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
object_of(make_popcorn_s2, popcorn_kernel). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
uses(make_popcorn_s2, pot). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
uses(make_popcorn_s2, lid). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
before(make_popcorn_s1, make_popcorn_s2). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
step(make_popcorn, make_popcorn_s3). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
action(make_popcorn_s3, shake). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
object_of(make_popcorn_s3, pot). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
until(make_popcorn_s3, popping_slows). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
takes_time(make_popcorn_s3, 3, 4, minute). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
before(make_popcorn_s2, make_popcorn_s3). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
step(make_popcorn, make_popcorn_s4). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
action(make_popcorn_s4, pour). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
object_of(make_popcorn_s4, popcorn). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
step(make_popcorn, make_popcorn_s5). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
action(make_popcorn_s5, sprinkle). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
object_of(make_popcorn_s5, salt). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
before(make_popcorn_s4, make_popcorn_s5). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
object_of(make_popcorn_s4, bowl). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
before(make_popcorn_s3, make_popcorn_s4). @conf 1 @source teacher:curriculum @conv s1794946333233-131028-72
```
