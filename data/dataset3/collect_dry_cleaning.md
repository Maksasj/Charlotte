---
id: collect_dry_cleaning
stage: procedures
task: collect_dry_cleaning
schema: v3
---
# Collect dry cleaning

## requests
- Where do I pick up my dry cleaning, and what do I need to bring with me?
- Could you tell me the steps for getting my clothes back from the dry cleaner?
- I'm heading out to collect my suits from the cleaners. What's the routine?

## situation
- I have my claim ticket, a payment card and a garment bag.
- The cleaner closes at six, so I need to get there in time.

## ground truth
```datalog
task(collect_dry_cleaning).
domain(collect_dry_cleaning, errands).
goal(collect_dry_cleaning, clothes_collected).
needs(collect_dry_cleaning, claim_ticket).
needs(collect_dry_cleaning, payment_card).
needs(collect_dry_cleaning, garment_bag).
takes_time(collect_dry_cleaning, 10, 15, minute).
step(collect_dry_cleaning, collect_dry_cleaning_s1).
action(collect_dry_cleaning_s1, go).
object_of(collect_dry_cleaning_s1, dry_cleaner).
step(collect_dry_cleaning, collect_dry_cleaning_s2).
action(collect_dry_cleaning_s2, show).
object_of(collect_dry_cleaning_s2, claim_ticket).
uses(collect_dry_cleaning_s2, claim_ticket).
role(collect_dry_cleaning_s2, claim_ticket, hold).
step(collect_dry_cleaning, collect_dry_cleaning_s3).
action(collect_dry_cleaning_s3, check).
object_of(collect_dry_cleaning_s3, garments).
check(collect_dry_cleaning_s3, no_stains).
step(collect_dry_cleaning, collect_dry_cleaning_s4).
action(collect_dry_cleaning_s4, pay).
object_of(collect_dry_cleaning_s4, bill).
uses(collect_dry_cleaning_s4, payment_card).
role(collect_dry_cleaning_s4, payment_card, hold).
step(collect_dry_cleaning, collect_dry_cleaning_s5).
action(collect_dry_cleaning_s5, carry).
object_of(collect_dry_cleaning_s5, garments).
uses(collect_dry_cleaning_s5, garment_bag).
role(collect_dry_cleaning_s5, garment_bag, contain).
into(collect_dry_cleaning_s5, garment_bag).
before(collect_dry_cleaning_s1, collect_dry_cleaning_s2).
before(collect_dry_cleaning_s2, collect_dry_cleaning_s3).
before(collect_dry_cleaning_s3, collect_dry_cleaning_s4).
before(collect_dry_cleaning_s4, collect_dry_cleaning_s5).
```

## aliases
```datalog
task_alias(collect_dry_cleaning, "pick up dry cleaning").
task_alias(collect_dry_cleaning, "get my dry cleaning").
alias(dry_cleaner, "dry cleaner").
alias(claim_ticket, "claim ticket").
alias(payment_card, "card").
alias(garment_bag, "garment bag").
alias(garments, "clothes").
alias(id_card, "photo ID").
alias(clothes_collected, "clothes collected").
alias(no_stains, "no stains").
```

## teach
- To collect dry cleaning you need a claim ticket, a payment card and a garment bag.
- Collecting dry cleaning takes 10 to 15 minutes.
- Step 1 of collecting dry cleaning: go to the dry cleaner.
- Step 2 of collecting dry cleaning: show the claim ticket at the counter.
- Step 3 of collecting dry cleaning: check the garments for stains before you pay.
- Step 4 of collecting dry cleaning: pay the bill with your payment card.
- Step 5 of collecting dry cleaning: put the garments into the garment bag to carry them home.

## checklist
- names what is needed: a claim ticket, a payment card and a garment bag
- gives the steps in order: go to the cleaner, show the ticket, check for stains, pay, carry the clothes home in the bag
- gives a total time of about 10 to 15 minutes

## variant
- change: lost the claim ticket: show a photo ID at the counter instead
- request: I've lost my claim ticket for my dry cleaning. What do I do?
- situation: I have a photo ID, a payment card and a garment bag, but no claim ticket.
- expect: The reply says to go to the cleaner, show your photo ID instead of the ticket, check the garments, pay and carry them home in the bag.

```datalog situation
lacks(user, claim_ticket).
has(user, id_card).
has(user, payment_card).
has(user, garment_bag).
```

```datalog difference
extra_need(id_card).
no_need(claim_ticket).
drop(collect_dry_cleaning_s2).
add_step(collect_dry_cleaning_s9, collect_dry_cleaning_s1).
step(collect_dry_cleaning, collect_dry_cleaning_s9).
action(collect_dry_cleaning_s9, show).
object_of(collect_dry_cleaning_s9, id_card).
uses(collect_dry_cleaning_s9, id_card).
role(collect_dry_cleaning_s9, id_card, hold).
before(collect_dry_cleaning_s9, collect_dry_cleaning_s3).
```

## variant
- change: only 15 minutes before closing: skip the stain check and just collect and go
- request: I only have 15 minutes before the cleaner shuts. Can I still get my clothes?
- situation: I have my claim ticket, a payment card and a garment bag, and 15 minutes.
- expect: The reply goes straight to the counter, shows the ticket, pays and carries the clothes home, leaving out the stain check to fit in 15 minutes.

```datalog situation
has(user, claim_ticket).
has(user, payment_card).
has(user, garment_bag).
time_limit(user, 15, minute).
```

```datalog difference
drop(collect_dry_cleaning_s3).
change(collect_dry_cleaning, takes_time, 10_15, 5_10).
```
