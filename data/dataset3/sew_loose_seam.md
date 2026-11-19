---
id: sew_loose_seam
stage: procedures
task: sew_loose_seam
schema: v3
---
# Sew a loose seam

## requests
- My trouser hem has come loose at the seam. How do I sew it back by hand?
- Can you tell me how to fix a split seam on a shirt with a needle and thread?
- What's the process for resewing a loose seam, and how long will it take?

## situation
- I have a shirt with a loose seam under the arm, a needle, thread and scissors.
- I have about half an hour free this evening.

## ground truth
```datalog
task(sew_loose_seam).
domain(sew_loose_seam, repair).
goal(sew_loose_seam, repaired_seam).
needs(sew_loose_seam, thread).
needs(sew_loose_seam, needle).
needs(sew_loose_seam, scissors).
takes_time(sew_loose_seam, 15, 25, minute).
step(sew_loose_seam, sew_loose_seam_s1).
action(sew_loose_seam_s1, cut).
object_of(sew_loose_seam_s1, thread).
uses(sew_loose_seam_s1, scissors).
role(sew_loose_seam_s1, scissors, cut).
takes_time(sew_loose_seam_s1, 1, 2, minute).
step(sew_loose_seam, sew_loose_seam_s2).
action(sew_loose_seam_s2, thread).
object_of(sew_loose_seam_s2, needle).
uses(sew_loose_seam_s2, thread).
role(sew_loose_seam_s2, thread, hold).
takes_time(sew_loose_seam_s2, 1, 2, minute).
step(sew_loose_seam, sew_loose_seam_s3).
action(sew_loose_seam_s3, stitch).
object_of(sew_loose_seam_s3, seam).
uses(sew_loose_seam_s3, needle).
role(sew_loose_seam_s3, needle, pierce).
uses(sew_loose_seam_s3, thread).
role(sew_loose_seam_s3, thread, fasten).
check(sew_loose_seam_s3, seam_firm).
takes_time(sew_loose_seam_s3, 10, 15, minute).
step(sew_loose_seam, sew_loose_seam_s4).
action(sew_loose_seam_s4, tie).
object_of(sew_loose_seam_s4, thread).
uses(sew_loose_seam_s4, thread).
role(sew_loose_seam_s4, thread, fasten).
takes_time(sew_loose_seam_s4, 1, 1, minute).
step(sew_loose_seam, sew_loose_seam_s5).
action(sew_loose_seam_s5, cut).
object_of(sew_loose_seam_s5, thread).
uses(sew_loose_seam_s5, scissors).
role(sew_loose_seam_s5, scissors, cut).
takes_time(sew_loose_seam_s5, 1, 1, minute).
before(sew_loose_seam_s1, sew_loose_seam_s2).
before(sew_loose_seam_s2, sew_loose_seam_s3).
before(sew_loose_seam_s3, sew_loose_seam_s4).
before(sew_loose_seam_s4, sew_loose_seam_s5).
```

## aliases
```datalog
task_alias(sew_loose_seam, "sew up a loose seam").
task_alias(sew_loose_seam, "resew a seam").
task_alias(sew_loose_seam, "fix a split seam").
alias(seam, "loose seam").
alias(needle, "sewing needle").
alias(thread, "sewing thread").
alias(stitch, "sew").
alias(tie, "knot").
alias(seam_firm, "seam is firm").
```

## teach
- To sew a loose seam you need thread, a needle and scissors.
- Sewing a loose seam takes 15 to 25 minutes.
- Step 1 of sewing a loose seam: cut a length of thread with the scissors, for 1 to 2 minutes.
- Step 2 of sewing a loose seam: thread the needle, for 1 to 2 minutes.
- Step 3 of sewing a loose seam: stitch the seam closed with the needle and thread until the seam is firm, for 10 to 15 minutes.
- Step 4 of sewing a loose seam: tie the thread in a knot at the end, for 1 minute.
- Step 5 of sewing a loose seam: cut the thread off close to the knot with the scissors, for 1 minute.

## checklist
- names what is needed: thread, a needle and scissors
- gives the steps in order: cut thread, thread the needle, stitch the seam, knot the thread, cut the thread
- gives a total time of about 15 to 25 minutes
- says the stitching takes the longest, about 10 to 15 minutes

## variant
- change: only 15 minutes: use larger running stitches in the stitching step and finish in about 8 to 12 minutes.
- request: My seam is loose and I only have about 15 minutes before I need to leave. Can I still do it?
- situation: I have a shirt, a needle, thread, scissors, and about 15 minutes.
- expect: The reply uses larger running stitches for the seam, so the stitching takes about 5 to 8 minutes and the whole job fits in 8 to 12 minutes.

```datalog situation
has(user, needle).
has(user, thread).
has(user, scissors).
time_limit(user, 15, minute).
```

```datalog difference
change(sew_loose_seam_s3, takes_time, 10_15, 5_8).
change(sew_loose_seam, takes_time, 15_25, 8_12).
```

- change: no scissors: cut the thread with a small knife instead.
- request: My seam came loose but I can't find my scissors. Can I still sew it with the needle and thread?
- situation: I have a shirt, a needle, thread and a small kitchen knife, and about half an hour.
- expect: The reply uses the small knife to cut the thread in place of the scissors in the first and last steps, and otherwise follows the same five steps.

```datalog situation
has(user, needle).
has(user, thread).
has(user, knife).
lacks(user, scissors).
time_limit(user, 30, minute).
```

```datalog difference
swap(sew_loose_seam_s1, scissors, knife).
swap(sew_loose_seam_s5, scissors, knife).
extra_need(knife).
no_need(scissors).
```
