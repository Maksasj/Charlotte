---
id: oil_squeaky_hinge
stage: tools
task: oil_squeaky_hinge
schema: v3
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
domain(oil_squeaky_hinge, repair).
goal(oil_squeaky_hinge, quiet_hinge).
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
role(oil_squeaky_hinge_s2, lubricant, lubricate).
with_part(oil_squeaky_hinge_s2, top).
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
role(oil_squeaky_hinge_s4, cloth, wipe).
takes_time(oil_squeaky_hinge_s4, 1, 1, minute).
before(oil_squeaky_hinge_s1, oil_squeaky_hinge_s2).
before(oil_squeaky_hinge_s2, oil_squeaky_hinge_s3).
before(oil_squeaky_hinge_s3, oil_squeaky_hinge_s4).
```

## aliases
```datalog
task_alias(oil_squeaky_hinge, "fixing a squeaky hinge").
task_alias(oil_squeaky_hinge, "fix a squeaky hinge").
task_alias(oil_squeaky_hinge, "oil a creaky door hinge").
task_alias(oil_squeaky_hinge, "stop the squeak").
alias(lubricant, "spray lubricant").
alias(lubricant, "spray oil").
alias(lubricant, "extra lubricant").
alias(hinge, "squeaky hinge").
alias(hinge, "creaky door hinge").
alias(door, "bedroom door").
alias(quiet, "the squeak stops").
alias(top, "top of the hinge").
alias(hinge_pin, "hinge pin").
alias(petroleum_jelly, "petroleum jelly").
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

```datalog situation
lacks(user, lubricant).
has(user, petroleum_jelly).
has(user, cloth).
has(user, hammer).
has(user, screwdriver).
```

```datalog difference
drop(oil_squeaky_hinge_s2).
no_need(lubricant).
extra_need(petroleum_jelly).
extra_need(hammer).
extra_need(screwdriver).
add_step(oil_squeaky_hinge_s9, oil_squeaky_hinge_s1).
step(oil_squeaky_hinge, oil_squeaky_hinge_s9).
action(oil_squeaky_hinge_s9, tap).
object_of(oil_squeaky_hinge_s9, hinge_pin).
uses(oil_squeaky_hinge_s9, screwdriver).
role(oil_squeaky_hinge_s9, screwdriver, lift).
uses(oil_squeaky_hinge_s9, hammer).
role(oil_squeaky_hinge_s9, hammer, strike).
add_step(oil_squeaky_hinge_s10, oil_squeaky_hinge_s9).
step(oil_squeaky_hinge, oil_squeaky_hinge_s10).
action(oil_squeaky_hinge_s10, rub).
object_of(oil_squeaky_hinge_s10, hinge_pin).
uses(oil_squeaky_hinge_s10, petroleum_jelly).
role(oil_squeaky_hinge_s10, petroleum_jelly, lubricate).
add_step(oil_squeaky_hinge_s11, oil_squeaky_hinge_s10).
step(oil_squeaky_hinge, oil_squeaky_hinge_s11).
action(oil_squeaky_hinge_s11, insert).
object_of(oil_squeaky_hinge_s11, hinge_pin).
into(oil_squeaky_hinge_s11, hinge).
before(oil_squeaky_hinge_s1, oil_squeaky_hinge_s9).
before(oil_squeaky_hinge_s9, oil_squeaky_hinge_s10).
before(oil_squeaky_hinge_s10, oil_squeaky_hinge_s11).
before(oil_squeaky_hinge_s11, oil_squeaky_hinge_s3).
```
