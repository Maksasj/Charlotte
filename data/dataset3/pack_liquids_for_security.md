---
id: pack_liquids_for_security
stage: procedures
task: pack_liquids_for_security
schema: v3
---
# Pack liquids for security

## requests
- I'm flying next week and need to get my shampoo through airport security in my carry-on. How do I pack it?
- What's the proper way to pack toiletry liquids so they're allowed through the security checkpoint?
- Could you walk me through putting my bottles of liquid into a bag for airport security?

## situation
- I have a bottle of shampoo, an empty small travel bottle, a clear resealable liquid bag and a carry-on bag.
- I'm flying tomorrow and want the liquids in my hand luggage.

## ground truth
```datalog
task(pack_liquids_for_security).
domain(pack_liquids_for_security, travel).
goal(pack_liquids_for_security, liquids_through_security).
needs(pack_liquids_for_security, shampoo).
needs(pack_liquids_for_security, travel_bottle).
needs(pack_liquids_for_security, liquid_bag).
needs(pack_liquids_for_security, carry_on_bag).
takes_time(pack_liquids_for_security, 10, 15, minute).
step(pack_liquids_for_security, pack_liquids_for_security_s1).
action(pack_liquids_for_security_s1, pour).
object_of(pack_liquids_for_security_s1, shampoo).
into(pack_liquids_for_security_s1, travel_bottle).
uses(pack_liquids_for_security_s1, travel_bottle).
role(pack_liquids_for_security_s1, travel_bottle, contain).
amount(pack_liquids_for_security_s1, shampoo, 100, millilitre).
step(pack_liquids_for_security, pack_liquids_for_security_s2).
action(pack_liquids_for_security_s2, fasten).
object_of(pack_liquids_for_security_s2, travel_bottle).
uses(pack_liquids_for_security_s2, travel_bottle).
role(pack_liquids_for_security_s2, travel_bottle, contain).
check(pack_liquids_for_security_s2, lid_tight).
step(pack_liquids_for_security, pack_liquids_for_security_s3).
action(pack_liquids_for_security_s3, pack).
object_of(pack_liquids_for_security_s3, travel_bottle).
into(pack_liquids_for_security_s3, liquid_bag).
uses(pack_liquids_for_security_s3, liquid_bag).
role(pack_liquids_for_security_s3, liquid_bag, contain).
step(pack_liquids_for_security, pack_liquids_for_security_s4).
action(pack_liquids_for_security_s4, fasten).
object_of(pack_liquids_for_security_s4, liquid_bag).
uses(pack_liquids_for_security_s4, liquid_bag).
role(pack_liquids_for_security_s4, liquid_bag, contain).
check(pack_liquids_for_security_s4, bag_sealed).
step(pack_liquids_for_security, pack_liquids_for_security_s5).
action(pack_liquids_for_security_s5, pack).
object_of(pack_liquids_for_security_s5, liquid_bag).
into(pack_liquids_for_security_s5, carry_on_bag).
uses(pack_liquids_for_security_s5, carry_on_bag).
role(pack_liquids_for_security_s5, carry_on_bag, contain).
purpose(pack_liquids_for_security_s5, easy_screening).
before(pack_liquids_for_security_s1, pack_liquids_for_security_s2).
before(pack_liquids_for_security_s2, pack_liquids_for_security_s3).
before(pack_liquids_for_security_s3, pack_liquids_for_security_s4).
before(pack_liquids_for_security_s4, pack_liquids_for_security_s5).
```

## aliases
```datalog
task_alias(pack_liquids_for_security, "pack liquids for security").
task_alias(pack_liquids_for_security, "packing liquids for airport security").
task_alias(pack_liquids_for_security, "carry liquids through security").
alias(travel_bottle, "small travel bottle").
alias(travel_bottle, "small bottle").
alias(liquid_bag, "clear liquid bag").
alias(liquid_bag, "clear resealable bag").
alias(liquid_bag, "clear bag").
alias(food_bag, "clear food bag").
alias(carry_on_bag, "carry-on bag").
alias(fasten, "close").
alias(easy_screening, "easy to screen").
alias(lid_tight, "tight").
alias(bag_sealed, "sealed").
```

## teach
- To pack liquids for airport security you need shampoo, a travel bottle, a clear liquid bag and a carry-on bag.
- Packing liquids for airport security takes 10 to 15 minutes.
- Step 1 of packing liquids for airport security: pour the shampoo into the travel bottle, no more than 100 millilitres.
- Step 2 of packing liquids for airport security: fasten the lid of the travel bottle so it is tight.
- Step 3 of packing liquids for airport security: put the travel bottle into the clear liquid bag.
- Step 4 of packing liquids for airport security: fasten the liquid bag so it is sealed.
- Step 5 of packing liquids for airport security: put the liquid bag into the carry-on bag so screeners can check it easily.

## checklist
- names what is needed, including shampoo, a travel bottle, a clear liquid bag and a carry-on bag
- gives the steps in order: pour under 100 ml into the travel bottle, fasten the lid, put the bottle in the liquid bag, seal the bag, put the bag in the carry-on
- says the 100 ml limit applies to each container
- gives a total time of about 10 to 15 minutes

## variant
- change: the user has no clear liquid bag, only a clear food bag, so the food bag replaces the liquid bag
- request: My liquid bag broke and I only have a clear food bag. Can I still pack my shampoo for security?
- situation: I have shampoo, a small travel bottle, a clear food bag and a carry-on bag.
- expect: The reply pours the shampoo into the travel bottle, seals the bottle in the clear food bag instead of a liquid bag, and places that in the carry-on bag.

```datalog situation
has(user, shampoo).
has(user, travel_bottle).
has(user, food_bag).
has(user, carry_on_bag).
lacks(user, liquid_bag).
```

```datalog difference
swap(pack_liquids_for_security_s3, liquid_bag, food_bag).
swap(pack_liquids_for_security_s4, liquid_bag, food_bag).
extra_need(food_bag).
no_need(liquid_bag).
```

## variant
- change: the shampoo bottle holds 250 ml, which is over the 100 ml limit, so it cannot go in hand luggage
- request: I want to take my full 250 ml shampoo bottle in my hand luggage. How do I pack it for security?
- situation: I have a 250 ml bottle of shampoo and want to carry it in my hand luggage.
- expect: The reply says the 250 ml bottle is over the 100 ml limit and cannot go in hand luggage, and suggests decanting into a 100 ml bottle or packing it in checked luggage.

```datalog situation
has(user, shampoo).
wants(user, carry_in_hand).
```

```datalog difference
cannot(over_100ml_limit).
```
