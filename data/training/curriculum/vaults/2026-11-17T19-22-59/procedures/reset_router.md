---
id: reset_router
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Reset the router

Power-cycle the router and check that the connection is back.

```datalog
task(reset_router).
needs(reset_router, router).
needs(reset_router, phone).
takes_time(reset_router, 5, 10, minute).
step(reset_router, reset_router_s1).
action(reset_router_s1, unplug).
object_of(reset_router_s1, router).
step(reset_router, reset_router_s2).
action(reset_router_s2, wait).
object_of(reset_router_s2, router).
takes_time(reset_router_s2, 1, 1, minute).
step(reset_router, reset_router_s3).
action(reset_router_s3, plug).
object_of(reset_router_s3, router).
step(reset_router, reset_router_s4).
action(reset_router_s4, wait).
object_of(reset_router_s4, status_light).
takes_time(reset_router_s4, 2, 5, minute).
until(reset_router_s4, steady).
step(reset_router, reset_router_s5).
action(reset_router_s5, test).
object_of(reset_router_s5, connection).
uses(reset_router_s5, phone).
takes_time(reset_router_s5, 1, 2, minute).
before(reset_router_s1, reset_router_s2).
before(reset_router_s2, reset_router_s3).
before(reset_router_s3, reset_router_s4).
before(reset_router_s4, reset_router_s5).
```
