---
id: hang_picture
stage: tools
task: hang_picture
---
# Hang a picture

## requests
- I bought a framed print and want to put it up in the hallway. How do I hang it?
- Can you tell me how to hang a picture on the wall so it's straight?
- I've got a small painting to put up above my desk. What's the best way to do it?

## situation
- I have the picture, a hammer, a nail, a pencil and a tape measure.
- I've got about 15 minutes.

## ground truth
```datalog
task(hang_picture).
needs(hang_picture, picture).
needs(hang_picture, hammer).
needs(hang_picture, nail).
needs(hang_picture, pencil).
needs(hang_picture, tape_measure).
takes_time(hang_picture, 10, 15, minute).
step(hang_picture, hang_picture_s1).
action(hang_picture_s1, measure).
object_of(hang_picture_s1, wall).
uses(hang_picture_s1, tape_measure).
takes_time(hang_picture_s1, 3, 5, minute).
step(hang_picture, hang_picture_s2).
action(hang_picture_s2, mark).
object_of(hang_picture_s2, spot).
uses(hang_picture_s2, pencil).
step(hang_picture, hang_picture_s3).
action(hang_picture_s3, hammer).
object_of(hang_picture_s3, nail).
uses(hang_picture_s3, hammer).
takes_time(hang_picture_s3, 1, 2, minute).
step(hang_picture, hang_picture_s4).
action(hang_picture_s4, hang).
object_of(hang_picture_s4, picture).
step(hang_picture, hang_picture_s5).
action(hang_picture_s5, straighten).
object_of(hang_picture_s5, picture).
until(hang_picture_s5, level).
before(hang_picture_s1, hang_picture_s2).
before(hang_picture_s2, hang_picture_s3).
before(hang_picture_s3, hang_picture_s4).
before(hang_picture_s4, hang_picture_s5).
```

## teach
- To hang a picture you need the picture, a hammer, a nail, a pencil and a tape measure.
- Hanging a picture takes 10 to 15 minutes.
- Step 1 of hanging a picture: measure the wall with the tape measure to find the centre at eye level, for 3 to 5 minutes.
- Step 2 of hanging a picture: mark the spot with the pencil.
- Step 3 of hanging a picture: hammer the nail into the mark at a slight downward angle for 1 to 2 minutes.
- Step 4 of hanging a picture: hang the picture on the nail.
- Step 5 of hanging a picture: straighten the picture until it is level.

## checklist
- names what is needed, including a hammer, a nail and the picture
- gives the steps in order: measure, mark, hammer nail, hang, straighten
- gives a total time of about 10 to 15 minutes
- closes politely

## variant
- change: renting, no holes allowed: use adhesive picture-hanging strips instead of a nail
- request: I want to hang a picture but my landlord doesn't allow nails in the walls. What can I do?
- situation: I'm renting so I can't make holes. I have the picture, a pencil, a tape measure and a pack of adhesive hanging strips.
- expect: The reply sticks adhesive strips to the frame and presses it to the wall instead of hammering a nail, and keeps measuring and marking.
