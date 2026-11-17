---
id: clean_shoe
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# clean shoe

Learned in conversation.

```datalog
task(clean_shoe). @conf 1 @source ""
needs(clean_shoe, suede_brush). @conf 1 @source teacher:curriculum @conv s1794946584066-131028-236
needs(clean_shoe, suede_eraser). @conf 1 @source teacher:curriculum @conv s1794946584066-131028-236
needs(clean_shoe, newspaper). @conf 1 @source teacher:curriculum @conv s1794946584066-131028-236
step(clean_shoe, clean_shoe_s1). @conf 1 @source teacher:curriculum @conv s1794946584066-131028-236
action(clean_shoe_s1, brush). @conf 1 @source teacher:curriculum @conv s1794946584066-131028-236
object_of(clean_shoe_s1, nap). @conf 1 @source teacher:curriculum @conv s1794946584066-131028-236
uses(clean_shoe_s1, suede_brush). @conf 1 @source teacher:curriculum @conv s1794946584066-131028-236
takes_time(clean_shoe_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946584066-131028-236
step(clean_shoe, clean_shoe_s2). @conf 1 @source teacher:curriculum @conv s1794946584066-131028-236
action(clean_shoe_s2, rub). @conf 1 @source teacher:curriculum @conv s1794946584066-131028-236
object_of(clean_shoe_s2, mark). @conf 1 @source teacher:curriculum @conv s1794946584066-131028-236
uses(clean_shoe_s2, suede_eraser). @conf 1 @source teacher:curriculum @conv s1794946584066-131028-236
before(clean_shoe_s1, clean_shoe_s2). @conf 1 @source teacher:curriculum @conv s1794946584066-131028-236
```
