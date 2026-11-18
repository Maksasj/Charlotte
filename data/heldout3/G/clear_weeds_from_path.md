---
id: clear_weeds_from_path
stage: household
task: clear_weeds_from_path
schema: v3
tier: G
---
# Clear weeds from a path

## requests
- Weeds are sprouting up between all the paving slabs on my garden path. How do I get them out?
- What's the best way to clear weeds from the cracks in a patio?
- My gravel path is turning green with weeds. How do I sort it out?

## situation
- I have a weeding knife, a stiff broom, a spray bottle of path weedkiller and gloves.

## ground truth
```datalog
task(clear_weeds_from_path).
domain(clear_weeds_from_path, gardening).
goal(clear_weeds_from_path, clear_path).
needs(clear_weeds_from_path, glove).
needs(clear_weeds_from_path, weeding_knife).
needs(clear_weeds_from_path, broom).
needs(clear_weeds_from_path, weedkiller).
takes_time(clear_weeds_from_path, 30, 45, minute).
step(clear_weeds_from_path, clear_weeds_from_path_s1).
action(clear_weeds_from_path_s1, put_on).
object_of(clear_weeds_from_path_s1, glove).
uses(clear_weeds_from_path_s1, glove).
role(clear_weeds_from_path_s1, glove, protect).
step(clear_weeds_from_path, clear_weeds_from_path_s2).
action(clear_weeds_from_path_s2, scrape).
object_of(clear_weeds_from_path_s2, weed).
from(clear_weeds_from_path_s2, crack).
uses(clear_weeds_from_path_s2, weeding_knife).
role(clear_weeds_from_path_s2, weeding_knife, cut).
takes_time(clear_weeds_from_path_s2, 20, 30, minute).
step(clear_weeds_from_path, clear_weeds_from_path_s3).
action(clear_weeds_from_path_s3, sweep).
object_of(clear_weeds_from_path_s3, path).
uses(clear_weeds_from_path_s3, broom).
role(clear_weeds_from_path_s3, broom, clean).
step(clear_weeds_from_path, clear_weeds_from_path_s4).
action(clear_weeds_from_path_s4, spray).
object_of(clear_weeds_from_path_s4, crack).
uses(clear_weeds_from_path_s4, weedkiller).
role(clear_weeds_from_path_s4, weedkiller, kill).
condition(clear_weeds_from_path_s4, dry_still_day).
warning(clear_weeds_from_path_s4, harms_nearby_plants).
purpose(clear_weeds_from_path_s4, stop_regrowth).
before(clear_weeds_from_path_s1, clear_weeds_from_path_s2).
before(clear_weeds_from_path_s2, clear_weeds_from_path_s3).
before(clear_weeds_from_path_s3, clear_weeds_from_path_s4).
```

## aliases
```datalog
task_alias(clear_weeds_from_path, "clearing weeds from a path").
task_alias(clear_weeds_from_path, "weed the patio").
task_alias(clear_weeds_from_path, "get weeds out from between the slabs").
alias(weed, "weeds").
alias(crack, "cracks").
alias(crack, "gaps between the paving slabs").
alias(path, "patio").
alias(path, "garden path").
alias(weeding_knife, "patio knife").
alias(broom, "stiff broom").
alias(weedkiller, "path weedkiller").
alias(glove, "gloves").
alias(kettle, "kettle").
alias(boiling_water, "boiling water").
alias(harms_nearby_plants, "drift can kill plants nearby").
```

## teach
- To clear weeds from a path you need gloves, a weeding knife, a stiff broom and path weedkiller.
- Clearing weeds from a path takes 30 to 45 minutes.
- Step 1 of clearing weeds from a path: put on the gloves to protect your hands.
- Step 2 of clearing weeds from a path: scrape the weeds out of the cracks with the weeding knife, for 20 to 30 minutes.
- Step 3 of clearing weeds from a path: sweep the path with the stiff broom to clean it.
- Step 4 of clearing weeds from a path: on a dry, still day, spray the cracks with weedkiller to kill the roots and stop regrowth, taking care because drift can kill plants nearby.

## checklist
- names what is needed: gloves, a weeding knife, a broom and weedkiller
- gives the steps in order: gloves, scrape, sweep, spray
- says to spray on a dry, still day and mind nearby plants
- gives a total time of about 30 to 45 minutes

## variant
- change: no weedkiller wanted: pour boiling water into the cracks instead of spraying
- request: I don't want any weedkiller near the path because my dog walks on it. How else can I stop the weeds coming back?
- situation: I have gloves, a weeding knife, a stiff broom and a kettle, and I won't use weedkiller.
- expect: The reply leaves out the weedkiller spray and pours boiling water from the kettle into the cracks to kill the roots, with care not to splash.

```datalog situation
excludes(user, weedkiller).
has(user, glove).
has(user, weeding_knife).
has(user, broom).
has(user, kettle).
```

```datalog difference
no_need(weedkiller).
extra_need(kettle).
drop(clear_weeds_from_path_s4).
add_step(clear_weeds_from_path_s9, clear_weeds_from_path_s3).
step(clear_weeds_from_path, clear_weeds_from_path_s9).
action(clear_weeds_from_path_s9, pour).
object_of(clear_weeds_from_path_s9, boiling_water).
onto(clear_weeds_from_path_s9, crack).
uses(clear_weeds_from_path_s9, kettle).
role(clear_weeds_from_path_s9, kettle, pour).
warning(clear_weeds_from_path_s9, scald).
purpose(clear_weeds_from_path_s9, stop_regrowth).
before(clear_weeds_from_path_s3, clear_weeds_from_path_s9).
```

## variant
- change: only 15 minutes: a short scrape, skip sweeping
- request: Visitors are coming in a quarter of an hour and the patio looks awful. What's the quickest way to deal with the weeds?
- situation: I have gloves, a weeding knife, a stiff broom and path weedkiller, and about 15 minutes.
- expect: The reply keeps scraping short, leaves out sweeping, and sprays the cracks, so the job fits in about 15 minutes.

```datalog situation
has(user, glove).
has(user, weeding_knife).
has(user, broom).
has(user, weedkiller).
time_limit(user, 15, minute).
```

```datalog difference
drop(clear_weeds_from_path_s3).
no_need(broom).
change(clear_weeds_from_path_s2, takes_time, 20_30, 10_10).
change(clear_weeds_from_path, takes_time, 30_45, 15_15).
```
