---
id: fix_tap
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# fix tap

Learned in conversation.

```datalog
task(fix_tap). @conf 1 @source ""
needs(fix_tap, adjustable_spanner). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
needs(fix_tap, screwdriver). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
needs(fix_tap, washer). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
takes_time(fix_tap, 15, 25, minute). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
step(fix_tap, fix_tap_s1). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
action(fix_tap_s1, shut_off). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
object_of(fix_tap_s1, water_supply). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
takes_time(fix_tap_s1, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
step(fix_tap, fix_tap_s2). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
action(fix_tap_s2, remove). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
object_of(fix_tap_s2, tap_handle). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
uses(fix_tap_s2, screwdriver). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
takes_time(fix_tap_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
before(fix_tap_s1, fix_tap_s2). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
step(fix_tap, fix_tap_s3). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
action(fix_tap_s3, unscrew). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
object_of(fix_tap_s3, valve). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
uses(fix_tap_s3, spanner). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
takes_time(fix_tap_s3, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
before(fix_tap_s2, fix_tap_s3). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
step(fix_tap, fix_tap_s4). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
action(fix_tap_s4, replace). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
object_of(fix_tap_s4, washer). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
takes_time(fix_tap_s4, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
before(fix_tap_s3, fix_tap_s4). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
step(fix_tap, fix_tap_s6). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
action(fix_tap_s6, open). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
object_of(fix_tap_s6, water_supply). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
step(fix_tap, fix_tap_s7). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
action(fix_tap_s7, check). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
object_of(fix_tap_s7, tap). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
takes_time(fix_tap_s7, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
before(fix_tap_s6, fix_tap_s7). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
before(fix_tap_s4, fix_tap_s6). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
step(fix_tap, fix_tap_s5). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
action(fix_tap_s5, put). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
action(fix_tap_s5, tighten). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
object_of(fix_tap_s5, valve). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
object_of(fix_tap_s5, tap_handle). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
uses(fix_tap_s5, spanner). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
uses(fix_tap_s5, screwdriver). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
takes_time(fix_tap_s5, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
before(fix_tap_s4, fix_tap_s5). @conf 1 @source teacher:curriculum @conv s1794946298169-131028-42
needs(fix_tap, spray_lubricant). @conf 1 @source teacher:curriculum @conv s1794946316429-131028-63
needs(fix_tap, cloth). @conf 1 @source teacher:curriculum @conv s1794946316429-131028-63
needs(fix_tap, cartridge). @conf 1 @source teacher:curriculum @conv s1794946548453-131028-204
object_of(fix_tap_s4, cartridge). @conf 1 @source teacher:curriculum @conv s1794946548453-131028-204
uses(fix_tap_s4, ceramic_cartridge). @conf 1 @source teacher:curriculum @conv s1794946548453-131028-204
```
