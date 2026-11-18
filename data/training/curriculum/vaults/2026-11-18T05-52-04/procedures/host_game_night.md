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
needs(host_game_night, board_game). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
needs(host_game_night, snack). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
needs(host_game_night, drink). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
needs(host_game_night, table). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
takes_time(host_game_night, 60, 90, minute). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
step(host_game_night, host_game_night_s1). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
action(host_game_night_s1, choose). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
object_of(host_game_night_s1, date). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
takes_time(host_game_night_s1, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
step(host_game_night, host_game_night_s2). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
action(host_game_night_s2, invite). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
object_of(host_game_night_s2, friend). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
takes_time(host_game_night_s2, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
before(host_game_night_s1, host_game_night_s2). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
step(host_game_night, host_game_night_s3). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
action(host_game_night_s3, choose). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
object_of(host_game_night_s3, board_game). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
takes_time(host_game_night_s3, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
before(host_game_night_s2, host_game_night_s3). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
step(host_game_night, host_game_night_s4). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
action(host_game_night_s4, buy). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
object_of(host_game_night_s4, snack). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
object_of(host_game_night_s4, drink). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
takes_time(host_game_night_s4, 30, 45, minute). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
before(host_game_night_s3, host_game_night_s4). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
step(host_game_night, host_game_night_s5). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
action(host_game_night_s5, clear). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
object_of(host_game_night_s5, table). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
takes_time(host_game_night_s5, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
step(host_game_night, host_game_night_s6). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
action(host_game_night_s6, set_out). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
object_of(host_game_night_s6, game). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
object_of(host_game_night_s6, snack). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
object_of(host_game_night_s6, drink). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
before(host_game_night_s5, host_game_night_s6). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
before(host_game_night_s4, host_game_night_s5). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
step(host_game_night, host_game_night_s7). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
action(host_game_night_s7, clear). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
object_of(host_game_night_s7, table). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
step(host_game_night, host_game_night_s8). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
action(host_game_night_s8, set_out). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
object_of(host_game_night_s8, game). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
object_of(host_game_night_s8, snack). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
object_of(host_game_night_s8, drink). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
takes_time(host_game_night_s8, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
before(host_game_night_s7, host_game_night_s8). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
before(host_game_night_s6, host_game_night_s7). @conf 1 @source teacher:curriculum @conv s1794981129502-218805-21
step(host_game_night, host_game_night_s9). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
action(host_game_night_s9, choose). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
object_of(host_game_night_s9, date). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
takes_time(host_game_night_s9, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
before(host_game_night_s8, host_game_night_s9). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
step(host_game_night, host_game_night_s10). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
action(host_game_night_s10, invite). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
object_of(host_game_night_s10, friend). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
uses(host_game_night_s10, message). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
takes_time(host_game_night_s10, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
before(host_game_night_s9, host_game_night_s10). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
step(host_game_night, host_game_night_s11). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
action(host_game_night_s11, choose). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
object_of(host_game_night_s11, board_game). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
takes_time(host_game_night_s11, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
before(host_game_night_s10, host_game_night_s11). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
step(host_game_night, host_game_night_s12). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
action(host_game_night_s12, buy). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
object_of(host_game_night_s12, snack). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
object_of(host_game_night_s12, drink). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
takes_time(host_game_night_s12, 30, 45, minute). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
before(host_game_night_s11, host_game_night_s12). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
step(host_game_night, host_game_night_s13). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
action(host_game_night_s13, clear). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
object_of(host_game_night_s13, table). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
step(host_game_night, host_game_night_s14). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
action(host_game_night_s14, set_out). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
object_of(host_game_night_s14, game). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
object_of(host_game_night_s14, snack). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
object_of(host_game_night_s14, drink). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
takes_time(host_game_night_s14, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
before(host_game_night_s13, host_game_night_s14). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
before(host_game_night_s12, host_game_night_s13). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
step(host_game_night, host_game_night_s15). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
action(host_game_night_s15, clear). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
object_of(host_game_night_s15, table). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
step(host_game_night, host_game_night_s16). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
action(host_game_night_s16, set_out). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
object_of(host_game_night_s16, game). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
object_of(host_game_night_s16, snack). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
object_of(host_game_night_s16, drink). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
takes_time(host_game_night_s16, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
before(host_game_night_s15, host_game_night_s16). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
before(host_game_night_s14, host_game_night_s15). @conf 1 @source teacher:curriculum @conv s1794981263958-218805-338
```
