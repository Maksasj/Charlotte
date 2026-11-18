---
id: do_grocery_shopping
stage: social
task: do_grocery_shopping
schema: v3
tier: T
---
# Do the grocery shopping

## requests
- How do I do a big food shop without forgetting stuff or overspending?
- I need to stock up the kitchen this week. What's the best way to go about the shopping?
- Can you give me a routine for the weekly supermarket trip?

## situation
- I have paper, a pen, reusable shopping bags and my bank card.

## ground truth
```datalog
task(do_grocery_shopping).
domain(do_grocery_shopping, errands).
goal(do_grocery_shopping, groceries).
goal(do_grocery_shopping, stocked_kitchen).
needs(do_grocery_shopping, paper).
needs(do_grocery_shopping, pen).
needs(do_grocery_shopping, bag).
needs(do_grocery_shopping, payment_card).
takes_time(do_grocery_shopping, 45, 60, minute).
step(do_grocery_shopping, do_grocery_shopping_s1).
action(do_grocery_shopping_s1, check).
object_of(do_grocery_shopping_s1, cupboard).
purpose(do_grocery_shopping_s1, know_what_is_missing).
step(do_grocery_shopping, do_grocery_shopping_s2).
action(do_grocery_shopping_s2, write).
object_of(do_grocery_shopping_s2, shopping_list).
uses(do_grocery_shopping_s2, paper).
role(do_grocery_shopping_s2, paper, record).
uses(do_grocery_shopping_s2, pen).
role(do_grocery_shopping_s2, pen, write).
step(do_grocery_shopping, do_grocery_shopping_s3).
action(do_grocery_shopping_s3, take).
object_of(do_grocery_shopping_s3, bag).
step(do_grocery_shopping, do_grocery_shopping_s4).
action(do_grocery_shopping_s4, buy).
object_of(do_grocery_shopping_s4, item).
condition(do_grocery_shopping_s4, on_list).
step(do_grocery_shopping, do_grocery_shopping_s5).
action(do_grocery_shopping_s5, pay).
object_of(do_grocery_shopping_s5, item).
uses(do_grocery_shopping_s5, payment_card).
role(do_grocery_shopping_s5, payment_card, pay).
step(do_grocery_shopping, do_grocery_shopping_s6).
action(do_grocery_shopping_s6, pack).
object_of(do_grocery_shopping_s6, bag).
uses(do_grocery_shopping_s6, bag).
role(do_grocery_shopping_s6, bag, contain).
avoid(do_grocery_shopping_s6, put, heavy_item).
before(do_grocery_shopping_s1, do_grocery_shopping_s2).
before(do_grocery_shopping_s2, do_grocery_shopping_s3).
before(do_grocery_shopping_s3, do_grocery_shopping_s4).
before(do_grocery_shopping_s4, do_grocery_shopping_s5).
before(do_grocery_shopping_s5, do_grocery_shopping_s6).
```

## aliases
```datalog
task_alias(do_grocery_shopping, "a food shop").
task_alias(do_grocery_shopping, "the weekly shop").
task_alias(do_grocery_shopping, "going to the supermarket").
alias(item, "groceries").
alias(bag, "shopping bags").
alias(cupboard, "cupboards and fridge").
alias(heavy_item, "heavy things on top").
```

## teach
- To do the grocery shopping you need paper, a pen, shopping bags and a payment card.
- Doing the grocery shopping takes 45 to 60 minutes.
- Step 1 of doing the grocery shopping: check the cupboards and fridge to know what is missing.
- Step 2 of doing the grocery shopping: write a shopping list with the pen on paper.
- Step 3 of doing the grocery shopping: take your bags.
- Step 4 of doing the grocery shopping: buy only the items on the list.
- Step 5 of doing the grocery shopping: pay with your card.
- Step 6 of doing the grocery shopping: pack the bags; do not put heavy things on top.

## checklist
- names what is needed: paper, a pen, bags and a card
- gives the steps in order: check cupboards, write list, take bags, buy, pay, pack

## variant
- change: only 20 minutes: skip the cupboard check and buy essentials only
- request: I've got twenty minutes before the shop shuts. How do I grab what I need?
- situation: I have paper, a pen, bags and my card, and 20 minutes.
- expect: The reply skips checking the cupboards, jots a short list of essentials and buys only those.

```datalog situation
has(user, paper).
has(user, pen).
has(user, bag).
has(user, payment_card).
time_limit(user, 20, minute).
```

```datalog difference
drop(do_grocery_shopping_s1).
change(do_grocery_shopping_s4, object_of, item, essential_item).
change(do_grocery_shopping, takes_time, 45_60, 20_20).
```

## variant
- change: plastic-free: choose loose produce and refill containers
- request: I'm trying to cut out plastic. How can I do my food shopping without bringing home a pile of packaging?
- situation: I have paper, a pen, cloth bags, glass jars and my card. I want to avoid plastic.
- expect: The reply brings glass jars as well as the cloth bags and buys loose produce and refills instead of packed items.

```datalog situation
has(user, paper).
has(user, pen).
has(user, bag).
has(user, jar).
has(user, payment_card).
excludes(user, plastic).
```

```datalog difference
change(do_grocery_shopping_s4, object_of, item, loose_item).
extra_need(jar).
add_step(do_grocery_shopping_s9, do_grocery_shopping_s3).
step(do_grocery_shopping, do_grocery_shopping_s9).
action(do_grocery_shopping_s9, take).
object_of(do_grocery_shopping_s9, jar).
purpose(do_grocery_shopping_s9, refill).
before(do_grocery_shopping_s3, do_grocery_shopping_s9).
before(do_grocery_shopping_s9, do_grocery_shopping_s4).
```
