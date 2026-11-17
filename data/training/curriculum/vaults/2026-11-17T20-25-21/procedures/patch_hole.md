---
id: patch_hole
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# patch hole

Learned in conversation.

```datalog
task(patch_hole). @conf 1 @source ""
needs(patch_hole, spackle). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
needs(patch_hole, putty_knife). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
needs(patch_hole, sandpaper). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
needs(patch_hole, paint). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
needs(patch_hole, paintbrush). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
takes_time(patch_hole, 40, 60, minute). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
step(patch_hole, patch_hole_s1). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
action(patch_hole_s1, clean). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
object_of(patch_hole_s1, loose_bit). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
takes_time(patch_hole_s1, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
step(patch_hole, patch_hole_s2). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
action(patch_hole_s2, fill). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
object_of(patch_hole_s2, hole). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
uses(patch_hole_s2, spackle). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
uses(patch_hole_s2, putty_knife). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
step(patch_hole, patch_hole_s3). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
action(patch_hole_s3, smooth). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
object_of(patch_hole_s3, spackle). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
takes_time(patch_hole_s3, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
before(patch_hole_s2, patch_hole_s3). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
before(patch_hole_s1, patch_hole_s2). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
action(patch_hole_s3, dry). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
takes_time(patch_hole_s3, 30, 40, minute). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
step(patch_hole, patch_hole_s4). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
action(patch_hole_s4, sand). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
object_of(patch_hole_s4, spackle). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
until(patch_hole_s4, smooth). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
until(patch_hole_s4, level). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
takes_time(patch_hole_s4, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
before(patch_hole_s3, patch_hole_s4). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
step(patch_hole, patch_hole_s5). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
action(patch_hole_s5, paint). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
object_of(patch_hole_s5, patch). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
takes_time(patch_hole_s5, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
before(patch_hole_s4, patch_hole_s5). @conf 1 @source teacher:curriculum @conv s1794946408527-131028-109
needs(patch_hole, mesh_patch). @conf 1 @source teacher:curriculum @conv s1794946654036-131028-313
action(patch_hole_s2, stick). @conf 1 @source teacher:curriculum @conv s1794946654036-131028-313
object_of(patch_hole_s2, mesh_patch). @conf 1 @source teacher:curriculum @conv s1794946654036-131028-313
action(patch_hole_s2, spread). @conf 1 @source teacher:curriculum @conv s1794946654036-131028-313
object_of(patch_hole_s2, spackle). @conf 1 @source teacher:curriculum @conv s1794946654036-131028-313
until(patch_hole_s2, flat). @conf 1 @source teacher:curriculum @conv s1794946654036-131028-313
```
