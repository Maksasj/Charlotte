---
id: make_toast
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Make toast

Toast a slice of bread until golden and butter it.

```datalog
task(make_toast).
solves(make_toast, plan_meal).
needs(make_toast, bread).
needs(make_toast, butter).
needs(make_toast, toaster).
needs(make_toast, knife).
takes_time(make_toast, 3, 5, minute).
step(make_toast, make_toast_s1).
action(make_toast_s1, toast).
object_of(make_toast_s1, bread).
uses(make_toast_s1, toaster).
takes_time(make_toast_s1, 2, 3, minute).
until(make_toast_s1, golden).
step(make_toast, make_toast_s2).
action(make_toast_s2, spread).
object_of(make_toast_s2, butter).
uses(make_toast_s2, knife).
takes_time(make_toast_s2, 1, 1, minute).
step(make_toast, make_toast_s3).
action(make_toast_s3, serve).
object_of(make_toast_s3, toast).
before(make_toast_s1, make_toast_s2).
before(make_toast_s2, make_toast_s3).
```
