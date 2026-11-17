---
id: organise_meeting
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# organise meeting

Learned in conversation.

```datalog
task(organise_meeting). @conf 1 @source ""
needs(organise_meeting, book). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
needs(organise_meeting, notebook). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
needs(organise_meeting, snack). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
needs(organise_meeting, drink). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
takes_time(organise_meeting, 35, 50, minute). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
step(organise_meeting, organise_meeting_s1). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
action(organise_meeting_s1, choose). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
object_of(organise_meeting_s1, book). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
takes_time(organise_meeting_s1, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
uses(organise_meeting_s1, group). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
step(organise_meeting, organise_meeting_s2). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
action(organise_meeting_s2, set). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
object_of(organise_meeting_s2, date). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
object_of(organise_meeting_s2, place). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
takes_time(organise_meeting_s2, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
before(organise_meeting_s1, organise_meeting_s2). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
step(organise_meeting, organise_meeting_s3). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
action(organise_meeting_s3, send). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
object_of(organise_meeting_s3, reminder). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
takes_time(organise_meeting_s3, 5, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
before(organise_meeting_s2, organise_meeting_s3). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
step(organise_meeting, organise_meeting_s4). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
action(organise_meeting_s4, write). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
object_of(organise_meeting_s4, discussion_question). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
uses(organise_meeting_s4, notebook). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
takes_time(organise_meeting_s4, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
before(organise_meeting_s3, organise_meeting_s4). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
step(organise_meeting, organise_meeting_s5). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
action(organise_meeting_s5, prepare). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
object_of(organise_meeting_s5, snack). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
object_of(organise_meeting_s5, drink). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
takes_time(organise_meeting_s5, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
before(organise_meeting_s4, organise_meeting_s5). @conf 1 @source teacher:curriculum @conv s1794946384669-131028-100
needs(organise_meeting, sign_up_list). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
needs(organise_meeting, plate). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
needs(organise_meeting, cutlery). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
needs(organise_meeting, serving_spoon). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
needs(organise_meeting, table). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
takes_time(organise_meeting, 40, 60, minute). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
object_of(organise_meeting_s1, date). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
action(organise_meeting_s2, choose). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
action(organise_meeting_s2, invite). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
object_of(organise_meeting_s2, guest). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
takes_time(organise_meeting_s2, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
uses(organise_meeting_s2, dish). @conf 1 @source teacher:curriculum @conv s1794946424843-131028-122
needs(organise_meeting, laptop). @conf 1 @source teacher:curriculum @conv s1794946601115-131028-256
object_of(organise_meeting_s3, video_call_link). @conf 1 @source teacher:curriculum @conv s1794946601115-131028-256
takes_time(organise_meeting_s3, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946601115-131028-256
```
