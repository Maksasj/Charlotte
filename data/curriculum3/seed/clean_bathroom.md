---
id: clean_bathroom
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Clean the bathroom

Scrub the toilet and sink, wipe the mirror and mop the floor.

## ground truth
```datalog
task(clean_bathroom).
domain(clean_bathroom, cleaning).
goal(clean_bathroom, bathroom_clean).
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
role(clean_bathroom_s1, bathroom_cleaner, clean).
takes_time(clean_bathroom_s1, 2, 3, minute).
step(clean_bathroom, clean_bathroom_s2).
action(clean_bathroom_s2, scrub).
object_of(clean_bathroom_s2, toilet).
uses(clean_bathroom_s2, toilet_brush).
role(clean_bathroom_s2, toilet_brush, scrub).
uses(clean_bathroom_s2, bathroom_cleaner).
role(clean_bathroom_s2, bathroom_cleaner, clean).
takes_time(clean_bathroom_s2, 5, 10, minute).
until(clean_bathroom_s2, clean).
step(clean_bathroom, clean_bathroom_s3).
action(clean_bathroom_s3, scrub).
object_of(clean_bathroom_s3, sink).
uses(clean_bathroom_s3, sponge).
role(clean_bathroom_s3, sponge, scrub).
takes_time(clean_bathroom_s3, 5, 10, minute).
until(clean_bathroom_s3, clean).
step(clean_bathroom, clean_bathroom_s4).
action(clean_bathroom_s4, wipe).
object_of(clean_bathroom_s4, mirror).
uses(clean_bathroom_s4, glass_cleaner).
role(clean_bathroom_s4, glass_cleaner, clean).
takes_time(clean_bathroom_s4, 2, 5, minute).
until(clean_bathroom_s4, clean).
step(clean_bathroom, clean_bathroom_s5).
action(clean_bathroom_s5, mop).
object_of(clean_bathroom_s5, floor).
uses(clean_bathroom_s5, mop).
role(clean_bathroom_s5, mop, wipe).
takes_time(clean_bathroom_s5, 10, 15, minute).
until(clean_bathroom_s5, clean).
before(clean_bathroom_s1, clean_bathroom_s2).
before(clean_bathroom_s2, clean_bathroom_s3).
before(clean_bathroom_s3, clean_bathroom_s4).
before(clean_bathroom_s4, clean_bathroom_s5).
```

## aliases
```datalog
task_alias(clean_bathroom, "cleaning the bathroom").
task_alias(clean_bathroom, "bathroom cleaning").
task_alias(clean_bathroom, "clean the toilet and sink").
alias(bathroom_cleaner, "bathroom cleaner").
alias(bathroom_cleaner, "bathroom spray").
alias(toilet_brush, "toilet brush").
alias(glass_cleaner, "glass cleaner").
alias(glass_cleaner, "window spray").
alias(floor, "bathroom floor").
alias(mop, "mop the floor").
```
