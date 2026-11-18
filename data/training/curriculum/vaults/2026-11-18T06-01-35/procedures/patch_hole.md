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
needs(patch_hole, spackle). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
needs(patch_hole, putty_knife). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
needs(patch_hole, sandpaper). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
needs(patch_hole, paint). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
needs(patch_hole, paintbrush). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
takes_time(patch_hole, 40, 60, minute). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
step(patch_hole, patch_hole_s1). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
action(patch_hole_s1, clean). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
object_of(patch_hole_s1, loose_bit). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
takes_time(patch_hole_s1, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
step(patch_hole, patch_hole_s2). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
action(patch_hole_s2, fill). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
object_of(patch_hole_s2, hole). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
uses(patch_hole_s2, spackle). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
uses(patch_hole_s2, putty_knife). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
takes_time(patch_hole_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
before(patch_hole_s1, patch_hole_s2). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
step(patch_hole, patch_hole_s3). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
action(patch_hole_s3, dry). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
object_of(patch_hole_s3, spackle). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
takes_time(patch_hole_s3, 30, 40, minute). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
before(patch_hole_s2, patch_hole_s3). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
step(patch_hole, patch_hole_s4). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
action(patch_hole_s4, sand). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
object_of(patch_hole_s4, spackle). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
until(patch_hole_s4, smooth). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
until(patch_hole_s4, level). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
takes_time(patch_hole_s4, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
before(patch_hole_s3, patch_hole_s4). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
step(patch_hole, patch_hole_s5). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
action(patch_hole_s5, paint). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
object_of(patch_hole_s5, patch). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
takes_time(patch_hole_s5, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
before(patch_hole_s4, patch_hole_s5). @conf 1 @source teacher:curriculum @conv s1794981751948-278172-153
```
