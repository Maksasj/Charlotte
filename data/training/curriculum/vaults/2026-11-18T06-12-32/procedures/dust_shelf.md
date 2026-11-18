---
id: dust_shelf
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# dust shelf

Learned in conversation.

```datalog
task(dust_shelf). @conf 1 @source ""
step(dust_shelf, dust_shelf_s1). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
action(dust_shelf_s1, clear). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
object_of(dust_shelf_s1, item). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
uses(dust_shelf_s1, step_stool). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
takes_time(dust_shelf_s1, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
step(dust_shelf, dust_shelf_s2). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
action(dust_shelf_s2, dust). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
object_of(dust_shelf_s2, shelf). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
uses(dust_shelf_s2, dry_cloth). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
takes_time(dust_shelf_s2, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
before(dust_shelf_s1, dust_shelf_s2). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
step(dust_shelf, dust_shelf_s3). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
action(dust_shelf_s3, wipe). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
object_of(dust_shelf_s3, item). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
uses(dust_shelf_s3, cloth). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
uses(dust_shelf_s3, water). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
takes_time(dust_shelf_s3, 5, 7, minute). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
before(dust_shelf_s2, dust_shelf_s3). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
step(dust_shelf, dust_shelf_s4). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
action(dust_shelf_s4, put_back). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
object_of(dust_shelf_s4, item). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
takes_time(dust_shelf_s4, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
before(dust_shelf_s3, dust_shelf_s4). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
needs(dust_shelf, microfibre_cloth). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
needs(dust_shelf, water). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
needs(dust_shelf, step_stool). @conf 1 @source teacher:curriculum @conv s1794982364443-336211-53
takes_time(dust_shelf, 15, 25, minute). @conf 1 @source teacher:curriculum @conv s1794982466442-336211-300
uses(dust_shelf_s2, cloth). @conf 1 @source teacher:curriculum @conv s1794982466442-336211-300
```
