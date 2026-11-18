---
id: fix_sticking_drawer
stage: tools
task: fix_sticking_drawer
schema: v3
tier: T
---
# Fix a sticking drawer

## requests
- My kitchen drawer sticks and I have to yank it open. How do I fix it?
- How do I make a wooden drawer slide smoothly again?
- The drawer in my old dresser jams halfway. Any fix?

## situation
- I have a cloth and a plain white candle.

## ground truth
```datalog
task(fix_sticking_drawer).
domain(fix_sticking_drawer, repair).
goal(fix_sticking_drawer, smooth_drawer).
needs(fix_sticking_drawer, cloth).
needs(fix_sticking_drawer, candle).
takes_time(fix_sticking_drawer, 15, 20, minute).
step(fix_sticking_drawer, fix_sticking_drawer_s1).
action(fix_sticking_drawer_s1, remove).
object_of(fix_sticking_drawer_s1, drawer).
step(fix_sticking_drawer, fix_sticking_drawer_s2).
action(fix_sticking_drawer_s2, inspect).
object_of(fix_sticking_drawer_s2, runner).
check(fix_sticking_drawer_s2, no_debris).
step(fix_sticking_drawer, fix_sticking_drawer_s3).
action(fix_sticking_drawer_s3, clean).
object_of(fix_sticking_drawer_s3, runner).
uses(fix_sticking_drawer_s3, cloth).
role(fix_sticking_drawer_s3, cloth, wipe).
step(fix_sticking_drawer, fix_sticking_drawer_s4).
action(fix_sticking_drawer_s4, rub).
object_of(fix_sticking_drawer_s4, runner).
uses(fix_sticking_drawer_s4, candle).
role(fix_sticking_drawer_s4, candle, lubricate).
purpose(fix_sticking_drawer_s4, less_friction).
step(fix_sticking_drawer, fix_sticking_drawer_s5).
action(fix_sticking_drawer_s5, replace).
object_of(fix_sticking_drawer_s5, drawer).
step(fix_sticking_drawer, fix_sticking_drawer_s6).
action(fix_sticking_drawer_s6, test).
object_of(fix_sticking_drawer_s6, drawer).
check(fix_sticking_drawer_s6, slides_smoothly).
before(fix_sticking_drawer_s1, fix_sticking_drawer_s2).
before(fix_sticking_drawer_s2, fix_sticking_drawer_s3).
before(fix_sticking_drawer_s3, fix_sticking_drawer_s4).
before(fix_sticking_drawer_s4, fix_sticking_drawer_s5).
before(fix_sticking_drawer_s5, fix_sticking_drawer_s6).
```

## aliases
```datalog
task_alias(fix_sticking_drawer, "fixing a sticking drawer").
task_alias(fix_sticking_drawer, "making a drawer slide").
task_alias(fix_sticking_drawer, "unsticking a drawer").
alias(runner, "the runners").
alias(candle, "candle wax").
alias(slides_smoothly, "it slides smoothly").
alias(less_friction, "it slides easier").
```

## teach
- To fix a sticking drawer you need a cloth and a plain candle.
- Fixing a sticking drawer takes 15 to 20 minutes.
- Step 1 of fixing a sticking drawer: remove the drawer.
- Step 2 of fixing a sticking drawer: inspect the runners and check there is nothing in the way.
- Step 3 of fixing a sticking drawer: clean the runners with the cloth.
- Step 4 of fixing a sticking drawer: rub the runners with the candle so they slide easier.
- Step 5 of fixing a sticking drawer: replace the drawer.
- Step 6 of fixing a sticking drawer: test that the drawer slides smoothly.

## checklist
- names what is needed: a cloth and a candle
- gives the steps in order: remove, inspect, clean, wax runners, replace, test

## variant
- change: no candle: rub the runners with a dry bar of soap
- request: I've no candles in the house. What else can I rub on a sticky drawer runner?
- situation: I have a cloth and a dry bar of soap.
- expect: The reply rubs the runners with the dry bar of soap instead of candle wax.

```datalog situation
has(user, cloth).
has(user, soap_bar).
lacks(user, candle).
```

```datalog difference
swap(fix_sticking_drawer_s4, candle, soap_bar).
extra_need(soap_bar).
no_need(candle).
```

## variant
- change: three sticking drawers: repeat for each
- request: All three drawers in my chest stick. How long will fixing them take?
- situation: I have a cloth and a candle and three drawers that stick.
- expect: The reply repeats the routine for each drawer and estimates about 45 to 60 minutes in total.

```datalog situation
has(user, cloth).
has(user, candle).
count(user, drawer, 3).
```

```datalog difference
change(fix_sticking_drawer, takes_time, 15_20, 45_60).
```
