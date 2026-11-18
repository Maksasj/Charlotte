---
id: make_guacamole
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make guacamole

Learned in conversation.

```datalog
task(make_guacamole). @conf 1 @source ""
needs(make_guacamole, avocado). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
needs(make_guacamole, lime). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
needs(make_guacamole, onion). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
needs(make_guacamole, salt). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
needs(make_guacamole, knife). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
needs(make_guacamole, bowl). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
needs(make_guacamole, fork). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
takes_time(make_guacamole, 10, 14, minute). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
step(make_guacamole, make_guacamole_s1). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
action(make_guacamole_s1, chop). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
object_of(make_guacamole_s1, onion). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
uses(make_guacamole_s1, knife). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
takes_time(make_guacamole_s1, 3, 4, minute). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
step(make_guacamole, make_guacamole_s2). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
action(make_guacamole_s2, halve). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
object_of(make_guacamole_s2, avocado). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
uses(make_guacamole_s2, knife). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
action(make_guacamole_s2, scoop). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
object_of(make_guacamole_s2, flesh). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
uses(make_guacamole_s2, bowl). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
takes_time(make_guacamole_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
before(make_guacamole_s1, make_guacamole_s2). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
step(make_guacamole, make_guacamole_s3). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
action(make_guacamole_s3, mash). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
object_of(make_guacamole_s3, avocado). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
uses(make_guacamole_s3, fork). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
takes_time(make_guacamole_s3, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
before(make_guacamole_s2, make_guacamole_s3). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
step(make_guacamole, make_guacamole_s4). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
action(make_guacamole_s4, mix). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
object_of(make_guacamole_s4, onion). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
object_of(make_guacamole_s4, lime_juice). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
object_of(make_guacamole_s4, salt). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
takes_time(make_guacamole_s4, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
before(make_guacamole_s3, make_guacamole_s4). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
action(make_guacamole_s3, scoop). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
object_of(make_guacamole_s3, flesh). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
uses(make_guacamole_s3, bowl). @conf 1 @source teacher:curriculum @conv s1794982365941-336211-61
```
