---
id: clean_microwave
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# clean microwave

Learned in conversation.

```datalog
task(clean_microwave). @conf 1 @source ""
needs(clean_microwave, microwave_safe_bowl). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
needs(clean_microwave, water). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
needs(clean_microwave, lemon). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
needs(clean_microwave, cloth). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
takes_time(clean_microwave, 11, 14, minute). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
step(clean_microwave, clean_microwave_s1). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
action(clean_microwave_s1, fill). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
object_of(clean_microwave_s1, bowl). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
uses(clean_microwave_s1, water). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
uses(clean_microwave_s1, lemon). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
takes_time(clean_microwave_s1, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
step(clean_microwave, clean_microwave_s3). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
action(clean_microwave_s3, leave). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
object_of(clean_microwave_s3, microwave_door). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
takes_time(clean_microwave_s3, 5, 5, minute). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
before(clean_microwave_s1, clean_microwave_s3). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
step(clean_microwave, clean_microwave_s4). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
action(clean_microwave_s4, wipe). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
object_of(clean_microwave_s4, microwave). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
uses(clean_microwave_s4, cloth). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
takes_time(clean_microwave_s4, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
before(clean_microwave_s3, clean_microwave_s4). @conf 1 @source teacher:curriculum @conv s1794981704166-278172-29
```
