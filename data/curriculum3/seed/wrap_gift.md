---
id: wrap_gift
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Wrap a gift

Cut paper to size, wrap the gift neatly and tie a ribbon.

## ground truth
```datalog
task(wrap_gift).
domain(wrap_gift, crafts).
goal(wrap_gift, wrapped_gift).
needs(wrap_gift, gift).
needs(wrap_gift, wrapping_paper).
needs(wrap_gift, scissors).
needs(wrap_gift, tape).
needs(wrap_gift, ribbon).
takes_time(wrap_gift, 10, 15, minute).
step(wrap_gift, wrap_gift_s1).
action(wrap_gift_s1, measure).
object_of(wrap_gift_s1, wrapping_paper).
uses(wrap_gift_s1, gift).
role(wrap_gift_s1, gift, measure).
takes_time(wrap_gift_s1, 1, 2, minute).
step(wrap_gift, wrap_gift_s2).
action(wrap_gift_s2, cut).
object_of(wrap_gift_s2, wrapping_paper).
uses(wrap_gift_s2, scissors).
role(wrap_gift_s2, scissors, cut).
takes_time(wrap_gift_s2, 1, 2, minute).
step(wrap_gift, wrap_gift_s3).
action(wrap_gift_s3, fold).
object_of(wrap_gift_s3, wrapping_paper).
uses(wrap_gift_s3, gift).
role(wrap_gift_s3, gift, hold).
takes_time(wrap_gift_s3, 2, 4, minute).
step(wrap_gift, wrap_gift_s4).
action(wrap_gift_s4, tape).
object_of(wrap_gift_s4, seam).
uses(wrap_gift_s4, tape).
role(wrap_gift_s4, tape, fasten).
takes_time(wrap_gift_s4, 2, 3, minute).
step(wrap_gift, wrap_gift_s5).
action(wrap_gift_s5, tie).
object_of(wrap_gift_s5, ribbon).
uses(wrap_gift_s5, scissors).
role(wrap_gift_s5, scissors, cut).
takes_time(wrap_gift_s5, 2, 3, minute).
onto(wrap_gift_s5, gift).
before(wrap_gift_s1, wrap_gift_s2).
before(wrap_gift_s2, wrap_gift_s3).
before(wrap_gift_s3, wrap_gift_s4).
before(wrap_gift_s4, wrap_gift_s5).
```

## aliases
```datalog
task_alias(wrap_gift, "wrapping a gift").
task_alias(wrap_gift, "wrap a present").
task_alias(wrap_gift, "gift wrapping").
alias(gift, "present").
alias(wrapping_paper, "wrapping paper").
alias(wrapping_paper, "gift paper").
alias(tape, "sticky tape").
alias(seam, "edges").
alias(measure, "cut to size").
alias(fold, "wrap").
```
