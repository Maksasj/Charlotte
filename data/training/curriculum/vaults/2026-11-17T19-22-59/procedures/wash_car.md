---
id: wash_car
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Wash the car

Rinse, soap, rinse again and dry the car by hand.

```datalog
task(wash_car).
needs(wash_car, hose).
needs(wash_car, bucket).
needs(wash_car, car_shampoo).
needs(wash_car, sponge).
needs(wash_car, towel).
takes_time(wash_car, 40, 60, minute).
step(wash_car, wash_car_s1).
action(wash_car_s1, rinse).
object_of(wash_car_s1, car).
uses(wash_car_s1, hose).
takes_time(wash_car_s1, 5, 5, minute).
step(wash_car, wash_car_s2).
action(wash_car_s2, fill).
object_of(wash_car_s2, bucket).
uses(wash_car_s2, car_shampoo).
takes_time(wash_car_s2, 2, 3, minute).
step(wash_car, wash_car_s3).
action(wash_car_s3, wash).
object_of(wash_car_s3, car).
uses(wash_car_s3, sponge).
uses(wash_car_s3, bucket).
takes_time(wash_car_s3, 20, 30, minute).
until(wash_car_s3, clean).
step(wash_car, wash_car_s4).
action(wash_car_s4, rinse).
object_of(wash_car_s4, car).
uses(wash_car_s4, hose).
takes_time(wash_car_s4, 5, 10, minute).
step(wash_car, wash_car_s5).
action(wash_car_s5, dry).
object_of(wash_car_s5, car).
uses(wash_car_s5, towel).
takes_time(wash_car_s5, 10, 15, minute).
until(wash_car_s5, dry).
overlaps(wash_car_s2, wash_car_s1).
before(wash_car_s1, wash_car_s3).
before(wash_car_s2, wash_car_s3).
before(wash_car_s3, wash_car_s4).
before(wash_car_s4, wash_car_s5).
```
