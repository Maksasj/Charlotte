---
id: wrap_gift__wants_gift_tag
task: wrap_gift
schema: v3
tier: S
base: wrap_gift
---
# Wrap a gift

## requests
- I'm wrapping my nephew's present and want to put a little name tag on it too. When do I do that?

## ground truth
```datalog
task(wrap_gift).
domain(wrap_gift, crafts).
goal(wrap_gift, wrapped_gift).
needs(wrap_gift, gift).
needs(wrap_gift, wrapping_paper).
needs(wrap_gift, scissors).
needs(wrap_gift, tape).
needs(wrap_gift, ribbon).
takes_time(wrap_gift, 10, 15, minute).
step(wrap_gift, wrap_gift_s1).
action(wrap_gift_s1, measure).
object_of(wrap_gift_s1, wrapping_paper).
uses(wrap_gift_s1, gift).
role(wrap_gift_s1, gift, measure).
takes_time(wrap_gift_s1, 1, 2, minute).
step(wrap_gift, wrap_gift_s2).
action(wrap_gift_s2, cut).
object_of(wrap_gift_s2, wrapping_paper).
uses(wrap_gift_s2, scissors).
role(wrap_gift_s2, scissors, cut).
takes_time(wrap_gift_s2, 1, 2, minute).
step(wrap_gift, wrap_gift_s3).
action(wrap_gift_s3, fold).
object_of(wrap_gift_s3, wrapping_paper).
uses(wrap_gift_s3, gift).
role(wrap_gift_s3, gift, hold).
takes_time(wrap_gift_s3, 2, 4, minute).
step(wrap_gift, wrap_gift_s4).
action(wrap_gift_s4, tape).
object_of(wrap_gift_s4, seam).
uses(wrap_gift_s4, tape).
role(wrap_gift_s4, tape, fasten).
takes_time(wrap_gift_s4, 2, 3, minute).
step(wrap_gift, wrap_gift_s5).
action(wrap_gift_s5, tie).
object_of(wrap_gift_s5, ribbon).
uses(wrap_gift_s5, scissors).
role(wrap_gift_s5, scissors, cut).
takes_time(wrap_gift_s5, 2, 3, minute).
onto(wrap_gift_s5, gift).
before(wrap_gift_s1, wrap_gift_s2).
before(wrap_gift_s2, wrap_gift_s3).
before(wrap_gift_s3, wrap_gift_s4).
before(wrap_gift_s4, wrap_gift_s5).
```

## aliases
```datalog
task_alias(wrap_gift, "wrapping a gift").
task_alias(wrap_gift, "wrap a present").
task_alias(wrap_gift, "gift wrapping").
alias(gift, "present").
alias(wrapping_paper, "wrapping paper").
alias(wrapping_paper, "gift paper").
alias(tape, "sticky tape").
alias(seam, "edges").
alias(measure, "cut to size").
alias(fold, "wrap").
```

## teach
- To wrap a gift you need the gift, wrapping paper, scissors, tape and ribbon.
- Wrapping a gift takes 10 to 15 minutes.
- Step 1 of wrapping a gift: measure the wrapping paper against the gift for 1 to 2 minutes.
- Step 2 of wrapping a gift: cut the wrapping paper with the scissors for 1 to 2 minutes.
- Step 3 of wrapping a gift: fold the wrapping paper around the gift for 2 to 4 minutes.
- Step 4 of wrapping a gift: tape the seams with the tape for 2 to 3 minutes.
- Step 5 of wrapping a gift: tie the ribbon onto the gift and trim it with the scissors, for 2 to 3 minutes.

## variant
- change: add a gift tag: tape it onto the ribbon at the end
- request: I'm wrapping my nephew's present and want to put a little name tag on it too. When do I do that?
- situation: I have the present, wrapping paper, scissors, tape, ribbon and a gift tag.
- expect: The reply wraps and ties the present as usual and then fixes the tag to the ribbon with a bit of tape.

```datalog situation
has(user, gift).
has(user, wrapping_paper).
has(user, scissors).
has(user, tape).
has(user, ribbon).
has(user, gift_tag).
wants(user, gift_tag).
```

```datalog difference
extra_need(gift_tag).
add_step(wrap_gift_s9, wrap_gift_s5).
step(wrap_gift, wrap_gift_s9).
action(wrap_gift_s9, attach).
object_of(wrap_gift_s9, gift_tag).
onto(wrap_gift_s9, ribbon).
uses(wrap_gift_s9, tape).
role(wrap_gift_s9, tape, fasten).
before(wrap_gift_s5, wrap_gift_s9).
```
