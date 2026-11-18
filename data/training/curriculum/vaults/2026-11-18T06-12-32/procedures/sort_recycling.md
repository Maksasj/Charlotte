---
id: sort_recycling
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# sort recycling

Learned in conversation.

```datalog
task(sort_recycling). @conf 1 @source ""
needs(sort_recycling, water). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
needs(sort_recycling, recycling_bin). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
takes_time(sort_recycling, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
step(sort_recycling, sort_recycling_s1). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
action(sort_recycling_s1, rinse). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
object_of(sort_recycling_s1, food_container). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
object_of(sort_recycling_s1, tin). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
uses(sort_recycling_s1, water). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
takes_time(sort_recycling_s1, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
step(sort_recycling, sort_recycling_s2). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
action(sort_recycling_s2, flatten). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
object_of(sort_recycling_s2, cardboard_box). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
takes_time(sort_recycling_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
before(sort_recycling_s1, sort_recycling_s2). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
step(sort_recycling, sort_recycling_s3). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
action(sort_recycling_s3, separate). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
object_of(sort_recycling_s3, glass_bottle). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
object_of(sort_recycling_s3, plastic). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
object_of(sort_recycling_s3, paper). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
takes_time(sort_recycling_s3, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
before(sort_recycling_s2, sort_recycling_s3). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
step(sort_recycling, sort_recycling_s4). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
action(sort_recycling_s4, put). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
object_of(sort_recycling_s4, waste). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
uses(sort_recycling_s4, recycling_bin). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
takes_time(sort_recycling_s4, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
before(sort_recycling_s3, sort_recycling_s4). @conf 1 @source teacher:curriculum @conv s1794982419025-336211-192
```
