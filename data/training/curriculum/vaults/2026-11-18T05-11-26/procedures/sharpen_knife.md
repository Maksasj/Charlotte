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
needs(sharpen_knife, whetstone). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
needs(sharpen_knife, water). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
needs(sharpen_knife, towel). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
takes_time(sharpen_knife, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
step(sharpen_knife, sharpen_knife_s1). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
action(sharpen_knife_s1, soak). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
object_of(sharpen_knife_s1, whetstone). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
uses(sharpen_knife_s1, water). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
takes_time(sharpen_knife_s1, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
step(sharpen_knife, sharpen_knife_s2). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
action(sharpen_knife_s2, place). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
object_of(sharpen_knife_s2, whetstone). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
uses(sharpen_knife_s2, towel). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
takes_time(sharpen_knife_s2, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
before(sharpen_knife_s1, sharpen_knife_s2). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
step(sharpen_knife, sharpen_knife_s3). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
action(sharpen_knife_s3, sharpen). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
object_of(sharpen_knife_s3, knife). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
uses(sharpen_knife_s3, whetstone). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
takes_time(sharpen_knife_s3, 5, 8, minute). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
before(sharpen_knife_s2, sharpen_knife_s3). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
step(sharpen_knife, sharpen_knife_s4). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
action(sharpen_knife_s4, hone). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
object_of(sharpen_knife_s4, knife). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
uses(sharpen_knife_s4, whetstone). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
takes_time(sharpen_knife_s4, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
before(sharpen_knife_s3, sharpen_knife_s4). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
step(sharpen_knife, sharpen_knife_s5). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
action(sharpen_knife_s5, rinse). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
object_of(sharpen_knife_s5, knife). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
action(sharpen_knife_s5, dry). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
uses(sharpen_knife_s5, towel). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
takes_time(sharpen_knife_s5, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
before(sharpen_knife_s4, sharpen_knife_s5). @conf 1 @source teacher:curriculum @conv s1794978850659-85704-145
```
