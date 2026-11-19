---
id: embroider_monogram
stage: procedures
task: embroider_monogram
schema: v3
---
# Embroider a monogram

## requests
- I want to stitch my initials onto a cloth napkin. How do I embroider a monogram?
- What's the process for doing monogram embroidery by hand?
- Could you explain how to sew letters onto fabric with embroidery floss?

## situation
- I have fabric, a pencil, an embroidery hoop, a needle, embroidery floss and scissors.
- I've got about 45 minutes this evening.

## ground truth
```datalog
task(embroider_monogram).
domain(embroider_monogram, crafts).
goal(embroider_monogram, monogram).
needs(embroider_monogram, fabric).
needs(embroider_monogram, pencil).
needs(embroider_monogram, embroidery_hoop).
needs(embroider_monogram, needle).
needs(embroider_monogram, embroidery_floss).
needs(embroider_monogram, scissors).
takes_time(embroider_monogram, 30, 45, minute).
step(embroider_monogram, embroider_monogram_s1).
action(embroider_monogram_s1, trace).
object_of(embroider_monogram_s1, fabric).
uses(embroider_monogram_s1, pencil).
role(embroider_monogram_s1, pencil, mark).
step(embroider_monogram, embroider_monogram_s2).
action(embroider_monogram_s2, fasten).
object_of(embroider_monogram_s2, fabric).
uses(embroider_monogram_s2, embroidery_hoop).
role(embroider_monogram_s2, embroidery_hoop, hold).
purpose(embroider_monogram_s2, fabric_stays_taut).
step(embroider_monogram, embroider_monogram_s3).
action(embroider_monogram_s3, thread).
object_of(embroider_monogram_s3, needle).
uses(embroider_monogram_s3, embroidery_floss).
role(embroider_monogram_s3, embroidery_floss, thread).
overlaps(embroider_monogram_s3, embroider_monogram_s1).
step(embroider_monogram, embroider_monogram_s4).
action(embroider_monogram_s4, stitch).
object_of(embroider_monogram_s4, fabric).
uses(embroider_monogram_s4, needle).
role(embroider_monogram_s4, needle, pierce).
uses(embroider_monogram_s4, embroidery_floss).
role(embroider_monogram_s4, embroidery_floss, stitch).
takes_time(embroider_monogram_s4, 20, 25, minute).
step(embroider_monogram, embroider_monogram_s5).
action(embroider_monogram_s5, cut).
object_of(embroider_monogram_s5, embroidery_floss).
uses(embroider_monogram_s5, scissors).
role(embroider_monogram_s5, scissors, cut).
before(embroider_monogram_s1, embroider_monogram_s2).
before(embroider_monogram_s2, embroider_monogram_s4).
before(embroider_monogram_s3, embroider_monogram_s4).
before(embroider_monogram_s4, embroider_monogram_s5).
```

## aliases
```datalog
task_alias(embroider_monogram, "monogram embroidery").
task_alias(embroider_monogram, "embroidering a monogram").
alias(fabric, "cloth").
alias(embroidery_hoop, "hoop").
alias(embroidery_floss, "thread").
alias(stitch, "sew").
alias(fabric_stays_taut, "stays taut").
```

## teach
- To embroider a monogram you need fabric, a pencil, an embroidery hoop, a needle, embroidery floss and scissors.
- Embroidering a monogram takes 30 to 45 minutes.
- Step 1 of embroidering a monogram: trace the letters onto the fabric with the pencil.
- Step 2 of embroidering a monogram: fasten the fabric in the hoop so it stays taut.
- While step 1 runs, step 3 of embroidering a monogram: thread the needle with the floss.
- Step 4 of embroidering a monogram: stitch the letters with the needle and floss, for 20 to 25 minutes.
- Step 5 of embroidering a monogram: cut the floss with the scissors.

## checklist
- names what is needed, including fabric, a pencil, a hoop, a needle, floss and scissors
- gives the steps in order: trace the letters, fasten the fabric in the hoop, thread the needle, stitch the letters, cut the floss
- says the threading can be done while the letters are traced
- gives a total time of about 30 to 45 minutes

## variant
- change: no hoop; the user holds the fabric taut with a binder clip instead of the hoop
- request: I want to do my monogram but I don't have an embroidery hoop. Can I use something else?
- situation: I have fabric, a pencil, a binder clip, a needle, embroidery floss and scissors.
- expect: The reply says to clip the fabric taut with the binder clip in place of the hoop, then follows the same steps: trace, thread, stitch, cut.

```datalog situation
lacks(user, embroidery_hoop).
has(user, binder_clip).
```

```datalog difference
no_need(embroidery_hoop).
extra_need(binder_clip).
swap(embroider_monogram_s2, embroidery_hoop, binder_clip).
```

## variant
- change: add a border stitched around the letters, which takes about 10 more minutes
- request: Can you add a border around the monogram? I have about an hour.
- situation: I have fabric, a pencil, an embroidery hoop, a needle, embroidery floss, scissors, and about an hour.
- expect: The reply gives the same steps and adds a border stitch after the letters are stitched, bringing the total to about 40 to 55 minutes.

```datalog situation
has(user, fabric).
has(user, embroidery_hoop).
wants(user, border).
time_limit(user, 60, minute).
```

```datalog difference
add_step(embroider_monogram_s9, embroider_monogram_s4).
step(embroider_monogram, embroider_monogram_s9).
action(embroider_monogram_s9, stitch).
object_of(embroider_monogram_s9, fabric).
uses(embroider_monogram_s9, needle).
role(embroider_monogram_s9, needle, pierce).
uses(embroider_monogram_s9, embroidery_floss).
role(embroider_monogram_s9, embroidery_floss, stitch).
before(embroider_monogram_s4, embroider_monogram_s9).
before(embroider_monogram_s9, embroider_monogram_s5).
change(embroider_monogram, takes_time, 30_45, 40_55).
```
