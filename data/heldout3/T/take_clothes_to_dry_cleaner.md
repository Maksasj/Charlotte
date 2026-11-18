---
id: take_clothes_to_dry_cleaner
stage: social
task: take_clothes_to_dry_cleaner
schema: v3
tier: T
---
# Take clothes to the dry cleaner

## requests
- My suit needs cleaning before a wedding. How does dry cleaning work?
- What should I do before dropping a coat off at the dry cleaners?
- I spilt wine on a silk dress that says dry clean only. What now?

## situation
- I have a wool suit and a silk dress, both dry clean only.

## ground truth
```datalog
task(take_clothes_to_dry_cleaner).
domain(take_clothes_to_dry_cleaner, errands).
goal(take_clothes_to_dry_cleaner, clean_clothes).
needs(take_clothes_to_dry_cleaner, ticket).
takes_time(take_clothes_to_dry_cleaner, 2, 3, day).
step(take_clothes_to_dry_cleaner, take_clothes_to_dry_cleaner_s1).
action(take_clothes_to_dry_cleaner_s1, check).
object_of(take_clothes_to_dry_cleaner_s1, care_label).
step(take_clothes_to_dry_cleaner, take_clothes_to_dry_cleaner_s2).
action(take_clothes_to_dry_cleaner_s2, empty).
object_of(take_clothes_to_dry_cleaner_s2, pocket).
step(take_clothes_to_dry_cleaner, take_clothes_to_dry_cleaner_s3).
action(take_clothes_to_dry_cleaner_s3, point_out).
object_of(take_clothes_to_dry_cleaner_s3, stain).
purpose(take_clothes_to_dry_cleaner_s3, stain_treated).
step(take_clothes_to_dry_cleaner, take_clothes_to_dry_cleaner_s4).
action(take_clothes_to_dry_cleaner_s4, take).
object_of(take_clothes_to_dry_cleaner_s4, ticket).
purpose(take_clothes_to_dry_cleaner_s4, collect_later).
step(take_clothes_to_dry_cleaner, take_clothes_to_dry_cleaner_s5).
action(take_clothes_to_dry_cleaner_s5, collect).
object_of(take_clothes_to_dry_cleaner_s5, clothes).
uses(take_clothes_to_dry_cleaner_s5, ticket).
role(take_clothes_to_dry_cleaner_s5, ticket, inform).
takes_time(take_clothes_to_dry_cleaner_s5, 2, 3, day).
before(take_clothes_to_dry_cleaner_s1, take_clothes_to_dry_cleaner_s2).
before(take_clothes_to_dry_cleaner_s2, take_clothes_to_dry_cleaner_s3).
before(take_clothes_to_dry_cleaner_s3, take_clothes_to_dry_cleaner_s4).
before(take_clothes_to_dry_cleaner_s4, take_clothes_to_dry_cleaner_s5).
```

## aliases
```datalog
task_alias(take_clothes_to_dry_cleaner, "dry cleaning").
task_alias(take_clothes_to_dry_cleaner, "dropping off dry cleaning").
task_alias(take_clothes_to_dry_cleaner, "getting clothes dry cleaned").
alias(care_label, "the care label").
alias(pocket, "the pockets").
alias(ticket, "the ticket").
alias(point_out, "show").
alias(stain_treated, "they treat it").
```

## teach
- To take clothes to the dry cleaner you need the clothes and the ticket they give you.
- Taking clothes to the dry cleaner takes 2 to 3 days.
- Step 1 of taking clothes to the dry cleaner: check the care label.
- Step 2 of taking clothes to the dry cleaner: empty the pockets.
- Step 3 of taking clothes to the dry cleaner: point out any stain so they treat it.
- Step 4 of taking clothes to the dry cleaner: take the ticket so you can collect them later.
- Step 5 of taking clothes to the dry cleaner: collect the clothes with the ticket after 2 to 3 days.

## checklist
- gives the steps in order: check label, empty pockets, point out stains, take ticket, collect
- says it usually takes 2 to 3 days

## variant
- change: needed tomorrow: ask for same-day express service
- request: The wedding's tomorrow and my suit is filthy. Can a dry cleaner do it in time?
- situation: I have the suit, and I need it in 1 day.
- expect: The reply asks for the express same-day service when dropping off, so the clothes can be collected the same evening.

```datalog situation
time_limit(user, 1, day).
```

```datalog difference
add_step(take_clothes_to_dry_cleaner_s9, take_clothes_to_dry_cleaner_s3).
step(take_clothes_to_dry_cleaner, take_clothes_to_dry_cleaner_s9).
action(take_clothes_to_dry_cleaner_s9, ask).
object_of(take_clothes_to_dry_cleaner_s9, express_service).
before(take_clothes_to_dry_cleaner_s3, take_clothes_to_dry_cleaner_s9).
before(take_clothes_to_dry_cleaner_s9, take_clothes_to_dry_cleaner_s4).
change(take_clothes_to_dry_cleaner_s5, takes_time, 2_3_day, 1_1_day).
change(take_clothes_to_dry_cleaner, takes_time, 2_3_day, 1_1_day).
```

## variant
- change: lost the ticket: show the photo of it on the phone
- request: I've lost my dry cleaning ticket, but I took a picture of it. Can I still get my coat back?
- situation: I have a photo of the ticket on my phone but the paper ticket is lost.
- expect: The reply shows the photo of the ticket on the phone when collecting instead of the paper ticket.

```datalog situation
has(user, phone).
lacks(user, ticket).
```

```datalog difference
swap(take_clothes_to_dry_cleaner_s5, ticket, phone).
extra_need(phone).
no_need(ticket).
```
