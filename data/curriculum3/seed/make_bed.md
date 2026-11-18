---
id: make_bed
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Make the bed

Straighten the sheet, spread the duvet and set the pillows.

## ground truth
```datalog
task(make_bed).
domain(make_bed, cleaning).
goal(make_bed, made_bed).
needs(make_bed, sheet).
needs(make_bed, duvet).
needs(make_bed, pillow).
takes_time(make_bed, 5, 10, minute).
step(make_bed, make_bed_s1).
action(make_bed_s1, smooth).
object_of(make_bed_s1, sheet).
takes_time(make_bed_s1, 1, 2, minute).
until(make_bed_s1, flat).
step(make_bed, make_bed_s2).
action(make_bed_s2, tuck).
object_of(make_bed_s2, sheet).
takes_time(make_bed_s2, 1, 2, minute).
step(make_bed, make_bed_s3).
action(make_bed_s3, shake).
object_of(make_bed_s3, duvet).
takes_time(make_bed_s3, 1, 2, minute).
step(make_bed, make_bed_s4).
action(make_bed_s4, spread).
object_of(make_bed_s4, duvet).
takes_time(make_bed_s4, 1, 2, minute).
until(make_bed_s4, flat).
onto(make_bed_s4, bed).
step(make_bed, make_bed_s5).
action(make_bed_s5, place).
object_of(make_bed_s5, pillow).
takes_time(make_bed_s5, 1, 1, minute).
onto(make_bed_s5, bed).
before(make_bed_s1, make_bed_s2).
before(make_bed_s2, make_bed_s3).
before(make_bed_s3, make_bed_s4).
before(make_bed_s4, make_bed_s5).
```

## aliases
```datalog
task_alias(make_bed, "making the bed").
task_alias(make_bed, "make my bed").
task_alias(make_bed, "make a bed").
alias(sheet, "bed sheet").
alias(duvet, "quilt").
alias(duvet, "comforter").
alias(pillow, "pillows").
alias(smooth, "straighten").
alias(place, "set").
alias(flat, "smooth").
```
