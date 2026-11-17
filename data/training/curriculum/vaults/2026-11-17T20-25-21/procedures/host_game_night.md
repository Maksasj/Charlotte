---
id: host_game_night
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# host game night

Learned in conversation.

```datalog
task(host_game_night). @conf 1 @source ""
needs(host_game_night, board_game). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
needs(host_game_night, snack). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
needs(host_game_night, drink). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
needs(host_game_night, table). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
takes_time(host_game_night, 60, 90, minute). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
step(host_game_night, host_game_night_s1). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
action(host_game_night_s1, choose). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
object_of(host_game_night_s1, date). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
takes_time(host_game_night_s1, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
step(host_game_night, host_game_night_s2). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
action(host_game_night_s2, invite). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
object_of(host_game_night_s2, friend). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
takes_time(host_game_night_s2, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
before(host_game_night_s1, host_game_night_s2). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
step(host_game_night, host_game_night_s3). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
action(host_game_night_s3, choose). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
object_of(host_game_night_s3, board_game). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
takes_time(host_game_night_s3, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
before(host_game_night_s2, host_game_night_s3). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
step(host_game_night, host_game_night_s4). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
action(host_game_night_s4, buy). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
object_of(host_game_night_s4, snack). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
object_of(host_game_night_s4, drink). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
takes_time(host_game_night_s4, 30, 45, minute). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
before(host_game_night_s3, host_game_night_s4). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
step(host_game_night, host_game_night_s5). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
action(host_game_night_s5, clear). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
object_of(host_game_night_s5, table). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
action(host_game_night_s5, set_out). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
object_of(host_game_night_s5, game). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
object_of(host_game_night_s5, snack). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
object_of(host_game_night_s5, drink). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
takes_time(host_game_night_s5, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
before(host_game_night_s4, host_game_night_s5). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
action(host_game_night_s1, clear). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
object_of(host_game_night_s1, table). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
action(host_game_night_s2, set_out). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
object_of(host_game_night_s2, game). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
action(host_game_night_s3, set_out). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
object_of(host_game_night_s3, snack). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
action(host_game_night_s4, set_out). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
takes_time(host_game_night_s1, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794946287610-131028-24
object_of(host_game_night_s3, party_game). @conf 1 @source teacher:curriculum @conv s1794946559707-131028-217
```
