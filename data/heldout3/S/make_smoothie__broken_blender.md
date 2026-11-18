---
id: make_smoothie__broken_blender
stage: procedures
task: make_smoothie
schema: v3
tier: S
base: make_smoothie
---
# Make a smoothie

## requests
- My blender's motor just burnt out. Is there any way to make my banana and strawberry drink?

## ground truth
```datalog
task(make_smoothie).
domain(make_smoothie, cooking).
goal(make_smoothie, smoothie).
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
role(make_smoothie_s2, blender, contain).
achieves(make_smoothie_s2, in_pieces).
step(make_smoothie, make_smoothie_s3).
action(make_smoothie_s3, add).
object_of(make_smoothie_s3, strawberry).
into(make_smoothie_s3, blender).
uses(make_smoothie_s3, yogurt).
role(make_smoothie_s3, yogurt, thicken).
uses(make_smoothie_s3, milk).
role(make_smoothie_s3, milk, moisten).
step(make_smoothie, make_smoothie_s4).
action(make_smoothie_s4, blend).
object_of(make_smoothie_s4, smoothie).
uses(make_smoothie_s4, blender).
role(make_smoothie_s4, blender, blend).
until(make_smoothie_s4, smooth).
takes_time(make_smoothie_s4, 1, 1, minute).
step(make_smoothie, make_smoothie_s5).
action(make_smoothie_s5, pour).
object_of(make_smoothie_s5, smoothie).
uses(make_smoothie_s5, glass).
role(make_smoothie_s5, glass, contain).
before(make_smoothie_s1, make_smoothie_s2).
before(make_smoothie_s2, make_smoothie_s3).
before(make_smoothie_s3, make_smoothie_s4).
before(make_smoothie_s4, make_smoothie_s5).
```

## aliases
```datalog
task_alias(make_smoothie, "making a smoothie").
task_alias(make_smoothie, "fruit smoothie").
task_alias(make_smoothie, "banana and strawberry smoothie").
alias(strawberry, "strawberries").
alias(strawberry, "some strawberries").
alias(yogurt, "the yogurt").
alias(in_pieces, "into pieces").
alias(break, "break into pieces").
```

## teach
- To make a smoothie you need a banana, strawberries, yogurt, milk, a blender and a glass.
- Making a smoothie takes 5 to 8 minutes.
- Step 1 of making a smoothie: peel the banana.
- Step 2 of making a smoothie: break the banana into pieces into the blender.
- Step 3 of making a smoothie: add the strawberries with the yogurt and the milk.
- Step 4 of making a smoothie: blend the smoothie in the blender until it is smooth, for 1 minute.
- Step 5 of making a smoothie: pour the smoothie into the glass.

## variant
- change: broken blender and nothing else to blend with: the smoothie cannot be made
- request: My blender's motor just burnt out. Is there any way to make my banana and strawberry drink?
- situation: The blender is broken and I have no other blender or food processor. I have a banana, strawberries, yogurt, milk and a glass.
- expect: The reply says a smooth drink cannot be made without a blender.

```datalog situation
has(user, banana).
has(user, strawberry).
has(user, yogurt).
has(user, milk).
has(user, glass).
lacks(user, blender).
```

```datalog difference
cannot(no_blender).
```
