---
id: make_couscous
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make couscous

Learned in conversation.

```datalog
task(make_couscous). @conf 1 @source ""
needs(make_couscous, couscous). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
needs(make_couscous, salt). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
needs(make_couscous, water). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
needs(make_couscous, kettle). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
needs(make_couscous, bowl). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
needs(make_couscous, plate). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
needs(make_couscous, fork). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
takes_time(make_couscous, 9, 11, minute). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
step(make_couscous, make_couscous_s1). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
action(make_couscous_s1, boil). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
object_of(make_couscous_s1, water). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
uses(make_couscous_s1, kettle). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
takes_time(make_couscous_s1, 3, 4, minute). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
step(make_couscous, make_couscous_s2). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
action(make_couscous_s2, measure). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
object_of(make_couscous_s2, couscous). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
uses(make_couscous_s2, bowl). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
uses(make_couscous_s2, salt). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
takes_time(make_couscous_s2, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
overlaps(make_couscous_s2, make_couscous_s1). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
step(make_couscous, make_couscous_s4). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
action(make_couscous_s4, cover). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
object_of(make_couscous_s4, bowl). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
uses(make_couscous_s4, plate). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
step(make_couscous, make_couscous_s5). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
action(make_couscous_s5, stand). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
takes_time(make_couscous_s5, 5, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
before(make_couscous_s4, make_couscous_s5). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
before(make_couscous_s2, make_couscous_s4). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
before(make_couscous_s1, make_couscous_s4). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
action(make_couscous_s5, fluff). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
object_of(make_couscous_s5, couscous). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
uses(make_couscous_s5, fork). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
takes_time(make_couscous_s5, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
step(make_couscous, make_couscous_s3). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
action(make_couscous_s3, pour). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
object_of(make_couscous_s3, water). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
uses(make_couscous_s3, bowl). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
until(make_couscous_s3, covered). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
before(make_couscous_s2, make_couscous_s3). @conf 1 @source teacher:curriculum @conv s1794946311554-131028-58
action(make_couscous_s1, heat). @conf 1 @source teacher:curriculum @conv s1794946569078-131028-223
object_of(make_couscous_s1, chicken_stock). @conf 1 @source teacher:curriculum @conv s1794946569078-131028-223
uses(make_couscous_s1, saucepan). @conf 1 @source teacher:curriculum @conv s1794946569078-131028-223
```
