---
id: renew_passport
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# renew passport

Learned in conversation.

```datalog
task(renew_passport). @conf 1 @source ""
takes_time(renew_passport, 40, 60, minute). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
step(renew_passport, renew_passport_s1). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
action(renew_passport_s1, check). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
object_of(renew_passport_s1, expiry_date). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
uses(renew_passport_s1, passport). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
takes_time(renew_passport_s1, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
step(renew_passport, renew_passport_s2). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
action(renew_passport_s2, get). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
object_of(renew_passport_s2, passport_photo). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
takes_time(renew_passport_s2, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
before(renew_passport_s1, renew_passport_s2). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
step(renew_passport, renew_passport_s3). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
action(renew_passport_s3, fill). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
object_of(renew_passport_s3, application_form). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
takes_time(renew_passport_s3, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
before(renew_passport_s2, renew_passport_s3). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
step(renew_passport, renew_passport_s4). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
action(renew_passport_s4, pay). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
object_of(renew_passport_s4, fee). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
uses(renew_passport_s4, payment_card). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
takes_time(renew_passport_s4, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
before(renew_passport_s3, renew_passport_s4). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
step(renew_passport, renew_passport_s5). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
action(renew_passport_s5, send). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
object_of(renew_passport_s5, application_form). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
object_of(renew_passport_s5, passport_photo). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
object_of(renew_passport_s5, passport). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
takes_time(renew_passport_s5, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
before(renew_passport_s4, renew_passport_s5). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
needs(renew_passport, passport). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
needs(renew_passport, passport_photo). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
needs(renew_passport, application_form). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
needs(renew_passport, payment_card). @conf 1 @source teacher:curriculum @conv s1794946408746-131028-110
action(renew_passport_s1, book). @conf 1 @source teacher:curriculum @conv s1794946631710-131028-291
object_of(renew_passport_s1, fast_track_appointment). @conf 1 @source teacher:curriculum @conv s1794946631710-131028-291
action(renew_passport_s2, take). @conf 1 @source teacher:curriculum @conv s1794946631710-131028-291
object_of(renew_passport_s2, form). @conf 1 @source teacher:curriculum @conv s1794946631710-131028-291
object_of(renew_passport_s2, passport). @conf 1 @source teacher:curriculum @conv s1794946631710-131028-291
uses(renew_passport_s2, fast_track_appointment). @conf 1 @source teacher:curriculum @conv s1794946631710-131028-291
action(renew_passport_s3, pay). @conf 1 @source teacher:curriculum @conv s1794946631710-131028-291
object_of(renew_passport_s3, fee). @conf 1 @source teacher:curriculum @conv s1794946631710-131028-291
uses(renew_passport_s3, payment_card). @conf 1 @source teacher:curriculum @conv s1794946631710-131028-291
```
