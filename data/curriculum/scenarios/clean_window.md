---
id: clean_window
stage: household
task: clean_window
---
# Clean a window

## requests
- My windows are covered in streaks and dust. How do I get them properly clean?
- What's the best way to wash a window without leaving smears?
- I want to clean the big window in my living room. Where do I start?

## situation
- I have a bucket, warm water, dish soap, a sponge, a squeegee and a dry cloth.
- I'd like it done in about 15 minutes.

## ground truth
```datalog
task(clean_window).
needs(clean_window, bucket).
needs(clean_window, water).
needs(clean_window, dish_soap).
needs(clean_window, sponge).
needs(clean_window, squeegee).
needs(clean_window, cloth).
takes_time(clean_window, 10, 15, minute).
step(clean_window, clean_window_s1).
action(clean_window_s1, mix).
object_of(clean_window_s1, dish_soap).
uses(clean_window_s1, water).
uses(clean_window_s1, bucket).
takes_time(clean_window_s1, 1, 2, minute).
step(clean_window, clean_window_s2).
action(clean_window_s2, wash).
object_of(clean_window_s2, window).
uses(clean_window_s2, sponge).
takes_time(clean_window_s2, 3, 5, minute).
step(clean_window, clean_window_s3).
action(clean_window_s3, dry).
object_of(clean_window_s3, window).
uses(clean_window_s3, squeegee).
takes_time(clean_window_s3, 2, 3, minute).
step(clean_window, clean_window_s4).
action(clean_window_s4, wipe).
object_of(clean_window_s4, frame).
uses(clean_window_s4, cloth).
takes_time(clean_window_s4, 2, 3, minute).
before(clean_window_s1, clean_window_s2).
before(clean_window_s2, clean_window_s3).
before(clean_window_s3, clean_window_s4).
```

## teach
- To clean a window you need a bucket, water, dish soap, a sponge, a squeegee and a cloth.
- Cleaning a window takes 10 to 15 minutes.
- Step 1 of cleaning a window: mix a little dish soap into warm water in the bucket for 1 to 2 minutes.
- Step 2 of cleaning a window: wash the window with the soapy sponge for 3 to 5 minutes.
- Step 3 of cleaning a window: dry the window with the squeegee from top to bottom for 2 to 3 minutes.
- Step 4 of cleaning a window: wipe the frame and edges with the dry cloth for 2 to 3 minutes.

## checklist
- names what is needed, including soapy water, a sponge and a squeegee
- gives the steps in order: mix soapy water, wash, squeegee dry, wipe frame
- gives a total time of about 10 to 15 minutes
- closes politely

## variant
- change: no squeegee: dry the glass with crumpled newspaper or a microfibre cloth
- request: I want to clean my windows but I don't have one of those rubber squeegee things. Will it still work?
- situation: I have a bucket, warm water, dish soap, a sponge and some old newspapers, but no squeegee.
- expect: The reply dries the glass with crumpled newspaper or a dry cloth in place of the squeegee and keeps the other steps.
