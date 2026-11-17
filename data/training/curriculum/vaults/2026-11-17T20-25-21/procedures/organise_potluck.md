---
id: organise_potluck
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# organise potluck

Learned in conversation.

```datalog
task(organise_potluck). @conf 1 @source ""
step(organise_potluck, organise_potluck_s3). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
action(organise_potluck_s3, make). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
object_of(organise_potluck_s3, sign_up_list). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
takes_time(organise_potluck_s3, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
uses(organise_potluck_s3, sign_up_list). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
step(organise_potluck, organise_potluck_s4). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
action(organise_potluck_s4, set_out). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
object_of(organise_potluck_s4, plate). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
object_of(organise_potluck_s4, cutlery). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
object_of(organise_potluck_s4, serving_spoon). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
uses(organise_potluck_s4, table). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
takes_time(organise_potluck_s4, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
before(organise_potluck_s3, organise_potluck_s4). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
step(organise_potluck, organise_potluck_s1). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
action(organise_potluck_s1, choose). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
object_of(organise_potluck_s1, date). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
object_of(organise_potluck_s1, place). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
takes_time(organise_potluck_s1, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
step(organise_potluck, organise_potluck_s2). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
action(organise_potluck_s2, invite). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
object_of(organise_potluck_s2, guest). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
takes_time(organise_potluck_s2, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
before(organise_potluck_s1, organise_potluck_s2). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
needs(organise_potluck, sign_up_list). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
needs(organise_potluck, plate). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
needs(organise_potluck, cutlery). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
needs(organise_potluck, serving_spoon). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
needs(organise_potluck, table). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
before(organise_potluck_s2, organise_potluck_s3). @conf 1 @source teacher:curriculum @conv s1794946761638-131028-424
```
