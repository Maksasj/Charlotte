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
needs(cook_rice, rice). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
needs(cook_rice, water). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
needs(cook_rice, pot). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
needs(cook_rice, sieve). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
takes_time(cook_rice, 25, 30, minute). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
step(cook_rice, cook_rice_s1). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
step(cook_rice, cook_rice_s2). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
action(cook_rice_s2, rinse). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
object_of(cook_rice_s2, rice). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
uses(cook_rice_s2, sieve). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
takes_time(cook_rice_s2, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
overlaps(cook_rice_s2, cook_rice_s1). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
step(cook_rice, cook_rice_s3). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
action(cook_rice_s3, add). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
object_of(cook_rice_s3, rice). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
uses(cook_rice_s3, water). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
before(cook_rice_s2, cook_rice_s3). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
step(cook_rice, cook_rice_s4). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
action(cook_rice_s4, simmer). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
object_of(cook_rice_s4, rice). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
uses(cook_rice_s4, lid). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
takes_time(cook_rice_s4, 15, 18, minute). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
before(cook_rice_s3, cook_rice_s4). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
step(cook_rice, cook_rice_s5). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
action(cook_rice_s5, rest). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
object_of(cook_rice_s5, rice). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
takes_time(cook_rice_s5, 5, 6, minute). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
before(cook_rice_s4, cook_rice_s5). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
action(cook_rice_s1, boil). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
object_of(cook_rice_s1, water). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
uses(cook_rice_s1, pot). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
takes_time(cook_rice_s1, 5, 6, minute). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
until(cook_rice_s2, clear). @conf 1 @source teacher:curriculum @conv s1795001311155-711708-31
```

## Taught wording

- cook_rice_s3: add the rice to the boiling water
- cook_rice_s4: simmer the rice with the lid on
- cook_rice_s5: rest the rice off the heat
- cook_rice_s1: boil the water in the pot
- cook_rice_s2: rinse the rice in the sieve until the water runs clear
