---
id: bleed_radiator
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# bleed radiator

Learned in conversation.

```datalog
task(bleed_radiator). @conf 1 @source ""
needs(bleed_radiator, radiator_key). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
needs(bleed_radiator, cloth). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
takes_time(bleed_radiator, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
step(bleed_radiator, bleed_radiator_s1). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
action(bleed_radiator_s1, switch_off). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
object_of(bleed_radiator_s1, heating). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
step(bleed_radiator, bleed_radiator_s2). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
action(bleed_radiator_s2, cool). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
object_of(bleed_radiator_s2, radiator). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
takes_time(bleed_radiator_s2, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
before(bleed_radiator_s1, bleed_radiator_s2). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
action(bleed_radiator_s2, hold). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
object_of(bleed_radiator_s2, cloth). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
uses(bleed_radiator_s2, cloth). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
step(bleed_radiator, bleed_radiator_s3). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
action(bleed_radiator_s3, open). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
object_of(bleed_radiator_s3, bleed_valve). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
uses(bleed_radiator_s3, radiator_key). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
until(bleed_radiator_s3, water_drip). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
takes_time(bleed_radiator_s3, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
before(bleed_radiator_s2, bleed_radiator_s3). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
step(bleed_radiator, bleed_radiator_s4). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
action(bleed_radiator_s4, close). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
object_of(bleed_radiator_s4, bleed_valve). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
uses(bleed_radiator_s4, radiator_key). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
takes_time(bleed_radiator_s4, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
before(bleed_radiator_s3, bleed_radiator_s4). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
step(bleed_radiator, bleed_radiator_s5). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
action(bleed_radiator_s5, check). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
object_of(bleed_radiator_s5, boiler_pressure). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
step(bleed_radiator, bleed_radiator_s6). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
action(bleed_radiator_s6, switch). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
object_of(bleed_radiator_s6, heating). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
takes_time(bleed_radiator_s6, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
before(bleed_radiator_s5, bleed_radiator_s6). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
before(bleed_radiator_s4, bleed_radiator_s5). @conf 1 @source teacher:curriculum @conv s1794946280424-131028-19
action(bleed_radiator_s3, start). @conf 1 @source teacher:curriculum @conv s1794946610020-131028-265
object_of(bleed_radiator_s3, radiator). @conf 1 @source teacher:curriculum @conv s1794946610020-131028-265
action(bleed_radiator_s3, check). @conf 1 @source teacher:curriculum @conv s1794946610020-131028-265
object_of(bleed_radiator_s3, boiler_pressure). @conf 1 @source teacher:curriculum @conv s1794946610020-131028-265
action(bleed_radiator_s4, switch). @conf 1 @source teacher:curriculum @conv s1794946610020-131028-265
object_of(bleed_radiator_s4, heating). @conf 1 @source teacher:curriculum @conv s1794946610020-131028-265
takes_time(bleed_radiator_s4, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946610020-131028-265
```
