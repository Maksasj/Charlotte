---
id: crochet_granny_square
stage: procedures
task: crochet_granny_square
schema: v3
---
# Crochet a granny square

## requests
- How do I crochet a granny square? I'm a beginner.
- Can you show me the steps for making a crocheted granny square?
- What's the way to work up a granny square with a hook and some wool?

## situation
- I have yarn, a crochet hook, scissors and a yarn needle.
- I want to finish one square this evening.

## ground truth
```datalog
task(crochet_granny_square).
domain(crochet_granny_square, crafts).
goal(crochet_granny_square, granny_square).
needs(crochet_granny_square, yarn).
needs(crochet_granny_square, crochet_hook).
needs(crochet_granny_square, scissors).
needs(crochet_granny_square, yarn_needle).
takes_time(crochet_granny_square, 45, 60, minute).
step(crochet_granny_square, crochet_granny_square_s1).
action(crochet_granny_square_s1, loop).
object_of(crochet_granny_square_s1, yarn).
uses(crochet_granny_square_s1, yarn).
role(crochet_granny_square_s1, yarn, hold).
uses(crochet_granny_square_s1, crochet_hook).
role(crochet_granny_square_s1, crochet_hook, lift).
purpose(crochet_granny_square_s1, closed_ring).
step(crochet_granny_square, crochet_granny_square_s2).
action(crochet_granny_square_s2, crochet).
object_of(crochet_granny_square_s2, ring).
uses(crochet_granny_square_s2, crochet_hook).
role(crochet_granny_square_s2, crochet_hook, lift).
uses(crochet_granny_square_s2, yarn).
role(crochet_granny_square_s2, yarn, hold).
takes_time(crochet_granny_square_s2, 10, 15, minute).
step(crochet_granny_square, crochet_granny_square_s3).
action(crochet_granny_square_s3, crochet).
object_of(crochet_granny_square_s3, square).
uses(crochet_granny_square_s3, crochet_hook).
role(crochet_granny_square_s3, crochet_hook, lift).
uses(crochet_granny_square_s3, yarn).
role(crochet_granny_square_s3, yarn, hold).
takes_time(crochet_granny_square_s3, 15, 20, minute).
step(crochet_granny_square, crochet_granny_square_s4).
action(crochet_granny_square_s4, cut).
object_of(crochet_granny_square_s4, yarn).
uses(crochet_granny_square_s4, scissors).
role(crochet_granny_square_s4, scissors, cut).
check(crochet_granny_square_s4, tail_left).
step(crochet_granny_square, crochet_granny_square_s5).
action(crochet_granny_square_s5, weave).
object_of(crochet_granny_square_s5, yarn).
uses(crochet_granny_square_s5, yarn_needle).
role(crochet_granny_square_s5, yarn_needle, pierce).
purpose(crochet_granny_square_s5, hidden_ends).
before(crochet_granny_square_s1, crochet_granny_square_s2).
before(crochet_granny_square_s2, crochet_granny_square_s3).
before(crochet_granny_square_s3, crochet_granny_square_s4).
before(crochet_granny_square_s4, crochet_granny_square_s5).
```

## aliases
```datalog
task_alias(crochet_granny_square, "granny square").
task_alias(crochet_granny_square, "crocheted square").
task_alias(crochet_granny_square, "crochet a granny square").
alias(crochet_hook, "hook").
alias(yarn, "wool").
alias(yarn_needle, "tapestry needle").
alias(loop, "make a loop").
alias(ring, "magic ring").
alias(weave, "weave in ends").
alias(tail_left, "tail left").
alias(closed_ring, "closed ring").
alias(hidden_ends, "ends hidden").
```

## teach
- To crochet a granny square you need yarn, a crochet hook, scissors and a yarn needle.
- Crocheting a granny square takes 45 to 60 minutes.
- Step 1 of crocheting a granny square: make a loop with the yarn and close it with the hook so the ring holds.
- Step 2 of crocheting a granny square: work the first round of stitches into the ring with the hook, which takes 10 to 15 minutes.
- Step 3 of crocheting a granny square: crochet the corners of the square with the hook, which takes 15 to 20 minutes.
- Step 4 of crocheting a granny square: cut the yarn with the scissors, leaving a tail.
- Step 5 of crocheting a granny square: weave the tail in with the yarn needle so the ends are hidden.

## checklist
- names what is needed: yarn, a crochet hook, scissors and a yarn needle
- gives the steps in order: make a loop, first round, corners, cut the yarn, weave in the ends
- gives a total time of about 45 to 60 minutes
- says the cut yarn should leave a tail for weaving in

## variant
- change: only 30 minutes: work the first round and the corners more quickly so the square is done sooner
- request: I only have 30 minutes. Can I still make a granny square?
- situation: I have yarn, a crochet hook, scissors and a yarn needle, and 30 minutes.
- expect: The reply keeps the same steps but shortens the first round and the corners, so the square takes about 30 to 40 minutes.

```datalog situation
has(user, yarn).
has(user, crochet_hook).
has(user, scissors).
has(user, yarn_needle).
time_limit(user, 30, minute).
```

```datalog difference
change(crochet_granny_square_s2, takes_time, 10_15, 7_9).
change(crochet_granny_square_s3, takes_time, 15_20, 10_12).
change(crochet_granny_square, takes_time, 45_60, 30_40).
```

- change: add a border round after the corners
- request: I want my granny square to have a border round after the corners. How do I add it?
- situation: I have yarn, a crochet hook, scissors and a yarn needle, and I want a border.
- expect: The reply adds a border round after the corners, worked with the hook and yarn, so the whole task takes about 55 to 72 minutes.

```datalog situation
has(user, yarn).
has(user, crochet_hook).
has(user, scissors).
has(user, yarn_needle).
wants(user, border).
```

```datalog difference
add_step(crochet_granny_square_s9, crochet_granny_square_s3).
step(crochet_granny_square, crochet_granny_square_s9).
action(crochet_granny_square_s9, crochet).
object_of(crochet_granny_square_s9, border).
uses(crochet_granny_square_s9, crochet_hook).
role(crochet_granny_square_s9, crochet_hook, lift).
uses(crochet_granny_square_s9, yarn).
role(crochet_granny_square_s9, yarn, hold).
takes_time(crochet_granny_square_s9, 10, 12, minute).
before(crochet_granny_square_s3, crochet_granny_square_s9).
before(crochet_granny_square_s9, crochet_granny_square_s4).
change(crochet_granny_square, takes_time, 45_60, 55_72).
```
