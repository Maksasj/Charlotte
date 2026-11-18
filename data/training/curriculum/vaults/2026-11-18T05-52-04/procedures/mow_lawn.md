---
id: mow_lawn
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# mow lawn

Learned in conversation.

```datalog
task(mow_lawn). @conf 1 @source ""
needs(mow_lawn, lawn_mower). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
needs(mow_lawn, garden_shears). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
needs(mow_lawn, rake). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
takes_time(mow_lawn, 50, 70, minute). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
step(mow_lawn, mow_lawn_s1). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
action(mow_lawn_s1, clear). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
object_of(mow_lawn_s1, lawn). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
takes_time(mow_lawn_s1, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
step(mow_lawn, mow_lawn_s2). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
action(mow_lawn_s2, set). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
object_of(mow_lawn_s2, blade). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
uses(mow_lawn_s2, lawn_mower). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
takes_time(mow_lawn_s2, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
before(mow_lawn_s1, mow_lawn_s2). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
step(mow_lawn, mow_lawn_s3). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
action(mow_lawn_s3, mow). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
object_of(mow_lawn_s3, lawn). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
uses(mow_lawn_s3, lawn_mower). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
takes_time(mow_lawn_s3, 30, 40, minute). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
before(mow_lawn_s2, mow_lawn_s3). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
step(mow_lawn, mow_lawn_s4). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
action(mow_lawn_s4, trim). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
object_of(mow_lawn_s4, edge). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
uses(mow_lawn_s4, shears). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
takes_time(mow_lawn_s4, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
before(mow_lawn_s3, mow_lawn_s4). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
step(mow_lawn, mow_lawn_s5). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
action(mow_lawn_s5, rake). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
object_of(mow_lawn_s5, clipping). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
takes_time(mow_lawn_s5, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
before(mow_lawn_s4, mow_lawn_s5). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
needs(mow_lawn, garden_shear). @conf 1 @source teacher:curriculum @conv s1794981148547-218805-70
```
