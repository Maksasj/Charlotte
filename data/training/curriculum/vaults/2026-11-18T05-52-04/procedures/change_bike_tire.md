---
id: change_bike_tire
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Change a bike tire

Take off the wheel, swap the inner tube and pump the tire up.

```datalog
task(change_bike_tire).
needs(change_bike_tire, wrench).
needs(change_bike_tire, tire_lever).
needs(change_bike_tire, inner_tube).
needs(change_bike_tire, pump).
takes_time(change_bike_tire, 20, 40, minute).
step(change_bike_tire, change_bike_tire_s1).
action(change_bike_tire_s1, remove).
object_of(change_bike_tire_s1, wheel).
uses(change_bike_tire_s1, wrench).
takes_time(change_bike_tire_s1, 3, 5, minute).
step(change_bike_tire, change_bike_tire_s2).
action(change_bike_tire_s2, remove).
object_of(change_bike_tire_s2, tire).
uses(change_bike_tire_s2, tire_lever).
takes_time(change_bike_tire_s2, 3, 5, minute).
step(change_bike_tire, change_bike_tire_s3).
action(change_bike_tire_s3, replace).
object_of(change_bike_tire_s3, inner_tube).
takes_time(change_bike_tire_s3, 3, 5, minute).
step(change_bike_tire, change_bike_tire_s4).
action(change_bike_tire_s4, fit).
object_of(change_bike_tire_s4, tire).
uses(change_bike_tire_s4, tire_lever).
takes_time(change_bike_tire_s4, 3, 5, minute).
step(change_bike_tire, change_bike_tire_s5).
action(change_bike_tire_s5, inflate).
object_of(change_bike_tire_s5, tire).
uses(change_bike_tire_s5, pump).
takes_time(change_bike_tire_s5, 3, 5, minute).
until(change_bike_tire_s5, firm).
step(change_bike_tire, change_bike_tire_s6).
action(change_bike_tire_s6, mount).
object_of(change_bike_tire_s6, wheel).
uses(change_bike_tire_s6, wrench).
takes_time(change_bike_tire_s6, 3, 5, minute).
before(change_bike_tire_s1, change_bike_tire_s2).
before(change_bike_tire_s2, change_bike_tire_s3).
before(change_bike_tire_s3, change_bike_tire_s4).
before(change_bike_tire_s4, change_bike_tire_s5).
before(change_bike_tire_s5, change_bike_tire_s6).
uses(change_bike_tire_s1, bike). @conf 1 @source teacher:curriculum @conv s1794981454946-218805-454
takes_time(change_bike_tire_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981454946-218805-454
```
