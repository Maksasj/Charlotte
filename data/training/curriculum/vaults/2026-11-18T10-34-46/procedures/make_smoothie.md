---
id: make_smoothie
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make smoothie

Learned in conversation.

```datalog
task(make_smoothie). @conf 1 @source ""
needs(make_smoothie, banana). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
needs(make_smoothie, strawberry). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
needs(make_smoothie, yogurt). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
needs(make_smoothie, milk). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
needs(make_smoothie, blender). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
needs(make_smoothie, glass). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
takes_time(make_smoothie, 5, 8, minute). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
step(make_smoothie, make_smoothie_s1). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
action(make_smoothie_s1, peel). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
object_of(make_smoothie_s1, banana). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
step(make_smoothie, make_smoothie_s3). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
action(make_smoothie_s3, add). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
object_of(make_smoothie_s3, strawberry). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
object_of(make_smoothie_s3, yogurt). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
object_of(make_smoothie_s3, milk). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
before(make_smoothie_s1, make_smoothie_s3). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
step(make_smoothie, make_smoothie_s4). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
action(make_smoothie_s4, blend). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
object_of(make_smoothie_s4, smoothie). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
uses(make_smoothie_s4, blender). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
until(make_smoothie_s4, smooth). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
takes_time(make_smoothie_s4, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
before(make_smoothie_s3, make_smoothie_s4). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
step(make_smoothie, make_smoothie_s5). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
action(make_smoothie_s5, pour). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
object_of(make_smoothie_s5, smoothie). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
uses(make_smoothie_s5, glass). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
before(make_smoothie_s4, make_smoothie_s5). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
step(make_smoothie, make_smoothie_s2). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
action(make_smoothie_s2, break). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
object_of(make_smoothie_s2, banana). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
uses(make_smoothie_s2, blender). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
before(make_smoothie_s1, make_smoothie_s2). @conf 1 @source teacher:curriculum @conv s1794998097776-490820-52
before(make_smoothie_s2, make_smoothie_s3). @conf 1 @source teacher:curriculum @conv s1794998104499-490820-78
```
