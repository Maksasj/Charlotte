---
id: arrange_pet_sitter
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# arrange pet sitter

Learned in conversation.

```datalog
task(arrange_pet_sitter). @conf 1 @source ""
needs(arrange_pet_sitter, pet_food). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
needs(arrange_pet_sitter, spare_key). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
needs(arrange_pet_sitter, paper). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
needs(arrange_pet_sitter, pen). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
takes_time(arrange_pet_sitter, 65, 90, minute). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
step(arrange_pet_sitter, arrange_pet_sitter_s1). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
action(arrange_pet_sitter_s1, ask). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
object_of(arrange_pet_sitter_s1, friend). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
takes_time(arrange_pet_sitter_s1, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
step(arrange_pet_sitter, arrange_pet_sitter_s2). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
action(arrange_pet_sitter_s2, meet). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
object_of(arrange_pet_sitter_s2, pet_sitter). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
takes_time(arrange_pet_sitter_s2, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
before(arrange_pet_sitter_s1, arrange_pet_sitter_s2). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
step(arrange_pet_sitter, arrange_pet_sitter_s3). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
action(arrange_pet_sitter_s3, write). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
object_of(arrange_pet_sitter_s3, instruction). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
uses(arrange_pet_sitter_s3, paper). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
uses(arrange_pet_sitter_s3, pen). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
takes_time(arrange_pet_sitter_s3, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
before(arrange_pet_sitter_s2, arrange_pet_sitter_s3). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
step(arrange_pet_sitter, arrange_pet_sitter_s4). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
action(arrange_pet_sitter_s4, buy). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
object_of(arrange_pet_sitter_s4, pet_food). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
takes_time(arrange_pet_sitter_s4, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
before(arrange_pet_sitter_s3, arrange_pet_sitter_s4). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
step(arrange_pet_sitter, arrange_pet_sitter_s5). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
action(arrange_pet_sitter_s5, hand). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
object_of(arrange_pet_sitter_s5, spare_key). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
object_of(arrange_pet_sitter_s5, instruction). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
takes_time(arrange_pet_sitter_s5, 5, 5, minute). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
before(arrange_pet_sitter_s4, arrange_pet_sitter_s5). @conf 1 @source teacher:curriculum @conv s1794981719302-278172-75
```
