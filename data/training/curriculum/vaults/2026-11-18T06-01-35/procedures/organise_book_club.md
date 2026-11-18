---
id: organise_book_club
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# organise book club

Learned in conversation.

```datalog
task(organise_book_club). @conf 1 @source ""
needs(organise_book_club, book). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
needs(organise_book_club, notebook). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
needs(organise_book_club, snack). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
needs(organise_book_club, drink). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
takes_time(organise_book_club, 35, 50, minute). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
step(organise_book_club, organise_book_club_s1). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
action(organise_book_club_s1, choose). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
object_of(organise_book_club_s1, book). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
uses(organise_book_club_s1, group). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
takes_time(organise_book_club_s1, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
step(organise_book_club, organise_book_club_s2). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
action(organise_book_club_s2, set). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
object_of(organise_book_club_s2, date). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
object_of(organise_book_club_s2, place). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
takes_time(organise_book_club_s2, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
before(organise_book_club_s1, organise_book_club_s2). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
step(organise_book_club, organise_book_club_s3). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
action(organise_book_club_s3, send). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
object_of(organise_book_club_s3, reminder). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
takes_time(organise_book_club_s3, 5, 5, minute). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
before(organise_book_club_s2, organise_book_club_s3). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
step(organise_book_club, organise_book_club_s4). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
action(organise_book_club_s4, write). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
object_of(organise_book_club_s4, discussion_question). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
takes_time(organise_book_club_s4, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
uses(organise_book_club_s4, notebook). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
before(organise_book_club_s3, organise_book_club_s4). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
step(organise_book_club, organise_book_club_s5). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
action(organise_book_club_s5, prepare). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
object_of(organise_book_club_s5, snack). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
object_of(organise_book_club_s5, drink). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
takes_time(organise_book_club_s5, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
before(organise_book_club_s4, organise_book_club_s5). @conf 1 @source teacher:curriculum @conv s1794981754097-278172-158
```
