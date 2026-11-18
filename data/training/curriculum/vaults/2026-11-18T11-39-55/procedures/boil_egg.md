---
id: boil_egg
status: hypothesis
source: teacher:curriculum
uses: 0
successes: 0
---
# boil egg

Learned in conversation.

```datalog
task(boil_egg). @conf 1 @source ""
needs(boil_egg, egg). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
needs(boil_egg, water). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
needs(boil_egg, pot). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
needs(boil_egg, bowl). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
takes_time(boil_egg, 12, 16, minute). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
step(boil_egg, boil_egg_s1). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
action(boil_egg_s1, boil). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
object_of(boil_egg_s1, water). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
uses(boil_egg_s1, pot). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
takes_time(boil_egg_s1, 3, 5, minute). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
step(boil_egg, boil_egg_s2). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
action(boil_egg_s2, lower). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
object_of(boil_egg_s2, egg). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
uses(boil_egg_s2, spoon). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
before(boil_egg_s1, boil_egg_s2). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
step(boil_egg, boil_egg_s3). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
action(boil_egg_s3, boil). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
object_of(boil_egg_s3, egg). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
takes_time(boil_egg_s3, 7, 9, minute). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
before(boil_egg_s2, boil_egg_s3). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
step(boil_egg, boil_egg_s4). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
action(boil_egg_s4, cool). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
object_of(boil_egg_s4, egg). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
uses(boil_egg_s4, bowl). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
uses(boil_egg_s4, water). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
takes_time(boil_egg_s4, 2, 2, minute). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
before(boil_egg_s3, boil_egg_s4). @conf 1 @source teacher:curriculum @conv s1795001999539-744073-37
```

## Taught wording

- task: boil an egg
- boil_egg_s1: boil the water in the pot
- boil_egg_s2: lower the egg into the water with a spoon
- boil_egg_s3: boil the egg
- boil_egg_s4: cool the egg in a bowl of cold water
