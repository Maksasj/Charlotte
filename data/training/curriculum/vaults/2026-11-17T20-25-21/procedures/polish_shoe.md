---
id: polish_shoe
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# polish shoe

Learned in conversation.

```datalog
task(polish_shoe). @conf 1 @source ""
needs(polish_shoe, shoe_polish). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
needs(polish_shoe, shoe_brush). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
needs(polish_shoe, cloth). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
needs(polish_shoe, newspaper). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
takes_time(polish_shoe, 15, 25, minute). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
step(polish_shoe, polish_shoe_s1). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
action(polish_shoe_s1, take_out). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
object_of(polish_shoe_s1, lace). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
step(polish_shoe, polish_shoe_s2). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
action(polish_shoe_s2, set). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
object_of(polish_shoe_s2, shoe). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
uses(polish_shoe_s2, newspaper). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
takes_time(polish_shoe_s2, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
before(polish_shoe_s1, polish_shoe_s2). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
action(polish_shoe_s2, brush). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
uses(polish_shoe_s2, shoe_brush). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
object_of(polish_shoe_s2, dirt). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
takes_time(polish_shoe_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
step(polish_shoe, polish_shoe_s3). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
action(polish_shoe_s3, rub). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
object_of(polish_shoe_s3, shoe_polish). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
uses(polish_shoe_s3, cloth). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
takes_time(polish_shoe_s3, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
before(polish_shoe_s2, polish_shoe_s3). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
step(polish_shoe, polish_shoe_s4). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
action(polish_shoe_s4, dry). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
object_of(polish_shoe_s4, shoe_polish). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
takes_time(polish_shoe_s4, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
before(polish_shoe_s3, polish_shoe_s4). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
step(polish_shoe, polish_shoe_s5). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
action(polish_shoe_s5, buff). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
object_of(polish_shoe_s5, shoe). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
uses(polish_shoe_s5, shoe_brush). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
until(polish_shoe_s5, shine). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
takes_time(polish_shoe_s5, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
before(polish_shoe_s4, polish_shoe_s5). @conf 1 @source teacher:curriculum @conv s1794946423332-131028-120
```
