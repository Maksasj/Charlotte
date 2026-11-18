---
id: squeeze_orange_juice
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# squeeze orange juice

Learned in conversation.

```datalog
task(squeeze_orange_juice). @conf 1 @source ""
needs(squeeze_orange_juice, orange). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
needs(squeeze_orange_juice, knife). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
needs(squeeze_orange_juice, citrus_juicer). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
needs(squeeze_orange_juice, sieve). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
needs(squeeze_orange_juice, glass). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
takes_time(squeeze_orange_juice, 5, 8, minute). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
step(squeeze_orange_juice, squeeze_orange_juice_s1). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
action(squeeze_orange_juice_s1, roll). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
object_of(squeeze_orange_juice_s1, orange). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
uses(squeeze_orange_juice_s1, counter). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
takes_time(squeeze_orange_juice_s1, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
step(squeeze_orange_juice, squeeze_orange_juice_s2). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
action(squeeze_orange_juice_s2, cut). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
object_of(squeeze_orange_juice_s2, orange). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
uses(squeeze_orange_juice_s2, knife). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
takes_time(squeeze_orange_juice_s2, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
before(squeeze_orange_juice_s1, squeeze_orange_juice_s2). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
step(squeeze_orange_juice, squeeze_orange_juice_s3). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
action(squeeze_orange_juice_s3, squeeze). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
object_of(squeeze_orange_juice_s3, orange). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
uses(squeeze_orange_juice_s3, juicer). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
takes_time(squeeze_orange_juice_s3, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
before(squeeze_orange_juice_s2, squeeze_orange_juice_s3). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
step(squeeze_orange_juice, squeeze_orange_juice_s4). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
action(squeeze_orange_juice_s4, pour). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
object_of(squeeze_orange_juice_s4, juice). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
uses(squeeze_orange_juice_s4, sieve). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
before(squeeze_orange_juice_s3, squeeze_orange_juice_s4). @conf 1 @source teacher:curriculum @conv s1794981238419-218805-280
```
