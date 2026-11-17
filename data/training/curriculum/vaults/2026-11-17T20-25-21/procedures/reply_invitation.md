---
id: reply_invitation
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# reply invitation

Learned in conversation.

```datalog
task(reply_invitation). @conf 1 @source ""
needs(reply_invitation, rsvp_card). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
needs(reply_invitation, pen). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
needs(reply_invitation, stamp). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
needs(reply_invitation, calendar). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
takes_time(reply_invitation, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
step(reply_invitation, reply_invitation_s1). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
action(reply_invitation_s1, check). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
object_of(reply_invitation_s1, date). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
uses(reply_invitation_s1, calendar). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
takes_time(reply_invitation_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
step(reply_invitation, reply_invitation_s2). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
action(reply_invitation_s2, fill). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
object_of(reply_invitation_s2, rsvp_card). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
uses(reply_invitation_s2, pen). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
takes_time(reply_invitation_s2, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
before(reply_invitation_s1, reply_invitation_s2). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
step(reply_invitation, reply_invitation_s3). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
action(reply_invitation_s3, tick). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
object_of(reply_invitation_s3, meal_choice). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
action(reply_invitation_s3, note). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
object_of(reply_invitation_s3, allergy). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
takes_time(reply_invitation_s3, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
before(reply_invitation_s2, reply_invitation_s3). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
step(reply_invitation, reply_invitation_s4). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
action(reply_invitation_s4, stamp). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
object_of(reply_invitation_s4, rsvp_card). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
action(reply_invitation_s4, post). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
takes_time(reply_invitation_s4, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
before(reply_invitation_s3, reply_invitation_s4). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
action(reply_invitation_s4, note). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
object_of(reply_invitation_s4, allergy). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
takes_time(reply_invitation_s4, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946510104-131028-170
action(reply_invitation_s2, tick). @conf 1 @source teacher:curriculum @conv s1794946705218-131028-373
action(reply_invitation_s3, write). @conf 1 @source teacher:curriculum @conv s1794946705218-131028-373
object_of(reply_invitation_s3, note). @conf 1 @source teacher:curriculum @conv s1794946705218-131028-373
action(reply_invitation_s3, skip). @conf 1 @source teacher:curriculum @conv s1794946705218-131028-373
step(reply_invitation, reply_invitation_s5). @conf 1 @source teacher:curriculum @conv s1794946705218-131028-373
action(reply_invitation_s5, post). @conf 1 @source teacher:curriculum @conv s1794946705218-131028-373
object_of(reply_invitation_s5, rsvp_card). @conf 1 @source teacher:curriculum @conv s1794946705218-131028-373
before(reply_invitation_s4, reply_invitation_s5). @conf 1 @source teacher:curriculum @conv s1794946705218-131028-373
```
