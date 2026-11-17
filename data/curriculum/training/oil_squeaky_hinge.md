---
id: oil_squeaky_hinge
stage: tools
task: oil_squeaky_hinge
---
# Fix a squeaky hinge

## requests
- My bedroom door squeaks every time I open it. How do I stop it?
- Can you tell me how to oil a creaky door hinge?
- The squeak from the hall door is driving me mad. What's the fix?

## situation
- I have spray lubricant and a cloth.
- I have 10 minutes.

## ground truth
```datalog
task(oil_squeaky_hinge).
needs(oil_squeaky_hinge, lubricant).
needs(oil_squeaky_hinge, cloth).
takes_time(oil_squeaky_hinge, 4, 6, minute).
step(oil_squeaky_hinge, oil_squeaky_hinge_s1).
action(oil_squeaky_hinge_s1, find).
object_of(oil_squeaky_hinge_s1, hinge).
takes_time(oil_squeaky_hinge_s1, 1, 1, minute).
step(oil_squeaky_hinge, oil_squeaky_hinge_s2).
action(oil_squeaky_hinge_s2, spray).
object_of(oil_squeaky_hinge_s2, hinge).
uses(oil_squeaky_hinge_s2, lubricant).
takes_time(oil_squeaky_hinge_s2, 1, 1, minute).
step(oil_squeaky_hinge, oil_squeaky_hinge_s3).
action(oil_squeaky_hinge_s3, swing).
object_of(oil_squeaky_hinge_s3, door).
until(oil_squeaky_hinge_s3, quiet).
takes_time(oil_squeaky_hinge_s3, 1, 2, minute).
step(oil_squeaky_hinge, oil_squeaky_hinge_s4).
action(oil_squeaky_hinge_s4, wipe).
object_of(oil_squeaky_hinge_s4, lubricant).
uses(oil_squeaky_hinge_s4, cloth).
takes_time(oil_squeaky_hinge_s4, 1, 1, minute).
before(oil_squeaky_hinge_s1, oil_squeaky_hinge_s2).
before(oil_squeaky_hinge_s2, oil_squeaky_hinge_s3).
before(oil_squeaky_hinge_s3, oil_squeaky_hinge_s4).
```

## teach
- To fix a squeaky hinge you need spray lubricant and a cloth.
- Fixing a squeaky hinge takes 4 to 6 minutes.
- Step 1 of fixing a squeaky hinge: find the squeaky hinge by swinging the door slowly for 1 minute.
- Step 2 of fixing a squeaky hinge: spray lubricant into the top of the hinge for 1 minute.
- Step 3 of fixing a squeaky hinge: swing the door back and forth until the squeak stops, for 1 to 2 minutes.
- Step 4 of fixing a squeaky hinge: wipe off the extra lubricant with the cloth for 1 minute.

## checklist
- names what is needed: spray lubricant and a cloth
- gives the steps in order: find hinge, spray hinge, swing door, wipe lubricant
- gives a total time of about 4 to 6 minutes
- closes politely

## variant
- change: no spray lubricant: rub a little petroleum jelly onto the hinge pin
- request: My door squeaks but I don't have any spray oil. Is there something from the bathroom cabinet I can use?
- situation: I have petroleum jelly, a cloth, a hammer and a screwdriver.
- expect: The reply taps the hinge pin up with the screwdriver and hammer, rubs petroleum jelly on it, puts it back and keeps the swing and wipe steps.
