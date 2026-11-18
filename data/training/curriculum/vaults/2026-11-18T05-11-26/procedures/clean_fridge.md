---
id: clean_fridge
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# clean fridge

Learned in conversation.

```datalog
task(clean_fridge). @conf 1 @source ""
needs(clean_fridge, cool_box). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
needs(clean_fridge, bin_bag). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
needs(clean_fridge, dish_soap). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
needs(clean_fridge, water). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
needs(clean_fridge, baking_soda). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
needs(clean_fridge, sponge). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
needs(clean_fridge, towel). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
takes_time(clean_fridge, 40, 60, minute). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
step(clean_fridge, clean_fridge_s1). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
action(clean_fridge_s1, take). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
object_of(clean_fridge_s1, food). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
step(clean_fridge, clean_fridge_s2). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
action(clean_fridge_s2, put). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
object_of(clean_fridge_s2, food). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
uses(clean_fridge_s2, cool_box). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
takes_time(clean_fridge_s2, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
before(clean_fridge_s1, clean_fridge_s2). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
action(clean_fridge_s2, throw). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
uses(clean_fridge_s2, bin_bag). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
takes_time(clean_fridge_s2, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
step(clean_fridge, clean_fridge_s5). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
action(clean_fridge_s5, wipe). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
object_of(clean_fridge_s5, fridge). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
uses(clean_fridge_s5, sponge). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
uses(clean_fridge_s5, baking_soda). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
uses(clean_fridge_s5, water). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
takes_time(clean_fridge_s5, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
before(clean_fridge_s2, clean_fridge_s5). @conf 1 @source teacher:curriculum @conv s1794978698016-85704-25
```
