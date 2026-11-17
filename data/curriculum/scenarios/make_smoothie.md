---
id: make_smoothie
stage: procedures
task: make_smoothie
---
# Make a smoothie

## requests
- I want a fruit smoothie for breakfast. How do I make one?
- Could you tell me how to blend a banana and strawberry smoothie?
- I just got a blender and want to try a smoothie. What are the steps?

## situation
- I have a banana, some strawberries, yogurt, milk, a blender and a glass.
- I've only got about 10 minutes.

## ground truth
```datalog
task(make_smoothie).
needs(make_smoothie, banana).
needs(make_smoothie, strawberry).
needs(make_smoothie, yogurt).
needs(make_smoothie, milk).
needs(make_smoothie, blender).
needs(make_smoothie, glass).
takes_time(make_smoothie, 5, 8, minute).
step(make_smoothie, make_smoothie_s1).
action(make_smoothie_s1, peel).
object_of(make_smoothie_s1, banana).
step(make_smoothie, make_smoothie_s2).
action(make_smoothie_s2, break).
object_of(make_smoothie_s2, banana).
uses(make_smoothie_s2, blender).
step(make_smoothie, make_smoothie_s3).
action(make_smoothie_s3, add).
object_of(make_smoothie_s3, strawberry).
uses(make_smoothie_s3, yogurt).
uses(make_smoothie_s3, milk).
step(make_smoothie, make_smoothie_s4).
action(make_smoothie_s4, blend).
object_of(make_smoothie_s4, smoothie).
uses(make_smoothie_s4, blender).
until(make_smoothie_s4, smooth).
takes_time(make_smoothie_s4, 1, 1, minute).
step(make_smoothie, make_smoothie_s5).
action(make_smoothie_s5, pour).
object_of(make_smoothie_s5, smoothie).
uses(make_smoothie_s5, glass).
before(make_smoothie_s1, make_smoothie_s2).
before(make_smoothie_s2, make_smoothie_s3).
before(make_smoothie_s3, make_smoothie_s4).
before(make_smoothie_s4, make_smoothie_s5).
```

## teach
- To make a smoothie you need a banana, strawberries, yogurt, milk, a blender and a glass.
- Making a smoothie takes 5 to 8 minutes.
- Step 1 of making a smoothie: peel the banana.
- Step 2 of making a smoothie: break the banana into pieces into the blender.
- Step 3 of making a smoothie: add the strawberries with the yogurt and the milk.
- Step 4 of making a smoothie: blend the smoothie in the blender until it is smooth, for 1 minute.
- Step 5 of making a smoothie: pour the smoothie into the glass.

## checklist
- names what is needed, including banana, strawberries, yogurt, milk and a blender
- gives the steps in order: peel, break banana into blender, add strawberries, yogurt and milk, blend, pour
- gives a total time of about 5 to 8 minutes
- closes politely
