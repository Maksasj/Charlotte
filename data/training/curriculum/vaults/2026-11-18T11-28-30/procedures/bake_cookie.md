---
id: bake_cookie
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Bake cookies

Mix a simple dough and bake cookies until golden.

```datalog
task(bake_cookie).
solves(bake_cookie, plan_meal).
needs(bake_cookie, flour).
needs(bake_cookie, sugar).
needs(bake_cookie, butter).
needs(bake_cookie, egg).
needs(bake_cookie, oven).
needs(bake_cookie, baking_tray).
takes_time(bake_cookie, 25, 40, minute).
step(bake_cookie, bake_cookie_s1).
action(bake_cookie_s1, heat).
object_of(bake_cookie_s1, oven).
takes_time(bake_cookie_s1, 10, 15, minute).
until(bake_cookie_s1, hot).
step(bake_cookie, bake_cookie_s2).
action(bake_cookie_s2, mix).
object_of(bake_cookie_s2, butter).
uses(bake_cookie_s2, sugar).
takes_time(bake_cookie_s2, 3, 5, minute).
step(bake_cookie, bake_cookie_s3).
action(bake_cookie_s3, mix).
object_of(bake_cookie_s3, flour).
uses(bake_cookie_s3, egg).
takes_time(bake_cookie_s3, 3, 5, minute).
step(bake_cookie, bake_cookie_s4).
action(bake_cookie_s4, shape).
object_of(bake_cookie_s4, dough).
uses(bake_cookie_s4, baking_tray).
takes_time(bake_cookie_s4, 5, 5, minute).
step(bake_cookie, bake_cookie_s5).
action(bake_cookie_s5, bake).
object_of(bake_cookie_s5, dough).
uses(bake_cookie_s5, oven).
takes_time(bake_cookie_s5, 10, 12, minute).
until(bake_cookie_s5, golden).
step(bake_cookie, bake_cookie_s6).
action(bake_cookie_s6, cool).
object_of(bake_cookie_s6, cookie).
takes_time(bake_cookie_s6, 5, 10, minute).
overlaps(bake_cookie_s2, bake_cookie_s1).
before(bake_cookie_s2, bake_cookie_s3).
before(bake_cookie_s3, bake_cookie_s4).
before(bake_cookie_s1, bake_cookie_s5).
before(bake_cookie_s4, bake_cookie_s5).
before(bake_cookie_s5, bake_cookie_s6).
```
