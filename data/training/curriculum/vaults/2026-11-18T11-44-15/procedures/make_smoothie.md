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
needs(make_smoothie, banana). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
needs(make_smoothie, strawberry). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
needs(make_smoothie, yogurt). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
needs(make_smoothie, milk). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
needs(make_smoothie, blender). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
needs(make_smoothie, glass). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
takes_time(make_smoothie, 5, 8, minute). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
step(make_smoothie, make_smoothie_s1). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
action(make_smoothie_s1, peel). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
object_of(make_smoothie_s1, banana). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
step(make_smoothie, make_smoothie_s3). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
before(make_smoothie_s1, make_smoothie_s3). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
step(make_smoothie, make_smoothie_s4). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
action(make_smoothie_s4, blend). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
object_of(make_smoothie_s4, smoothie). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
uses(make_smoothie_s4, blender). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
until(make_smoothie_s4, smooth). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
takes_time(make_smoothie_s4, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
before(make_smoothie_s3, make_smoothie_s4). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
step(make_smoothie, make_smoothie_s5). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
action(make_smoothie_s5, pour). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
object_of(make_smoothie_s5, smoothie). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
uses(make_smoothie_s5, glass). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
before(make_smoothie_s4, make_smoothie_s5). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
step(make_smoothie, make_smoothie_s2). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
action(make_smoothie_s2, break). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
object_of(make_smoothie_s2, banana). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
uses(make_smoothie_s2, blender). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
before(make_smoothie_s1, make_smoothie_s2). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
action(make_smoothie_s3, add). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
object_of(make_smoothie_s3, strawberry). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
uses(make_smoothie_s3, yogurt). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
uses(make_smoothie_s3, milk). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
before(make_smoothie_s2, make_smoothie_s3). @conf 1 @source teacher:curriculum @conv s1795002256402-760026-20
```

## Taught wording

- task: make a smoothie
- make_smoothie_s1: peel the banana
- make_smoothie_s4: blend the smoothie in the blender until it is smooth
- make_smoothie_s5: pour the smoothie into the glass
- make_smoothie_s2: break the banana into pieces into the blender
- make_smoothie_s3: add the strawberries with the yogurt and the milk
