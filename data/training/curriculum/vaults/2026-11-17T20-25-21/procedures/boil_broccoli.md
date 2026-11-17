---
id: boil_broccoli
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# boil broccoli

Learned in conversation.

```datalog
task(boil_broccoli). @conf 1 @source ""
step(boil_broccoli, boil_broccoli_s2). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
action(boil_broccoli_s2, cut). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
object_of(boil_broccoli_s2, broccoli). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
uses(boil_broccoli_s2, knife). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
takes_time(boil_broccoli_s2, 2, 3, minute). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
step(boil_broccoli, boil_broccoli_s3). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
action(boil_broccoli_s3, boil). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
object_of(boil_broccoli_s3, floret). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
uses(boil_broccoli_s3, water). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
takes_time(boil_broccoli_s3, 3, 4, minute). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
step(boil_broccoli, boil_broccoli_s4). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
action(boil_broccoli_s4, drain). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
object_of(boil_broccoli_s4, floret). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
before(boil_broccoli_s3, boil_broccoli_s4). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
before(boil_broccoli_s2, boil_broccoli_s3). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
action(boil_broccoli_s4, season). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
object_of(boil_broccoli_s4, broccoli). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
uses(boil_broccoli_s4, salt). @conf 1 @source teacher:curriculum @conv s1794946777606-131028-433
```
