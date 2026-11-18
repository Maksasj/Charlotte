---
id: replace_shower_head
stage: tools
task: replace_shower_head
schema: v3
tier: T
---
# Replace a shower head

## requests
- How do I swap out my old shower head for a new one?
- I bought a rain shower head. How do I put it on?
- The old shower head is cracked and leaking everywhere. How do I change it?

## situation
- I have an adjustable spanner, a cloth, PTFE tape and the new shower head.

## ground truth
```datalog
task(replace_shower_head).
domain(replace_shower_head, repair).
goal(replace_shower_head, new_shower_head_fitted).
needs(replace_shower_head, spanner).
needs(replace_shower_head, cloth).
needs(replace_shower_head, ptfe_tape).
needs(replace_shower_head, new_shower_head).
takes_time(replace_shower_head, 15, 20, minute).
step(replace_shower_head, replace_shower_head_s1).
action(replace_shower_head_s1, unscrew).
object_of(replace_shower_head_s1, old_shower_head).
uses(replace_shower_head_s1, spanner).
role(replace_shower_head_s1, spanner, loosen).
uses(replace_shower_head_s1, cloth).
role(replace_shower_head_s1, cloth, protect).
step(replace_shower_head, replace_shower_head_s2).
action(replace_shower_head_s2, clean).
object_of(replace_shower_head_s2, thread).
uses(replace_shower_head_s2, cloth).
role(replace_shower_head_s2, cloth, wipe).
step(replace_shower_head, replace_shower_head_s3).
action(replace_shower_head_s3, wrap).
object_of(replace_shower_head_s3, thread).
uses(replace_shower_head_s3, ptfe_tape).
role(replace_shower_head_s3, ptfe_tape, cover).
purpose(replace_shower_head_s3, no_leak).
step(replace_shower_head, replace_shower_head_s4).
action(replace_shower_head_s4, screw).
object_of(replace_shower_head_s4, new_shower_head).
onto(replace_shower_head_s4, shower_arm).
step(replace_shower_head, replace_shower_head_s5).
action(replace_shower_head_s5, tighten).
object_of(replace_shower_head_s5, new_shower_head).
uses(replace_shower_head_s5, spanner).
role(replace_shower_head_s5, spanner, fasten).
uses(replace_shower_head_s5, cloth).
role(replace_shower_head_s5, cloth, protect).
avoid(replace_shower_head_s5, overtighten, new_shower_head).
step(replace_shower_head, replace_shower_head_s6).
action(replace_shower_head_s6, run).
object_of(replace_shower_head_s6, water).
check(replace_shower_head_s6, no_leak).
before(replace_shower_head_s1, replace_shower_head_s2).
before(replace_shower_head_s2, replace_shower_head_s3).
before(replace_shower_head_s3, replace_shower_head_s4).
before(replace_shower_head_s4, replace_shower_head_s5).
before(replace_shower_head_s5, replace_shower_head_s6).
```

## aliases
```datalog
task_alias(replace_shower_head, "replacing a shower head").
task_alias(replace_shower_head, "fitting a new shower head").
task_alias(replace_shower_head, "changing the shower head").
alias(spanner, "adjustable spanner").
alias(ptfe_tape, "plumber's tape").
alias(thread, "the thread on the pipe").
alias(shower_arm, "the shower arm").
alias(new_shower_head, "new head").
alias(old_shower_head, "old head").
alias(no_leak, "it does not leak").
```

## teach
- To replace a shower head you need an adjustable spanner, a cloth, PTFE tape and the new shower head.
- Replacing a shower head takes 15 to 20 minutes.
- Step 1 of replacing a shower head: unscrew the old shower head with the spanner, with the cloth round it to protect the finish.
- Step 2 of replacing a shower head: clean the thread with the cloth.
- Step 3 of replacing a shower head: wrap the thread with PTFE tape so it does not leak.
- Step 4 of replacing a shower head: screw the new shower head onto the shower arm by hand.
- Step 5 of replacing a shower head: tighten the new head with the spanner over the cloth; do not overtighten it.
- Step 6 of replacing a shower head: run the water and check it does not leak.

## checklist
- names what is needed: a spanner, a cloth, PTFE tape and the new head
- gives the steps in order: unscrew old, clean thread, wrap tape, screw on, tighten, test
- warns not to overtighten

## variant
- change: no spanner: use a pipe wrench
- request: I don't have an adjustable spanner. Will a pipe wrench do for changing the shower head?
- situation: I have a pipe wrench, a cloth, PTFE tape and the new shower head.
- expect: The reply uses the wrench, padded with the cloth, to loosen the old head and tighten the new one.

```datalog situation
has(user, wrench).
has(user, cloth).
has(user, ptfe_tape).
has(user, new_shower_head).
lacks(user, spanner).
```

```datalog difference
swap(replace_shower_head_s1, spanner, wrench).
swap(replace_shower_head_s5, spanner, wrench).
extra_need(wrench).
no_need(spanner).
```

## variant
- change: no PTFE tape: rely on the head's rubber washer
- request: I've got the new shower head but no plumber's tape. Can I fit it anyway?
- situation: I have an adjustable spanner, a cloth and the new shower head, which has a rubber washer inside.
- expect: The reply skips wrapping the thread, relying on the head's washer, and checks carefully for leaks at the end.

```datalog situation
has(user, spanner).
has(user, cloth).
has(user, new_shower_head).
lacks(user, ptfe_tape).
```

```datalog difference
drop(replace_shower_head_s3).
no_need(ptfe_tape).
```
