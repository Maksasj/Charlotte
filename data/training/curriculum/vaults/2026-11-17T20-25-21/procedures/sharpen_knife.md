---
id: sharpen_knife
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# sharpen knife

Learned in conversation.

```datalog
task(sharpen_knife). @conf 1 @source ""
needs(sharpen_knife, whetstone). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
needs(sharpen_knife, water). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
needs(sharpen_knife, towel). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
takes_time(sharpen_knife, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
step(sharpen_knife, sharpen_knife_s1). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
action(sharpen_knife_s1, soak). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
object_of(sharpen_knife_s1, whetstone). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
uses(sharpen_knife_s1, water). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
takes_time(sharpen_knife_s1, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
step(sharpen_knife, sharpen_knife_s2). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
action(sharpen_knife_s2, place). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
object_of(sharpen_knife_s2, whetstone). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
uses(sharpen_knife_s2, damp_towel). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
takes_time(sharpen_knife_s2, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
before(sharpen_knife_s1, sharpen_knife_s2). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
step(sharpen_knife, sharpen_knife_s3). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
action(sharpen_knife_s3, sharpen). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
object_of(sharpen_knife_s3, knife). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
uses(sharpen_knife_s3, whetstone). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
takes_time(sharpen_knife_s3, 5, 8, minute). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
before(sharpen_knife_s2, sharpen_knife_s3). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
step(sharpen_knife, sharpen_knife_s4). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
action(sharpen_knife_s4, hone). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
object_of(sharpen_knife_s4, knife). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
uses(sharpen_knife_s4, whetstone). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
takes_time(sharpen_knife_s4, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
before(sharpen_knife_s3, sharpen_knife_s4). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
step(sharpen_knife, sharpen_knife_s5). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
action(sharpen_knife_s5, rinse). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
object_of(sharpen_knife_s5, knife). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
step(sharpen_knife, sharpen_knife_s6). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
action(sharpen_knife_s6, dry). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
object_of(sharpen_knife_s6, knife). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
uses(sharpen_knife_s6, towel). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
takes_time(sharpen_knife_s6, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
before(sharpen_knife_s5, sharpen_knife_s6). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
before(sharpen_knife_s4, sharpen_knife_s5). @conf 1 @source teacher:curriculum @conv s1794946556854-131028-213
action(sharpen_knife_s1, place). @conf 1 @source teacher:curriculum @conv s1794946784106-131028-436
object_of(sharpen_knife_s1, mug). @conf 1 @source teacher:curriculum @conv s1794946784106-131028-436
uses(sharpen_knife_s1, towel). @conf 1 @source teacher:curriculum @conv s1794946784106-131028-436
```
