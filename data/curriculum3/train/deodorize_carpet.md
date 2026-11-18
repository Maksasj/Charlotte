---
id: deodorize_carpet
stage: household
task: deodorize_carpet
schema: v3
---
# Freshen a carpet

## requests
- My living room carpet smells musty. How do I freshen it up?
- Can you tell me how to get rid of a smell in the carpet with baking soda?
- The dog has made the rug smell. What can I do at home?

## situation
- I have baking soda, a sieve and a vacuum cleaner.
- I have about an hour.

## ground truth
```datalog
task(deodorize_carpet).
domain(deodorize_carpet, cleaning).
goal(deodorize_carpet, fresh_carpet).
needs(deodorize_carpet, baking_soda).
needs(deodorize_carpet, sieve).
needs(deodorize_carpet, vacuum_cleaner).
takes_time(deodorize_carpet, 30, 50, minute).
step(deodorize_carpet, deodorize_carpet_s1).
action(deodorize_carpet_s1, vacuum).
object_of(deodorize_carpet_s1, carpet).
uses(deodorize_carpet_s1, vacuum_cleaner).
role(deodorize_carpet_s1, vacuum_cleaner, clean).
purpose(deodorize_carpet_s1, loose_dirt_lifted).
takes_time(deodorize_carpet_s1, 5, 10, minute).
step(deodorize_carpet, deodorize_carpet_s2).
action(deodorize_carpet_s2, sprinkle).
object_of(deodorize_carpet_s2, baking_soda).
uses(deodorize_carpet_s2, sieve).
role(deodorize_carpet_s2, sieve, spread).
onto(deodorize_carpet_s2, carpet).
takes_time(deodorize_carpet_s2, 2, 3, minute).
step(deodorize_carpet, deodorize_carpet_s3).
action(deodorize_carpet_s3, leave).
object_of(deodorize_carpet_s3, baking_soda).
purpose(deodorize_carpet_s3, smell_absorbed).
takes_time(deodorize_carpet_s3, 15, 30, minute).
step(deodorize_carpet, deodorize_carpet_s4).
action(deodorize_carpet_s4, vacuum).
object_of(deodorize_carpet_s4, baking_soda).
uses(deodorize_carpet_s4, vacuum_cleaner).
role(deodorize_carpet_s4, vacuum_cleaner, clean).
takes_time(deodorize_carpet_s4, 5, 10, minute).
before(deodorize_carpet_s1, deodorize_carpet_s2).
before(deodorize_carpet_s2, deodorize_carpet_s3).
before(deodorize_carpet_s3, deodorize_carpet_s4).
```

## aliases
```datalog
task_alias(deodorize_carpet, "freshening a carpet").
task_alias(deodorize_carpet, "freshen a carpet").
task_alias(deodorize_carpet, "get rid of a smell in the carpet").
task_alias(deodorize_carpet, "deodorize the carpet").
alias(carpet, "rug").
alias(carpet, "living room carpet").
alias(vacuum, "vacuum up").
alias(vacuum_cleaner, "vacuum").
alias(loose_dirt_lifted, "lift loose dirt").
alias(smell_absorbed, "soak up the smell").
alias(keep_off, "keep people and pets off").
```

## teach
- To freshen a carpet you need baking soda, a sieve and a vacuum cleaner.
- Freshening a carpet takes 30 to 50 minutes.
- Step 1 of freshening a carpet: vacuum the carpet with the vacuum cleaner to lift loose dirt for 5 to 10 minutes.
- Step 2 of freshening a carpet: sprinkle baking soda evenly over the carpet through the sieve for 2 to 3 minutes.
- Step 3 of freshening a carpet: leave the baking soda to soak up the smell for 15 to 30 minutes.
- Step 4 of freshening a carpet: vacuum up the baking soda thoroughly with the vacuum cleaner for 5 to 10 minutes.

## checklist
- names what is needed: baking soda, a sieve and a vacuum cleaner
- gives the steps in order: vacuum carpet, sprinkle baking soda, leave baking soda, vacuum baking soda
- gives a total time of about 30 to 50 minutes
- closes politely

## variant
- change: strong smell: leave the baking soda overnight before vacuuming
- request: I tried a quick freshen-up but the carpet still smells. Is there something stronger I can do?
- situation: I have baking soda, a sieve and a vacuum cleaner, and nobody needs the room tonight.
- expect: The reply keeps the same steps but leaves the baking soda on overnight, keeping people and pets off the carpet, before vacuuming it up.

```datalog situation
has(user, baking_soda).
has(user, sieve).
has(user, vacuum_cleaner).
```

```datalog difference
change(deodorize_carpet_s3, takes_time, 15_30, overnight).
add_step(deodorize_carpet_s9, deodorize_carpet_s2).
step(deodorize_carpet, deodorize_carpet_s9).
action(deodorize_carpet_s9, keep_off).
object_of(deodorize_carpet_s9, carpet).
overlaps(deodorize_carpet_s9, deodorize_carpet_s3).
```
