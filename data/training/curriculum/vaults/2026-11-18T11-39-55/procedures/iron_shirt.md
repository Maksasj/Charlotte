---
id: iron_shirt
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# iron shirt

Learned in conversation.

```datalog
task(iron_shirt). @conf 1 @source ""
needs(iron_shirt, shirt). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
needs(iron_shirt, iron). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
needs(iron_shirt, ironing_board). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
needs(iron_shirt, water). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
needs(iron_shirt, hanger). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
takes_time(iron_shirt, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
step(iron_shirt, iron_shirt_s1). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
action(iron_shirt_s1, fill). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
object_of(iron_shirt_s1, iron). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
uses(iron_shirt_s1, water). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
step(iron_shirt, iron_shirt_s2). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
action(iron_shirt_s2, heat). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
object_of(iron_shirt_s2, iron). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
takes_time(iron_shirt_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
before(iron_shirt_s1, iron_shirt_s2). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
step(iron_shirt, iron_shirt_s3). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
action(iron_shirt_s3, iron). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
object_of(iron_shirt_s3, collar). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
uses(iron_shirt_s3, ironing_board). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
takes_time(iron_shirt_s3, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
before(iron_shirt_s2, iron_shirt_s3). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
step(iron_shirt, iron_shirt_s4). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
action(iron_shirt_s4, iron). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
object_of(iron_shirt_s4, sleeve). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
uses(iron_shirt_s4, ironing_board). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
takes_time(iron_shirt_s4, 3, 4, minute). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
before(iron_shirt_s3, iron_shirt_s4). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
step(iron_shirt, iron_shirt_s5). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
action(iron_shirt_s5, iron). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
object_of(iron_shirt_s5, shirt). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
uses(iron_shirt_s5, ironing_board). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
takes_time(iron_shirt_s5, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
before(iron_shirt_s4, iron_shirt_s5). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
step(iron_shirt, iron_shirt_s6). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
action(iron_shirt_s6, hang). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
object_of(iron_shirt_s6, shirt). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
uses(iron_shirt_s6, hanger). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
before(iron_shirt_s5, iron_shirt_s6). @conf 1 @source teacher:curriculum @conv s1795001996467-744073-24
```

## Taught wording

- task: iron a shirt
- iron_shirt_s1: fill the iron with water
- iron_shirt_s2: heat the iron on the cotton setting
- iron_shirt_s3: iron the collar on the ironing board
- iron_shirt_s4: iron each sleeve on the ironing board
- iron_shirt_s5: iron the front and back of the shirt on the ironing board
- iron_shirt_s6: hang the shirt on the hanger
