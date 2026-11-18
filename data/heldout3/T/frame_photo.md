---
id: frame_photo
stage: tools
task: frame_photo
schema: v3
tier: T
---
# Put a photo in a frame

## requests
- I just bought a frame for my wedding photo. How do I put the picture in neatly?
- What's the right way to frame a print so there's no dust under the glass?
- How do I get a photo into a picture frame with a mount?

## situation
- I have a frame with glass, glass cleaner, a lint-free cloth, a card mount and the photo.

## ground truth
```datalog
task(frame_photo).
domain(frame_photo, crafts).
goal(frame_photo, framed_photo).
needs(frame_photo, frame).
needs(frame_photo, glass_cleaner).
needs(frame_photo, cloth).
needs(frame_photo, mount).
needs(frame_photo, photo).
takes_time(frame_photo, 10, 15, minute).
step(frame_photo, frame_photo_s1).
action(frame_photo_s1, open).
object_of(frame_photo_s1, frame).
with_part(frame_photo_s1, back).
step(frame_photo, frame_photo_s2).
action(frame_photo_s2, clean).
object_of(frame_photo_s2, glass).
uses(frame_photo_s2, glass_cleaner).
role(frame_photo_s2, glass_cleaner, clean).
uses(frame_photo_s2, cloth).
role(frame_photo_s2, cloth, wipe).
check(frame_photo_s2, no_dust).
step(frame_photo, frame_photo_s3).
action(frame_photo_s3, place).
object_of(frame_photo_s3, mount).
onto(frame_photo_s3, glass).
step(frame_photo, frame_photo_s4).
action(frame_photo_s4, place).
object_of(frame_photo_s4, photo).
onto(frame_photo_s4, mount).
check(frame_photo_s4, centred).
step(frame_photo, frame_photo_s5).
action(frame_photo_s5, close).
object_of(frame_photo_s5, frame).
with_part(frame_photo_s5, back).
before(frame_photo_s1, frame_photo_s2).
before(frame_photo_s2, frame_photo_s3).
before(frame_photo_s3, frame_photo_s4).
before(frame_photo_s4, frame_photo_s5).
```

## aliases
```datalog
task_alias(frame_photo, "framing a photo").
task_alias(frame_photo, "putting a picture in a frame").
task_alias(frame_photo, "framing a print").
alias(frame, "picture frame").
alias(mount, "card mount").
alias(cloth, "lint-free cloth").
alias(back, "the back").
alias(no_dust, "no dust or smears").
alias(centred, "it is straight and centred").
```

## teach
- To put a photo in a frame you need the frame, glass cleaner, a cloth, a mount and the photo.
- Putting a photo in a frame takes 10 to 15 minutes.
- Step 1 of framing a photo: open the back of the frame.
- Step 2 of framing a photo: clean the glass with glass cleaner and the cloth until there is no dust or smears.
- Step 3 of framing a photo: place the mount onto the glass.
- Step 4 of framing a photo: place the photo onto the mount and check it is straight and centred.
- Step 5 of framing a photo: close the back of the frame.

## checklist
- names what is needed: the frame, glass cleaner, a cloth, a mount and the photo
- gives the steps in order: open back, clean glass, mount, photo, close

## variant
- change: no glass cleaner: clean the glass with rubbing alcohol
- request: I don't have any glass spray. What can I clean the frame glass with?
- situation: I have the frame, rubbing alcohol, a cloth, a mount and the photo.
- expect: The reply cleans the glass with a little rubbing alcohol on the cloth instead of glass cleaner.

```datalog situation
has(user, frame).
has(user, rubbing_alcohol).
has(user, cloth).
has(user, mount).
has(user, photo).
lacks(user, glass_cleaner).
```

```datalog difference
swap(frame_photo_s2, glass_cleaner, rubbing_alcohol).
extra_need(rubbing_alcohol).
no_need(glass_cleaner).
```

## variant
- change: no mount: put the photo straight onto the glass
- request: My frame didn't come with a mount. Can I just put the photo in?
- situation: I have the frame, glass cleaner, a cloth and the photo.
- expect: The reply skips the mount and places the photo straight onto the glass.

```datalog situation
has(user, frame).
has(user, glass_cleaner).
has(user, cloth).
has(user, photo).
lacks(user, mount).
```

```datalog difference
drop(frame_photo_s3).
no_need(mount).
change(frame_photo_s4, onto, mount, glass).
```
