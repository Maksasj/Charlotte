---
id: make_freshen_carpet
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make freshen carpet

Learned in conversation.

```datalog
task(make_freshen_carpet). @conf 1 @source ""
needs(make_freshen_carpet, baking_soda). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
needs(make_freshen_carpet, sieve). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
needs(make_freshen_carpet, vacuum_cleaner). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
takes_time(make_freshen_carpet, 30, 50, minute). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
step(make_freshen_carpet, make_freshen_carpet_s1). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
action(make_freshen_carpet_s1, vacuum). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
object_of(make_freshen_carpet_s1, carpet). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
uses(make_freshen_carpet_s1, vacuum_cleaner). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
takes_time(make_freshen_carpet_s1, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
step(make_freshen_carpet, make_freshen_carpet_s2). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
action(make_freshen_carpet_s2, sprinkle). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
object_of(make_freshen_carpet_s2, carpet). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
uses(make_freshen_carpet_s2, baking_soda). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
uses(make_freshen_carpet_s2, sieve). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
takes_time(make_freshen_carpet_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
before(make_freshen_carpet_s1, make_freshen_carpet_s2). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
step(make_freshen_carpet, make_freshen_carpet_s3). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
action(make_freshen_carpet_s3, leave). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
object_of(make_freshen_carpet_s3, baking_soda). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
takes_time(make_freshen_carpet_s3, 15, 30, minute). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
before(make_freshen_carpet_s2, make_freshen_carpet_s3). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
step(make_freshen_carpet, make_freshen_carpet_s4). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
action(make_freshen_carpet_s4, vacuum). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
object_of(make_freshen_carpet_s4, baking_soda). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
uses(make_freshen_carpet_s4, vacuum_cleaner). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
takes_time(make_freshen_carpet_s4, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
before(make_freshen_carpet_s3, make_freshen_carpet_s4). @conf 1 @source teacher:curriculum @conv s1794946286104-131028-22
```
