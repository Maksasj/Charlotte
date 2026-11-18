---
id: hang_picture__broken_tape_measure
stage: tools
task: hang_picture
schema: v3
tier: S
base: hang_picture
---
# Hang a picture

## requests
- Want to put up a framed print, but my tape measure's spring broke and it won't extend. I've got a long ruler.

## ground truth
```datalog
task(hang_picture).
domain(hang_picture, repair).
goal(hang_picture, hung_picture).
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
role(hang_picture_s1, tape_measure, measure).
purpose(hang_picture_s1, centre_at_eye_level).
takes_time(hang_picture_s1, 3, 5, minute).
step(hang_picture, hang_picture_s2).
action(hang_picture_s2, mark).
object_of(hang_picture_s2, spot).
uses(hang_picture_s2, pencil).
role(hang_picture_s2, pencil, mark).
step(hang_picture, hang_picture_s3).
action(hang_picture_s3, hammer).
object_of(hang_picture_s3, nail).
into(hang_picture_s3, spot).
uses(hang_picture_s3, hammer).
role(hang_picture_s3, hammer, strike).
setting(hang_picture_s3, angle, slight_downward, level).
takes_time(hang_picture_s3, 1, 2, minute).
step(hang_picture, hang_picture_s4).
action(hang_picture_s4, hang).
object_of(hang_picture_s4, picture).
onto(hang_picture_s4, nail).
step(hang_picture, hang_picture_s5).
action(hang_picture_s5, straighten).
object_of(hang_picture_s5, picture).
until(hang_picture_s5, level).
before(hang_picture_s1, hang_picture_s2).
before(hang_picture_s2, hang_picture_s3).
before(hang_picture_s3, hang_picture_s4).
before(hang_picture_s4, hang_picture_s5).
```

## aliases
```datalog
task_alias(hang_picture, "hanging a picture").
task_alias(hang_picture, "hang a picture on the wall").
task_alias(hang_picture, "put up a picture").
alias(picture, "framed print").
alias(picture, "painting").
alias(spot, "mark").
alias(level, "straight").
alias(centre_at_eye_level, "centre at eye level").
alias(slight_downward, "slight downward angle").
alias(adhesive_strip, "adhesive hanging strips").
alias(adhesive_strip, "adhesive picture-hanging strips").
alias(adhesive_strip, "adhesive strips").
```

## teach
- To hang a picture you need the picture, a hammer, a nail, a pencil and a tape measure.
- Hanging a picture takes 10 to 15 minutes.
- Step 1 of hanging a picture: measure the wall with the tape measure to find the centre at eye level, for 3 to 5 minutes.
- Step 2 of hanging a picture: mark the spot with the pencil.
- Step 3 of hanging a picture: hammer the nail into the mark at a slight downward angle for 1 to 2 minutes.
- Step 4 of hanging a picture: hang the picture on the nail.
- Step 5 of hanging a picture: straighten the picture until it is level.

## variant
- change: broken tape measure: measure the wall with a ruler
- request: Want to put up a framed print, but my tape measure's spring broke and it won't extend. I've got a long ruler.
- situation: My tape measure is broken. I have the picture, a hammer, a nail, a pencil and a metre ruler.
- expect: The reply measures the wall with the ruler instead of the tape measure and keeps marking, hammering, hanging and straightening.

```datalog situation
has(user, picture).
has(user, hammer).
has(user, nail).
has(user, pencil).
has(user, ruler).
lacks(user, tape_measure).
```

```datalog difference
swap(hang_picture_s1, tape_measure, ruler).
no_need(tape_measure).
extra_need(ruler).
```
