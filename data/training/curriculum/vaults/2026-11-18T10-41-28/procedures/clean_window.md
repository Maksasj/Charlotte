---
id: clean_window
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# clean window

Learned in conversation.

```datalog
task(clean_window). @conf 1 @source ""
needs(clean_window, bucket). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
needs(clean_window, water). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
needs(clean_window, dish_soap). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
needs(clean_window, sponge). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
needs(clean_window, squeegee). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
needs(clean_window, cloth). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
takes_time(clean_window, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
step(clean_window, clean_window_s1). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
action(clean_window_s1, mix). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
object_of(clean_window_s1, dish_soap). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
uses(clean_window_s1, bucket). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
uses(clean_window_s1, water). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
takes_time(clean_window_s1, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
step(clean_window, clean_window_s2). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
action(clean_window_s2, wash). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
object_of(clean_window_s2, window). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
uses(clean_window_s2, sponge). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
takes_time(clean_window_s2, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
before(clean_window_s1, clean_window_s2). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
step(clean_window, clean_window_s3). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
action(clean_window_s3, dry). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
object_of(clean_window_s3, window). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
uses(clean_window_s3, squeegee). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
takes_time(clean_window_s3, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
before(clean_window_s2, clean_window_s3). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
step(clean_window, clean_window_s4). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
action(clean_window_s4, wipe). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
object_of(clean_window_s4, frame). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
uses(clean_window_s4, cloth). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
takes_time(clean_window_s4, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
before(clean_window_s3, clean_window_s4). @conf 1 @source teacher:curriculum @conv s1794998498327-527283-49
```
