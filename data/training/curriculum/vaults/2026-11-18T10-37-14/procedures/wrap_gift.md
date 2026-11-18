---
id: wrap_gift
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Wrap a gift

Cut paper to size, wrap the gift neatly and tie a ribbon.

```datalog
task(wrap_gift).
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
takes_time(wrap_gift_s1, 1, 2, minute).
step(wrap_gift, wrap_gift_s2).
action(wrap_gift_s2, cut).
object_of(wrap_gift_s2, wrapping_paper).
uses(wrap_gift_s2, scissors).
takes_time(wrap_gift_s2, 1, 2, minute).
step(wrap_gift, wrap_gift_s3).
action(wrap_gift_s3, fold).
object_of(wrap_gift_s3, wrapping_paper).
uses(wrap_gift_s3, gift).
takes_time(wrap_gift_s3, 2, 4, minute).
step(wrap_gift, wrap_gift_s4).
action(wrap_gift_s4, tape).
object_of(wrap_gift_s4, seam).
uses(wrap_gift_s4, tape).
takes_time(wrap_gift_s4, 2, 3, minute).
step(wrap_gift, wrap_gift_s5).
action(wrap_gift_s5, tie).
object_of(wrap_gift_s5, ribbon).
uses(wrap_gift_s5, scissors).
takes_time(wrap_gift_s5, 2, 3, minute).
before(wrap_gift_s1, wrap_gift_s2).
before(wrap_gift_s2, wrap_gift_s3).
before(wrap_gift_s3, wrap_gift_s4).
before(wrap_gift_s4, wrap_gift_s5).
```
