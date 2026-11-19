---
id: plan_hiking_route
stage: procedures
task: plan_hiking_route
schema: v3
---
# Plan a hiking route

## requests
- I'm going hiking on Saturday. How should I plan the route and get ready?
- What's the best way to set up a day hike so I don't get caught out?
- Can you give me a checklist for planning a hike, from the weather to packing?

## situation
- I have a map, a phone, a water bottle, a backpack, a snack and boots.
- I have about an hour before I need to leave.

## ground truth
```datalog
task(plan_hiking_route).
domain(plan_hiking_route, planning).
goal(plan_hiking_route, plan_trip).
needs(plan_hiking_route, map).
needs(plan_hiking_route, water_bottle).
needs(plan_hiking_route, backpack).
needs(plan_hiking_route, snack).
needs(plan_hiking_route, phone).
needs(plan_hiking_route, boots).
takes_time(plan_hiking_route, 30, 45, minute).
step(plan_hiking_route, plan_hiking_route_s1).
action(plan_hiking_route_s1, check).
object_of(plan_hiking_route_s1, weather).
uses(plan_hiking_route_s1, phone).
role(plan_hiking_route_s1, phone, hold).
step(plan_hiking_route, plan_hiking_route_s2).
action(plan_hiking_route_s2, mark).
object_of(plan_hiking_route_s2, route).
uses(plan_hiking_route_s2, map).
role(plan_hiking_route_s2, map, spread).
step(plan_hiking_route, plan_hiking_route_s3).
action(plan_hiking_route_s3, pack).
object_of(plan_hiking_route_s3, water_bottle).
into(plan_hiking_route_s3, backpack).
uses(plan_hiking_route_s3, backpack).
role(plan_hiking_route_s3, backpack, contain).
step(plan_hiking_route, plan_hiking_route_s4).
action(plan_hiking_route_s4, pack).
object_of(plan_hiking_route_s4, snack).
into(plan_hiking_route_s4, backpack).
uses(plan_hiking_route_s4, backpack).
role(plan_hiking_route_s4, backpack, contain).
step(plan_hiking_route, plan_hiking_route_s5).
action(plan_hiking_route_s5, tell).
object_of(plan_hiking_route_s5, route).
purpose(plan_hiking_route_s5, someone_knows_route).
step(plan_hiking_route, plan_hiking_route_s6).
action(plan_hiking_route_s6, lace).
object_of(plan_hiking_route_s6, boots).
uses(plan_hiking_route_s6, boots).
role(plan_hiking_route_s6, boots, fasten).
overlaps(plan_hiking_route_s3, plan_hiking_route_s1).
before(plan_hiking_route_s1, plan_hiking_route_s2).
before(plan_hiking_route_s2, plan_hiking_route_s5).
before(plan_hiking_route_s3, plan_hiking_route_s4).
before(plan_hiking_route_s4, plan_hiking_route_s6).
```

## aliases
```datalog
task_alias(plan_hiking_route, "hiking route").
task_alias(plan_hiking_route, "day hike plan").
alias(route, "trail").
alias(weather, "forecast").
alias(water_bottle, "water bottle").
alias(backpack, "rucksack").
alias(boots, "hiking boots").
alias(lace, "lace up").
alias(spread, "spread out").
alias(someone_knows_route, "someone knows where you are").
```

## teach
- To plan a hiking route you need a map, a water bottle, a backpack, a snack, a phone and boots.
- Planning a hiking route takes 30 to 45 minutes.
- Step 1 of planning a hiking route: check the weather forecast on your phone.
- While step 1 runs, step 3 of planning a hiking route: pack the water bottle into the backpack.
- Step 2 of planning a hiking route: spread the map out and mark the route on it.
- Step 4 of planning a hiking route: pack a snack into the backpack.
- Step 5 of planning a hiking route: tell someone at home the route so they know where you are.
- Step 6 of planning a hiking route: lace up the boots.

## checklist
- names what is needed: a map, a water bottle, a backpack, a snack, a phone and boots
- gives the steps in order: check the weather, mark the route on the map, pack the water and snack, tell someone the route, lace up the boots
- says packing the water bottle can be done while checking the weather
- gives a total time of about 30 to 45 minutes

## variant
- change: only 15 minutes: skip marking the route on the map and plan in about 10 to 15 minutes.
- request: I only have about 15 minutes to plan this hike. What's the quick version?
- situation: I have a map, a phone, a water bottle, a backpack, a snack and boots, and 15 minutes.
- expect: The reply checks the weather, packs the water and snack, tells someone the route and laces the boots, without marking the map, in about 10 to 15 minutes.

```datalog situation
has(user, map).
has(user, phone).
has(user, water_bottle).
has(user, backpack).
has(user, snack).
has(user, boots).
time_limit(user, 15, minute).
```

```datalog difference
drop(plan_hiking_route_s2).
no_need(map).
change(plan_hiking_route, takes_time, 30_45, 10_15).
```

- change: add a first aid kit to the pack after the snack.
- request: I'm bringing a first aid kit too. Can you fit that into the plan?
- situation: I have a map, a phone, a water bottle, a backpack, a snack, boots and a first aid kit, and I want to bring the kit.
- expect: The reply adds packing the first aid kit into the backpack after the snack and before lacing the boots, keeping the other steps.

```datalog situation
has(user, map).
has(user, phone).
has(user, water_bottle).
has(user, backpack).
has(user, snack).
has(user, boots).
has(user, first_aid_kit).
wants(user, first_aid_kit).
```

```datalog difference
extra_need(first_aid_kit).
add_step(plan_hiking_route_s9, plan_hiking_route_s4).
step(plan_hiking_route, plan_hiking_route_s9).
action(plan_hiking_route_s9, pack).
object_of(plan_hiking_route_s9, first_aid_kit).
into(plan_hiking_route_s9, backpack).
uses(plan_hiking_route_s9, backpack).
role(plan_hiking_route_s9, backpack, contain).
before(plan_hiking_route_s9, plan_hiking_route_s6).
```
