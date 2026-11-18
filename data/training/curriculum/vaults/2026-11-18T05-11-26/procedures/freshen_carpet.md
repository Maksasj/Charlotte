---
id: freshen_carpet
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# freshen carpet

Learned in conversation.

```datalog
task(freshen_carpet). @conf 1 @source ""
needs(freshen_carpet, baking_soda). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
needs(freshen_carpet, sieve). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
needs(freshen_carpet, vacuum_cleaner). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
takes_time(freshen_carpet, 30, 50, minute). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
step(freshen_carpet, freshen_carpet_s1). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
action(freshen_carpet_s1, vacuum). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
object_of(freshen_carpet_s1, carpet). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
uses(freshen_carpet_s1, vacuum_cleaner). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
takes_time(freshen_carpet_s1, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
step(freshen_carpet, freshen_carpet_s2). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
action(freshen_carpet_s2, sprinkle). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
object_of(freshen_carpet_s2, baking_soda). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
uses(freshen_carpet_s2, sieve). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
takes_time(freshen_carpet_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
before(freshen_carpet_s1, freshen_carpet_s2). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
step(freshen_carpet, freshen_carpet_s3). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
action(freshen_carpet_s3, leave). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
object_of(freshen_carpet_s3, baking_soda). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
takes_time(freshen_carpet_s3, 15, 30, minute). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
before(freshen_carpet_s2, freshen_carpet_s3). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
step(freshen_carpet, freshen_carpet_s4). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
action(freshen_carpet_s4, vacuum). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
object_of(freshen_carpet_s4, baking_soda). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
uses(freshen_carpet_s4, vacuum_cleaner). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
takes_time(freshen_carpet_s4, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
before(freshen_carpet_s3, freshen_carpet_s4). @conf 1 @source teacher:curriculum @conv s1794978697849-85704-24
```
