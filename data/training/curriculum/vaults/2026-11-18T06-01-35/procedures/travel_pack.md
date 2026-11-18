---
id: travel_pack
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# travel pack

Learned in conversation.

```datalog
task(travel_pack). @conf 1 @source ""
needs(travel_pack, suitcase). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
needs(travel_pack, clothing). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
needs(travel_pack, toiletry_bag). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
needs(travel_pack, passport). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
takes_time(travel_pack, 30, 45, minute). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
step(travel_pack, travel_pack_s1). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
action(travel_pack_s1, check). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
object_of(travel_pack_s1, weather). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
takes_time(travel_pack_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
step(travel_pack, travel_pack_s2). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
action(travel_pack_s2, write). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
object_of(travel_pack_s2, packing_list). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
takes_time(travel_pack_s2, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
before(travel_pack_s1, travel_pack_s2). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
step(travel_pack, travel_pack_s3). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
action(travel_pack_s3, lay_out). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
object_of(travel_pack_s3, clothing). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
uses(travel_pack_s3, bed). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
takes_time(travel_pack_s3, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
before(travel_pack_s2, travel_pack_s3). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
step(travel_pack, travel_pack_s4). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
action(travel_pack_s4, roll). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
object_of(travel_pack_s4, clothing). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
uses(travel_pack_s4, suitcase). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
takes_time(travel_pack_s4, 10, 12, minute). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
before(travel_pack_s3, travel_pack_s4). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
step(travel_pack, travel_pack_s5). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
action(travel_pack_s5, pack). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
object_of(travel_pack_s5, toiletry_bag). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
takes_time(travel_pack_s5, 5, 7, minute). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
before(travel_pack_s4, travel_pack_s5). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
step(travel_pack, travel_pack_s6). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
action(travel_pack_s6, put). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
object_of(travel_pack_s6, passport). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
uses(travel_pack_s6, hand_luggage). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
takes_time(travel_pack_s6, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
before(travel_pack_s5, travel_pack_s6). @conf 1 @source teacher:curriculum @conv s1794982000956-278172-469
```
