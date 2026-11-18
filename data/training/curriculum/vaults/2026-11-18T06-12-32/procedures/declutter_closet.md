---
id: declutter_closet
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# declutter closet

Learned in conversation.

```datalog
task(declutter_closet). @conf 1 @source ""
needs(declutter_closet, bag). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
needs(declutter_closet, damp_cloth). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
needs(declutter_closet, hanger). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
takes_time(declutter_closet, 60, 90, minute). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
step(declutter_closet, declutter_closet_s1). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
action(declutter_closet_s1, take_out). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
object_of(declutter_closet_s1, closet). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
takes_time(declutter_closet_s1, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
step(declutter_closet, declutter_closet_s2). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
action(declutter_closet_s2, sort). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
object_of(declutter_closet_s2, clothes). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
takes_time(declutter_closet_s2, 25, 40, minute). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
before(declutter_closet_s1, declutter_closet_s2). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
step(declutter_closet, declutter_closet_s3). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
action(declutter_closet_s3, pack). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
object_of(declutter_closet_s3, donate_pile). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
object_of(declutter_closet_s3, throw_away_pile). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
uses(declutter_closet_s3, bag). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
takes_time(declutter_closet_s3, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
before(declutter_closet_s2, declutter_closet_s3). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
step(declutter_closet, declutter_closet_s5). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
action(declutter_closet_s5, hang). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
object_of(declutter_closet_s5, clothes). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
uses(declutter_closet_s5, hanger). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
takes_time(declutter_closet_s5, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
before(declutter_closet_s3, declutter_closet_s5). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
object_of(declutter_closet_s1, bag). @conf 1 @source teacher:curriculum @conv s1794982360730-336211-41
```
