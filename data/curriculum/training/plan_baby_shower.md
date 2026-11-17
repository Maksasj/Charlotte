---
id: plan_baby_shower
stage: social
task: plan_baby_shower
---
# Plan a baby shower

## requests
- My best friend is pregnant and I want to throw her a baby shower. Where do I begin?
- Can you help me plan a baby shower for my sister?
- What are the main things to organise for a baby shower?

## situation
- I have a guest list started, a living room to host in and a small budget for a cake, games and decorations.
- The shower is in five weeks.

## ground truth
```datalog
task(plan_baby_shower).
needs(plan_baby_shower, guest_list).
needs(plan_baby_shower, invitation).
needs(plan_baby_shower, cake).
needs(plan_baby_shower, game).
needs(plan_baby_shower, decoration).
takes_time(plan_baby_shower, 90, 130, minute).
step(plan_baby_shower, plan_baby_shower_s1).
action(plan_baby_shower_s1, ask).
object_of(plan_baby_shower_s1, parent).
takes_time(plan_baby_shower_s1, 10, 15, minute).
step(plan_baby_shower, plan_baby_shower_s2).
action(plan_baby_shower_s2, make).
object_of(plan_baby_shower_s2, guest_list).
takes_time(plan_baby_shower_s2, 10, 15, minute).
step(plan_baby_shower, plan_baby_shower_s3).
action(plan_baby_shower_s3, send).
object_of(plan_baby_shower_s3, invitation).
uses(plan_baby_shower_s3, guest_list).
takes_time(plan_baby_shower_s3, 15, 20, minute).
step(plan_baby_shower, plan_baby_shower_s4).
action(plan_baby_shower_s4, order).
object_of(plan_baby_shower_s4, cake).
takes_time(plan_baby_shower_s4, 15, 20, minute).
step(plan_baby_shower, plan_baby_shower_s5).
action(plan_baby_shower_s5, plan).
object_of(plan_baby_shower_s5, game).
takes_time(plan_baby_shower_s5, 20, 30, minute).
step(plan_baby_shower, plan_baby_shower_s6).
action(plan_baby_shower_s6, buy).
object_of(plan_baby_shower_s6, decoration).
takes_time(plan_baby_shower_s6, 20, 30, minute).
before(plan_baby_shower_s1, plan_baby_shower_s2).
before(plan_baby_shower_s2, plan_baby_shower_s3).
before(plan_baby_shower_s3, plan_baby_shower_s4).
before(plan_baby_shower_s4, plan_baby_shower_s5).
before(plan_baby_shower_s5, plan_baby_shower_s6).
```

## teach
- To plan a baby shower you need a guest list, invitations, a cake, games and decorations.
- Planning a baby shower takes 90 to 130 minutes.
- Step 1 of planning a baby shower: ask the parents-to-be about a date and who they want there for 10 to 15 minutes.
- Step 2 of planning a baby shower: make the guest list with the parents-to-be for 10 to 15 minutes.
- Step 3 of planning a baby shower: send the invitations to everyone on the guest list at least three weeks ahead for 15 to 20 minutes.
- Step 4 of planning a baby shower: order the cake for 15 to 20 minutes.
- Step 5 of planning a baby shower: plan two or three simple games for 20 to 30 minutes.
- Step 6 of planning a baby shower: buy decorations in a soft colour scheme for 20 to 30 minutes.

## checklist
- names what is needed: a guest list, invitations, a cake, games and decorations
- gives the steps in order: ask parent, make guest list, send invitation, order cake, plan game, buy decoration
- gives a total time of about 90 to 130 minutes
- closes politely

## variant
- change: surprise shower: ask the partner for the date and guests and keep the parent-to-be out of it
- request: I want the baby shower to be a surprise for my friend. How does that change the planning?
- situation: It is a surprise. I have her partner's number, a living room to host in and a small budget.
- expect: The reply asks the partner instead of the parent-to-be about the date and guest list, asks guests to keep it secret on the invitations and keeps the cake, games and decorations steps.
