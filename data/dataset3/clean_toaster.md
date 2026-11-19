---
id: clean_toaster
stage: procedures
task: clean_toaster
schema: v3
---
# Clean a toaster

## requests
- How do I clean the crumbs out of my toaster?
- What's the right way to give my toaster a proper clean?
- Can you walk me through cleaning a toaster, including the crumb tray?

## situation
- I have a toaster with a crumb tray, a small brush and a cloth.
- I've got about 15 minutes.

## ground truth
```datalog
task(clean_toaster).
domain(clean_toaster, cleaning).
goal(clean_toaster, clean_appliance).
needs(clean_toaster, brush).
needs(clean_toaster, cloth).
takes_time(clean_toaster, 10, 15, minute).
step(clean_toaster, clean_toaster_s1).
action(clean_toaster_s1, unplug).
object_of(clean_toaster_s1, toaster).
step(clean_toaster, clean_toaster_s2).
action(clean_toaster_s2, empty).
object_of(clean_toaster_s2, crumb_tray).
step(clean_toaster, clean_toaster_s3).
action(clean_toaster_s3, brush).
object_of(clean_toaster_s3, crumbs).
uses(clean_toaster_s3, brush).
role(clean_toaster_s3, brush, scrub).
check(clean_toaster_s3, slots_clear).
step(clean_toaster, clean_toaster_s4).
action(clean_toaster_s4, wipe).
object_of(clean_toaster_s4, toaster).
uses(clean_toaster_s4, cloth).
role(clean_toaster_s4, cloth, wipe).
warning(clean_toaster_s4, water_in_slots).
step(clean_toaster, clean_toaster_s5).
action(clean_toaster_s5, slide_in).
object_of(clean_toaster_s5, crumb_tray).
step(clean_toaster, clean_toaster_s6).
action(clean_toaster_s6, plug_in).
object_of(clean_toaster_s6, toaster).
before(clean_toaster_s1, clean_toaster_s2).
before(clean_toaster_s2, clean_toaster_s3).
before(clean_toaster_s3, clean_toaster_s4).
before(clean_toaster_s4, clean_toaster_s5).
before(clean_toaster_s5, clean_toaster_s6).
```

## aliases
```datalog
task_alias(clean_toaster, "clean a toaster").
task_alias(clean_toaster, "cleaning a toaster").
task_alias(clean_toaster, "clean my toaster").
alias(brush, "small brush").
alias(cloth, "damp cloth").
alias(crumb_tray, "crumb tray").
alias(crumbs, "crumbs").
alias(water_in_slots, "water in the slots").
alias(toothbrush, "old toothbrush").
alias(plug_in, "plug in").
alias(slide_in, "slide back in").
```

## teach
- To clean a toaster you need a brush and a cloth.
- Cleaning a toaster takes 10 to 15 minutes.
- Step 1 of cleaning a toaster: unplug the toaster.
- Step 2 of cleaning a toaster: empty the crumb tray.
- Step 3 of cleaning a toaster: brush the crumbs out of the slots with a small brush until the slots are clear.
- Step 4 of cleaning a toaster: wipe the outside of the toaster with a damp cloth, keeping water out of the slots.
- Step 5 of cleaning a toaster: slide the crumb tray back in.
- Step 6 of cleaning a toaster: plug the toaster back in.

## checklist
- names what is needed, including a brush and a cloth
- gives the steps in order: unplug, empty crumb tray, brush slots, wipe outside, slide tray back in, plug in
- warns to keep water out of the slots while wiping
- gives a total time of about 10 to 15 minutes

## variant
- change: the brush is missing, so the old toothbrush cleans the slots instead
- request: I've lost my brush. Can I clean the toaster slots with something else?
- situation: I have an old toothbrush and a cloth, and no brush.
- expect: The reply uses the old toothbrush instead of the brush to clear the slots and keeps the other steps in the same order.

```datalog situation
lacks(user, brush).
has(user, toothbrush).
has(user, cloth).
```

```datalog difference
swap(clean_toaster_s3, brush, toothbrush).
no_need(brush).
extra_need(toothbrush).
```

## variant
- change: the toaster has no removable crumb tray, so emptying and sliding it back in are dropped
- request: My toaster has no crumb tray. What do I still need to do to clean it?
- situation: My toaster has no crumb tray, and I have a brush and a cloth.
- expect: The reply skips emptying and sliding in the crumb tray, keeps the brushing, wiping and plugging in steps, and gives a shorter time of about 5 to 8 minutes.

```datalog situation
lacks(user, crumb_tray).
has(user, brush).
has(user, cloth).
```

```datalog difference
drop(clean_toaster_s2).
drop(clean_toaster_s5).
change(clean_toaster, takes_time, 10_15, 5_8).
```
