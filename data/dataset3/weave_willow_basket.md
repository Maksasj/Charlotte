---
id: weave_willow_basket
stage: procedures
task: weave_willow_basket
schema: v3
---
# Weave a willow basket

## requests
- How would I go about making a basket out of willow rods? I've never woven anything before.
- Can you explain the steps for a simple willow basket, from cutting the rods to finishing it?
- I want to try basket weaving with willow. What do I need and what order do I do things in?

## situation
- I have willow rods, pruning shears, a bucket, twine and a tap for water.
- I have a free weekend and can leave the basket to dry for a couple of days.

## ground truth
```datalog
task(weave_willow_basket).
domain(weave_willow_basket, crafts).
goal(weave_willow_basket, willow_basket).
needs(weave_willow_basket, willow_rods).
needs(weave_willow_basket, pruning_shears).
needs(weave_willow_basket, bucket).
needs(weave_willow_basket, water).
needs(weave_willow_basket, twine).
takes_time(weave_willow_basket, 150, 180, minute).
step(weave_willow_basket, weave_willow_basket_s1).
action(weave_willow_basket_s1, cut).
object_of(weave_willow_basket_s1, willow_rods).
uses(weave_willow_basket_s1, pruning_shears).
role(weave_willow_basket_s1, pruning_shears, cut).
step(weave_willow_basket, weave_willow_basket_s2).
action(weave_willow_basket_s2, soak).
object_of(weave_willow_basket_s2, willow_rods).
uses(weave_willow_basket_s2, bucket).
role(weave_willow_basket_s2, bucket, contain).
uses(weave_willow_basket_s2, water).
role(weave_willow_basket_s2, water, cover).
purpose(weave_willow_basket_s2, rods_pliable).
takes_time(weave_willow_basket_s2, 30, 40, minute).
step(weave_willow_basket, weave_willow_basket_s3).
action(weave_willow_basket_s3, weave).
object_of(weave_willow_basket_s3, willow_rods).
takes_time(weave_willow_basket_s3, 60, 90, minute).
step(weave_willow_basket, weave_willow_basket_s4).
action(weave_willow_basket_s4, fasten).
object_of(weave_willow_basket_s4, rim).
uses(weave_willow_basket_s4, twine).
role(weave_willow_basket_s4, twine, fasten).
step(weave_willow_basket, weave_willow_basket_s5).
action(weave_willow_basket_s5, dry).
object_of(weave_willow_basket_s5, basket).
takes_time(weave_willow_basket_s5, 24, 48, hour).
before(weave_willow_basket_s1, weave_willow_basket_s2).
before(weave_willow_basket_s2, weave_willow_basket_s3).
before(weave_willow_basket_s3, weave_willow_basket_s4).
before(weave_willow_basket_s4, weave_willow_basket_s5).
```

## aliases
```datalog
task_alias(weave_willow_basket, "willow basket").
task_alias(weave_willow_basket, "basket weaving").
task_alias(weave_willow_basket, "weaving a willow basket").
alias(willow_rods, "willow withies").
alias(pruning_shears, "secateurs").
alias(twine, "string").
alias(rods_pliable, "rods bend easily").
alias(willow_basket, "willow basket").
```

## teach
- To weave a willow basket you need willow rods, pruning shears, a bucket, water and twine.
- Weaving a willow basket takes 150 to 180 minutes of work, not counting drying.
- Step 1 of weaving a willow basket: cut the willow rods to length with the pruning shears.
- Step 2 of weaving a willow basket: soak the willow rods in a bucket of water for 30 to 40 minutes so they become pliable.
- Step 3 of weaving a willow basket: weave the rods into a basket base, which takes 60 to 90 minutes.
- Step 4 of weaving a willow basket: fasten the rim with twine.
- Step 5 of weaving a willow basket: leave the basket to dry for 24 to 48 hours.

## checklist
- names the willow rods, pruning shears, bucket, water and twine
- gives the steps in order: cut, soak, weave, fasten the rim, dry
- says the soaking is needed so the rods bend without snapping
- gives the working time of about 150 to 180 minutes, plus drying time

## variant
- change: add a handle made from twine before the rim is fastened
- request: Can I put a handle on the basket while I weave it? I have spare twine.
- situation: I have willow rods, twine and a wish for a handle.
- expect: The reply keeps the base steps and adds a handle made with twine after weaving and before the rim is fastened.

```datalog situation
has(user, willow_rods).
wants(user, handle).
```

```datalog difference
add_step(weave_willow_basket_s9, weave_willow_basket_s3).
step(weave_willow_basket, weave_willow_basket_s9).
action(weave_willow_basket_s9, fasten).
object_of(weave_willow_basket_s9, handle).
uses(weave_willow_basket_s9, twine).
role(weave_willow_basket_s9, twine, fasten).
takes_time(weave_willow_basket_s9, 15, 20, minute).
before(weave_willow_basket_s3, weave_willow_basket_s9).
before(weave_willow_basket_s9, weave_willow_basket_s4).
```

- change: make the basket waterproof by coating it with linseed oil after it dries
- request: I want the finished basket to be waterproof for outdoor use. What else should I add?
- situation: I have willow rods, pruning shears, a bucket, water, twine and linseed oil, and I want the basket to be waterproof.
- expect: The reply keeps the base steps and adds a coating of linseed oil on the dried basket as a final step.

```datalog situation
has(user, willow_rods).
has(user, linseed_oil).
wants(user, waterproof_basket).
```

```datalog difference
add_step(weave_willow_basket_s9, weave_willow_basket_s5).
step(weave_willow_basket, weave_willow_basket_s9).
action(weave_willow_basket_s9, coat).
object_of(weave_willow_basket_s9, basket).
uses(weave_willow_basket_s9, linseed_oil).
role(weave_willow_basket_s9, linseed_oil, coat).
takes_time(weave_willow_basket_s9, 20, 30, minute).
before(weave_willow_basket_s5, weave_willow_basket_s9).
extra_need(linseed_oil).
```
