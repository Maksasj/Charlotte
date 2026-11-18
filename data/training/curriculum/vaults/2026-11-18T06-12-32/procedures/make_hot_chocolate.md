---
id: make_hot_chocolate
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make hot chocolate

Learned in conversation.

```datalog
task(make_hot_chocolate). @conf 1 @source ""
needs(make_hot_chocolate, milk). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
needs(make_hot_chocolate, cocoa_powder). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
needs(make_hot_chocolate, sugar). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
needs(make_hot_chocolate, saucepan). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
needs(make_hot_chocolate, mug). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
takes_time(make_hot_chocolate, 6, 9, minute). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
step(make_hot_chocolate, make_hot_chocolate_s1). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
action(make_hot_chocolate_s1, heat). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
object_of(make_hot_chocolate_s1, milk). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
uses(make_hot_chocolate_s1, saucepan). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
takes_time(make_hot_chocolate_s1, 4, 5, minute). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
action(make_hot_chocolate_s1, mix). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
object_of(make_hot_chocolate_s1, cocoa_powder). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
object_of(make_hot_chocolate_s1, sugar). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
uses(make_hot_chocolate_s1, mug). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
uses(make_hot_chocolate_s1, cold_milk). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
takes_time(make_hot_chocolate_s1, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
step(make_hot_chocolate, make_hot_chocolate_s2). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
action(make_hot_chocolate_s2, mix). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
object_of(make_hot_chocolate_s2, cocoa_powder). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
object_of(make_hot_chocolate_s2, sugar). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
uses(make_hot_chocolate_s2, mug). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
uses(make_hot_chocolate_s2, cold_milk). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
takes_time(make_hot_chocolate_s2, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
before(make_hot_chocolate_s1, make_hot_chocolate_s2). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
step(make_hot_chocolate, make_hot_chocolate_s3). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
action(make_hot_chocolate_s3, pour). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
object_of(make_hot_chocolate_s3, milk). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
uses(make_hot_chocolate_s3, mug). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
before(make_hot_chocolate_s2, make_hot_chocolate_s3). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
step(make_hot_chocolate, make_hot_chocolate_s4). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
action(make_hot_chocolate_s4, stir). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
object_of(make_hot_chocolate_s4, cocoa_powder). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
uses(make_hot_chocolate_s4, milk). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
until(make_hot_chocolate_s4, smooth). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
takes_time(make_hot_chocolate_s4, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
before(make_hot_chocolate_s3, make_hot_chocolate_s4). @conf 1 @source teacher:curriculum @conv s1794982396034-336211-138
```
