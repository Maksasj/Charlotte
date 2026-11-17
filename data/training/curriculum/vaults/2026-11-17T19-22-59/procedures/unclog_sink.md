---
id: unclog_sink
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Unclog a sink

Clear a slow drain with baking soda, vinegar, hot water and a plunger.

```datalog
task(unclog_sink).
needs(unclog_sink, baking_soda).
needs(unclog_sink, vinegar).
needs(unclog_sink, water).
needs(unclog_sink, kettle).
needs(unclog_sink, plunger).
takes_time(unclog_sink, 20, 30, minute).
step(unclog_sink, unclog_sink_s1).
action(unclog_sink_s1, pour).
object_of(unclog_sink_s1, baking_soda).
takes_time(unclog_sink_s1, 1, 1, minute).
step(unclog_sink, unclog_sink_s2).
action(unclog_sink_s2, pour).
object_of(unclog_sink_s2, vinegar).
takes_time(unclog_sink_s2, 1, 1, minute).
step(unclog_sink, unclog_sink_s3).
action(unclog_sink_s3, wait).
object_of(unclog_sink_s3, drain).
takes_time(unclog_sink_s3, 10, 15, minute).
step(unclog_sink, unclog_sink_s4).
action(unclog_sink_s4, boil).
object_of(unclog_sink_s4, water).
uses(unclog_sink_s4, kettle).
takes_time(unclog_sink_s4, 3, 4, minute).
step(unclog_sink, unclog_sink_s5).
action(unclog_sink_s5, pour).
object_of(unclog_sink_s5, water).
takes_time(unclog_sink_s5, 1, 1, minute).
step(unclog_sink, unclog_sink_s6).
action(unclog_sink_s6, plunge).
object_of(unclog_sink_s6, drain).
uses(unclog_sink_s6, plunger).
takes_time(unclog_sink_s6, 2, 5, minute).
until(unclog_sink_s6, clear).
before(unclog_sink_s1, unclog_sink_s2).
before(unclog_sink_s2, unclog_sink_s3).
overlaps(unclog_sink_s4, unclog_sink_s3).
before(unclog_sink_s3, unclog_sink_s5).
before(unclog_sink_s4, unclog_sink_s5).
before(unclog_sink_s5, unclog_sink_s6).
```
