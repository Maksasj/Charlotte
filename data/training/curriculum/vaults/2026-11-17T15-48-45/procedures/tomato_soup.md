---
id: tomato_soup
status: confirmed
source: seed
uses: 0
successes: 0
---
# Tomato soup

```datalog
task(tomato_soup).
solves(tomato_soup, plan_meal).
needs(tomato_soup, tomato).
needs(tomato_soup, onion).
needs(tomato_soup, stock).
takes_time(tomato_soup, 30, 40, minute).
step(tomato_soup, ts_s1).
action(ts_s1, fry).
object_of(ts_s1, onion).
step(tomato_soup, ts_s2).
action(ts_s2, simmer).
object_of(ts_s2, tomato).
uses(ts_s2, stock).
takes_time(ts_s2, 25, 30, minute).
before(ts_s1, ts_s2).
```
