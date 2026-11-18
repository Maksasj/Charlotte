---
id: post_parcel
stage: social
task: post_parcel
schema: v3
tier: T
---
# Post a parcel

## requests
- How do I send a birthday present to my nephew abroad by post?
- What do I need to do to mail a parcel?
- I've got a package to send across the country. Walk me through posting it.

## situation
- I have a box, parcel tape, a marker and my bank card.

## ground truth
```datalog
task(post_parcel).
domain(post_parcel, errands).
goal(post_parcel, parcel_sent).
needs(post_parcel, box).
needs(post_parcel, tape).
needs(post_parcel, marker).
needs(post_parcel, payment_card).
takes_time(post_parcel, 30, 45, minute).
step(post_parcel, post_parcel_s1).
action(post_parcel_s1, pack).
object_of(post_parcel_s1, item).
uses(post_parcel_s1, box).
role(post_parcel_s1, box, contain).
step(post_parcel, post_parcel_s2).
action(post_parcel_s2, seal).
object_of(post_parcel_s2, box).
uses(post_parcel_s2, tape).
role(post_parcel_s2, tape, fasten).
step(post_parcel, post_parcel_s3).
action(post_parcel_s3, write).
object_of(post_parcel_s3, address).
onto(post_parcel_s3, box).
uses(post_parcel_s3, marker).
role(post_parcel_s3, marker, write).
step(post_parcel, post_parcel_s4).
action(post_parcel_s4, weigh).
object_of(post_parcel_s4, box).
into(post_parcel_s4, post_office).
step(post_parcel, post_parcel_s5).
action(post_parcel_s5, pay).
object_of(post_parcel_s5, postage).
uses(post_parcel_s5, payment_card).
role(post_parcel_s5, payment_card, pay).
step(post_parcel, post_parcel_s6).
action(post_parcel_s6, keep).
object_of(post_parcel_s6, receipt).
purpose(post_parcel_s6, tracking).
before(post_parcel_s1, post_parcel_s2).
before(post_parcel_s2, post_parcel_s3).
before(post_parcel_s3, post_parcel_s4).
before(post_parcel_s4, post_parcel_s5).
before(post_parcel_s5, post_parcel_s6).
```

## aliases
```datalog
task_alias(post_parcel, "posting a parcel").
task_alias(post_parcel, "mailing a package").
task_alias(post_parcel, "sending something by post").
alias(item, "the present").
alias(address, "the address").
alias(payment_card, "bank card").
alias(postage, "postage").
alias(tracking, "you can track it").
```

## teach
- To post a parcel you need a box, tape, a marker and a payment card.
- Posting a parcel takes 30 to 45 minutes.
- Step 1 of posting a parcel: pack the item in the box.
- Step 2 of posting a parcel: seal the box with tape.
- Step 3 of posting a parcel: write the address on the box with the marker.
- Step 4 of posting a parcel: weigh the box at the post office.
- Step 5 of posting a parcel: pay the postage with your card.
- Step 6 of posting a parcel: keep the receipt so you can track it.

## checklist
- names what is needed: a box, tape, a marker and a card
- gives the steps in order: pack, seal, address, weigh, pay, keep receipt

## variant
- change: wants it insured: pay for tracked and signed-for postage
- request: The thing I'm posting is quite valuable. How do I make sure it arrives?
- situation: I have a box, tape, a marker and my card; the item is worth a lot.
- expect: The reply pays for tracked, signed-for postage with insurance instead of standard postage and keeps the receipt.

```datalog situation
has(user, box).
has(user, tape).
has(user, marker).
has(user, payment_card).
wants(user, insured).
```

```datalog difference
change(post_parcel_s5, object_of, postage, tracked_postage).
```

## variant
- change: no marker: write the address on a sticky label with a ballpoint pen
- request: I can't find a marker pen to write the address on my parcel. What else can I do?
- situation: I have a box, tape, a sheet of sticky labels and a ballpoint pen, and my card.
- expect: The reply writes the address on a sticky label with the ballpoint and sticks it on the box instead of writing on the box with a marker.

```datalog situation
has(user, box).
has(user, tape).
has(user, sticky_label).
has(user, payment_card).
lacks(user, marker).
```

```datalog difference
swap(post_parcel_s3, marker, sticky_label).
extra_need(sticky_label).
no_need(marker).
```
