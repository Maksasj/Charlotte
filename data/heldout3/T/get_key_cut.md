---
id: get_key_cut
stage: social
task: get_key_cut
schema: v3
tier: T
---
# Get a spare key cut

## requests
- How do I get a copy of my front door key made?
- My flatmate needs a spare key. Where do I start?
- I keep locking myself out. How do I get a duplicate made?

## situation
- I have my front door key and my bank card. There is a key cutting shop in town.

## ground truth
```datalog
task(get_key_cut).
domain(get_key_cut, errands).
goal(get_key_cut, spare_key).
needs(get_key_cut, key).
needs(get_key_cut, payment_card).
takes_time(get_key_cut, 15, 30, minute).
step(get_key_cut, get_key_cut_s1).
action(get_key_cut_s1, take).
object_of(get_key_cut_s1, key).
into(get_key_cut_s1, key_shop).
step(get_key_cut, get_key_cut_s2).
action(get_key_cut_s2, ask).
object_of(get_key_cut_s2, copy).
amount(get_key_cut_s2, copy, 1, piece).
step(get_key_cut, get_key_cut_s3).
action(get_key_cut_s3, pay).
object_of(get_key_cut_s3, copy).
uses(get_key_cut_s3, payment_card).
role(get_key_cut_s3, payment_card, pay).
step(get_key_cut, get_key_cut_s4).
action(get_key_cut_s4, test).
object_of(get_key_cut_s4, copy).
check(get_key_cut_s4, turns_smoothly).
step(get_key_cut, get_key_cut_s5).
action(get_key_cut_s5, return).
object_of(get_key_cut_s5, copy).
into(get_key_cut_s5, key_shop).
condition(get_key_cut_s5, copy_sticks).
optional(get_key_cut_s5).
before(get_key_cut_s1, get_key_cut_s2).
before(get_key_cut_s2, get_key_cut_s3).
before(get_key_cut_s3, get_key_cut_s4).
before(get_key_cut_s4, get_key_cut_s5).
```

## aliases
```datalog
task_alias(get_key_cut, "getting a key cut").
task_alias(get_key_cut, "a spare key").
task_alias(get_key_cut, "duplicating a key").
alias(key, "front door key").
alias(copy, "spare key").
alias(key_shop, "key cutting shop").
alias(turns_smoothly, "it turns smoothly in the lock").
alias(copy_sticks, "it sticks").
```

## teach
- To get a spare key cut you need the original key and a payment card.
- Getting a spare key cut takes 15 to 30 minutes.
- Step 1 of getting a key cut: take the key to a key cutting shop.
- Step 2 of getting a key cut: ask for 1 copy.
- Step 3 of getting a key cut: pay for the copy with your card.
- Step 4 of getting a key cut: test the copy at home and check it turns smoothly in the lock.
- Step 5 of getting a key cut: if the copy sticks, return it to the shop to be adjusted.

## checklist
- names what is needed: the key and a card
- gives the steps in order: take key, ask for copy, pay, test, return if it sticks

## variant
- change: three copies for the family
- request: Everyone in the house needs their own key. How do I get three copies made?
- situation: I have the key and my card; three people need copies.
- expect: The reply asks for three copies and tests each one in the lock.

```datalog situation
has(user, key).
has(user, payment_card).
count(user, copy, 3).
```

```datalog difference
change(get_key_cut_s2, amount, copy_1_piece, copy_3_piece).
change(get_key_cut_s4, repeat, 1, 3).
```

## variant
- change: restricted security key without its card: a shop cannot copy it
- request: My key says "do not duplicate" and I don't have the security card that came with the lock. Can I still get it copied?
- situation: I have the key and my bank card, but not the security card for the lock.
- expect: The reply says a restricted key cannot be copied without the security card, and suggests contacting the landlord or lock maker.

```datalog situation
has(user, key).
has(user, payment_card).
lacks(user, security_card).
```

```datalog difference
cannot(restricted_key).
```
