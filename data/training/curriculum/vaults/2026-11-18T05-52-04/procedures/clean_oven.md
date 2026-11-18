---
id: clean_oven
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# clean oven

Learned in conversation.

```datalog
task(clean_oven). @conf 1 @source ""
needs(clean_oven, baking_soda). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
needs(clean_oven, water). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
needs(clean_oven, bowl). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
needs(clean_oven, white_vinegar). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
needs(clean_oven, spray_bottle). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
needs(clean_oven, sponge). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
needs(clean_oven, dish_soap). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
takes_time(clean_oven, 45, 60, minute). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
step(clean_oven, clean_oven_s1). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
action(clean_oven_s1, take_out). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
object_of(clean_oven_s1, oven_rack). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
takes_time(clean_oven_s1, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
uses(clean_oven_s1, oven). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
step(clean_oven, clean_oven_s2). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
action(clean_oven_s2, mix). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
object_of(clean_oven_s2, baking_soda). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
uses(clean_oven_s2, water). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
uses(clean_oven_s2, bowl). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
takes_time(clean_oven_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
before(clean_oven_s1, clean_oven_s2). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
step(clean_oven, clean_oven_s3). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
action(clean_oven_s3, spread). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
object_of(clean_oven_s3, paste). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
uses(clean_oven_s3, sponge). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
takes_time(clean_oven_s3, 5, 7, minute). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
before(clean_oven_s2, clean_oven_s3). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
step(clean_oven, clean_oven_s4). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
action(clean_oven_s4, leave). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
object_of(clean_oven_s4, paste). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
takes_time(clean_oven_s4, 30, 40, minute). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
before(clean_oven_s3, clean_oven_s4). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
step(clean_oven, clean_oven_s5). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
action(clean_oven_s5, scrub). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
object_of(clean_oven_s5, oven_rack). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
uses(clean_oven_s5, sponge). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
uses(clean_oven_s5, water). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
uses(clean_oven_s5, dish_soap). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
takes_time(clean_oven_s5, 10, 15, minute). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
overlaps(clean_oven_s5, clean_oven_s4). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
step(clean_oven, clean_oven_s6). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
action(clean_oven_s6, spray). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
object_of(clean_oven_s6, paste). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
uses(clean_oven_s6, vinegar). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
uses(clean_oven_s6, spray_bottle). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
action(clean_oven_s6, wipe). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
uses(clean_oven_s6, sponge). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
until(clean_oven_s6, clean). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
takes_time(clean_oven_s6, 5, 10, minute). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
before(clean_oven_s5, clean_oven_s6). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
before(clean_oven_s4, clean_oven_s6). @conf 1 @source teacher:curriculum @conv s1794981130966-218805-24
```
