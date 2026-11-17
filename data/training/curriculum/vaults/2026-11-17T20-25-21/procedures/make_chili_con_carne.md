---
id: make_chili_con_carne
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# make chili con carne

Learned in conversation.

```datalog
task(make_chili_con_carne). @conf 1 @source ""
needs(make_chili_con_carne, minced_beef). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
needs(make_chili_con_carne, onion). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
needs(make_chili_con_carne, kidney_bean). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
needs(make_chili_con_carne, chopped_tomato). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
needs(make_chili_con_carne, chili_powder). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
needs(make_chili_con_carne, oil). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
needs(make_chili_con_carne, knife). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
needs(make_chili_con_carne, pot). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
takes_time(make_chili_con_carne, 45, 60, minute). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
step(make_chili_con_carne, make_chili_con_carne_s1). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
action(make_chili_con_carne_s1, chop). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
object_of(make_chili_con_carne_s1, onion). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
uses(make_chili_con_carne_s1, knife). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
takes_time(make_chili_con_carne_s1, 3, 4, minute). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
step(make_chili_con_carne, make_chili_con_carne_s2). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
action(make_chili_con_carne_s2, fry). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
object_of(make_chili_con_carne_s2, onion). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
uses(make_chili_con_carne_s2, oil). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
uses(make_chili_con_carne_s2, pot). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
until(make_chili_con_carne_s2, soft). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
takes_time(make_chili_con_carne_s2, 5, 6, minute). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
before(make_chili_con_carne_s1, make_chili_con_carne_s2). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
step(make_chili_con_carne, make_chili_con_carne_s3). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
action(make_chili_con_carne_s3, brown). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
object_of(make_chili_con_carne_s3, minced_beef). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
uses(make_chili_con_carne_s3, pot). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
takes_time(make_chili_con_carne_s3, 5, 6, minute). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
step(make_chili_con_carne, make_chili_con_carne_s4). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
action(make_chili_con_carne_s4, brown). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
object_of(make_chili_con_carne_s4, onion). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
uses(make_chili_con_carne_s4, pot). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
overlaps(make_chili_con_carne_s4, make_chili_con_carne_s3). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
before(make_chili_con_carne_s2, make_chili_con_carne_s3). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
action(make_chili_con_carne_s4, add). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
object_of(make_chili_con_carne_s4, chopped_tomato). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
object_of(make_chili_con_carne_s4, kidney_bean). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
object_of(make_chili_con_carne_s4, chili_powder). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
takes_time(make_chili_con_carne_s4, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
before(make_chili_con_carne_s3, make_chili_con_carne_s4). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
step(make_chili_con_carne, make_chili_con_carne_s5). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
action(make_chili_con_carne_s5, simmer). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
object_of(make_chili_con_carne_s5, chili). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
uses(make_chili_con_carne_s5, pot). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
takes_time(make_chili_con_carne_s5, 30, 40, minute). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
before(make_chili_con_carne_s4, make_chili_con_carne_s5). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
before(make_chili_con_carne_s3, make_chili_con_carne_s5). @conf 1 @source teacher:curriculum @conv s1794946293382-131028-35
action(make_chili_con_carne_s3, add). @conf 1 @source teacher:curriculum @conv s1794946556479-131028-211
object_of(make_chili_con_carne_s3, chopped_tomato). @conf 1 @source teacher:curriculum @conv s1794946556479-131028-211
object_of(make_chili_con_carne_s3, tinned_lentil). @conf 1 @source teacher:curriculum @conv s1794946556479-131028-211
object_of(make_chili_con_carne_s3, kidney_bean). @conf 1 @source teacher:curriculum @conv s1794946556479-131028-211
object_of(make_chili_con_carne_s3, chili_powder). @conf 1 @source teacher:curriculum @conv s1794946556479-131028-211
takes_time(make_chili_con_carne_s3, 1, 2, minute). @conf 1 @source teacher:curriculum @conv s1794946556479-131028-211
action(make_chili_con_carne_s4, simmer). @conf 1 @source teacher:curriculum @conv s1794946556479-131028-211
object_of(make_chili_con_carne_s4, chili_con_carne). @conf 1 @source teacher:curriculum @conv s1794946556479-131028-211
uses(make_chili_con_carne_s4, lid). @conf 1 @source teacher:curriculum @conv s1794946556479-131028-211
takes_time(make_chili_con_carne_s4, 20, 20, minute). @conf 1 @source teacher:curriculum @conv s1794946556479-131028-211
```
