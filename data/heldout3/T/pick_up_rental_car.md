---
id: pick_up_rental_car
stage: social
task: pick_up_rental_car
schema: v3
tier: T
---
# Pick up a rental car

## requests
- I've booked a hire car at the airport. What happens when I collect it?
- How do I pick up a rental car without getting stung for damage later?
- What should I check when I collect a car from a rental desk?

## situation
- I have my driving licence, my phone with the booking, and my credit card.

## ground truth
```datalog
task(pick_up_rental_car).
domain(pick_up_rental_car, travel).
goal(pick_up_rental_car, rental_car).
needs(pick_up_rental_car, driving_licence).
needs(pick_up_rental_car, phone).
needs(pick_up_rental_car, payment_card).
takes_time(pick_up_rental_car, 30, 45, minute).
step(pick_up_rental_car, pick_up_rental_car_s1).
action(pick_up_rental_car_s1, show).
object_of(pick_up_rental_car_s1, driving_licence).
step(pick_up_rental_car, pick_up_rental_car_s2).
action(pick_up_rental_car_s2, show).
object_of(pick_up_rental_car_s2, booking).
uses(pick_up_rental_car_s2, phone).
role(pick_up_rental_car_s2, phone, inform).
step(pick_up_rental_car, pick_up_rental_car_s3).
action(pick_up_rental_car_s3, pay).
object_of(pick_up_rental_car_s3, deposit).
uses(pick_up_rental_car_s3, payment_card).
role(pick_up_rental_car_s3, payment_card, pay).
step(pick_up_rental_car, pick_up_rental_car_s4).
action(pick_up_rental_car_s4, photograph).
object_of(pick_up_rental_car_s4, car).
uses(pick_up_rental_car_s4, phone).
role(pick_up_rental_car_s4, phone, record).
purpose(pick_up_rental_car_s4, proof_of_damage).
step(pick_up_rental_car, pick_up_rental_car_s5).
action(pick_up_rental_car_s5, check).
object_of(pick_up_rental_car_s5, fuel_level).
check(pick_up_rental_car_s5, full).
step(pick_up_rental_car, pick_up_rental_car_s6).
action(pick_up_rental_car_s6, adjust).
object_of(pick_up_rental_car_s6, mirror).
purpose(pick_up_rental_car_s6, safe_driving).
before(pick_up_rental_car_s1, pick_up_rental_car_s2).
before(pick_up_rental_car_s2, pick_up_rental_car_s3).
before(pick_up_rental_car_s3, pick_up_rental_car_s4).
before(pick_up_rental_car_s4, pick_up_rental_car_s5).
before(pick_up_rental_car_s5, pick_up_rental_car_s6).
```

## aliases
```datalog
task_alias(pick_up_rental_car, "collecting a hire car").
task_alias(pick_up_rental_car, "renting a car").
task_alias(pick_up_rental_car, "picking up a car hire").
alias(driving_licence, "licence").
alias(booking, "the booking").
alias(car, "hire car").
alias(deposit, "the deposit").
alias(proof_of_damage, "proof of any existing damage").
```

## teach
- To pick up a rental car you need your driving licence, your phone with the booking and a payment card.
- Picking up a rental car takes 30 to 45 minutes.
- Step 1 of picking up a rental car: show your driving licence.
- Step 2 of picking up a rental car: show the booking on your phone.
- Step 3 of picking up a rental car: pay the deposit with your card.
- Step 4 of picking up a rental car: photograph the car all round with your phone as proof of any existing damage.
- Step 5 of picking up a rental car: check the fuel level is full.
- Step 6 of picking up a rental car: adjust the mirrors to drive safely.

## checklist
- names what is needed: licence, booking on the phone and a card
- gives the steps in order: licence, booking, deposit, photos, fuel, mirrors
- says to photograph existing damage

## variant
- change: forgot the driving licence: cannot collect the car
- request: I'm at the rental desk and realised my driving licence is at home. Can I still take the car?
- situation: I have my phone with the booking and my card, but not my driving licence.
- expect: The reply says the car cannot be released without the licence, and suggests asking whether another named driver can collect it or getting the licence brought.

```datalog situation
has(user, phone).
has(user, payment_card).
lacks(user, driving_licence).
```

```datalog difference
cannot(no_licence).
```

## variant
- change: travelling with a toddler: fit a child seat before driving off
- request: I'm collecting a hire car with my two-year-old. What extra should I do?
- situation: I have my licence, phone and card, and I booked a child seat with the car.
- expect: The reply adds fitting the child seat and checking it is firm before adjusting the mirrors.

```datalog situation
has(user, driving_licence).
has(user, phone).
has(user, payment_card).
has(user, child_seat).
wants(user, child_seat).
```

```datalog difference
extra_need(child_seat).
add_step(pick_up_rental_car_s9, pick_up_rental_car_s5).
step(pick_up_rental_car, pick_up_rental_car_s9).
action(pick_up_rental_car_s9, fit).
object_of(pick_up_rental_car_s9, child_seat).
check(pick_up_rental_car_s9, firm).
before(pick_up_rental_car_s5, pick_up_rental_car_s9).
before(pick_up_rental_car_s9, pick_up_rental_car_s6).
```
