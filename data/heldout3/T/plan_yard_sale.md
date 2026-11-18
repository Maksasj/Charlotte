---
id: plan_yard_sale
stage: social
task: plan_yard_sale
schema: v3
tier: T
---
# Plan a yard sale

## requests
- I've got loads of stuff to get rid of. How do I run a garage sale?
- How should I organise a yard sale outside my house?
- What steps would I take to sell old furniture and toys from my driveway one weekend?

## situation
- I have a marker, price stickers, card for a sign, a folding table, a cash box with change and a phone.

## ground truth
```datalog
task(plan_yard_sale).
domain(plan_yard_sale, planning).
goal(plan_yard_sale, yard_sale).
goal(plan_yard_sale, clear_clutter).
needs(plan_yard_sale, marker).
needs(plan_yard_sale, price_sticker).
needs(plan_yard_sale, sign).
needs(plan_yard_sale, table).
needs(plan_yard_sale, cash_box).
needs(plan_yard_sale, phone).
takes_time(plan_yard_sale, 1, 2, week).
step(plan_yard_sale, plan_yard_sale_s1).
action(plan_yard_sale_s1, sort).
object_of(plan_yard_sale_s1, item).
purpose(plan_yard_sale_s1, decide_what_to_sell).
step(plan_yard_sale, plan_yard_sale_s2).
action(plan_yard_sale_s2, price).
object_of(plan_yard_sale_s2, item).
uses(plan_yard_sale_s2, price_sticker).
role(plan_yard_sale_s2, price_sticker, mark).
uses(plan_yard_sale_s2, marker).
role(plan_yard_sale_s2, marker, write).
step(plan_yard_sale, plan_yard_sale_s3).
action(plan_yard_sale_s3, make).
object_of(plan_yard_sale_s3, sign).
uses(plan_yard_sale_s3, marker).
role(plan_yard_sale_s3, marker, write).
step(plan_yard_sale, plan_yard_sale_s4).
action(plan_yard_sale_s4, advertise).
object_of(plan_yard_sale_s4, sale).
uses(plan_yard_sale_s4, phone).
role(plan_yard_sale_s4, phone, inform).
step(plan_yard_sale, plan_yard_sale_s5).
action(plan_yard_sale_s5, set_up).
object_of(plan_yard_sale_s5, table).
purpose(plan_yard_sale_s5, items_visible).
step(plan_yard_sale, plan_yard_sale_s6).
action(plan_yard_sale_s6, keep).
object_of(plan_yard_sale_s6, cash).
uses(plan_yard_sale_s6, cash_box).
role(plan_yard_sale_s6, cash_box, contain).
requires_state(plan_yard_sale_s6, change_ready).
before(plan_yard_sale_s1, plan_yard_sale_s2).
before(plan_yard_sale_s2, plan_yard_sale_s3).
before(plan_yard_sale_s3, plan_yard_sale_s4).
before(plan_yard_sale_s4, plan_yard_sale_s5).
before(plan_yard_sale_s5, plan_yard_sale_s6).
```

## aliases
```datalog
task_alias(plan_yard_sale, "a garage sale").
task_alias(plan_yard_sale, "a yard sale").
task_alias(plan_yard_sale, "selling things from the driveway").
alias(item, "things to sell").
alias(sign, "signs").
alias(cash_box, "cash tin").
alias(set_up, "set out").
alias(table, "folding table").
```

## teach
- To plan a yard sale you need a marker, price stickers, a sign, a table, a cash box and a phone.
- Planning a yard sale takes 1 to 2 weeks.
- Step 1 of planning a yard sale: sort your items and decide what to sell.
- Step 2 of planning a yard sale: price each item with a sticker and the marker.
- Step 3 of planning a yard sale: make a sign with the marker.
- Step 4 of planning a yard sale: advertise the sale on your phone.
- Step 5 of planning a yard sale: set up the table so the items are easy to see.
- Step 6 of planning a yard sale: keep the cash in the cash box, with change ready.

## checklist
- names what is needed, including stickers, a marker, a sign, a table and a cash box
- gives the steps in order: sort, price, sign, advertise, set up, keep cash
- says to have change ready

## variant
- change: no cash box: keep the money in a biscuit tin
- request: I don't have a cash box for my garage sale. What could I keep the money in?
- situation: I have a marker, stickers, a sign, a table, a phone and an old biscuit tin.
- expect: The reply keeps the money and change in the biscuit tin instead of a cash box.

```datalog situation
has(user, marker).
has(user, price_sticker).
has(user, sign).
has(user, table).
has(user, phone).
has(user, biscuit_tin).
lacks(user, cash_box).
```

```datalog difference
swap(plan_yard_sale_s6, cash_box, biscuit_tin).
extra_need(biscuit_tin).
no_need(cash_box).
```

## variant
- change: wants to take cards: set up a card reader on the phone
- request: Nobody carries cash these days. Can I take card payments at my yard sale?
- situation: I have everything for the sale plus a card reader that pairs with my phone.
- expect: The reply adds setting up the card reader with the phone before the sale opens.

```datalog situation
has(user, marker).
has(user, price_sticker).
has(user, sign).
has(user, table).
has(user, cash_box).
has(user, phone).
has(user, card_reader).
wants(user, card_payment).
```

```datalog difference
extra_need(card_reader).
add_step(plan_yard_sale_s9, plan_yard_sale_s5).
step(plan_yard_sale, plan_yard_sale_s9).
action(plan_yard_sale_s9, set_up).
object_of(plan_yard_sale_s9, card_reader).
uses(plan_yard_sale_s9, phone).
role(plan_yard_sale_s9, phone, access).
before(plan_yard_sale_s5, plan_yard_sale_s9).
before(plan_yard_sale_s9, plan_yard_sale_s6).
```
