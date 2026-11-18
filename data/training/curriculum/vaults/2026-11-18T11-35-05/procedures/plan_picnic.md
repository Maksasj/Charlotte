---
id: plan_picnic
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# plan picnic

Learned in conversation.

```datalog
task(plan_picnic). @conf 1 @source ""
needs(plan_picnic, basket). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
needs(plan_picnic, blanket). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
needs(plan_picnic, bread). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
needs(plan_picnic, fruit). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
needs(plan_picnic, drink). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
takes_time(plan_picnic, 45, 60, minute). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
step(plan_picnic, plan_picnic_s1). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
step(plan_picnic, plan_picnic_s2). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
action(plan_picnic_s2, choose). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
object_of(plan_picnic_s2, park). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
takes_time(plan_picnic_s2, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
before(plan_picnic_s1, plan_picnic_s2). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
step(plan_picnic, plan_picnic_s3). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
action(plan_picnic_s3, invite). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
object_of(plan_picnic_s3, friend). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
takes_time(plan_picnic_s3, 10, 10, minute). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
before(plan_picnic_s2, plan_picnic_s3). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
step(plan_picnic, plan_picnic_s4). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
action(plan_picnic_s4, make). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
object_of(plan_picnic_s4, sandwich). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
uses(plan_picnic_s4, bread). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
takes_time(plan_picnic_s4, 15, 20, minute). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
before(plan_picnic_s3, plan_picnic_s4). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
step(plan_picnic, plan_picnic_s5). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
action(plan_picnic_s5, pack). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
object_of(plan_picnic_s5, basket). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
uses(plan_picnic_s5, sandwich). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
uses(plan_picnic_s5, fruit). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
uses(plan_picnic_s5, drink). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
uses(plan_picnic_s5, blanket). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
takes_time(plan_picnic_s5, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
before(plan_picnic_s4, plan_picnic_s5). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
action(plan_picnic_s1, check). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
object_of(plan_picnic_s1, weather_forecast). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
takes_time(plan_picnic_s1, 5, 5, minute). @conf 1 @source teacher:curriculum @conv s1795001705946-730152-20
```

## Taught wording

- plan_picnic_s2: choose a park with some shade
- plan_picnic_s3: invite your friends and tell them the time and place
- plan_picnic_s4: make the sandwiches with the bread
- plan_picnic_s5: pack the basket with the sandwiches, the fruit, the drinks and the blanket
- plan_picnic_s1: check the weather forecast for the day
