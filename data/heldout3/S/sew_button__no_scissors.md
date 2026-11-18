---
id: sew_button__no_scissors
task: sew_button
schema: v3
tier: S
base: sew_button
---
# Sew on a button

## requests
- A button popped off my coat. I've got a needle and thread but can't find scissors anywhere, only nail clippers.

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

## teach
- To sew on a button you need a button, a needle, thread and scissors.
- Sewing on a button takes 10 to 15 minutes.
- Step 1 of sewing on a button: thread the needle with the thread for 1 to 2 minutes.
- Step 2 of sewing on a button: knot the end of the thread.
- Step 3 of sewing on a button: place the button onto the fabric.
- Step 4 of sewing on a button: sew the button on, pushing the needle through the holes until it is firm, for 5 to 8 minutes.
- Step 5 of sewing on a button: knot the thread at the back for 1 to 2 minutes.
- Step 6 of sewing on a button: cut the spare thread with the scissors.

## variant
- change: no scissors: snip the thread with nail clippers
- request: A button popped off my coat. I've got a needle and thread but can't find scissors anywhere, only nail clippers.
- situation: I have the button, a needle and thread, and nail clippers, but no scissors.
- expect: The reply sews the button on as usual and cuts the spare thread with the nail clippers instead of scissors.

```datalog situation
has(user, button).
has(user, needle).
has(user, thread).
has(user, nail_clipper).
lacks(user, scissors).
```

```datalog difference
swap(sew_button_s6, scissors, nail_clipper).
no_need(scissors).
extra_need(nail_clipper).
```
