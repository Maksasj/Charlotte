---
id: clean_range_hood_filter
stage: procedures
task: clean_range_hood_filter
schema: v3
---
# Clean a range hood filter

## requests
- How do I clean the metal filter from my range hood? It's really greasy.
- What's the best way to degrease a range hood filter?
- Can you tell me the steps for cleaning the grease filter above my stove?

## situation
- I have a greasy range hood filter, a sink, hot water, dish soap, a sponge and a towel.
- I want it clean before the weekend.

## ground truth
```datalog
task(clean_range_hood_filter).
domain(clean_range_hood_filter, cleaning).
goal(clean_range_hood_filter, clean_range_hood).
needs(clean_range_hood_filter, sink).
needs(clean_range_hood_filter, hot_water).
needs(clean_range_hood_filter, dish_soap).
needs(clean_range_hood_filter, sponge).
needs(clean_range_hood_filter, towel).
takes_time(clean_range_hood_filter, 30, 45, minute).
step(clean_range_hood_filter, clean_range_hood_filter_s1).
action(clean_range_hood_filter_s1, remove).
object_of(clean_range_hood_filter_s1, range_hood_filter).
takes_time(clean_range_hood_filter_s1, 5, 5, minute).
step(clean_range_hood_filter, clean_range_hood_filter_s2).
action(clean_range_hood_filter_s2, soak).
object_of(clean_range_hood_filter_s2, range_hood_filter).
uses(clean_range_hood_filter_s2, sink).
role(clean_range_hood_filter_s2, sink, contain).
uses(clean_range_hood_filter_s2, hot_water).
role(clean_range_hood_filter_s2, hot_water, clean).
purpose(clean_range_hood_filter_s2, grease_loosens).
takes_time(clean_range_hood_filter_s2, 15, 20, minute).
step(clean_range_hood_filter, clean_range_hood_filter_s3).
action(clean_range_hood_filter_s3, scrub).
object_of(clean_range_hood_filter_s3, range_hood_filter).
uses(clean_range_hood_filter_s3, sponge).
role(clean_range_hood_filter_s3, sponge, scrub).
uses(clean_range_hood_filter_s3, dish_soap).
role(clean_range_hood_filter_s3, dish_soap, clean).
purpose(clean_range_hood_filter_s3, grease_removed).
takes_time(clean_range_hood_filter_s3, 5, 10, minute).
step(clean_range_hood_filter, clean_range_hood_filter_s4).
action(clean_range_hood_filter_s4, rinse).
object_of(clean_range_hood_filter_s4, range_hood_filter).
uses(clean_range_hood_filter_s4, hot_water).
role(clean_range_hood_filter_s4, hot_water, clean).
takes_time(clean_range_hood_filter_s4, 3, 5, minute).
step(clean_range_hood_filter, clean_range_hood_filter_s5).
action(clean_range_hood_filter_s5, dry).
object_of(clean_range_hood_filter_s5, range_hood_filter).
uses(clean_range_hood_filter_s5, towel).
role(clean_range_hood_filter_s5, towel, dry).
check(clean_range_hood_filter_s5, filter_dry).
takes_time(clean_range_hood_filter_s5, 3, 5, minute).
before(clean_range_hood_filter_s1, clean_range_hood_filter_s2).
before(clean_range_hood_filter_s2, clean_range_hood_filter_s3).
before(clean_range_hood_filter_s3, clean_range_hood_filter_s4).
before(clean_range_hood_filter_s4, clean_range_hood_filter_s5).
```

## aliases
```datalog
task_alias(clean_range_hood_filter, "cleaning a range hood filter").
task_alias(clean_range_hood_filter, "degreasing a range hood filter").
alias(range_hood_filter, "range hood filter").
alias(range_hood_filter, "grease filter").
alias(dish_soap, "washing up liquid").
alias(hot_water, "hot water").
alias(grease_loosens, "grease loosens").
alias(grease_removed, "grease comes off").
alias(filter_dry, "filter is dry").
```

## teach
- To clean a range hood filter you need a sink, hot water, dish soap, a sponge and a towel.
- Cleaning a range hood filter takes 30 to 45 minutes.
- Step 1 of cleaning a range hood filter: take the filter out of the hood, which takes about 5 minutes.
- Step 2 of cleaning a range hood filter: soak the filter in hot water in the sink for 15 to 20 minutes so the grease loosens.
- Step 3 of cleaning a range hood filter: scrub the filter with the sponge and dish soap for 5 to 10 minutes so the grease comes off.
- Step 4 of cleaning a range hood filter: rinse the filter in hot water for 3 to 5 minutes.
- Step 5 of cleaning a range hood filter: dry the filter with the towel for 3 to 5 minutes until the filter is dry.

## checklist
- names what is needed, including a sink, hot water, dish soap, a sponge and a towel
- gives the steps in order: remove the filter, soak in hot water, scrub with soap and sponge, rinse, dry
- says the soaking is what loosens the grease before scrubbing
- gives a total time of about 30 to 45 minutes

## variant
- change: the sink is broken, so soak the filter in a large basin of hot water instead
- request: My kitchen sink is broken. Can I still clean the range hood filter?
- situation: I have a large basin, hot water, dish soap, a sponge and a towel, but no working sink.
- expect: The reply soaks the filter in the basin of hot water instead of the sink, then scrubs, rinses and dries it as usual.

```datalog situation
lacks(user, sink).
has(user, basin).
has(user, hot_water).
has(user, dish_soap).
has(user, sponge).
has(user, towel).
```

```datalog difference
swap(clean_range_hood_filter_s2, sink, basin).
extra_need(basin).
no_need(sink).
```

## variant
- change: no dish soap, so scrub with a baking soda paste instead
- request: I've run out of dish soap. Can I clean the filter with something else?
- situation: I have a sink, hot water, a sponge, a towel and baking soda, but no dish soap.
- expect: The reply scrubs the filter with a baking soda paste in place of dish soap, then rinses and dries it.

```datalog situation
lacks(user, dish_soap).
has(user, sink).
has(user, hot_water).
has(user, sponge).
has(user, towel).
has(user, baking_soda).
```

```datalog difference
swap(clean_range_hood_filter_s3, dish_soap, baking_soda).
extra_need(baking_soda).
no_need(dish_soap).
```
