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
needs(organise_potluck, sign_up_list). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
needs(organise_potluck, plate). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
needs(organise_potluck, cutlery). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
needs(organise_potluck, serving_spoon). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
needs(organise_potluck, table). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
takes_time(organise_potluck, 40, 60, minute). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
step(organise_potluck, organise_potluck_s1). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
action(organise_potluck_s1, choose). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
object_of(organise_potluck_s1, date). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
object_of(organise_potluck_s1, place). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
takes_time(organise_potluck_s1, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
step(organise_potluck, organise_potluck_s2). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
action(organise_potluck_s2, invite). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
object_of(organise_potluck_s2, guest). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
step(organise_potluck, organise_potluck_s3). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
action(organise_potluck_s3, ask). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
object_of(organise_potluck_s3, guest). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
takes_time(organise_potluck_s3, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
before(organise_potluck_s2, organise_potluck_s3). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
before(organise_potluck_s1, organise_potluck_s2). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
action(organise_potluck_s3, make). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
object_of(organise_potluck_s3, sign_up_list). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
action(organise_potluck_s1, set_out). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
object_of(organise_potluck_s1, plate). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
object_of(organise_potluck_s1, cutlery). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
object_of(organise_potluck_s1, serving_spoon). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
takes_time(organise_potluck_s1, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
step(organise_potluck, organise_potluck_s4). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
action(organise_potluck_s4, set_out). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
object_of(organise_potluck_s4, plate). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
object_of(organise_potluck_s4, cutlery). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
object_of(organise_potluck_s4, serving_spoon). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
takes_time(organise_potluck_s4, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
before(organise_potluck_s3, organise_potluck_s4). @conf 1 @source teacher:curriculum @conv s1794978806497-85704-111
```
