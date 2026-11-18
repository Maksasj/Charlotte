---
id: rake_leaf
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# rake leaf

Learned in conversation.

```datalog
task(rake_leaf). @conf 1 @source ""
needs(rake_leaf, rake). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
needs(rake_leaf, tarp). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
needs(rake_leaf, garden_waste_bag). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
needs(rake_leaf, glove). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
takes_time(rake_leaf, 35, 50, minute). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
step(rake_leaf, rake_leaf_s1). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
action(rake_leaf_s1, lay). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
object_of(rake_leaf_s1, tarp). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
takes_time(rake_leaf_s1, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
step(rake_leaf, rake_leaf_s2). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
action(rake_leaf_s2, rake). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
object_of(rake_leaf_s2, leaf). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
uses(rake_leaf_s2, tarp). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
takes_time(rake_leaf_s2, 20, 30, minute). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
before(rake_leaf_s1, rake_leaf_s2). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
step(rake_leaf, rake_leaf_s3). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
action(rake_leaf_s3, fill). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
object_of(rake_leaf_s3, garden_bag). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
uses(rake_leaf_s3, tarp). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
uses(rake_leaf_s3, glove). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
takes_time(rake_leaf_s3, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
before(rake_leaf_s2, rake_leaf_s3). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
step(rake_leaf, rake_leaf_s4). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
action(rake_leaf_s4, tie). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
object_of(rake_leaf_s4, garden_bag). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
takes_time(rake_leaf_s4, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
before(rake_leaf_s3, rake_leaf_s4). @conf 1 @source teacher:curriculum @conv s1794981184398-218805-150
```
