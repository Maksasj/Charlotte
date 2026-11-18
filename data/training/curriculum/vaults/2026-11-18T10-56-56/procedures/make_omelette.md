---
id: make_omelette
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make omelette

Learned in conversation.

```datalog
task(make_omelette). @conf 1 @source ""
needs(make_omelette, egg). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
needs(make_omelette, butter). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
needs(make_omelette, salt). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
needs(make_omelette, bowl). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
needs(make_omelette, fork). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
needs(make_omelette, frying_pan). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
takes_time(make_omelette, 6, 10, minute). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
step(make_omelette, make_omelette_s1). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
action(make_omelette_s1, crack). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
object_of(make_omelette_s1, egg). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
uses(make_omelette_s1, bowl). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
step(make_omelette, make_omelette_s2). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
action(make_omelette_s2, beat). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
object_of(make_omelette_s2, egg). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
uses(make_omelette_s2, fork). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
uses(make_omelette_s2, salt). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
takes_time(make_omelette_s2, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
before(make_omelette_s1, make_omelette_s2). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
step(make_omelette, make_omelette_s3). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
action(make_omelette_s3, melt). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
object_of(make_omelette_s3, butter). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
uses(make_omelette_s3, pan). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
takes_time(make_omelette_s3, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
before(make_omelette_s2, make_omelette_s3). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
step(make_omelette, make_omelette_s4). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
action(make_omelette_s4, cook). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
object_of(make_omelette_s4, egg). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
uses(make_omelette_s4, pan). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
until(make_omelette_s4, set). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
takes_time(make_omelette_s4, 3, 4, minute). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
before(make_omelette_s3, make_omelette_s4). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
step(make_omelette, make_omelette_s5). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
action(make_omelette_s5, fold). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
object_of(make_omelette_s5, omelette). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
before(make_omelette_s4, make_omelette_s5). @conf 1 @source teacher:curriculum @conv s1794999417224-600462-21
```
