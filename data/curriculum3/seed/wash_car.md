---
id: wash_car
status: hypothesis
source: seed
uses: 0
successes: 0
schema: v3
---
# Wash the car

Rinse, soap, rinse again and dry the car by hand.

## ground truth
```datalog
task(wash_car).
domain(wash_car, cleaning).
goal(wash_car, clean_car).
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
role(wash_car_s1, hose, rinse).
takes_time(wash_car_s1, 5, 5, minute).
step(wash_car, wash_car_s2).
action(wash_car_s2, fill).
object_of(wash_car_s2, bucket).
uses(wash_car_s2, car_shampoo).
role(wash_car_s2, car_shampoo, clean).
takes_time(wash_car_s2, 2, 3, minute).
step(wash_car, wash_car_s3).
action(wash_car_s3, wash).
object_of(wash_car_s3, car).
uses(wash_car_s3, sponge).
role(wash_car_s3, sponge, scrub).
uses(wash_car_s3, bucket).
role(wash_car_s3, bucket, contain).
takes_time(wash_car_s3, 20, 30, minute).
until(wash_car_s3, clean).
step(wash_car, wash_car_s4).
action(wash_car_s4, rinse).
object_of(wash_car_s4, car).
uses(wash_car_s4, hose).
role(wash_car_s4, hose, rinse).
takes_time(wash_car_s4, 5, 10, minute).
step(wash_car, wash_car_s5).
action(wash_car_s5, dry).
object_of(wash_car_s5, car).
uses(wash_car_s5, towel).
role(wash_car_s5, towel, dry).
takes_time(wash_car_s5, 10, 15, minute).
until(wash_car_s5, dry).
overlaps(wash_car_s2, wash_car_s1).
before(wash_car_s1, wash_car_s3).
before(wash_car_s2, wash_car_s3).
before(wash_car_s3, wash_car_s4).
before(wash_car_s4, wash_car_s5).
```

## aliases
```datalog
task_alias(wash_car, "washing the car").
task_alias(wash_car, "car wash").
task_alias(wash_car, "clean the car").
alias(car_shampoo, "car shampoo").
alias(car_shampoo, "car soap").
alias(hose, "garden hose").
alias(hose, "hosepipe").
alias(towel, "drying towel").
alias(sponge, "wash mitt").
alias(wash, "soap").
alias(dry, "dry by hand").
```
