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
needs(clean_microwave, bowl). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
needs(clean_microwave, water). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
needs(clean_microwave, lemon). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
needs(clean_microwave, cloth). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
takes_time(clean_microwave, 11, 14, minute). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
step(clean_microwave, clean_microwave_s1). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
action(clean_microwave_s1, fill). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
object_of(clean_microwave_s1, bowl). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
uses(clean_microwave_s1, water). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
uses(clean_microwave_s1, lemon_juice). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
takes_time(clean_microwave_s1, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
step(clean_microwave, clean_microwave_s2). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
action(clean_microwave_s2, heat). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
object_of(clean_microwave_s2, bowl). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
uses(clean_microwave_s2, microwave). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
until(clean_microwave_s2, steam_up). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
takes_time(clean_microwave_s2, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
before(clean_microwave_s1, clean_microwave_s2). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
step(clean_microwave, clean_microwave_s3). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
action(clean_microwave_s3, leave). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
object_of(clean_microwave_s3, microwave_door). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
takes_time(clean_microwave_s3, 5, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
before(clean_microwave_s2, clean_microwave_s3). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
step(clean_microwave, clean_microwave_s4). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
action(clean_microwave_s4, wipe). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
object_of(clean_microwave_s4, microwave). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
uses(clean_microwave_s4, cloth). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
takes_time(clean_microwave_s4, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
before(clean_microwave_s3, clean_microwave_s4). @conf 1 @source teacher:curriculum @conv s1794946297908-131028-41
needs(clean_microwave, white_vinegar). @conf 1 @source teacher:curriculum @conv s1794946649600-131028-309
uses(clean_microwave_s1, white_vinegar). @conf 1 @source teacher:curriculum @conv s1794946649600-131028-309
until(clean_microwave_s2, steam). @conf 1 @source teacher:curriculum @conv s1794946799735-131028-444
action(clean_microwave_s3, shut). @conf 1 @source teacher:curriculum @conv s1794946799735-131028-444
```
