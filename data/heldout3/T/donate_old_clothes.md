---
id: donate_old_clothes
stage: social
task: donate_old_clothes
schema: v3
tier: T
---
# Donate old clothes

## requests
- My wardrobe is bursting. How do I give my old clothes to charity?
- What's the right way to donate clothes I don't wear any more?
- I want to pass on a pile of outgrown kids' clothes. How?

## situation
- I have a washing machine, some big bags and my phone.

## ground truth
```datalog
task(donate_old_clothes).
domain(donate_old_clothes, errands).
goal(donate_old_clothes, clothes_donated).
goal(donate_old_clothes, clear_clutter).
needs(donate_old_clothes, washing_machine).
needs(donate_old_clothes, bag).
needs(donate_old_clothes, phone).
takes_time(donate_old_clothes, 2, 4, hour).
step(donate_old_clothes, donate_old_clothes_s1).
action(donate_old_clothes_s1, sort).
object_of(donate_old_clothes_s1, clothes).
purpose(donate_old_clothes_s1, keep_only_wearable).
step(donate_old_clothes, donate_old_clothes_s2).
action(donate_old_clothes_s2, wash).
object_of(donate_old_clothes_s2, clothes).
uses(donate_old_clothes_s2, washing_machine).
role(donate_old_clothes_s2, washing_machine, contain).
step(donate_old_clothes, donate_old_clothes_s3).
action(donate_old_clothes_s3, fold).
object_of(donate_old_clothes_s3, clothes).
step(donate_old_clothes, donate_old_clothes_s4).
action(donate_old_clothes_s4, bag).
object_of(donate_old_clothes_s4, clothes).
uses(donate_old_clothes_s4, bag).
role(donate_old_clothes_s4, bag, contain).
step(donate_old_clothes, donate_old_clothes_s5).
action(donate_old_clothes_s5, find).
object_of(donate_old_clothes_s5, charity_shop).
uses(donate_old_clothes_s5, phone).
role(donate_old_clothes_s5, phone, navigate).
overlaps(donate_old_clothes_s5, donate_old_clothes_s4).
step(donate_old_clothes, donate_old_clothes_s6).
action(donate_old_clothes_s6, drop_off).
object_of(donate_old_clothes_s6, bag).
into(donate_old_clothes_s6, charity_shop).
before(donate_old_clothes_s1, donate_old_clothes_s2).
before(donate_old_clothes_s2, donate_old_clothes_s3).
before(donate_old_clothes_s3, donate_old_clothes_s4).
before(donate_old_clothes_s1, donate_old_clothes_s5).
before(donate_old_clothes_s4, donate_old_clothes_s6).
before(donate_old_clothes_s5, donate_old_clothes_s6).
```

## aliases
```datalog
task_alias(donate_old_clothes, "donating clothes").
task_alias(donate_old_clothes, "giving clothes to charity").
task_alias(donate_old_clothes, "clearing out old clothes for charity").
alias(clothes, "old clothes").
alias(charity_shop, "charity shop").
alias(drop_off, "drop off").
alias(keep_only_wearable, "only things in good condition").
```

## teach
- To donate old clothes you need a washing machine, bags and a phone.
- Donating old clothes takes 2 to 4 hours.
- Step 1 of donating old clothes: sort the clothes and keep only things in good condition.
- Step 2 of donating old clothes: wash the clothes in the washing machine.
- Step 3 of donating old clothes: fold the clothes.
- Step 4 of donating old clothes: put the clothes in bags.
- While step 4 runs, step 5 of donating old clothes: find a charity shop near you on your phone.
- Step 6 of donating old clothes: drop the bags off at the charity shop.

## checklist
- names what is needed: a washing machine, bags and a phone
- gives the steps in order: sort, wash, fold, bag, find a shop, drop off
- says to give only clothes in good condition

## variant
- change: washing machine broken: donate only what is already clean
- request: My washing machine has packed in, but I want to drop off clothes at the charity shop this weekend. What do I do?
- situation: I have bags and a phone; the washing machine is broken.
- expect: The reply skips washing and donates only the clothes that are already clean.

```datalog situation
has(user, bag).
has(user, phone).
lacks(user, washing_machine).
```

```datalog difference
drop(donate_old_clothes_s2).
change(donate_old_clothes_s1, purpose, keep_only_wearable, keep_only_clean).
no_need(washing_machine).
```

## variant
- change: six bags: book a free charity collection instead of carrying them
- request: I've ended up with six huge bags of clothes and no car. How do I get them to a charity?
- situation: I have six bags of clothes, a washing machine and a phone, and no car.
- expect: The reply books a charity collection from home by phone instead of dropping the bags off.

```datalog situation
has(user, washing_machine).
has(user, bag).
has(user, phone).
count(user, bag, 6).
```

```datalog difference
drop(donate_old_clothes_s6).
add_step(donate_old_clothes_s9, donate_old_clothes_s5).
step(donate_old_clothes, donate_old_clothes_s9).
action(donate_old_clothes_s9, book).
object_of(donate_old_clothes_s9, collection).
uses(donate_old_clothes_s9, phone).
role(donate_old_clothes_s9, phone, access).
before(donate_old_clothes_s5, donate_old_clothes_s9).
```
