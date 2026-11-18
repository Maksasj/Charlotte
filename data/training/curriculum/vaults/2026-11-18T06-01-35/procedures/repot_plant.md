---
id: repot_plant
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# repot plant

Learned in conversation.

```datalog
task(repot_plant). @conf 1 @source ""
needs(repot_plant, bigger_pot). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
needs(repot_plant, potting_soil). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
needs(repot_plant, trowel). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
needs(repot_plant, water). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
needs(repot_plant, newspaper). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
takes_time(repot_plant, 12, 18, minute). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
step(repot_plant, repot_plant_s1). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
action(repot_plant_s1, spread). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
object_of(repot_plant_s1, newspaper). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
takes_time(repot_plant_s1, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
step(repot_plant, repot_plant_s2). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
action(repot_plant_s2, remove). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
object_of(repot_plant_s2, plant). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
takes_time(repot_plant_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
uses(repot_plant_s2, pot). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
before(repot_plant_s1, repot_plant_s2). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
step(repot_plant, repot_plant_s3). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
action(repot_plant_s3, loosen). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
object_of(repot_plant_s3, root). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
uses(repot_plant_s3, finger). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
takes_time(repot_plant_s3, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
before(repot_plant_s2, repot_plant_s3). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
step(repot_plant, repot_plant_s4). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
action(repot_plant_s4, put). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
object_of(repot_plant_s4, potting_soil). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
uses(repot_plant_s4, pot). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
takes_time(repot_plant_s4, 4, 5, minute). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
step(repot_plant, repot_plant_s5). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
action(repot_plant_s5, set). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
object_of(repot_plant_s5, plant). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
uses(repot_plant_s5, pot). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
before(repot_plant_s4, repot_plant_s5). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
step(repot_plant, repot_plant_s6). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
action(repot_plant_s6, fill). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
object_of(repot_plant_s6, plant). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
uses(repot_plant_s6, trowel). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
overlaps(repot_plant_s6, repot_plant_s5). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
before(repot_plant_s3, repot_plant_s4). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
action(repot_plant_s5, water). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
until(repot_plant_s5, water_drains_from_bottom). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
takes_time(repot_plant_s5, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
uses(repot_plant_s4, trowel). @conf 1 @source teacher:curriculum @conv s1794981753277-278172-155
```
