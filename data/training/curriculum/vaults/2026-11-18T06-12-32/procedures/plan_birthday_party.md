---
id: plan_birthday_party
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# plan birthday party

Learned in conversation.

```datalog
task(plan_birthday_party). @conf 1 @source ""
takes_time(plan_birthday_party, 120, 180, minute). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
step(plan_birthday_party, plan_birthday_party_s1). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
action(plan_birthday_party_s1, choose). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
object_of(plan_birthday_party_s1, date). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
takes_time(plan_birthday_party_s1, 10, 10, minute). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
step(plan_birthday_party, plan_birthday_party_s2). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
action(plan_birthday_party_s2, write). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
object_of(plan_birthday_party_s2, guest_list). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
takes_time(plan_birthday_party_s2, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
before(plan_birthday_party_s1, plan_birthday_party_s2). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
step(plan_birthday_party, plan_birthday_party_s3). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
action(plan_birthday_party_s3, send). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
object_of(plan_birthday_party_s3, invitation). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
takes_time(plan_birthday_party_s3, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
before(plan_birthday_party_s2, plan_birthday_party_s3). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
step(plan_birthday_party, plan_birthday_party_s4). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
action(plan_birthday_party_s4, order). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
object_of(plan_birthday_party_s4, cake). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
takes_time(plan_birthday_party_s4, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
before(plan_birthday_party_s3, plan_birthday_party_s4). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
step(plan_birthday_party, plan_birthday_party_s5). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
action(plan_birthday_party_s5, buy). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
object_of(plan_birthday_party_s5, balloon). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
object_of(plan_birthday_party_s5, decoration). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
takes_time(plan_birthday_party_s5, 30, 45, minute). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
before(plan_birthday_party_s4, plan_birthday_party_s5). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
step(plan_birthday_party, plan_birthday_party_s6). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
action(plan_birthday_party_s6, decorate). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
object_of(plan_birthday_party_s6, room). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
uses(plan_birthday_party_s6, balloon). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
takes_time(plan_birthday_party_s6, 30, 55, minute). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
before(plan_birthday_party_s5, plan_birthday_party_s6). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
needs(plan_birthday_party, invitation). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
needs(plan_birthday_party, cake). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
needs(plan_birthday_party, balloon). @conf 1 @source teacher:curriculum @conv s1794982629643-336211-457
```
