---
id: write_birthday_card
status: hypothesis
source: seed
uses: 0
successes: 0
---
# Write a birthday card

Write a short personal message in a card and get it ready to post.

```datalog
task(write_birthday_card).
needs(write_birthday_card, card).
needs(write_birthday_card, pen).
needs(write_birthday_card, envelope).
needs(write_birthday_card, stamp).
takes_time(write_birthday_card, 10, 20, minute).
step(write_birthday_card, write_birthday_card_s1).
action(write_birthday_card_s1, choose).
object_of(write_birthday_card_s1, card).
takes_time(write_birthday_card_s1, 2, 5, minute).
step(write_birthday_card, write_birthday_card_s2).
action(write_birthday_card_s2, write).
object_of(write_birthday_card_s2, message).
uses(write_birthday_card_s2, pen).
takes_time(write_birthday_card_s2, 5, 10, minute).
step(write_birthday_card, write_birthday_card_s3).
action(write_birthday_card_s3, sign).
object_of(write_birthday_card_s3, card).
uses(write_birthday_card_s3, pen).
step(write_birthday_card, write_birthday_card_s4).
action(write_birthday_card_s4, seal).
object_of(write_birthday_card_s4, envelope).
takes_time(write_birthday_card_s4, 1, 1, minute).
step(write_birthday_card, write_birthday_card_s5).
action(write_birthday_card_s5, add).
object_of(write_birthday_card_s5, stamp).
before(write_birthday_card_s1, write_birthday_card_s2).
before(write_birthday_card_s2, write_birthday_card_s3).
before(write_birthday_card_s3, write_birthday_card_s4).
before(write_birthday_card_s4, write_birthday_card_s5).
```
