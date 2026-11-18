---
id: get_currency
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# get currency

Learned in conversation.

```datalog
task(get_currency). @conf 1 @source ""
needs(get_currency, phone). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
needs(get_currency, payment_card). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
needs(get_currency, passport). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
needs(get_currency, money_belt). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
takes_time(get_currency, 35, 50, minute). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
step(get_currency, get_currency_s1). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
action(get_currency_s1, compare). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
object_of(get_currency_s1, exchange_rate). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
uses(get_currency_s1, phone). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
takes_time(get_currency_s1, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
step(get_currency, get_currency_s2). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
action(get_currency_s2, decide). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
object_of(get_currency_s2, cash). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
takes_time(get_currency_s2, 5, 5, minute). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
before(get_currency_s1, get_currency_s2). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
step(get_currency, get_currency_s3). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
action(get_currency_s3, order). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
object_of(get_currency_s3, currency). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
uses(get_currency_s3, payment_card). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
takes_time(get_currency_s3, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
before(get_currency_s2, get_currency_s3). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
step(get_currency, get_currency_s4). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
action(get_currency_s4, collect). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
object_of(get_currency_s4, currency). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
uses(get_currency_s4, passport). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
takes_time(get_currency_s4, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
before(get_currency_s3, get_currency_s4). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
step(get_currency, get_currency_s5). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
action(get_currency_s5, split). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
object_of(get_currency_s5, cash). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
uses(get_currency_s5, wallet). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
uses(get_currency_s5, money_belt). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
takes_time(get_currency_s5, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
before(get_currency_s4, get_currency_s5). @conf 1 @source teacher:curriculum @conv s1794981142687-218805-59
```
