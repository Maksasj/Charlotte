---
id: make_hummus
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make hummus

Learned in conversation.

```datalog
task(make_hummus). @conf 1 @source ""
needs(make_hummus, chickpea). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
needs(make_hummus, tahini). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
needs(make_hummus, lemon). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
needs(make_hummus, garlic). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
needs(make_hummus, olive_oil). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
needs(make_hummus, sieve). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
needs(make_hummus, blender). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
takes_time(make_hummus, 8, 12, minute). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
step(make_hummus, make_hummus_s1). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
action(make_hummus_s1, drain). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
object_of(make_hummus_s1, chickpea). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
uses(make_hummus_s1, sieve). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
takes_time(make_hummus_s1, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
step(make_hummus, make_hummus_s2). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
action(make_hummus_s2, rinse). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
object_of(make_hummus_s2, chickpea). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
uses(make_hummus_s2, sieve). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
takes_time(make_hummus_s2, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
before(make_hummus_s1, make_hummus_s2). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
action(make_hummus_s2, squeeze). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
object_of(make_hummus_s2, lemon). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
step(make_hummus, make_hummus_s3). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
action(make_hummus_s3, blend). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
object_of(make_hummus_s3, chickpea). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
uses(make_hummus_s3, tahini). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
uses(make_hummus_s3, garlic). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
uses(make_hummus_s3, lemon). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
until(make_hummus_s3, smooth). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
takes_time(make_hummus_s3, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
before(make_hummus_s2, make_hummus_s3). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
step(make_hummus, make_hummus_s4). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
action(make_hummus_s4, drizzle). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
object_of(make_hummus_s4, olive_oil). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
takes_time(make_hummus_s4, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
before(make_hummus_s3, make_hummus_s4). @conf 1 @source teacher:curriculum @conv s1794978747102-85704-57
```
