---
id: make_yogurt_parfait
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make yogurt parfait

Learned in conversation.

```datalog
task(make_yogurt_parfait). @conf 1 @source ""
needs(make_yogurt_parfait, yogurt). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
needs(make_yogurt_parfait, granola). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
needs(make_yogurt_parfait, berry). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
needs(make_yogurt_parfait, honey). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
needs(make_yogurt_parfait, glass). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
takes_time(make_yogurt_parfait, 5, 7, minute). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
step(make_yogurt_parfait, make_yogurt_parfait_s1). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
action(make_yogurt_parfait_s1, rinse). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
object_of(make_yogurt_parfait_s1, berry). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
step(make_yogurt_parfait, make_yogurt_parfait_s2). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
action(make_yogurt_parfait_s2, pat_dry). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
object_of(make_yogurt_parfait_s2, berry). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
takes_time(make_yogurt_parfait_s2, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
before(make_yogurt_parfait_s1, make_yogurt_parfait_s2). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
action(make_yogurt_parfait_s2, spoon). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
object_of(make_yogurt_parfait_s2, yogurt). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
uses(make_yogurt_parfait_s2, glass). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
takes_time(make_yogurt_parfait_s2, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
step(make_yogurt_parfait, make_yogurt_parfait_s3). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
action(make_yogurt_parfait_s3, add). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
object_of(make_yogurt_parfait_s3, granola). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
object_of(make_yogurt_parfait_s3, berry). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
before(make_yogurt_parfait_s2, make_yogurt_parfait_s3). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
takes_time(make_yogurt_parfait_s3, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
step(make_yogurt_parfait, make_yogurt_parfait_s4). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
action(make_yogurt_parfait_s4, drizzle). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
object_of(make_yogurt_parfait_s4, honey). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
takes_time(make_yogurt_parfait_s4, 1, 1, minute). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
before(make_yogurt_parfait_s3, make_yogurt_parfait_s4). @conf 1 @source teacher:curriculum @conv s1794981139259-218805-51
takes_time(make_yogurt_parfait_s1, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794981253328-218805-309
```
