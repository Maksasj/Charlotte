---
id: make_flatbread_pizza
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make flatbread pizza

Learned in conversation.

```datalog
task(make_flatbread_pizza). @conf 1 @source ""
needs(make_flatbread_pizza, flatbread). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
needs(make_flatbread_pizza, tomato_sauce). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
needs(make_flatbread_pizza, mozzarella). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
needs(make_flatbread_pizza, basil). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
needs(make_flatbread_pizza, baking_tray). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
needs(make_flatbread_pizza, oven). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
takes_time(make_flatbread_pizza, 18, 24, minute). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
step(make_flatbread_pizza, make_flatbread_pizza_s1). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
action(make_flatbread_pizza_s1, preheat). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
object_of(make_flatbread_pizza_s1, oven). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
takes_time(make_flatbread_pizza_s1, 10, 12, minute). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
uses(make_flatbread_pizza_s1, oven). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
step(make_flatbread_pizza, make_flatbread_pizza_s2). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
action(make_flatbread_pizza_s2, spread). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
object_of(make_flatbread_pizza_s2, tomato_sauce). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
uses(make_flatbread_pizza_s2, baking_tray). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
takes_time(make_flatbread_pizza_s2, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
overlaps(make_flatbread_pizza_s2, make_flatbread_pizza_s1). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
step(make_flatbread_pizza, make_flatbread_pizza_s3). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
action(make_flatbread_pizza_s3, tear). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
object_of(make_flatbread_pizza_s3, mozzarella). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
takes_time(make_flatbread_pizza_s3, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
overlaps(make_flatbread_pizza_s1, make_flatbread_pizza_s3). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
before(make_flatbread_pizza_s2, make_flatbread_pizza_s3). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
before(make_flatbread_pizza_s1, make_flatbread_pizza_s3). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
step(make_flatbread_pizza, make_flatbread_pizza_s4). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
action(make_flatbread_pizza_s4, bake). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
object_of(make_flatbread_pizza_s4, flatbread). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
uses(make_flatbread_pizza_s4, oven). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
until(make_flatbread_pizza_s4, cheese_bubbles). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
takes_time(make_flatbread_pizza_s4, 8, 10, minute). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
before(make_flatbread_pizza_s3, make_flatbread_pizza_s4). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
step(make_flatbread_pizza, make_flatbread_pizza_s5). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
action(make_flatbread_pizza_s5, scatter). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
object_of(make_flatbread_pizza_s5, basil). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
before(make_flatbread_pizza_s4, make_flatbread_pizza_s5). @conf 1 @source teacher:curriculum @conv s1794982363723-336211-49
```
