---
id: steam_broccoli
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# steam broccoli

Learned in conversation.

```datalog
task(steam_broccoli). @conf 1 @source ""
needs(steam_broccoli, broccoli). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
needs(steam_broccoli, water). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
needs(steam_broccoli, salt). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
needs(steam_broccoli, knife). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
needs(steam_broccoli, pot). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
needs(steam_broccoli, lid). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
needs(steam_broccoli, steamer_basket). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
takes_time(steam_broccoli, 9, 12, minute). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
step(steam_broccoli, steam_broccoli_s1). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
action(steam_broccoli_s1, boil). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
object_of(steam_broccoli_s1, water). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
uses(steam_broccoli_s1, pot). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
takes_time(steam_broccoli_s1, 4, 5, minute). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
action(steam_broccoli_s1, cut). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
object_of(steam_broccoli_s1, broccoli). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
uses(steam_broccoli_s1, knife). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
takes_time(steam_broccoli_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
overlaps(steam_broccoli_s1, steam_broccoli_s2). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
step(steam_broccoli, steam_broccoli_s3). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
action(steam_broccoli_s3, steam). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
object_of(steam_broccoli_s3, broccoli). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
uses(steam_broccoli_s3, basket). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
uses(steam_broccoli_s3, water). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
uses(steam_broccoli_s3, pot). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
uses(steam_broccoli_s3, lid). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
until(steam_broccoli_s3, bright_green). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
until(steam_broccoli_s3, tender). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
takes_time(steam_broccoli_s3, 4, 6, minute). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
before(steam_broccoli_s1, steam_broccoli_s3). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
before(steam_broccoli_s2, steam_broccoli_s3). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
step(steam_broccoli, steam_broccoli_s4). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
action(steam_broccoli_s4, season). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
object_of(steam_broccoli_s4, broccoli). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
uses(steam_broccoli_s4, salt). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
before(steam_broccoli_s3, steam_broccoli_s4). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
step(steam_broccoli, steam_broccoli_s2). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
action(steam_broccoli_s2, cut). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
object_of(steam_broccoli_s2, broccoli). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
uses(steam_broccoli_s2, knife). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
takes_time(steam_broccoli_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
overlaps(steam_broccoli_s2, steam_broccoli_s1). @conf 1 @source teacher:curriculum @conv s1794981776002-278172-212
```
