---
id: clean_dryer_lint_trap
stage: procedures
task: clean_dryer_lint_trap
schema: v3
---
# Clean a dryer lint trap

## requests
- How do I clean the lint trap on my dryer?
- My dryer's lint filter is clogged up. What's the best way to clean it?
- Walk me through washing the lint screen on a dryer, please.

## situation
- I have a small brush, warm water and a cloth.
- I have about 15 minutes.

## ground truth
```datalog
task(clean_dryer_lint_trap).
domain(clean_dryer_lint_trap, cleaning).
goal(clean_dryer_lint_trap, clean_dryer).
needs(clean_dryer_lint_trap, brush).
needs(clean_dryer_lint_trap, warm_water).
needs(clean_dryer_lint_trap, cloth).
takes_time(clean_dryer_lint_trap, 10, 15, minute).
step(clean_dryer_lint_trap, clean_dryer_lint_trap_s1).
action(clean_dryer_lint_trap_s1, pull).
object_of(clean_dryer_lint_trap_s1, lint_trap).
from(clean_dryer_lint_trap_s1, dryer).
step(clean_dryer_lint_trap, clean_dryer_lint_trap_s2).
action(clean_dryer_lint_trap_s2, scrub).
object_of(clean_dryer_lint_trap_s2, lint_trap).
uses(clean_dryer_lint_trap_s2, brush).
role(clean_dryer_lint_trap_s2, brush, scrub).
purpose(clean_dryer_lint_trap_s2, loose_lint_removed).
step(clean_dryer_lint_trap, clean_dryer_lint_trap_s3).
action(clean_dryer_lint_trap_s3, rinse).
object_of(clean_dryer_lint_trap_s3, lint_trap).
uses(clean_dryer_lint_trap_s3, warm_water).
role(clean_dryer_lint_trap_s3, warm_water, clean).
step(clean_dryer_lint_trap, clean_dryer_lint_trap_s4).
action(clean_dryer_lint_trap_s4, wipe).
object_of(clean_dryer_lint_trap_s4, lint_trap).
uses(clean_dryer_lint_trap_s4, cloth).
role(clean_dryer_lint_trap_s4, cloth, wipe).
step(clean_dryer_lint_trap, clean_dryer_lint_trap_s5).
action(clean_dryer_lint_trap_s5, dry).
object_of(clean_dryer_lint_trap_s5, lint_trap).
takes_time(clean_dryer_lint_trap_s5, 5, 10, minute).
purpose(clean_dryer_lint_trap_s5, no_damp_lint).
step(clean_dryer_lint_trap, clean_dryer_lint_trap_s6).
action(clean_dryer_lint_trap_s6, insert).
object_of(clean_dryer_lint_trap_s6, lint_trap).
into(clean_dryer_lint_trap_s6, dryer).
before(clean_dryer_lint_trap_s1, clean_dryer_lint_trap_s2).
before(clean_dryer_lint_trap_s2, clean_dryer_lint_trap_s3).
before(clean_dryer_lint_trap_s3, clean_dryer_lint_trap_s4).
before(clean_dryer_lint_trap_s4, clean_dryer_lint_trap_s5).
before(clean_dryer_lint_trap_s5, clean_dryer_lint_trap_s6).
```

## aliases
```datalog
task_alias(clean_dryer_lint_trap, "clean dryer lint trap").
task_alias(clean_dryer_lint_trap, "clean lint filter").
task_alias(clean_dryer_lint_trap, "wash lint screen").
alias(lint_trap, "lint filter").
alias(lint_trap, "lint screen").
alias(brush, "small brush").
alias(warm_water, "warm water").
alias(cloth, "cloth").
alias(loose_lint_removed, "lint loosened").
alias(no_damp_lint, "no damp lint left").
```

## teach
- To clean a dryer lint trap you need a brush, warm water and a cloth.
- Cleaning a dryer lint trap takes 10 to 15 minutes.
- Step 1 of cleaning a dryer lint trap: pull the lint trap out of the dryer.
- Step 2 of cleaning a dryer lint trap: scrub the lint trap with the brush so the lint comes loose.
- Step 3 of cleaning a dryer lint trap: rinse the lint trap under warm water.
- Step 4 of cleaning a dryer lint trap: wipe the lint trap with the cloth.
- Step 5 of cleaning a dryer lint trap: let the lint trap dry for 5 to 10 minutes so no damp lint is left.
- Step 6 of cleaning a dryer lint trap: insert the lint trap back into the dryer.

## checklist
- names what is needed, including a brush, warm water and a cloth
- gives the steps in order: pull out, scrub, rinse, wipe, dry, insert
- says the lint trap must be fully dry before it goes back in the dryer
- gives a total time of about 10 to 15 minutes

## variant
- change: no cloth, so use paper towel to wipe the lint trap instead
- request: My dryer lint trap needs cleaning but I don't have a cloth. What can I use?
- situation: I have a small brush, warm water, paper towel and about 15 minutes, but no cloth.
- expect: The reply uses paper towel in place of the cloth for the wiping step and keeps the other steps the same.

```datalog situation
lacks(user, cloth).
has(user, paper_towel).
```

```datalog difference
swap(clean_dryer_lint_trap_s4, cloth, paper_towel).
extra_need(paper_towel).
no_need(cloth).
```

## variant
- change: only 6 minutes: skip the drying step and put the lint trap back once it is rinsed and wiped
- request: I only have about 6 minutes. Can I still clean the lint trap quickly?
- situation: I have a small brush, warm water and a cloth, and only 6 minutes.
- expect: The reply gives the pull, scrub, rinse, wipe and insert steps only, without the drying step, and says it takes about 6 minutes.

```datalog situation
time_limit(user, 6, minute).
```

```datalog difference
drop(clean_dryer_lint_trap_s5).
change(clean_dryer_lint_trap, takes_time, 10_15, 5_6).
```
