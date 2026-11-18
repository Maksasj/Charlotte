---
id: make_mulled_wine
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make mulled wine

Learned in conversation.

```datalog
task(make_mulled_wine). @conf 1 @source ""
step(make_mulled_wine, make_mulled_wine_s1). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
needs(make_mulled_wine, red_wine). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
needs(make_mulled_wine, orange). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
needs(make_mulled_wine, cinnamon_stick). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
needs(make_mulled_wine, clove). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
needs(make_mulled_wine, sugar). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
needs(make_mulled_wine, knife). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
needs(make_mulled_wine, saucepan). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
needs(make_mulled_wine, ladle). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
needs(make_mulled_wine, mug). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
takes_time(make_mulled_wine, 18, 24, minute). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
action(make_mulled_wine_s1, slice). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
object_of(make_mulled_wine_s1, orange). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
uses(make_mulled_wine_s1, knife). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
takes_time(make_mulled_wine_s1, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
step(make_mulled_wine, make_mulled_wine_s2). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
action(make_mulled_wine_s2, pour). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
object_of(make_mulled_wine_s2, red_wine). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
uses(make_mulled_wine_s2, saucepan). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
before(make_mulled_wine_s1, make_mulled_wine_s2). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
step(make_mulled_wine, make_mulled_wine_s3). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
action(make_mulled_wine_s3, add). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
object_of(make_mulled_wine_s3, orange). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
object_of(make_mulled_wine_s3, cinnamon_stick). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
object_of(make_mulled_wine_s3, clove). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
object_of(make_mulled_wine_s3, sugar). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
before(make_mulled_wine_s2, make_mulled_wine_s3). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
step(make_mulled_wine, make_mulled_wine_s4). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
action(make_mulled_wine_s4, warm). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
object_of(make_mulled_wine_s4, red_wine). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
until(make_mulled_wine_s4, steaming). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
takes_time(make_mulled_wine_s4, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
before(make_mulled_wine_s3, make_mulled_wine_s4). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
step(make_mulled_wine, make_mulled_wine_s5). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
action(make_mulled_wine_s5, ladle). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
object_of(make_mulled_wine_s5, red_wine). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
uses(make_mulled_wine_s5, ladle). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
before(make_mulled_wine_s4, make_mulled_wine_s5). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
action(make_mulled_wine_s1, need). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
object_of(make_mulled_wine_s5, mulled_wine). @conf 1 @source teacher:curriculum @conv s1794982433880-336211-228
```
