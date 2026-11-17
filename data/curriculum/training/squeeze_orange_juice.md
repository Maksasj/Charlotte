---
id: squeeze_orange_juice
stage: procedures
task: squeeze_orange_juice
---
# Squeeze orange juice

## requests
- I want fresh orange juice with breakfast. How do I squeeze it myself?
- Can you tell me how to make juice from a bag of oranges?
- What's the best way to get the most juice out of oranges?

## situation
- I have four oranges, a knife, a citrus juicer, a sieve and a glass.
- I have 10 minutes.

## ground truth
```datalog
task(squeeze_orange_juice).
needs(squeeze_orange_juice, orange).
needs(squeeze_orange_juice, knife).
needs(squeeze_orange_juice, juicer).
needs(squeeze_orange_juice, sieve).
needs(squeeze_orange_juice, glass).
takes_time(squeeze_orange_juice, 5, 8, minute).
step(squeeze_orange_juice, squeeze_orange_juice_s1).
action(squeeze_orange_juice_s1, roll).
object_of(squeeze_orange_juice_s1, orange).
takes_time(squeeze_orange_juice_s1, 1, 1, minute).
step(squeeze_orange_juice, squeeze_orange_juice_s2).
action(squeeze_orange_juice_s2, cut).
object_of(squeeze_orange_juice_s2, orange).
uses(squeeze_orange_juice_s2, knife).
takes_time(squeeze_orange_juice_s2, 1, 1, minute).
step(squeeze_orange_juice, squeeze_orange_juice_s3).
action(squeeze_orange_juice_s3, squeeze).
object_of(squeeze_orange_juice_s3, orange).
uses(squeeze_orange_juice_s3, juicer).
takes_time(squeeze_orange_juice_s3, 3, 5, minute).
step(squeeze_orange_juice, squeeze_orange_juice_s4).
action(squeeze_orange_juice_s4, pour).
object_of(squeeze_orange_juice_s4, juice).
uses(squeeze_orange_juice_s4, glass).
uses(squeeze_orange_juice_s4, sieve).
before(squeeze_orange_juice_s1, squeeze_orange_juice_s2).
before(squeeze_orange_juice_s2, squeeze_orange_juice_s3).
before(squeeze_orange_juice_s3, squeeze_orange_juice_s4).
```

## teach
- To squeeze orange juice you need oranges, a knife, a citrus juicer, a sieve and a glass.
- Squeezing orange juice takes 5 to 8 minutes.
- Step 1 of squeezing orange juice: roll the oranges firmly on the counter to loosen the juice for 1 minute.
- Step 2 of squeezing orange juice: cut the oranges in half with the knife for 1 minute.
- Step 3 of squeezing orange juice: squeeze each orange half on the juicer for 3 to 5 minutes.
- Step 4 of squeezing orange juice: pour the juice into the glass through the sieve.

## checklist
- names what is needed: oranges, a knife, a citrus juicer, a sieve and a glass
- gives the steps in order: roll orange, cut orange, squeeze orange, pour juice
- gives a total time of about 5 to 8 minutes
- closes politely
