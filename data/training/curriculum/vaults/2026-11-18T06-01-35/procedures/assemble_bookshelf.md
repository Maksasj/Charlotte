---
id: assemble_bookshelf
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# assemble bookshelf

Learned in conversation.

```datalog
task(assemble_bookshelf). @conf 1 @source ""
needs(assemble_bookshelf, allen_key). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
needs(assemble_bookshelf, screwdriver). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
needs(assemble_bookshelf, hammer). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
needs(assemble_bookshelf, wall_bracket). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
takes_time(assemble_bookshelf, 45, 70, minute). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
step(assemble_bookshelf, assemble_bookshelf_s1). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
action(assemble_bookshelf_s1, unpack). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
object_of(assemble_bookshelf_s1, part). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
action(assemble_bookshelf_s1, check). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
takes_time(assemble_bookshelf_s1, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
step(assemble_bookshelf, assemble_bookshelf_s2). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
action(assemble_bookshelf_s2, attach). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
object_of(assemble_bookshelf_s2, side_panel). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
uses(assemble_bookshelf_s2, screw). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
uses(assemble_bookshelf_s2, screwdriver). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
takes_time(assemble_bookshelf_s2, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
before(assemble_bookshelf_s1, assemble_bookshelf_s2). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
step(assemble_bookshelf, assemble_bookshelf_s4). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
action(assemble_bookshelf_s4, nail). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
object_of(assemble_bookshelf_s4, back_panel). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
uses(assemble_bookshelf_s4, hammer). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
takes_time(assemble_bookshelf_s4, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
before(assemble_bookshelf_s2, assemble_bookshelf_s4). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
step(assemble_bookshelf, assemble_bookshelf_s5). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
action(assemble_bookshelf_s5, screw). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
object_of(assemble_bookshelf_s5, bookshelf). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
uses(assemble_bookshelf_s5, wall_bracket). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
uses(assemble_bookshelf_s5, screwdriver). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
takes_time(assemble_bookshelf_s5, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
before(assemble_bookshelf_s4, assemble_bookshelf_s5). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
uses(assemble_bookshelf_s1, part_list). @conf 1 @source teacher:curriculum @conv s1794981706675-278172-41
```
