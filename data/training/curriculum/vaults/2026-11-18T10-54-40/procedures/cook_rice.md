---
id: cook_rice
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# cook rice

Learned in conversation.

```datalog
task(cook_rice). @conf 1 @source ""
needs(cook_rice, rice). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
needs(cook_rice, water). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
needs(cook_rice, pot). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
needs(cook_rice, sieve). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
takes_time(cook_rice, 25, 30, minute). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
step(cook_rice, cook_rice_s1). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
step(cook_rice, cook_rice_s2). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
action(cook_rice_s2, rinse). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
object_of(cook_rice_s2, rice). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
uses(cook_rice_s2, sieve). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
takes_time(cook_rice_s2, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
overlaps(cook_rice_s2, cook_rice_s1). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
step(cook_rice, cook_rice_s3). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
action(cook_rice_s3, add). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
object_of(cook_rice_s3, rice). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
uses(cook_rice_s3, water). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
before(cook_rice_s2, cook_rice_s3). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
step(cook_rice, cook_rice_s4). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
action(cook_rice_s4, simmer). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
object_of(cook_rice_s4, rice). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
uses(cook_rice_s4, lid). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
takes_time(cook_rice_s4, 15, 18, minute). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
before(cook_rice_s3, cook_rice_s4). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
step(cook_rice, cook_rice_s5). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
action(cook_rice_s5, rest). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
object_of(cook_rice_s5, rice). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
takes_time(cook_rice_s5, 5, 6, minute). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
before(cook_rice_s4, cook_rice_s5). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
action(cook_rice_s1, boil). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
object_of(cook_rice_s1, water). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
uses(cook_rice_s1, pot). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
takes_time(cook_rice_s1, 5, 6, minute). @conf 1 @source teacher:curriculum @conv s1794999281714-590706-24
```
