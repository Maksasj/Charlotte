---
id: make_bruschetta
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make bruschetta

Learned in conversation.

```datalog
task(make_bruschetta). @conf 1 @source ""
needs(make_bruschetta, baguette). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
needs(make_bruschetta, tomato). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
needs(make_bruschetta, garlic). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
needs(make_bruschetta, basil). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
needs(make_bruschetta, olive_oil). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
needs(make_bruschetta, salt). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
needs(make_bruschetta, knife). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
needs(make_bruschetta, bowl). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
needs(make_bruschetta, oven). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
takes_time(make_bruschetta, 10, 14, minute). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
step(make_bruschetta, make_bruschetta_s1). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
action(make_bruschetta_s1, dice). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
object_of(make_bruschetta_s1, tomato). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
uses(make_bruschetta_s1, knife). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
step(make_bruschetta, make_bruschetta_s2). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
action(make_bruschetta_s2, tear). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
object_of(make_bruschetta_s2, basil). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
takes_time(make_bruschetta_s2, 5, 6, minute). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
overlaps(make_bruschetta_s2, make_bruschetta_s1). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
action(make_bruschetta_s2, toast). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
object_of(make_bruschetta_s2, baguette). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
uses(make_bruschetta_s2, oven_grill). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
takes_time(make_bruschetta_s2, 4, 6, minute). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
until(make_bruschetta_s2, golden). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
step(make_bruschetta, make_bruschetta_s3). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
action(make_bruschetta_s3, mix). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
object_of(make_bruschetta_s3, tomato). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
uses(make_bruschetta_s3, olive_oil). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
uses(make_bruschetta_s3, salt). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
uses(make_bruschetta_s3, bowl). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
takes_time(make_bruschetta_s3, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
before(make_bruschetta_s2, make_bruschetta_s3). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
before(make_bruschetta_s1, make_bruschetta_s3). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
step(make_bruschetta, make_bruschetta_s4). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
action(make_bruschetta_s4, rub). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
object_of(make_bruschetta_s4, garlic). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
takes_time(make_bruschetta_s4, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
before(make_bruschetta_s3, make_bruschetta_s4). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
step(make_bruschetta, make_bruschetta_s5). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
action(make_bruschetta_s5, spoon). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
object_of(make_bruschetta_s5, tomato_mix). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
uses(make_bruschetta_s5, spoon). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
takes_time(make_bruschetta_s5, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
before(make_bruschetta_s4, make_bruschetta_s5). @conf 1 @source teacher:curriculum @conv s1794946292372-131028-34
```
