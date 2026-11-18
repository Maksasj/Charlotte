---
id: wash_car__no_towel
task: wash_car
schema: v3
tier: S
base: wash_car
---
# Wash the car

## requests
- The car's filthy and I want to give it a wash on the drive, but I've no drying towels left. I do have the rubber squeegee from the windows.

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

## teach
- To wash the car you need a hose, a bucket, car shampoo, a sponge and a towel.
- Washing the car takes 40 to 60 minutes.
- Step 1 of washing the car: rinse the car with the hose for 5 minutes.
- While step 1 runs, step 2 of washing the car: fill the bucket with water and car shampoo for 2 to 3 minutes.
- Step 3 of washing the car: wash the car with the sponge from the bucket until it is clean, for 20 to 30 minutes.
- Step 4 of washing the car: rinse the car again with the hose for 5 to 10 minutes.
- Step 5 of washing the car: dry the car with the towel until it is dry, for 10 to 15 minutes.

## variant
- change: no drying towel: dry the car with a squeegee
- request: The car's filthy and I want to give it a wash on the drive, but I've no drying towels left. I do have the rubber squeegee from the windows.
- situation: I have a hose, a bucket, car shampoo, a sponge and a window squeegee, but no towel.
- expect: The reply dries the car with the squeegee instead of a towel and keeps the rinse, soap and second rinse.

```datalog situation
has(user, hose).
has(user, bucket).
has(user, car_shampoo).
has(user, sponge).
has(user, squeegee).
lacks(user, towel).
```

```datalog difference
swap(wash_car_s5, towel, squeegee).
no_need(towel).
extra_need(squeegee).
```
