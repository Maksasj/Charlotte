---
id: make_chili
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make chili

Learned in conversation.

```datalog
task(make_chili). @conf 1 @source ""
needs(make_chili, minced_beef). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
needs(make_chili, onion). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
needs(make_chili, kidney_bean). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
needs(make_chili, chopped_tomato). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
needs(make_chili, chili_powder). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
needs(make_chili, oil). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
needs(make_chili, knife). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
needs(make_chili, pot). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
takes_time(make_chili, 45, 60, minute). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
step(make_chili, make_chili_s1). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
action(make_chili_s1, chop). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
object_of(make_chili_s1, onion). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
uses(make_chili_s1, knife). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
takes_time(make_chili_s1, 3, 4, minute). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
step(make_chili, make_chili_s2). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
action(make_chili_s2, fry). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
object_of(make_chili_s2, onion). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
uses(make_chili_s2, oil). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
uses(make_chili_s2, pot). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
until(make_chili_s2, soft). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
takes_time(make_chili_s2, 5, 6, minute). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
before(make_chili_s1, make_chili_s2). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
step(make_chili, make_chili_s3). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
action(make_chili_s3, brown). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
object_of(make_chili_s3, minced_beef). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
uses(make_chili_s3, onion). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
uses(make_chili_s3, pot). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
takes_time(make_chili_s3, 5, 6, minute). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
before(make_chili_s2, make_chili_s3). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
step(make_chili, make_chili_s4). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
action(make_chili_s4, add). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
object_of(make_chili_s4, chopped_tomato). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
object_of(make_chili_s4, kidney_bean). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
object_of(make_chili_s4, chili_powder). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
takes_time(make_chili_s4, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
before(make_chili_s3, make_chili_s4). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
step(make_chili, make_chili_s5). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
action(make_chili_s5, simmer). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
object_of(make_chili_s5, chili). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
uses(make_chili_s5, pot). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
uses(make_chili_s5, lid). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
takes_time(make_chili_s5, 30, 40, minute). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
before(make_chili_s4, make_chili_s5). @conf 1 @source teacher:curriculum @conv s1794981704242-278172-30
```
