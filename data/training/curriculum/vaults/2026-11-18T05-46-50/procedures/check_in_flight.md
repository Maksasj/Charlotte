---
id: check_in_flight
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Check in for a flight

Check in online and save the boarding pass on the phone.

```datalog
task(check_in_flight).
needs(check_in_flight, phone).
needs(check_in_flight, booking_reference).
needs(check_in_flight, passport).
takes_time(check_in_flight, 10, 15, minute).
step(check_in_flight, check_in_flight_s1).
action(check_in_flight_s1, open).
object_of(check_in_flight_s1, airline_app).
uses(check_in_flight_s1, phone).
takes_time(check_in_flight_s1, 1, 1, minute).
step(check_in_flight, check_in_flight_s2).
action(check_in_flight_s2, enter).
object_of(check_in_flight_s2, booking_reference).
uses(check_in_flight_s2, phone).
takes_time(check_in_flight_s2, 1, 2, minute).
step(check_in_flight, check_in_flight_s3).
action(check_in_flight_s3, enter).
object_of(check_in_flight_s3, passport_detail).
uses(check_in_flight_s3, passport).
takes_time(check_in_flight_s3, 2, 4, minute).
step(check_in_flight, check_in_flight_s4).
action(check_in_flight_s4, choose).
object_of(check_in_flight_s4, seat).
takes_time(check_in_flight_s4, 1, 3, minute).
step(check_in_flight, check_in_flight_s5).
action(check_in_flight_s5, save).
object_of(check_in_flight_s5, boarding_pass).
uses(check_in_flight_s5, phone).
takes_time(check_in_flight_s5, 1, 1, minute).
before(check_in_flight_s1, check_in_flight_s2).
before(check_in_flight_s2, check_in_flight_s3).
before(check_in_flight_s3, check_in_flight_s4).
before(check_in_flight_s4, check_in_flight_s5).
```
