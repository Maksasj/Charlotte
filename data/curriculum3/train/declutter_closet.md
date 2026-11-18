---
id: declutter_closet
stage: household
task: declutter_closet
schema: v3
---
# Declutter a closet

## requests
- My wardrobe is overflowing. How do I declutter it?
- Can you help me sort out my closet? I can't find anything in it.
- What's a good method for clearing out clothes I don't wear?

## situation
- I have bags, a cloth and plenty of hangers.
- I've set aside an hour and a half.

## ground truth
```datalog
task(declutter_closet).
domain(declutter_closet, cleaning).
goal(declutter_closet, tidy_closet).
needs(declutter_closet, bag).
needs(declutter_closet, cloth).
needs(declutter_closet, hanger).
takes_time(declutter_closet, 60, 90, minute).
step(declutter_closet, declutter_closet_s1).
action(declutter_closet_s1, empty).
object_of(declutter_closet_s1, closet).
takes_time(declutter_closet_s1, 10, 15, minute).
step(declutter_closet, declutter_closet_s2).
action(declutter_closet_s2, sort).
object_of(declutter_closet_s2, clothing).
into(declutter_closet_s2, pile).
takes_time(declutter_closet_s2, 25, 40, minute).
step(declutter_closet, declutter_closet_s3).
action(declutter_closet_s3, pack).
object_of(declutter_closet_s3, pile).
uses(declutter_closet_s3, bag).
role(declutter_closet_s3, bag, contain).
into(declutter_closet_s3, bag).
takes_time(declutter_closet_s3, 5, 10, minute).
step(declutter_closet, declutter_closet_s4).
action(declutter_closet_s4, wipe).
object_of(declutter_closet_s4, shelf).
uses(declutter_closet_s4, cloth).
role(declutter_closet_s4, cloth, wipe).
requires_state(declutter_closet_s4, cloth_damp).
takes_time(declutter_closet_s4, 5, 10, minute).
step(declutter_closet, declutter_closet_s5).
action(declutter_closet_s5, hang).
object_of(declutter_closet_s5, clothing).
uses(declutter_closet_s5, hanger).
role(declutter_closet_s5, hanger, hold).
into(declutter_closet_s5, closet).
condition(declutter_closet_s5, clothing_kept).
takes_time(declutter_closet_s5, 15, 20, minute).
before(declutter_closet_s1, declutter_closet_s2).
before(declutter_closet_s2, declutter_closet_s3).
before(declutter_closet_s3, declutter_closet_s4).
before(declutter_closet_s4, declutter_closet_s5).
```

## aliases
```datalog
task_alias(declutter_closet, "decluttering a closet").
task_alias(declutter_closet, "declutter my wardrobe").
task_alias(declutter_closet, "sort out my closet").
task_alias(declutter_closet, "clearing out clothes").
alias(closet, "wardrobe").
alias(clothing, "clothes").
alias(bag, "bags").
alias(hanger, "hangers").
alias(cloth, "damp cloth").
alias(pile, "piles").
alias(pile, "donate pile").
alias(pile, "throw-away pile").
alias(shelf, "shelves").
alias(empty, "take everything out").
alias(clothing_kept, "the clothes you are keeping").
```

## teach
- To declutter a closet you need bags, a damp cloth and hangers.
- Decluttering a closet takes 60 to 90 minutes.
- Step 1 of decluttering a closet: take everything out of the closet for 10 to 15 minutes.
- Step 2 of decluttering a closet: sort the clothes into three piles: keep, donate and throw away, for 25 to 40 minutes.
- Step 3 of decluttering a closet: pack the donate pile and the throw-away pile into separate bags for 5 to 10 minutes.
- Step 4 of decluttering a closet: wipe the empty shelves with the damp cloth for 5 to 10 minutes.
- Step 5 of decluttering a closet: hang the clothes you are keeping back in the closet on the hangers for 15 to 20 minutes.

## checklist
- names what is needed: bags, a damp cloth and hangers
- gives the steps in order: empty closet, sort clothing, pack pile, wipe shelf, hang clothing
- gives a total time of about 60 to 90 minutes
- closes politely
