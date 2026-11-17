---
id: make_popcorn
stage: procedures
task: make_popcorn
---
# Make popcorn

## requests
- We're watching a film tonight. How do I make popcorn on the stove?
- Can you explain how to pop corn kernels in a pot without burning them?
- I bought a bag of popcorn kernels. What do I do with them?

## situation
- I have popcorn kernels, oil, salt, a large pot with a lid and a bowl.
- The film starts in 15 minutes.

## ground truth
```datalog
task(make_popcorn).
needs(make_popcorn, popcorn_kernel).
needs(make_popcorn, oil).
needs(make_popcorn, salt).
needs(make_popcorn, pot).
needs(make_popcorn, bowl).
takes_time(make_popcorn, 6, 8, minute).
step(make_popcorn, make_popcorn_s1).
action(make_popcorn_s1, heat).
object_of(make_popcorn_s1, oil).
uses(make_popcorn_s1, pot).
takes_time(make_popcorn_s1, 2, 3, minute).
step(make_popcorn, make_popcorn_s2).
action(make_popcorn_s2, add).
object_of(make_popcorn_s2, popcorn_kernel).
uses(make_popcorn_s2, pot).
step(make_popcorn, make_popcorn_s3).
action(make_popcorn_s3, shake).
object_of(make_popcorn_s3, pot).
until(make_popcorn_s3, slow).
takes_time(make_popcorn_s3, 3, 4, minute).
step(make_popcorn, make_popcorn_s4).
action(make_popcorn_s4, pour).
object_of(make_popcorn_s4, popcorn).
uses(make_popcorn_s4, bowl).
uses(make_popcorn_s4, salt).
before(make_popcorn_s1, make_popcorn_s2).
before(make_popcorn_s2, make_popcorn_s3).
before(make_popcorn_s3, make_popcorn_s4).
```

## teach
- To make popcorn you need popcorn kernels, oil, salt, a large pot with a lid and a bowl.
- Making popcorn takes 6 to 8 minutes.
- Step 1 of making popcorn: heat the oil in the pot over medium-high heat for 2 to 3 minutes.
- Step 2 of making popcorn: add the popcorn kernels in one layer and put the lid on the pot.
- Step 3 of making popcorn: shake the pot gently until the popping slows to a few seconds apart, for 3 to 4 minutes.
- Step 4 of making popcorn: pour the popcorn into the bowl and sprinkle it with salt.

## checklist
- names what is needed: popcorn kernels, oil, salt, a large pot with a lid and a bowl
- gives the steps in order: heat oil, add popcorn kernel, shake pot, pour popcorn
- gives a total time of about 6 to 8 minutes
- closes politely
