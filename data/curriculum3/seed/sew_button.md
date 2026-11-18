---
id: sew_button
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Sew on a button

Sew a loose button back on with needle and thread.

## ground truth
```datalog
task(sew_button).
domain(sew_button, repair).
goal(sew_button, attached_button).
needs(sew_button, button).
needs(sew_button, needle).
needs(sew_button, thread).
needs(sew_button, scissors).
takes_time(sew_button, 10, 15, minute).
step(sew_button, sew_button_s1).
action(sew_button_s1, thread).
object_of(sew_button_s1, needle).
uses(sew_button_s1, thread).
role(sew_button_s1, thread, fasten).
takes_time(sew_button_s1, 1, 2, minute).
step(sew_button, sew_button_s2).
action(sew_button_s2, knot).
object_of(sew_button_s2, thread).
step(sew_button, sew_button_s3).
action(sew_button_s3, place).
object_of(sew_button_s3, button).
onto(sew_button_s3, fabric).
step(sew_button, sew_button_s4).
action(sew_button_s4, sew).
object_of(sew_button_s4, button).
uses(sew_button_s4, needle).
role(sew_button_s4, needle, pierce).
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
role(sew_button_s6, scissors, cut).
before(sew_button_s1, sew_button_s2).
before(sew_button_s2, sew_button_s3).
before(sew_button_s3, sew_button_s4).
before(sew_button_s4, sew_button_s5).
before(sew_button_s5, sew_button_s6).
```

## aliases
```datalog
task_alias(sew_button, "sewing on a button").
task_alias(sew_button, "sew a button back on").
task_alias(sew_button, "fix a loose button").
alias(button, "loose button").
alias(needle, "sewing needle").
alias(thread, "cotton thread").
alias(knot, "tie a knot").
alias(fabric, "clothes").
alias(firm, "secure").
```
