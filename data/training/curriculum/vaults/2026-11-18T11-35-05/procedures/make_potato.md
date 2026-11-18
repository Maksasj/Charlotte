---
id: make_potato
status: hypothesis
source: user
uses: 0
successes: 0
---
# make potato

Learned in conversation.

```datalog
task(make_potato). @conf 1 @source ""
step(make_potato, make_potato_s1). @conf 1 @source user @conv s1795001705042-730152-8
action(make_potato_s1, bake). @conf 1 @source user @conv s1795001705042-730152-8
object_of(make_potato_s1, potato). @conf 1 @source user @conv s1795001705042-730152-8
recommended_for(make_potato, dinner). @conf 1 @source user @conv s1795001705042-730152-8
needs(make_potato, potato). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
needs(make_potato, olive_oil). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
needs(make_potato, salt). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
needs(make_potato, fork). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
needs(make_potato, baking_tray). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
needs(make_potato, oven). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
takes_time(make_potato, 60, 70, minute). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
action(make_potato_s1, heat). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
object_of(make_potato_s1, oven). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
takes_time(make_potato_s1, 10, 10, minute). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
step(make_potato, make_potato_s2). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
action(make_potato_s2, wash). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
object_of(make_potato_s2, potato). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
overlaps(make_potato_s2, make_potato_s1). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
step(make_potato, make_potato_s3). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
action(make_potato_s3, prick). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
object_of(make_potato_s3, potato). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
uses(make_potato_s3, fork). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
overlaps(make_potato_s3, make_potato_s1). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
step(make_potato, make_potato_s4). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
action(make_potato_s4, rub). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
object_of(make_potato_s4, potato). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
uses(make_potato_s4, olive_oil). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
uses(make_potato_s4, salt). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
overlaps(make_potato_s4, make_potato_s1). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
step(make_potato, make_potato_s5). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
action(make_potato_s5, bake). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
object_of(make_potato_s5, potato). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
uses(make_potato_s5, baking_tray). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
until(make_potato_s5, soft). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
takes_time(make_potato_s5, 50, 60, minute). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
before(make_potato_s4, make_potato_s5). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
before(make_potato_s1, make_potato_s5). @conf 1 @source teacher:curriculum @conv s1795001712039-730152-51
```

## Taught wording

- make_potato_s1: heat the oven to 200 degrees
- make_potato_s2: wash the potato
- make_potato_s3: prick the potato all over with the fork
- make_potato_s4: rub the potato with olive oil and salt
- make_potato_s5: bake the potato on the baking tray until it is soft inside
