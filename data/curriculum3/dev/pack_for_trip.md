---
id: pack_for_trip
stage: social
task: pack_for_trip
schema: v3
---
# Pack for a trip

## requests
- I'm flying abroad for a week on Friday and I always forget something. How should I pack?
- Can you help me pack my suitcase for a holiday?
- I'm terrible at packing. What's a good way to get my bag ready for a trip?

## situation
- I have a suitcase, my clothes, a toiletry bag and my passport.
- I've got about 45 minutes tonight to do it.

## ground truth
```datalog
task(pack_for_trip).
domain(pack_for_trip, travel).
goal(pack_for_trip, packed_bag).
needs(pack_for_trip, suitcase).
needs(pack_for_trip, clothing).
needs(pack_for_trip, toiletry_bag).
needs(pack_for_trip, passport).
takes_time(pack_for_trip, 30, 45, minute).
step(pack_for_trip, pack_for_trip_s1).
action(pack_for_trip_s1, check).
object_of(pack_for_trip_s1, weather).
purpose(pack_for_trip_s1, right_clothes).
takes_time(pack_for_trip_s1, 2, 3, minute).
step(pack_for_trip, pack_for_trip_s2).
action(pack_for_trip_s2, write).
object_of(pack_for_trip_s2, packing_list).
purpose(pack_for_trip_s2, nothing_forgotten).
takes_time(pack_for_trip_s2, 5, 10, minute).
step(pack_for_trip, pack_for_trip_s3).
action(pack_for_trip_s3, lay).
object_of(pack_for_trip_s3, clothing).
onto(pack_for_trip_s3, bed).
takes_time(pack_for_trip_s3, 5, 10, minute).
step(pack_for_trip, pack_for_trip_s4).
action(pack_for_trip_s4, roll).
object_of(pack_for_trip_s4, clothing).
uses(pack_for_trip_s4, suitcase).
role(pack_for_trip_s4, suitcase, contain).
takes_time(pack_for_trip_s4, 10, 12, minute).
step(pack_for_trip, pack_for_trip_s5).
action(pack_for_trip_s5, pack).
object_of(pack_for_trip_s5, toiletry_bag).
takes_time(pack_for_trip_s5, 5, 7, minute).
step(pack_for_trip, pack_for_trip_s6).
action(pack_for_trip_s6, put).
object_of(pack_for_trip_s6, passport).
into(pack_for_trip_s6, hand_luggage).
takes_time(pack_for_trip_s6, 1, 2, minute).
before(pack_for_trip_s1, pack_for_trip_s2).
before(pack_for_trip_s2, pack_for_trip_s3).
before(pack_for_trip_s3, pack_for_trip_s4).
before(pack_for_trip_s4, pack_for_trip_s5).
before(pack_for_trip_s5, pack_for_trip_s6).
```

## aliases
```datalog
task_alias(pack_for_trip, "packing for a trip").
task_alias(pack_for_trip, "pack my suitcase").
task_alias(pack_for_trip, "packing").
task_alias(pack_for_trip, "get my bag ready").
alias(clothing, "clothes").
alias(clothing, "my clothes").
alias(clothing, "few clothes").
alias(weather, "weather at your destination").
alias(toiletry_bag, "toiletries").
alias(lay, "lay out").
alias(hand_luggage, "hand luggage").
alias(cabin_bag, "small cabin bag").
alias(clear_bag, "clear bag").
alias(liquid, "small liquids").
alias(pass_security, "airport security").
alias(right_clothes, "pack the right clothes").
alias(nothing_forgotten, "forget something").
```

## teach
- To pack for a trip you need a suitcase, clothing, a toiletry bag and your passport.
- Packing for a trip takes 30 to 45 minutes.
- Step 1 of packing for a trip: check the weather at your destination so you pack the right clothes, for 2 to 3 minutes.
- Step 2 of packing for a trip: write a packing list so you do not forget anything, for 5 to 10 minutes.
- Step 3 of packing for a trip: lay out the clothing on the bed for 5 to 10 minutes.
- Step 4 of packing for a trip: roll the clothing tightly into the suitcase for 10 to 12 minutes.
- Step 5 of packing for a trip: pack the toiletry bag for 5 to 7 minutes.
- Step 6 of packing for a trip: put the passport in your hand luggage for 1 to 2 minutes.

## checklist
- suggests checking the weather and making a packing list
- gives the steps in order: weather, list, lay out, roll into suitcase, toiletries, passport
- reminds the user to keep the passport with them
- gives a total time of about 30 to 45 minutes

## variant
- change: carry-on only for a weekend: a small cabin bag and liquids in a clear bag
- request: I'm only going away for the weekend and want to take just hand luggage. How do I pack for that?
- situation: It's two nights away. I only have a small cabin bag, a few clothes, my toiletries and my passport.
- expect: The reply packs fewer clothes into the cabin bag and puts small liquids in a clear bag for airport security.

```datalog situation
count(user, night, 2).
has(user, cabin_bag).
has(user, clothing).
has(user, toiletry_bag).
has(user, passport).
lacks(user, suitcase).
```

```datalog difference
swap(pack_for_trip_s4, suitcase, cabin_bag).
no_need(suitcase).
extra_need(cabin_bag).
extra_need(clear_bag).
add_step(pack_for_trip_s9, pack_for_trip_s5).
step(pack_for_trip, pack_for_trip_s9).
action(pack_for_trip_s9, put).
object_of(pack_for_trip_s9, liquid).
into(pack_for_trip_s9, clear_bag).
purpose(pack_for_trip_s9, pass_security).
before(pack_for_trip_s5, pack_for_trip_s9).
```
