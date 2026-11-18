---
id: sew_button
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Sew on a button

Sew a loose button back on with needle and thread.

```datalog
task(sew_button).
needs(sew_button, button).
needs(sew_button, needle).
needs(sew_button, thread).
needs(sew_button, scissors).
takes_time(sew_button, 10, 15, minute).
step(sew_button, sew_button_s1).
action(sew_button_s1, thread).
object_of(sew_button_s1, needle).
uses(sew_button_s1, thread).
takes_time(sew_button_s1, 1, 2, minute).
step(sew_button, sew_button_s2).
action(sew_button_s2, knot).
object_of(sew_button_s2, thread).
step(sew_button, sew_button_s3).
action(sew_button_s3, place).
object_of(sew_button_s3, button).
step(sew_button, sew_button_s4).
action(sew_button_s4, sew).
object_of(sew_button_s4, button).
uses(sew_button_s4, needle).
takes_time(sew_button_s4, 5, 8, minute).
until(sew_button_s4, firm).
step(sew_button, sew_button_s5).
action(sew_button_s5, knot).
object_of(sew_button_s5, thread).
takes_time(sew_button_s5, 1, 2, minute).
step(sew_button, sew_button_s6).
action(sew_button_s6, cut).
object_of(sew_button_s6, thread).
uses(sew_button_s6, scissors).
before(sew_button_s1, sew_button_s2).
before(sew_button_s2, sew_button_s3).
before(sew_button_s3, sew_button_s4).
before(sew_button_s4, sew_button_s5).
before(sew_button_s5, sew_button_s6).
```
