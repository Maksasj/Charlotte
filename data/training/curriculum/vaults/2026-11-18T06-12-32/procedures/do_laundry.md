---
id: do_laundry
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# do laundry

Learned in conversation.

```datalog
task(do_laundry). @conf 1 @source ""
needs(do_laundry, laundry). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
needs(do_laundry, washing_machine). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
needs(do_laundry, detergent). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
needs(do_laundry, drying_rack). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
takes_time(do_laundry, 75, 115, minute). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
step(do_laundry, do_laundry_s1). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
action(do_laundry_s1, sort). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
object_of(do_laundry_s1, laundry). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
takes_time(do_laundry_s1, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
step(do_laundry, do_laundry_s2). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
action(do_laundry_s2, load). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
object_of(do_laundry_s2, laundry). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
uses(do_laundry_s2, washing_machine). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
takes_time(do_laundry_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
before(do_laundry_s1, do_laundry_s2). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
step(do_laundry, do_laundry_s3). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
action(do_laundry_s3, add). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
object_of(do_laundry_s3, detergent). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
uses(do_laundry_s3, drawer). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
takes_time(do_laundry_s3, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
before(do_laundry_s2, do_laundry_s3). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
step(do_laundry, do_laundry_s4). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
action(do_laundry_s4, run). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
uses(do_laundry_s4, washing_machine). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
takes_time(do_laundry_s4, 60, 90, minute). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
before(do_laundry_s3, do_laundry_s4). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
step(do_laundry, do_laundry_s5). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
action(do_laundry_s5, hang). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
object_of(do_laundry_s5, laundry). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
uses(do_laundry_s5, drying_rack). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
takes_time(do_laundry_s5, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
before(do_laundry_s4, do_laundry_s5). @conf 1 @source teacher:curriculum @conv s1794982646253-336211-477
```
