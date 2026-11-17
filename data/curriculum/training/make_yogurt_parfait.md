---
id: make_yogurt_parfait
stage: procedures
task: make_yogurt_parfait
---
# Make a yogurt parfait

## requests
- I want a nice breakfast in a glass. How do I put together a yogurt parfait?
- Can you tell me how to layer yogurt, granola and berries?
- What's a quick healthy breakfast I can make with yogurt and fruit?

## situation
- I have yogurt, granola, berries, honey and a tall glass.
- I need to leave in 10 minutes.

## ground truth
```datalog
task(make_yogurt_parfait).
needs(make_yogurt_parfait, yogurt).
needs(make_yogurt_parfait, granola).
needs(make_yogurt_parfait, berry).
needs(make_yogurt_parfait, honey).
needs(make_yogurt_parfait, glass).
takes_time(make_yogurt_parfait, 5, 7, minute).
step(make_yogurt_parfait, make_yogurt_parfait_s1).
action(make_yogurt_parfait_s1, rinse).
object_of(make_yogurt_parfait_s1, berry).
takes_time(make_yogurt_parfait_s1, 1, 2, minute).
step(make_yogurt_parfait, make_yogurt_parfait_s2).
action(make_yogurt_parfait_s2, spoon).
object_of(make_yogurt_parfait_s2, yogurt).
uses(make_yogurt_parfait_s2, glass).
takes_time(make_yogurt_parfait_s2, 1, 1, minute).
step(make_yogurt_parfait, make_yogurt_parfait_s3).
action(make_yogurt_parfait_s3, layer).
object_of(make_yogurt_parfait_s3, granola).
uses(make_yogurt_parfait_s3, berry).
takes_time(make_yogurt_parfait_s3, 2, 3, minute).
step(make_yogurt_parfait, make_yogurt_parfait_s4).
action(make_yogurt_parfait_s4, drizzle).
object_of(make_yogurt_parfait_s4, honey).
takes_time(make_yogurt_parfait_s4, 1, 1, minute).
before(make_yogurt_parfait_s1, make_yogurt_parfait_s2).
before(make_yogurt_parfait_s2, make_yogurt_parfait_s3).
before(make_yogurt_parfait_s3, make_yogurt_parfait_s4).
```

## teach
- To make a yogurt parfait you need yogurt, granola, berries, honey and a glass.
- Making a yogurt parfait takes 5 to 7 minutes.
- Step 1 of making a yogurt parfait: rinse the berries and pat them dry for 1 to 2 minutes.
- Step 2 of making a yogurt parfait: spoon a layer of yogurt into the glass for 1 minute.
- Step 3 of making a yogurt parfait: add a layer of granola and berries, then repeat the layers, for 2 to 3 minutes.
- Step 4 of making a yogurt parfait: drizzle the honey over the top for 1 minute.

## checklist
- names what is needed: yogurt, granola, berries, honey and a glass
- gives the steps in order: rinse berry, spoon yogurt, layer granola, drizzle honey
- gives a total time of about 5 to 7 minutes
- closes politely
