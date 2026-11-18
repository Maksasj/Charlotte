---
id: make_apple_crumble
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make apple crumble

Learned in conversation.

```datalog
task(make_apple_crumble). @conf 1 @source ""
needs(make_apple_crumble, apple). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
needs(make_apple_crumble, flour). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
needs(make_apple_crumble, butter). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
needs(make_apple_crumble, sugar). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
needs(make_apple_crumble, knife). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
needs(make_apple_crumble, bowl). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
needs(make_apple_crumble, baking_dish). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
needs(make_apple_crumble, oven). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
takes_time(make_apple_crumble, 48, 55, minute). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
step(make_apple_crumble, make_apple_crumble_s1). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
action(make_apple_crumble_s1, preheat). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
object_of(make_apple_crumble_s1, oven). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
takes_time(make_apple_crumble_s1, 10, 12, minute). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
overlaps(make_apple_crumble_s2, make_apple_crumble_s1). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
step(make_apple_crumble, make_apple_crumble_s2). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
action(make_apple_crumble_s2, peel). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
object_of(make_apple_crumble_s2, apple). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
uses(make_apple_crumble_s2, knife). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
takes_time(make_apple_crumble_s2, 8, 10, minute). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
step(make_apple_crumble, make_apple_crumble_s3). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
action(make_apple_crumble_s3, slice). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
object_of(make_apple_crumble_s3, apple). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
uses(make_apple_crumble_s3, knife). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
overlaps(make_apple_crumble_s3, make_apple_crumble_s1). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
takes_time(make_apple_crumble_s3, 8, 10, minute). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
action(make_apple_crumble_s3, rub). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
object_of(make_apple_crumble_s3, butter). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
uses(make_apple_crumble_s3, flour). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
uses(make_apple_crumble_s3, sugar). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
uses(make_apple_crumble_s3, bowl). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
takes_time(make_apple_crumble_s3, 5, 6, minute). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
before(make_apple_crumble_s2, make_apple_crumble_s3). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
step(make_apple_crumble, make_apple_crumble_s4). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
action(make_apple_crumble_s4, spread). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
object_of(make_apple_crumble_s4, apple). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
uses(make_apple_crumble_s4, baking_dish). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
takes_time(make_apple_crumble_s4, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
before(make_apple_crumble_s3, make_apple_crumble_s4). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
before(make_apple_crumble_s1, make_apple_crumble_s4). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
step(make_apple_crumble, make_apple_crumble_s5). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
action(make_apple_crumble_s5, bake). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
object_of(make_apple_crumble_s5, crumble). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
uses(make_apple_crumble_s5, oven). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
until(make_apple_crumble_s5, golden). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
takes_time(make_apple_crumble_s5, 30, 35, minute). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
before(make_apple_crumble_s4, make_apple_crumble_s5). @conf 1 @source teacher:curriculum @conv s1794982352654-336211-20
```
