---
id: return_online_order
stage: social
task: return_online_order
schema: v3
tier: T
---
# Return an online order

## requests
- The jacket I ordered doesn't fit. How do I send it back?
- What's the process for returning something I bought online?
- I need a refund on a parcel that arrived broken. What do I do?

## situation
- I have my phone, the original box, parcel tape and a printer.

## ground truth
```datalog
task(return_online_order).
domain(return_online_order, errands).
goal(return_online_order, refund).
needs(return_online_order, phone).
needs(return_online_order, box).
needs(return_online_order, tape).
needs(return_online_order, printer).
needs(return_online_order, return_label).
takes_time(return_online_order, 1, 2, day).
step(return_online_order, return_online_order_s1).
action(return_online_order_s1, request).
object_of(return_online_order_s1, return).
uses(return_online_order_s1, phone).
role(return_online_order_s1, phone, access).
requires_state(return_online_order_s1, within_return_window).
step(return_online_order, return_online_order_s2).
action(return_online_order_s2, pack).
object_of(return_online_order_s2, item).
uses(return_online_order_s2, box).
role(return_online_order_s2, box, contain).
step(return_online_order, return_online_order_s3).
action(return_online_order_s3, seal).
object_of(return_online_order_s3, box).
uses(return_online_order_s3, tape).
role(return_online_order_s3, tape, fasten).
step(return_online_order, return_online_order_s4).
action(return_online_order_s4, print).
object_of(return_online_order_s4, return_label).
uses(return_online_order_s4, printer).
role(return_online_order_s4, printer, write).
step(return_online_order, return_online_order_s5).
action(return_online_order_s5, stick).
object_of(return_online_order_s5, return_label).
onto(return_online_order_s5, box).
step(return_online_order, return_online_order_s6).
action(return_online_order_s6, drop_off).
object_of(return_online_order_s6, box).
into(return_online_order_s6, post_office).
check(return_online_order_s6, receipt_kept).
before(return_online_order_s1, return_online_order_s2).
before(return_online_order_s2, return_online_order_s3).
before(return_online_order_s3, return_online_order_s4).
before(return_online_order_s4, return_online_order_s5).
before(return_online_order_s5, return_online_order_s6).
```

## aliases
```datalog
task_alias(return_online_order, "sending an order back").
task_alias(return_online_order, "returning a parcel").
task_alias(return_online_order, "getting a refund for an online purchase").
alias(item, "the order").
alias(return_label, "returns label").
alias(drop_off, "drop off").
alias(post_office, "the post office or drop-off point").
alias(within_return_window, "inside the return period").
```

## teach
- To return an online order you need a phone, a box, tape, a printer and the return label.
- Returning an online order takes 1 to 2 days.
- Step 1 of returning an online order: request the return on your phone while you are inside the return period.
- Step 2 of returning an online order: pack the item in the box.
- Step 3 of returning an online order: seal the box with tape.
- Step 4 of returning an online order: print the return label on the printer.
- Step 5 of returning an online order: stick the label onto the box.
- Step 6 of returning an online order: drop the box off at the post office and keep the receipt.

## checklist
- names what is needed, including a phone, a box, tape and a printed label
- gives the steps in order: request return, pack, seal, print label, stick on, drop off
- says to keep the receipt

## variant
- change: printer broken: show a QR code on the phone at the drop-off point
- request: My printer's out of ink and won't work. How do I return my parcel without a printed label?
- situation: I have my phone, the box and tape, but my printer is broken.
- expect: The reply skips printing and sticking the label and shows a QR code on the phone at the drop-off point so they print it there.

```datalog situation
has(user, phone).
has(user, box).
has(user, tape).
lacks(user, printer).
```

```datalog difference
drop(return_online_order_s4).
drop(return_online_order_s5).
no_need(printer).
no_need(return_label).
add_step(return_online_order_s9, return_online_order_s3).
step(return_online_order, return_online_order_s9).
action(return_online_order_s9, show).
object_of(return_online_order_s9, qr_code).
uses(return_online_order_s9, phone).
role(return_online_order_s9, phone, access).
before(return_online_order_s3, return_online_order_s9).
before(return_online_order_s9, return_online_order_s6).
```

## variant
- change: no box: send it in a strong plastic mailing bag
- request: I threw away the box my order came in. Can I still send it back?
- situation: I have my phone, a strong plastic mailing bag, tape and a printer.
- expect: The reply packs the item in the mailing bag instead of a box and sticks the label onto the bag.

```datalog situation
has(user, phone).
has(user, bag).
has(user, tape).
has(user, printer).
lacks(user, box).
```

```datalog difference
swap(return_online_order_s2, box, bag).
change(return_online_order_s3, object_of, box, bag).
change(return_online_order_s5, onto, box, bag).
change(return_online_order_s6, object_of, box, bag).
extra_need(bag).
no_need(box).
```
