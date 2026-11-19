---
id: mend_torn_jeans
stage: procedures
task: mend_torn_jeans
schema: v3
---
# Mend torn jeans

## requests
- My jeans have a hole at the knee. How do I sew it up by hand?
- What's the way to mend torn jeans with a needle and thread?
- Can you tell me the steps to fix a tear in my trousers?

## situation
- I have a pair of jeans with a tear at the knee, scissors, a needle, thread and an iron.
- I have about half an hour.

## ground truth
```datalog
task(mend_torn_jeans).
domain(mend_torn_jeans, repair).
goal(mend_torn_jeans, mended_jeans).
needs(mend_torn_jeans, jeans).
needs(mend_torn_jeans, scissors).
needs(mend_torn_jeans, needle).
needs(mend_torn_jeans, thread).
needs(mend_torn_jeans, iron).
takes_time(mend_torn_jeans, 20, 30, minute).
step(mend_torn_jeans, mend_torn_jeans_s1).
action(mend_torn_jeans_s1, cut).
object_of(mend_torn_jeans_s1, frayed_thread).
uses(mend_torn_jeans_s1, scissors).
role(mend_torn_jeans_s1, scissors, cut).
step(mend_torn_jeans, mend_torn_jeans_s2).
action(mend_torn_jeans_s2, thread).
object_of(mend_torn_jeans_s2, needle).
uses(mend_torn_jeans_s2, thread).
role(mend_torn_jeans_s2, thread, hold).
overlaps(mend_torn_jeans_s2, mend_torn_jeans_s1).
step(mend_torn_jeans, mend_torn_jeans_s3).
action(mend_torn_jeans_s3, sew).
object_of(mend_torn_jeans_s3, jeans).
uses(mend_torn_jeans_s3, needle).
role(mend_torn_jeans_s3, needle, pierce).
uses(mend_torn_jeans_s3, thread).
role(mend_torn_jeans_s3, thread, hold).
takes_time(mend_torn_jeans_s3, 10, 15, minute).
purpose(mend_torn_jeans_s3, tear_closed).
step(mend_torn_jeans, mend_torn_jeans_s4).
action(mend_torn_jeans_s4, tie).
object_of(mend_torn_jeans_s4, thread).
uses(mend_torn_jeans_s4, thread).
role(mend_torn_jeans_s4, thread, fasten).
step(mend_torn_jeans, mend_torn_jeans_s5).
action(mend_torn_jeans_s5, iron).
object_of(mend_torn_jeans_s5, jeans).
uses(mend_torn_jeans_s5, iron).
role(mend_torn_jeans_s5, iron, heat).
achieves(mend_torn_jeans_s5, flat).
before(mend_torn_jeans_s1, mend_torn_jeans_s3).
before(mend_torn_jeans_s2, mend_torn_jeans_s3).
before(mend_torn_jeans_s3, mend_torn_jeans_s4).
before(mend_torn_jeans_s4, mend_torn_jeans_s5).
```

## aliases
```datalog
task_alias(mend_torn_jeans, "fix torn jeans").
task_alias(mend_torn_jeans, "sew up jeans").
task_alias(mend_torn_jeans, "mending torn jeans").
alias(frayed_thread, "frayed threads").
alias(needle, "sewing needle").
alias(thread, "sewing thread").
alias(sew, "sew up").
alias(iron_on_patch, "iron-on patch").
alias(knife, "kitchen knife").
alias(tear_closed, "tear closed").
alias(flat, "lies flat").
```

## teach
- To mend torn jeans you need jeans, scissors, a needle, thread and an iron.
- Mending torn jeans takes 20 to 30 minutes.
- Step 1 of mending torn jeans: trim the frayed threads with scissors.
- While step 1 runs, step 2 of mending torn jeans: thread the needle.
- Step 3 of mending torn jeans: sew the tear closed with the needle and thread, so the hole is shut, for 10 to 15 minutes.
- Step 4 of mending torn jeans: tie a knot in the thread.
- Step 5 of mending torn jeans: iron the mended seam so it lies flat.

## checklist
- names what is needed, including jeans, scissors, a needle, thread and an iron
- gives the steps in order: trim the frayed threads, thread the needle, sew the tear closed, tie a knot, iron the seam
- says threading the needle can be done while the frayed threads are trimmed
- gives a total time of about 20 to 30 minutes

## variant
- change: no needle or thread, so use an iron-on patch instead of sewing: trim the threads, then iron the patch on the inside of the tear, which takes 10 to 15 minutes in total
- request: My jeans have a tear and I don't have a needle or thread. I have an iron-on patch, though. What do I do?
- situation: I have jeans with a tear, scissors, an iron-on patch and an iron. I have no needle or thread.
- expect: The reply skips the needle and thread, trims the frayed threads, then irons the iron-on patch over the inside of the tear for 5 to 10 minutes, finishing in about 10 to 15 minutes.

```datalog situation
lacks(user, needle).
lacks(user, thread).
has(user, iron_on_patch).
```

```datalog difference
drop(mend_torn_jeans_s2).
drop(mend_torn_jeans_s3).
drop(mend_torn_jeans_s4).
drop(mend_torn_jeans_s5).
add_step(mend_torn_jeans_s9, mend_torn_jeans_s1).
step(mend_torn_jeans, mend_torn_jeans_s9).
action(mend_torn_jeans_s9, apply).
object_of(mend_torn_jeans_s9, jeans).
uses(mend_torn_jeans_s9, iron_on_patch).
role(mend_torn_jeans_s9, iron_on_patch, cover).
uses(mend_torn_jeans_s9, iron).
role(mend_torn_jeans_s9, iron, heat).
takes_time(mend_torn_jeans_s9, 5, 10, minute).
before(mend_torn_jeans_s1, mend_torn_jeans_s9).
extra_need(iron_on_patch).
no_need(needle).
no_need(thread).
change(mend_torn_jeans, takes_time, 20_30, 10_15).
```

- change: the user has no scissors but has a kitchen knife, so the knife trims the frayed threads in place of scissors
- request: I need to fix a tear in my jeans, but I only have a kitchen knife, not scissors. Can I still do it?
- situation: I have jeans with a tear, a kitchen knife, a needle, thread and an iron. I have no scissors.
- expect: The reply trims the frayed threads with the kitchen knife instead of scissors, then follows the usual steps of threading the needle, sewing the tear closed, tying a knot and ironing the seam.

```datalog situation
has(user, knife).
lacks(user, scissors).
```

```datalog difference
swap(mend_torn_jeans_s1, scissors, knife).
extra_need(knife).
no_need(scissors).
```
