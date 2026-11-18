---
id: remove_wine_stain
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# remove wine stain

Learned in conversation.

```datalog
task(remove_wine_stain). @conf 1 @source ""
needs(remove_wine_stain, paper_towel). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
needs(remove_wine_stain, salt). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
needs(remove_wine_stain, water). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
needs(remove_wine_stain, dish_soap). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
takes_time(remove_wine_stain, 13, 20, minute). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
step(remove_wine_stain, remove_wine_stain_s1). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
action(remove_wine_stain_s1, blot). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
object_of(remove_wine_stain_s1, wine_stain). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
uses(remove_wine_stain_s1, paper_towel). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
takes_time(remove_wine_stain_s1, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
step(remove_wine_stain, remove_wine_stain_s2). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
action(remove_wine_stain_s2, cover). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
object_of(remove_wine_stain_s2, wine_stain). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
uses(remove_wine_stain_s2, salt). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
takes_time(remove_wine_stain_s2, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
until(remove_wine_stain_s2, soak_up_wine). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
before(remove_wine_stain_s1, remove_wine_stain_s2). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
step(remove_wine_stain, remove_wine_stain_s3). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
action(remove_wine_stain_s3, rinse). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
object_of(remove_wine_stain_s3, wine_stain). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
uses(remove_wine_stain_s3, water). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
takes_time(remove_wine_stain_s3, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
before(remove_wine_stain_s2, remove_wine_stain_s3). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
step(remove_wine_stain, remove_wine_stain_s4). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
action(remove_wine_stain_s4, rub). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
object_of(remove_wine_stain_s4, stain). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
uses(remove_wine_stain_s4, dish_soap). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
takes_time(remove_wine_stain_s4, 5, 5, minute). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
before(remove_wine_stain_s3, remove_wine_stain_s4). @conf 1 @source teacher:curriculum @conv s1794982398596-336211-147
```
