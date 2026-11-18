---
id: invite_friends_to_dinner
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Invite friends to dinner

Pick a date, send invitations and gather replies and dietary needs.

```datalog
task(invite_friends_to_dinner).
needs(invite_friends_to_dinner, phone).
needs(invite_friends_to_dinner, calendar).
needs(invite_friends_to_dinner, guest_list).
takes_time(invite_friends_to_dinner, 15, 30, minute).
step(invite_friends_to_dinner, invite_friends_to_dinner_s1).
action(invite_friends_to_dinner_s1, choose).
object_of(invite_friends_to_dinner_s1, date).
uses(invite_friends_to_dinner_s1, calendar).
takes_time(invite_friends_to_dinner_s1, 2, 5, minute).
step(invite_friends_to_dinner, invite_friends_to_dinner_s2).
action(invite_friends_to_dinner_s2, write).
object_of(invite_friends_to_dinner_s2, guest_list).
takes_time(invite_friends_to_dinner_s2, 5, 10, minute).
step(invite_friends_to_dinner, invite_friends_to_dinner_s3).
action(invite_friends_to_dinner_s3, send).
object_of(invite_friends_to_dinner_s3, invitation).
uses(invite_friends_to_dinner_s3, phone).
uses(invite_friends_to_dinner_s3, guest_list).
takes_time(invite_friends_to_dinner_s3, 5, 10, minute).
step(invite_friends_to_dinner, invite_friends_to_dinner_s4).
action(invite_friends_to_dinner_s4, collect).
object_of(invite_friends_to_dinner_s4, reply).
uses(invite_friends_to_dinner_s4, phone).
until(invite_friends_to_dinner_s4, answered).
step(invite_friends_to_dinner, invite_friends_to_dinner_s5).
action(invite_friends_to_dinner_s5, ask).
object_of(invite_friends_to_dinner_s5, allergy).
uses(invite_friends_to_dinner_s5, phone).
takes_time(invite_friends_to_dinner_s5, 2, 5, minute).
before(invite_friends_to_dinner_s1, invite_friends_to_dinner_s2).
before(invite_friends_to_dinner_s2, invite_friends_to_dinner_s3).
before(invite_friends_to_dinner_s3, invite_friends_to_dinner_s4).
before(invite_friends_to_dinner_s3, invite_friends_to_dinner_s5).
```
