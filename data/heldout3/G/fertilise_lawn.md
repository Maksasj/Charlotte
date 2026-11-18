---
id: fertilise_lawn
stage: household
task: fertilise_lawn
schema: v3
tier: G
---
# Fertilise a lawn

## requests
- My grass looks pale and thin this spring. How do I give it a feed?
- What's the right way to put lawn fertiliser down with a spreader?
- How should I fertilise the lawn so I don't scorch it?

## situation
- I have a lawn mower, a box of granular lawn feed, a push spreader and a garden hose.

## ground truth
```datalog
task(fertilise_lawn).
domain(fertilise_lawn, gardening).
goal(fertilise_lawn, green_lawn).
needs(fertilise_lawn, lawn_mower).
needs(fertilise_lawn, lawn_feed).
needs(fertilise_lawn, spreader).
needs(fertilise_lawn, hose).
takes_time(fertilise_lawn, 40, 60, minute).
step(fertilise_lawn, fertilise_lawn_s1).
action(fertilise_lawn_s1, mow).
object_of(fertilise_lawn_s1, lawn).
uses(fertilise_lawn_s1, lawn_mower).
role(fertilise_lawn_s1, lawn_mower, cut).
takes_time(fertilise_lawn_s1, 20, 30, minute).
step(fertilise_lawn, fertilise_lawn_s2).
action(fertilise_lawn_s2, pour).
object_of(fertilise_lawn_s2, lawn_feed).
into(fertilise_lawn_s2, spreader).
step(fertilise_lawn, fertilise_lawn_s3).
action(fertilise_lawn_s3, spread).
object_of(fertilise_lawn_s3, lawn_feed).
onto(fertilise_lawn_s3, lawn).
uses(fertilise_lawn_s3, spreader).
role(fertilise_lawn_s3, spreader, spread).
amount(fertilise_lawn_s3, lawn_feed, 35, gram).
avoid(fertilise_lawn_s3, overlap, strip).
warning(fertilise_lawn_s3, scorched_grass).
takes_time(fertilise_lawn_s3, 10, 15, minute).
step(fertilise_lawn, fertilise_lawn_s4).
action(fertilise_lawn_s4, water).
object_of(fertilise_lawn_s4, lawn).
uses(fertilise_lawn_s4, hose).
role(fertilise_lawn_s4, hose, pour).
condition(fertilise_lawn_s4, no_rain_forecast).
purpose(fertilise_lawn_s4, feed_washed_in).
before(fertilise_lawn_s1, fertilise_lawn_s2).
before(fertilise_lawn_s2, fertilise_lawn_s3).
before(fertilise_lawn_s3, fertilise_lawn_s4).
```

## aliases
```datalog
task_alias(fertilise_lawn, "fertilising a lawn").
task_alias(fertilise_lawn, "feed the lawn").
task_alias(fertilise_lawn, "give the grass a feed").
alias(lawn_feed, "lawn fertiliser").
alias(lawn_feed, "granular lawn feed").
alias(spreader, "push spreader").
alias(lawn, "grass").
alias(hose, "garden hose").
alias(strip, "strips").
alias(scorched_grass, "burns the grass").
alias(seaweed_feed, "organic seaweed lawn feed").
alias(seaweed_feed, "organic feed").
alias(chemical_feed, "chemical fertiliser").
```

## teach
- To fertilise a lawn you need a lawn mower, lawn feed, a spreader and a hose.
- Fertilising a lawn takes 40 to 60 minutes.
- Step 1 of fertilising a lawn: mow the lawn with the lawn mower, for 20 to 30 minutes.
- Step 2 of fertilising a lawn: pour the lawn feed into the spreader.
- Step 3 of fertilising a lawn: spread the feed over the lawn with the spreader at about 35 grams per square metre, for 10 to 15 minutes, and don't overlap the strips or it burns the grass.
- Step 4 of fertilising a lawn: if no rain is forecast, water the lawn with the hose to wash the feed in.

## checklist
- names what is needed: a mower, lawn feed, a spreader and a hose
- gives the steps in order: mow, fill spreader, spread, water in
- gives the rate of about 35 grams per square metre and warns against overlapping
- gives a total time of about 40 to 60 minutes

## variant
- change: no chemical fertiliser wanted: use an organic seaweed feed instead
- request: The kids and the dog play on the grass all the time, so I don't want chemical fertiliser on it. How do I feed the lawn?
- situation: I have a lawn mower, a spreader, a hose and a tub of organic seaweed lawn feed, and I want to avoid chemical feed.
- expect: The reply uses the organic seaweed feed in place of the chemical feed and keeps the other steps.

```datalog situation
excludes(user, chemical_feed).
has(user, seaweed_feed).
has(user, lawn_mower).
has(user, spreader).
has(user, hose).
```

```datalog difference
no_need(lawn_feed).
extra_need(seaweed_feed).
change(fertilise_lawn_s2, object_of, lawn_feed, seaweed_feed).
change(fertilise_lawn_s3, object_of, lawn_feed, seaweed_feed).
```

## variant
- change: only 20 minutes and the grass is short: skip mowing
- request: I mowed a couple of days ago and now I've got about 20 minutes. Can I get the feed down?
- situation: I have a lawn mower, lawn feed, a spreader and a hose, and about 20 minutes. The grass was cut recently.
- expect: The reply leaves out mowing, fills the spreader, spreads the feed and waters it in, all in about 20 minutes.

```datalog situation
has(user, lawn_mower).
has(user, lawn_feed).
has(user, spreader).
has(user, hose).
time_limit(user, 20, minute).
```

```datalog difference
drop(fertilise_lawn_s1).
no_need(lawn_mower).
change(fertilise_lawn, takes_time, 40_60, 15_20).
```
