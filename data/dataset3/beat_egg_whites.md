---
id: beat_egg_whites
stage: procedures
task: beat_egg_whites
schema: v3
---
# Beat egg whites

## requests
- How do I whip egg whites until they hold stiff peaks?
- What's the best way to beat egg whites by hand with a whisk?
- Can you walk me through beating egg whites for a pavlova topping?

## situation
- I have an egg, a bowl and a whisk.
- I need stiff egg whites for a meringue topping.

## ground truth
```datalog
task(beat_egg_whites).
domain(beat_egg_whites, cooking).
goal(beat_egg_whites, stiff_egg_whites).
needs(beat_egg_whites, egg).
needs(beat_egg_whites, bowl).
needs(beat_egg_whites, whisk).
takes_time(beat_egg_whites, 5, 10, minute).
step(beat_egg_whites, beat_egg_whites_s1).
action(beat_egg_whites_s1, separate).
object_of(beat_egg_whites_s1, egg).
uses(beat_egg_whites_s1, bowl).
role(beat_egg_whites_s1, bowl, hold).
purpose(beat_egg_whites_s1, keep_yolk_out).
step(beat_egg_whites, beat_egg_whites_s2).
action(beat_egg_whites_s2, beat).
object_of(beat_egg_whites_s2, egg_white).
uses(beat_egg_whites_s2, whisk).
role(beat_egg_whites_s2, whisk, beat).
setting(beat_egg_whites_s2, speed, medium, level).
until(beat_egg_whites_s2, foamy).
takes_time(beat_egg_whites_s2, 2, 4, minute).
step(beat_egg_whites, beat_egg_whites_s3).
action(beat_egg_whites_s3, beat).
object_of(beat_egg_whites_s3, egg_white).
uses(beat_egg_whites_s3, whisk).
role(beat_egg_whites_s3, whisk, beat).
until(beat_egg_whites_s3, stiff_peaks).
check(beat_egg_whites_s3, stiff_peaks).
warning(beat_egg_whites_s3, overbeat).
takes_time(beat_egg_whites_s3, 3, 6, minute).
before(beat_egg_whites_s1, beat_egg_whites_s2).
before(beat_egg_whites_s2, beat_egg_whites_s3).
```

## aliases
```datalog
task_alias(beat_egg_whites, "whipping egg whites").
task_alias(beat_egg_whites, "whipped egg whites").
alias(egg_white, "egg whites").
alias(stiff_peaks, "stiff peaks").
alias(whisk, "balloon whisk").
alias(bowl, "mixing bowl").
alias(overbeat, "overbeating").
alias(keep_yolk_out, "keeps the yolk out").
```

## teach
- To beat egg whites you need an egg, a bowl and a whisk.
- Beating egg whites takes 5 to 10 minutes.
- Step 1 of beating egg whites: separate the egg, holding the bowl under it so the yolk stays out.
- Step 2 of beating egg whites: beat the whites with the whisk at medium speed for 2 to 4 minutes until they are foamy.
- Step 3 of beating egg whites: beat for 3 to 6 minutes until stiff peaks form, stopping before the whites look dry.

## checklist
- names what is needed: an egg, a bowl and a whisk
- gives the steps in order: separate the egg, beat to foamy, beat to stiff peaks
- says to stop beating before the whites look dry
- gives a total time of about 5 to 10 minutes

## variant
- change: no whisk, so use a fork instead
- request: I don't have a whisk. Can I still whip egg whites with a fork?
- situation: I have an egg, a bowl and a fork, but no whisk.
- expect: The reply uses the fork in place of the whisk for both beating steps and says it takes longer, about 8 to 12 minutes.

```datalog situation
has(user, egg).
has(user, bowl).
has(user, fork).
lacks(user, whisk).
```

```datalog difference
no_need(whisk).
extra_need(fork).
swap(beat_egg_whites_s2, whisk, fork).
swap(beat_egg_whites_s3, whisk, fork).
change(beat_egg_whites, takes_time, 5_10, 8_12).
```

## variant
- change: add sugar gradually while the whites are beating
- request: I want to sweeten the meringue. Can I add sugar while I beat the whites?
- situation: I have an egg, a bowl, a whisk and some sugar, and I want the meringue sweetened.
- expect: The reply adds a step to pour the sugar into the bowl after the whites turn foamy, then continues beating to stiff peaks.

```datalog situation
has(user, egg).
has(user, bowl).
has(user, whisk).
has(user, sugar).
wants(user, sugar).
```

```datalog difference
extra_need(sugar).
add_step(beat_egg_whites_s9, beat_egg_whites_s2).
step(beat_egg_whites, beat_egg_whites_s9).
action(beat_egg_whites_s9, pour).
object_of(beat_egg_whites_s9, sugar).
into(beat_egg_whites_s9, bowl).
uses(beat_egg_whites_s9, sugar).
role(beat_egg_whites_s9, sugar, pour).
takes_time(beat_egg_whites_s9, 1, 1, minute).
before(beat_egg_whites_s2, beat_egg_whites_s9).
before(beat_egg_whites_s9, beat_egg_whites_s3).
```
