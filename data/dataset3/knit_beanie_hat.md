---
id: knit_beanie_hat
stage: procedures
task: knit_beanie_hat
schema: v3
---
# Knit a beanie hat

## requests
- I'd like to knit a warm winter hat for my brother. What do I need to do, step by step?
- Can you explain how to make a beanie with knitting needles?
- How would I go about knitting a hat that fits snugly over the ears?

## situation
- I have yarn, a pair of knitting needles and a pair of scissors.
- I have a free weekend afternoon to work on it.

## ground truth
```datalog
task(knit_beanie_hat).
domain(knit_beanie_hat, crafts).
goal(knit_beanie_hat, winter_hat).
needs(knit_beanie_hat, yarn).
needs(knit_beanie_hat, knitting_needles).
needs(knit_beanie_hat, scissors).
takes_time(knit_beanie_hat, 130, 190, minute).
step(knit_beanie_hat, knit_beanie_hat_s1).
action(knit_beanie_hat_s1, cast_on).
object_of(knit_beanie_hat_s1, yarn).
uses(knit_beanie_hat_s1, knitting_needles).
role(knit_beanie_hat_s1, knitting_needles, hold).
warning(knit_beanie_hat_s1, twisted_stitches).
takes_time(knit_beanie_hat_s1, 10, 15, minute).
step(knit_beanie_hat, knit_beanie_hat_s2).
action(knit_beanie_hat_s2, knit).
object_of(knit_beanie_hat_s2, yarn).
uses(knit_beanie_hat_s2, knitting_needles).
role(knit_beanie_hat_s2, knitting_needles, hold).
purpose(knit_beanie_hat_s2, snug_brim).
takes_time(knit_beanie_hat_s2, 20, 30, minute).
step(knit_beanie_hat, knit_beanie_hat_s3).
action(knit_beanie_hat_s3, knit).
object_of(knit_beanie_hat_s3, yarn).
uses(knit_beanie_hat_s3, knitting_needles).
role(knit_beanie_hat_s3, knitting_needles, hold).
amount(knit_beanie_hat_s3, yarn, 100, gram).
until(knit_beanie_hat_s3, long_enough).
takes_time(knit_beanie_hat_s3, 90, 120, minute).
step(knit_beanie_hat, knit_beanie_hat_s4).
action(knit_beanie_hat_s4, decrease).
object_of(knit_beanie_hat_s4, stitch).
uses(knit_beanie_hat_s4, knitting_needles).
role(knit_beanie_hat_s4, knitting_needles, hold).
purpose(knit_beanie_hat_s4, rounded_crown).
check(knit_beanie_hat_s4, crown_closed).
takes_time(knit_beanie_hat_s4, 15, 20, minute).
step(knit_beanie_hat, knit_beanie_hat_s5).
action(knit_beanie_hat_s5, cut).
object_of(knit_beanie_hat_s5, yarn).
uses(knit_beanie_hat_s5, scissors).
role(knit_beanie_hat_s5, scissors, cut).
takes_time(knit_beanie_hat_s5, 1, 2, minute).
before(knit_beanie_hat_s1, knit_beanie_hat_s2).
before(knit_beanie_hat_s2, knit_beanie_hat_s3).
before(knit_beanie_hat_s3, knit_beanie_hat_s4).
before(knit_beanie_hat_s4, knit_beanie_hat_s5).
```

## aliases
```datalog
task_alias(knit_beanie_hat, "knitted hat").
task_alias(knit_beanie_hat, "beanie").
task_alias(knit_beanie_hat, "knitting a hat").
alias(yarn, "wool").
alias(knitting_needles, "knitting needles").
alias(cast_on, "cast on").
alias(knit, "knitting").
alias(decrease, "decrease stitches").
alias(stitch, "stitches").
alias(long_enough, "long enough").
alias(snug_brim, "snug brim").
alias(rounded_crown, "rounded crown").
alias(crown_closed, "crown closed").
alias(twisted_stitches, "twisted stitches").
```

## teach
- To knit a beanie hat you need yarn, knitting needles and scissors.
- Knitting a beanie hat takes 130 to 190 minutes.
- Step 1 of knitting a beanie hat: cast on stitches onto the knitting needles, for 10 to 15 minutes, and keep the stitches untwisted.
- Step 2 of knitting a beanie hat: knit the ribbing with the yarn for 20 to 30 minutes so the brim sits snugly.
- Step 3 of knitting a beanie hat: knit the body with about 100 grams of yarn for 90 to 120 minutes, until it is long enough to cover the ears.
- Step 4 of knitting a beanie hat: decrease stitches on the needles for 15 to 20 minutes so the crown is rounded, until the crown is closed.
- Step 5 of knitting a beanie hat: cut the yarn with the scissors, leaving a short tail, for 1 to 2 minutes.

## checklist
- names what is needed: yarn, knitting needles and scissors
- gives the steps in order: cast on, knit the ribbing, knit the body, decrease for the crown, cut the yarn
- warns against twisted stitches when casting on
- gives a total time of about 130 to 190 minutes

## variant
- change: no scissors, so cut the yarn with a knife instead
- request: I want to make this hat but I don't own scissors. I only have a kitchen knife. Can I still do it?
- situation: I have yarn, knitting needles and a kitchen knife, and no scissors.
- expect: The reply gives the same five steps and says to cut the yarn with the knife, carefully, in place of the scissors.

```datalog situation
lacks(user, scissors).
has(user, knife).
has(user, yarn).
has(user, knitting_needles).
```

```datalog difference
swap(knit_beanie_hat_s5, scissors, knife).
extra_need(knife).
no_need(scissors).
```

## variant
- change: only 90 minutes: use a chunkier yarn and knit a shorter body
- request: I only have an hour and a half this evening. Could I still knit a hat for my brother?
- situation: I have chunky yarn, knitting needles and scissors, and 90 minutes.
- expect: The reply uses chunky yarn, knits the body for about 40 to 50 minutes, and fits the whole hat into about 80 to 100 minutes.

```datalog situation
has(user, yarn).
has(user, knitting_needles).
has(user, scissors).
time_limit(user, 90, minute).
```

```datalog difference
change(knit_beanie_hat, takes_time, 130_190, 80_100).
change(knit_beanie_hat_s3, takes_time, 90_120, 40_50).
```
