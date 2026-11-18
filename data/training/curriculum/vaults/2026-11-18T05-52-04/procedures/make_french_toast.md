---
id: make_french_toast
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make french toast

Learned in conversation.

```datalog
task(make_french_toast). @conf 1 @source ""
needs(make_french_toast, bread). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
needs(make_french_toast, egg). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
needs(make_french_toast, milk). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
needs(make_french_toast, butter). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
needs(make_french_toast, bowl). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
needs(make_french_toast, frying_pan). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
takes_time(make_french_toast, 12, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
step(make_french_toast, make_french_toast_s1). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
action(make_french_toast_s1, whisk). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
object_of(make_french_toast_s1, egg). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
object_of(make_french_toast_s1, milk). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
uses(make_french_toast_s1, bowl). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
takes_time(make_french_toast_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
step(make_french_toast, make_french_toast_s2). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
action(make_french_toast_s2, melt). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
object_of(make_french_toast_s2, butter). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
uses(make_french_toast_s2, frying_pan). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
takes_time(make_french_toast_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
overlaps(make_french_toast_s2, make_french_toast_s1). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
step(make_french_toast, make_french_toast_s3). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
action(make_french_toast_s3, dip). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
object_of(make_french_toast_s3, bread). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
uses(make_french_toast_s3, egg). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
uses(make_french_toast_s3, bowl). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
takes_time(make_french_toast_s3, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
before(make_french_toast_s2, make_french_toast_s3). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
before(make_french_toast_s1, make_french_toast_s3). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
step(make_french_toast, make_french_toast_s4). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
action(make_french_toast_s4, fry). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
object_of(make_french_toast_s4, bread). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
uses(make_french_toast_s4, frying_pan). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
takes_time(make_french_toast_s4, 6, 8, minute). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
until(make_french_toast_s4, golden_brown). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
before(make_french_toast_s3, make_french_toast_s4). @conf 1 @source teacher:curriculum @conv s1794981150456-218805-75
```
