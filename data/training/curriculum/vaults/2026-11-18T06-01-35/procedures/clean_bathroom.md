---
id: clean_bathroom
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Clean the bathroom

Scrub the toilet and sink, wipe the mirror and mop the floor.

```datalog
task(clean_bathroom).
needs(clean_bathroom, bathroom_cleaner).
needs(clean_bathroom, toilet_brush).
needs(clean_bathroom, sponge).
needs(clean_bathroom, glass_cleaner).
needs(clean_bathroom, mop).
takes_time(clean_bathroom, 25, 45, minute).
step(clean_bathroom, clean_bathroom_s1).
action(clean_bathroom_s1, spray).
object_of(clean_bathroom_s1, sink).
uses(clean_bathroom_s1, bathroom_cleaner).
takes_time(clean_bathroom_s1, 2, 3, minute).
step(clean_bathroom, clean_bathroom_s2).
action(clean_bathroom_s2, scrub).
object_of(clean_bathroom_s2, toilet).
uses(clean_bathroom_s2, toilet_brush).
uses(clean_bathroom_s2, bathroom_cleaner).
takes_time(clean_bathroom_s2, 5, 10, minute).
until(clean_bathroom_s2, clean).
step(clean_bathroom, clean_bathroom_s3).
action(clean_bathroom_s3, scrub).
object_of(clean_bathroom_s3, sink).
uses(clean_bathroom_s3, sponge).
takes_time(clean_bathroom_s3, 5, 10, minute).
until(clean_bathroom_s3, clean).
step(clean_bathroom, clean_bathroom_s4).
action(clean_bathroom_s4, wipe).
object_of(clean_bathroom_s4, mirror).
uses(clean_bathroom_s4, glass_cleaner).
takes_time(clean_bathroom_s4, 2, 5, minute).
until(clean_bathroom_s4, clean).
step(clean_bathroom, clean_bathroom_s5).
action(clean_bathroom_s5, mop).
object_of(clean_bathroom_s5, floor).
uses(clean_bathroom_s5, mop).
takes_time(clean_bathroom_s5, 10, 15, minute).
until(clean_bathroom_s5, clean).
before(clean_bathroom_s1, clean_bathroom_s2).
before(clean_bathroom_s2, clean_bathroom_s3).
before(clean_bathroom_s3, clean_bathroom_s4).
before(clean_bathroom_s4, clean_bathroom_s5).
```
