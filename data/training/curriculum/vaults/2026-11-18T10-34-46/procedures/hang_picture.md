---
id: hang_picture
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# hang picture

Learned in conversation.

```datalog
task(hang_picture). @conf 1 @source ""
needs(hang_picture, picture). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
needs(hang_picture, hammer). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
needs(hang_picture, nail). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
needs(hang_picture, pencil). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
needs(hang_picture, tape_measure). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
takes_time(hang_picture, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
step(hang_picture, hang_picture_s1). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
action(hang_picture_s1, measure). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
object_of(hang_picture_s1, wall). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
uses(hang_picture_s1, tape_measure). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
takes_time(hang_picture_s1, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
step(hang_picture, hang_picture_s2). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
action(hang_picture_s2, mark). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
object_of(hang_picture_s2, spot). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
uses(hang_picture_s2, pencil). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
before(hang_picture_s1, hang_picture_s2). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
step(hang_picture, hang_picture_s3). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
action(hang_picture_s3, hammer). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
object_of(hang_picture_s3, nail). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
takes_time(hang_picture_s3, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
before(hang_picture_s2, hang_picture_s3). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
step(hang_picture, hang_picture_s4). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
action(hang_picture_s4, hang). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
object_of(hang_picture_s4, picture). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
uses(hang_picture_s4, nail). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
before(hang_picture_s3, hang_picture_s4). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
step(hang_picture, hang_picture_s5). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
action(hang_picture_s5, straighten). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
object_of(hang_picture_s5, picture). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
until(hang_picture_s5, level). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
before(hang_picture_s4, hang_picture_s5). @conf 1 @source teacher:curriculum @conv s1794998096639-490820-46
```
