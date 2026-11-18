---
id: make_taco
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make taco

Learned in conversation.

```datalog
task(make_taco). @conf 1 @source ""
needs(make_taco, taco_shell). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
needs(make_taco, minced_beef). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
needs(make_taco, taco_seasoning). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
needs(make_taco, water). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
needs(make_taco, lettuce). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
needs(make_taco, grated_cheese). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
needs(make_taco, frying_pan). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
needs(make_taco, knife). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
needs(make_taco, oven). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
takes_time(make_taco, 14, 18, minute). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
step(make_taco, make_taco_s1). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
action(make_taco_s1, brown). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
object_of(make_taco_s1, minced_beef). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
uses(make_taco_s1, frying_pan). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
takes_time(make_taco_s1, 6, 8, minute). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
step(make_taco, make_taco_s2). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
action(make_taco_s2, shred). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
object_of(make_taco_s2, lettuce). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
uses(make_taco_s2, knife). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
takes_time(make_taco_s2, 3, 4, minute). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
overlaps(make_taco_s2, make_taco_s1). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
step(make_taco, make_taco_s3). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
action(make_taco_s3, stir). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
object_of(make_taco_s3, taco_seasoning). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
uses(make_taco_s3, water). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
takes_time(make_taco_s3, 4, 5, minute). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
step(make_taco, make_taco_s4). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
action(make_taco_s4, simmer). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
before(make_taco_s3, make_taco_s4). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
before(make_taco_s2, make_taco_s3). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
before(make_taco_s1, make_taco_s3). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
action(make_taco_s4, warm). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
object_of(make_taco_s4, taco_shell). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
uses(make_taco_s4, oven). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
takes_time(make_taco_s4, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
overlaps(make_taco_s4, make_taco_s3). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
step(make_taco, make_taco_s5). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
action(make_taco_s5, fill). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
object_of(make_taco_s5, taco_shell). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
uses(make_taco_s5, beef). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
uses(make_taco_s5, lettuce). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
uses(make_taco_s5, cheese). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
takes_time(make_taco_s5, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
before(make_taco_s4, make_taco_s5). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
before(make_taco_s3, make_taco_s5). @conf 1 @source teacher:curriculum @conv s1794978827657-85704-126
```
