---
id: make_quesadilla
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make quesadilla

Learned in conversation.

```datalog
task(make_quesadilla). @conf 1 @source ""
needs(make_quesadilla, tortilla). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
needs(make_quesadilla, cheese). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
needs(make_quesadilla, frying_pan). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
needs(make_quesadilla, spatula). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
needs(make_quesadilla, knife). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
takes_time(make_quesadilla, 8, 12, minute). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
step(make_quesadilla, make_quesadilla_s1). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
action(make_quesadilla_s1, heat). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
object_of(make_quesadilla_s1, frying_pan). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
uses(make_quesadilla_s1, medium_heat). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
takes_time(make_quesadilla_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
step(make_quesadilla, make_quesadilla_s2). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
action(make_quesadilla_s2, sprinkle). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
object_of(make_quesadilla_s2, cheese). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
before(make_quesadilla_s1, make_quesadilla_s2). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
step(make_quesadilla, make_quesadilla_s3). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
action(make_quesadilla_s3, put). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
object_of(make_quesadilla_s3, tortilla). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
uses(make_quesadilla_s3, tortilla). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
takes_time(make_quesadilla_s3, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
before(make_quesadilla_s2, make_quesadilla_s3). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
action(make_quesadilla_s3, fry). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
object_of(make_quesadilla_s3, quesadilla). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
uses(make_quesadilla_s3, frying_pan). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
until(make_quesadilla_s3, golden). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
takes_time(make_quesadilla_s3, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
step(make_quesadilla, make_quesadilla_s4). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
action(make_quesadilla_s4, flip). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
object_of(make_quesadilla_s4, quesadilla). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
uses(make_quesadilla_s4, spatula). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
step(make_quesadilla, make_quesadilla_s5). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
action(make_quesadilla_s5, fry). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
object_of(make_quesadilla_s5, other_side). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
until(make_quesadilla_s5, cheese_melt). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
takes_time(make_quesadilla_s5, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
before(make_quesadilla_s4, make_quesadilla_s5). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
before(make_quesadilla_s3, make_quesadilla_s4). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
action(make_quesadilla_s5, cut). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
object_of(make_quesadilla_s5, quesadilla). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
uses(make_quesadilla_s5, knife). @conf 1 @source teacher:curriculum @conv s1794978763281-85704-71
step(make_quesadilla, make_quesadilla_s0). @conf 1 @source teacher:curriculum @conv s1794978926946-85704-259
action(make_quesadilla_s0, need). @conf 1 @source teacher:curriculum @conv s1794978926946-85704-259
object_of(make_quesadilla_s0, tortilla). @conf 1 @source teacher:curriculum @conv s1794978926946-85704-259
until(make_quesadilla_s1, dry). @conf 1 @source teacher:curriculum @conv s1794978926946-85704-259
uses(make_quesadilla_s1, frying_pan). @conf 1 @source teacher:curriculum @conv s1794978926946-85704-259
before(make_quesadilla_s0, make_quesadilla_s1). @conf 1 @source teacher:curriculum @conv s1794978926946-85704-259
uses(make_quesadilla_s2, tortilla). @conf 1 @source teacher:curriculum @conv s1794978926946-85704-259
takes_time(make_quesadilla_s2, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794978926946-85704-259
needs(make_quesadilla, grated_cheese). @conf 1 @source teacher:curriculum @conv s1794978926946-85704-259
object_of(make_quesadilla_s2, grated_cheese). @conf 1 @source teacher:curriculum @conv s1794978926946-85704-259
until(make_quesadilla_s4, melts). @conf 1 @source teacher:curriculum @conv s1794978926946-85704-259
takes_time(make_quesadilla_s4, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794978926946-85704-259
```
