---
id: wash_dish
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# wash dish

Learned in conversation.

```datalog
task(wash_dish). @conf 1 @source ""
needs(wash_dish, sink). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
needs(wash_dish, water). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
needs(wash_dish, dish_soap). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
needs(wash_dish, sponge). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
needs(wash_dish, drying_rack). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
takes_time(wash_dish, 15, 25, minute). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
step(wash_dish, wash_dish_s1). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
action(wash_dish_s1, scrape). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
object_of(wash_dish_s1, leftover_food). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
uses(wash_dish_s1, bin). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
takes_time(wash_dish_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
step(wash_dish, wash_dish_s2). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
action(wash_dish_s2, fill). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
object_of(wash_dish_s2, sink). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
uses(wash_dish_s2, hot_water). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
uses(wash_dish_s2, dish_soap). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
takes_time(wash_dish_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
before(wash_dish_s1, wash_dish_s2). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
step(wash_dish, wash_dish_s3). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
action(wash_dish_s3, wash). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
object_of(wash_dish_s3, dish). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
uses(wash_dish_s3, sponge). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
takes_time(wash_dish_s3, 8, 14, minute). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
before(wash_dish_s2, wash_dish_s3). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
step(wash_dish, wash_dish_s4). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
action(wash_dish_s4, rinse). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
object_of(wash_dish_s4, dish). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
uses(wash_dish_s4, water). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
takes_time(wash_dish_s4, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
before(wash_dish_s3, wash_dish_s4). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
step(wash_dish, wash_dish_s5). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
action(wash_dish_s5, put). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
object_of(wash_dish_s5, dish). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
uses(wash_dish_s5, drying_rack). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
until(wash_dish_s5, dry). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
before(wash_dish_s4, wash_dish_s5). @conf 1 @source teacher:curriculum @conv s1794998832166-558984-69
```
