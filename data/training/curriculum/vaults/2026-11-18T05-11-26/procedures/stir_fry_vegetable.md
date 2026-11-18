---
id: stir_fry_vegetable
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# stir fry vegetable

Learned in conversation.

```datalog
task(stir_fry_vegetable). @conf 1 @source ""
takes_time(stir_fry_vegetable, 14, 19, minute). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
step(stir_fry_vegetable, stir_fry_vegetable_s1). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
action(stir_fry_vegetable_s1, chop). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
object_of(stir_fry_vegetable_s1, vegetable). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
step(stir_fry_vegetable, stir_fry_vegetable_s2). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
action(stir_fry_vegetable_s2, slice). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
object_of(stir_fry_vegetable_s2, garlic). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
uses(stir_fry_vegetable_s2, knife). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
takes_time(stir_fry_vegetable_s2, 8, 10, minute). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
before(stir_fry_vegetable_s1, stir_fry_vegetable_s2). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
action(stir_fry_vegetable_s2, heat). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
object_of(stir_fry_vegetable_s2, oil). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
uses(stir_fry_vegetable_s2, wok). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
until(stir_fry_vegetable_s2, hot). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
takes_time(stir_fry_vegetable_s2, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
step(stir_fry_vegetable, stir_fry_vegetable_s3). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
action(stir_fry_vegetable_s3, fry). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
object_of(stir_fry_vegetable_s3, garlic). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
uses(stir_fry_vegetable_s3, wok). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
until(stir_fry_vegetable_s3, fragrant). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
before(stir_fry_vegetable_s2, stir_fry_vegetable_s3). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
step(stir_fry_vegetable, stir_fry_vegetable_s4). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
action(stir_fry_vegetable_s4, fry). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
object_of(stir_fry_vegetable_s4, vegetable). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
uses(stir_fry_vegetable_s4, wok). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
takes_time(stir_fry_vegetable_s4, 4, 6, minute). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
until(stir_fry_vegetable_s4, tender). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
before(stir_fry_vegetable_s3, stir_fry_vegetable_s4). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
step(stir_fry_vegetable, stir_fry_vegetable_s5). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
action(stir_fry_vegetable_s5, add). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
object_of(stir_fry_vegetable_s5, soy_sauce). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
step(stir_fry_vegetable, stir_fry_vegetable_s6). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
action(stir_fry_vegetable_s6, toss). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
object_of(stir_fry_vegetable_s6, vegetable). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
takes_time(stir_fry_vegetable_s6, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
before(stir_fry_vegetable_s5, stir_fry_vegetable_s6). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
before(stir_fry_vegetable_s4, stir_fry_vegetable_s5). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
needs(stir_fry_vegetable, vegetable). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
needs(stir_fry_vegetable, garlic). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
needs(stir_fry_vegetable, oil). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
needs(stir_fry_vegetable, soy_sauce). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
needs(stir_fry_vegetable, knife). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
needs(stir_fry_vegetable, wok). @conf 1 @source teacher:curriculum @conv s1794978824062-85704-122
```
