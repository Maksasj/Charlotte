---
id: sew_button_on_shirt
stage: procedures
task: sew_button_on_shirt
schema: v3
---
# Sew a button on a shirt

## requests
- My shirt has a loose button. How do I sew it back on?
- What's the way to attach a button to a shirt by hand?
- Can you guide me through fixing a button that has come off my shirt?

## situation
- I have a shirt with a loose button, a needle, thread and scissors.
- I've got about 15 minutes.

## ground truth
```datalog
task(sew_button_on_shirt).
domain(sew_button_on_shirt, crafts).
goal(sew_button_on_shirt, sewn_button).
needs(sew_button_on_shirt, shirt).
needs(sew_button_on_shirt, button).
needs(sew_button_on_shirt, needle).
needs(sew_button_on_shirt, thread).
needs(sew_button_on_shirt, scissors).
takes_time(sew_button_on_shirt, 10, 15, minute).
step(sew_button_on_shirt, sew_button_on_shirt_s1).
action(sew_button_on_shirt_s1, cut).
object_of(sew_button_on_shirt_s1, thread).
uses(sew_button_on_shirt_s1, scissors).
role(sew_button_on_shirt_s1, scissors, cut).
step(sew_button_on_shirt, sew_button_on_shirt_s2).
action(sew_button_on_shirt_s2, thread).
object_of(sew_button_on_shirt_s2, needle).
uses(sew_button_on_shirt_s2, thread).
role(sew_button_on_shirt_s2, thread, pass).
step(sew_button_on_shirt, sew_button_on_shirt_s3).
action(sew_button_on_shirt_s3, sew).
object_of(sew_button_on_shirt_s3, button).
onto(sew_button_on_shirt_s3, shirt).
uses(sew_button_on_shirt_s3, needle).
role(sew_button_on_shirt_s3, needle, pierce).
until(sew_button_on_shirt_s3, button_secure).
step(sew_button_on_shirt, sew_button_on_shirt_s4).
action(sew_button_on_shirt_s4, knot).
object_of(sew_button_on_shirt_s4, thread).
uses(sew_button_on_shirt_s4, thread).
role(sew_button_on_shirt_s4, thread, fasten).
check(sew_button_on_shirt_s4, knot_holds).
before(sew_button_on_shirt_s1, sew_button_on_shirt_s2).
before(sew_button_on_shirt_s2, sew_button_on_shirt_s3).
before(sew_button_on_shirt_s3, sew_button_on_shirt_s4).
```

## aliases
```datalog
task_alias(sew_button_on_shirt, "sewing a button on a shirt").
task_alias(sew_button_on_shirt, "fixing a shirt button").
alias(needle, "sewing needle").
alias(thread, "sewing thread").
alias(button_secure, "button firmly attached").
alias(sewn_button, "sewn button").
alias(knot_holds, "knot holds").
```

## teach
- To sew a button on a shirt you need a shirt, a button, a needle, thread and scissors.
- Sewing a button on a shirt takes 10 to 15 minutes.
- Step 1 of sewing a button on a shirt: cut a length of thread with the scissors.
- Step 2 of sewing a button on a shirt: thread the needle with the thread.
- Step 3 of sewing a button on a shirt: sew the button onto the shirt with the needle until it is firmly attached.
- Step 4 of sewing a button on a shirt: tie a knot in the thread on the back of the shirt so the knot holds.

## checklist
- names what is needed, including a shirt, a button, a needle, thread and scissors
- gives the steps in order: cut thread, thread the needle, sew the button on, tie a knot on the back
- gives a total time of about 10 to 15 minutes
- says the knot on the back must hold the button firmly

## variant
- change: reinforce the button with a second pass of stitches
- request: I want the button extra secure, so can you add a second pass of stitches through it?
- situation: I have a shirt, a button, a needle, thread and scissors, and I want the button reinforced.
- expect: The reply adds a second sewing pass through the button after the first pass and before the knot, which adds about 3 more minutes.

```datalog situation
has(user, shirt).
has(user, button).
has(user, needle).
has(user, thread).
has(user, scissors).
wants(user, extra_stitches).
```

```datalog difference
add_step(sew_button_on_shirt_s9, sew_button_on_shirt_s3).
step(sew_button_on_shirt, sew_button_on_shirt_s9).
action(sew_button_on_shirt_s9, sew).
object_of(sew_button_on_shirt_s9, button).
onto(sew_button_on_shirt_s9, shirt).
uses(sew_button_on_shirt_s9, needle).
role(sew_button_on_shirt_s9, needle, pierce).
uses(sew_button_on_shirt_s9, thread).
role(sew_button_on_shirt_s9, thread, pass).
before(sew_button_on_shirt_s3, sew_button_on_shirt_s9).
before(sew_button_on_shirt_s9, sew_button_on_shirt_s4).
change(sew_button_on_shirt, takes_time, 10_15, 13_18).
```

- change: the user has no needle at all
- request: I have no sewing needle. Can I still sew this button back on?
- situation: I have a shirt, a button, thread and scissors, but no needle.
- expect: The reply says the button cannot be sewn on without a needle and suggests getting one first.

```datalog situation
has(user, shirt).
has(user, button).
has(user, thread).
has(user, scissors).
lacks(user, needle).
```

```datalog difference
cannot(no_needle).
```
