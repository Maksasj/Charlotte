---
id: pack_carry_on_bag
stage: social
task: pack_carry_on_bag
schema: v3
tier: T
---
# Pack a carry-on bag

## requests
- I'm flying hand luggage only for a long weekend. How do I fit everything in?
- How do you pack a cabin bag so it gets through security fast?
- Any method for packing a small suitcase for the plane without paying for hold luggage?

## situation
- I have a cabin-size carry-on, a clear plastic toiletry bag and a luggage scale.

## ground truth
```datalog
task(pack_carry_on_bag).
domain(pack_carry_on_bag, travel).
goal(pack_carry_on_bag, bag_packed).
goal(pack_carry_on_bag, fly_hand_luggage_only).
needs(pack_carry_on_bag, carry_on).
needs(pack_carry_on_bag, clear_bag).
needs(pack_carry_on_bag, luggage_scale).
takes_time(pack_carry_on_bag, 30, 45, minute).
step(pack_carry_on_bag, pack_carry_on_bag_s1).
action(pack_carry_on_bag_s1, check).
object_of(pack_carry_on_bag_s1, airline_limit).
step(pack_carry_on_bag, pack_carry_on_bag_s2).
action(pack_carry_on_bag_s2, roll).
object_of(pack_carry_on_bag_s2, clothes).
purpose(pack_carry_on_bag_s2, save_space).
step(pack_carry_on_bag, pack_carry_on_bag_s3).
action(pack_carry_on_bag_s3, put).
object_of(pack_carry_on_bag_s3, liquid).
into(pack_carry_on_bag_s3, clear_bag).
uses(pack_carry_on_bag_s3, clear_bag).
role(pack_carry_on_bag_s3, clear_bag, contain).
amount(pack_carry_on_bag_s3, liquid, 100, millilitre).
step(pack_carry_on_bag, pack_carry_on_bag_s4).
action(pack_carry_on_bag_s4, pack).
object_of(pack_carry_on_bag_s4, clothes).
into(pack_carry_on_bag_s4, carry_on).
uses(pack_carry_on_bag_s4, carry_on).
role(pack_carry_on_bag_s4, carry_on, contain).
step(pack_carry_on_bag, pack_carry_on_bag_s5).
action(pack_carry_on_bag_s5, put).
object_of(pack_carry_on_bag_s5, clear_bag).
into(pack_carry_on_bag_s5, carry_on).
purpose(pack_carry_on_bag_s5, quick_security).
step(pack_carry_on_bag, pack_carry_on_bag_s6).
action(pack_carry_on_bag_s6, weigh).
object_of(pack_carry_on_bag_s6, carry_on).
uses(pack_carry_on_bag_s6, luggage_scale).
role(pack_carry_on_bag_s6, luggage_scale, measure).
check(pack_carry_on_bag_s6, under_limit).
before(pack_carry_on_bag_s1, pack_carry_on_bag_s2).
before(pack_carry_on_bag_s2, pack_carry_on_bag_s3).
before(pack_carry_on_bag_s3, pack_carry_on_bag_s4).
before(pack_carry_on_bag_s4, pack_carry_on_bag_s5).
before(pack_carry_on_bag_s5, pack_carry_on_bag_s6).
```

## aliases
```datalog
task_alias(pack_carry_on_bag, "packing hand luggage").
task_alias(pack_carry_on_bag, "packing a cabin bag").
task_alias(pack_carry_on_bag, "travelling carry-on only").
alias(carry_on, "cabin bag").
alias(carry_on, "hand luggage").
alias(clear_bag, "clear toiletry bag").
alias(liquid, "liquids").
alias(airline_limit, "the airline's size and weight limit").
alias(quick_security, "it comes out quickly at security").
```

## teach
- To pack a carry-on bag you need the carry-on, a clear toiletry bag and a luggage scale.
- Packing a carry-on bag takes 30 to 45 minutes.
- Step 1 of packing a carry-on bag: check the airline's size and weight limit.
- Step 2 of packing a carry-on bag: roll the clothes to save space.
- Step 3 of packing a carry-on bag: put the liquids, each 100 millilitres or less, into the clear bag.
- Step 4 of packing a carry-on bag: pack the clothes into the carry-on.
- Step 5 of packing a carry-on bag: put the clear bag in the carry-on on top so it comes out quickly at security.
- Step 6 of packing a carry-on bag: weigh the carry-on with the luggage scale and check it is under the limit.

## checklist
- names what is needed: a carry-on, a clear bag and a luggage scale
- gives the steps in order: check limits, roll clothes, liquids in clear bag, pack, clear bag on top, weigh
- says liquids must be 100 millilitres or less

## variant
- change: no luggage scale: skip weighing and pack light
- request: I don't own a luggage scale. Is that a problem for my cabin bag?
- situation: I have a carry-on and a clear toiletry bag.
- expect: The reply skips the weighing step and suggests staying well within the limit.

```datalog situation
has(user, carry_on).
has(user, clear_bag).
lacks(user, luggage_scale).
```

```datalog difference
drop(pack_carry_on_bag_s6).
no_need(luggage_scale).
```

## variant
- change: no clear toiletry bag: use a resealable freezer bag
- request: I can't find one of those see-through toiletry bags. What do I put my liquids in?
- situation: I have a carry-on, a luggage scale and a box of resealable freezer bags.
- expect: The reply puts the liquids in a resealable freezer bag instead of the clear toiletry bag.

```datalog situation
has(user, carry_on).
has(user, luggage_scale).
has(user, freezer_bag).
lacks(user, clear_bag).
```

```datalog difference
swap(pack_carry_on_bag_s3, clear_bag, freezer_bag).
change(pack_carry_on_bag_s3, into, clear_bag, freezer_bag).
change(pack_carry_on_bag_s5, object_of, clear_bag, freezer_bag).
extra_need(freezer_bag).
no_need(clear_bag).
```
