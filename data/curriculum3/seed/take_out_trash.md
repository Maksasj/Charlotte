---
id: take_out_trash
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Take out the trash

Tie up the full bag, take it to the outdoor bin and put in a fresh bag.

## ground truth
```datalog
task(take_out_trash).
domain(take_out_trash, cleaning).
goal(take_out_trash, trash_removed).
needs(take_out_trash, trash_bag).
needs(take_out_trash, outdoor_bin).
takes_time(take_out_trash, 3, 5, minute).
step(take_out_trash, take_out_trash_s1).
action(take_out_trash_s1, tie).
object_of(take_out_trash_s1, trash_bag).
step(take_out_trash, take_out_trash_s2).
action(take_out_trash_s2, lift).
object_of(take_out_trash_s2, trash_bag).
from(take_out_trash_s2, trash_can).
step(take_out_trash, take_out_trash_s3).
action(take_out_trash_s3, carry).
object_of(take_out_trash_s3, trash_bag).
takes_time(take_out_trash_s3, 1, 3, minute).
step(take_out_trash, take_out_trash_s4).
action(take_out_trash_s4, put).
object_of(take_out_trash_s4, trash_bag).
uses(take_out_trash_s4, outdoor_bin).
role(take_out_trash_s4, outdoor_bin, contain).
into(take_out_trash_s4, outdoor_bin).
step(take_out_trash, take_out_trash_s5).
action(take_out_trash_s5, line).
object_of(take_out_trash_s5, trash_can).
uses(take_out_trash_s5, trash_bag).
role(take_out_trash_s5, trash_bag, contain).
takes_time(take_out_trash_s5, 1, 1, minute).
before(take_out_trash_s1, take_out_trash_s2).
before(take_out_trash_s2, take_out_trash_s3).
before(take_out_trash_s3, take_out_trash_s4).
before(take_out_trash_s4, take_out_trash_s5).
```

## aliases
```datalog
task_alias(take_out_trash, "taking out the trash").
task_alias(take_out_trash, "take out the rubbish").
task_alias(take_out_trash, "take the bins out").
alias(trash_bag, "bin bag").
alias(trash_bag, "rubbish bag").
alias(trash_bag, "garbage bag").
alias(trash_bag, "fresh bag").
alias(outdoor_bin, "outdoor bin").
alias(outdoor_bin, "wheelie bin").
alias(trash_can, "trash can").
alias(trash_can, "kitchen bin").
alias(tie, "tie up").
alias(line, "put in").
```
