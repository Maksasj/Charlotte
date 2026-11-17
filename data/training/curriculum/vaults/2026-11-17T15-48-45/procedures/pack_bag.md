---
id: pack_bag
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# pack bag

Learned in conversation.

```datalog
task(pack_bag). @conf 1 @source ""
needs(pack_bag, suitcase). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
needs(pack_bag, clothing). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
needs(pack_bag, toiletry_bag). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
needs(pack_bag, passport). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
takes_time(pack_bag, 30, 45, minute). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
step(pack_bag, pack_bag_s1). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
action(pack_bag_s1, check). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
object_of(pack_bag_s1, weather). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
takes_time(pack_bag_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
step(pack_bag, pack_bag_s2). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
action(pack_bag_s2, write). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
object_of(pack_bag_s2, packing_list). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
takes_time(pack_bag_s2, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
before(pack_bag_s1, pack_bag_s2). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
step(pack_bag, pack_bag_s3). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
action(pack_bag_s3, lay_out). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
object_of(pack_bag_s3, clothing). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
takes_time(pack_bag_s3, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
before(pack_bag_s2, pack_bag_s3). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
step(pack_bag, pack_bag_s4). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
action(pack_bag_s4, roll). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
object_of(pack_bag_s4, clothing). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
uses(pack_bag_s4, suitcase). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
takes_time(pack_bag_s4, 10, 12, minute). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
before(pack_bag_s3, pack_bag_s4). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
step(pack_bag, pack_bag_s5). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
action(pack_bag_s5, pack). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
object_of(pack_bag_s5, toiletry_bag). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
takes_time(pack_bag_s5, 5, 7, minute). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
before(pack_bag_s4, pack_bag_s5). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
step(pack_bag, pack_bag_s6). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
action(pack_bag_s6, put). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
object_of(pack_bag_s6, passport). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
uses(pack_bag_s6, hand_luggage). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
takes_time(pack_bag_s6, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
before(pack_bag_s5, pack_bag_s6). @conf 1 @source teacher:curriculum @conv s1794931029859-100704-41
```
