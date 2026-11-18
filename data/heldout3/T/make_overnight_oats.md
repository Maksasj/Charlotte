---
id: make_overnight_oats
stage: procedures
task: make_overnight_oats
schema: v3
tier: T
---
# Make overnight oats

## requests
- What's the deal with overnight oats? How do I make them?
- I need a breakfast I can make the night before and grab in the morning. Ideas and how?
- How do I soak oats in the fridge so they're ready for breakfast?

## situation
- I have rolled oats, milk, plain yogurt, honey, a jar with a lid, a spoon and a fridge.

## ground truth
```datalog
task(make_overnight_oats).
domain(make_overnight_oats, cooking).
goal(make_overnight_oats, overnight_oats).
goal(make_overnight_oats, breakfast).
needs(make_overnight_oats, rolled_oats).
needs(make_overnight_oats, milk).
needs(make_overnight_oats, yogurt).
needs(make_overnight_oats, honey).
needs(make_overnight_oats, jar).
needs(make_overnight_oats, spoon).
needs(make_overnight_oats, fridge).
serves(make_overnight_oats, 1).
takes_time(make_overnight_oats, 8, 12, hour).
step(make_overnight_oats, make_overnight_oats_s1).
action(make_overnight_oats_s1, put).
object_of(make_overnight_oats_s1, rolled_oats).
into(make_overnight_oats_s1, jar).
amount(make_overnight_oats_s1, rolled_oats, 50, gram).
step(make_overnight_oats, make_overnight_oats_s2).
action(make_overnight_oats_s2, pour).
object_of(make_overnight_oats_s2, milk).
into(make_overnight_oats_s2, jar).
amount(make_overnight_oats_s2, milk, 100, millilitre).
step(make_overnight_oats, make_overnight_oats_s3).
action(make_overnight_oats_s3, add).
object_of(make_overnight_oats_s3, yogurt).
into(make_overnight_oats_s3, jar).
amount(make_overnight_oats_s3, yogurt, 2, tablespoon).
step(make_overnight_oats, make_overnight_oats_s4).
action(make_overnight_oats_s4, stir).
object_of(make_overnight_oats_s4, mixture).
uses(make_overnight_oats_s4, spoon).
role(make_overnight_oats_s4, spoon, stir).
uses(make_overnight_oats_s4, honey).
role(make_overnight_oats_s4, honey, sweeten).
step(make_overnight_oats, make_overnight_oats_s5).
action(make_overnight_oats_s5, chill).
object_of(make_overnight_oats_s5, jar).
uses(make_overnight_oats_s5, fridge).
role(make_overnight_oats_s5, fridge, cool).
takes_time(make_overnight_oats_s5, 8, 12, hour).
achieves(make_overnight_oats_s5, soft).
before(make_overnight_oats_s1, make_overnight_oats_s2).
before(make_overnight_oats_s2, make_overnight_oats_s3).
before(make_overnight_oats_s3, make_overnight_oats_s4).
before(make_overnight_oats_s4, make_overnight_oats_s5).
```

## aliases
```datalog
task_alias(make_overnight_oats, "overnight oats").
task_alias(make_overnight_oats, "soaked oats").
task_alias(make_overnight_oats, "oats for breakfast").
alias(rolled_oats, "oats").
alias(jar, "jar with a lid").
alias(mixture, "oats").
alias(chill, "leave in the fridge").
```

## teach
- To make overnight oats you need rolled oats, milk, yogurt, honey, a jar, a spoon and a fridge.
- Making overnight oats takes 8 to 12 hours, mostly waiting.
- Step 1 of making overnight oats: put 50 grams of rolled oats into the jar.
- Step 2 of making overnight oats: pour 100 millilitres of milk into the jar.
- Step 3 of making overnight oats: add 2 tablespoons of yogurt to the jar.
- Step 4 of making overnight oats: stir it all with the spoon and sweeten with honey.
- Step 5 of making overnight oats: chill the jar in the fridge for 8 to 12 hours until the oats are soft.

## checklist
- names what is needed, including oats, milk, yogurt and a jar
- gives the steps in order: oats, milk, yogurt, stir with honey, chill
- says it rests in the fridge overnight, about 8 to 12 hours

## variant
- change: only 1 hour: use hot milk and leave 30 minutes
- request: I forgot to make overnight oats last night. Can I get something like them ready within an hour?
- situation: I have oats, milk, yogurt, honey, a jar, a spoon, a fridge and a microwave, and about an hour.
- expect: The reply warms the milk in the microwave first so the oats soften in about 30 minutes rather than overnight.

```datalog situation
has(user, rolled_oats).
has(user, milk).
has(user, yogurt).
has(user, honey).
has(user, jar).
has(user, spoon).
has(user, fridge).
has(user, microwave).
time_limit(user, 1, hour).
```

```datalog difference
extra_need(microwave).
add_step(make_overnight_oats_s9, start).
step(make_overnight_oats, make_overnight_oats_s9).
action(make_overnight_oats_s9, warm).
object_of(make_overnight_oats_s9, milk).
uses(make_overnight_oats_s9, microwave).
role(make_overnight_oats_s9, microwave, heat).
before(make_overnight_oats_s9, make_overnight_oats_s2).
change(make_overnight_oats_s5, takes_time, 8_12_hour, 30_30_minute).
change(make_overnight_oats, takes_time, 8_12_hour, 35_40_minute).
```

## variant
- change: vegan: plant milk and no yogurt or honey, use maple syrup
- request: I'm vegan. How would overnight oats work for me?
- situation: I have oats, soy milk, maple syrup, a jar, a spoon and a fridge. I don't eat animal products.
- expect: The reply uses soy milk, leaves out the yogurt and sweetens with maple syrup instead of honey.

```datalog situation
has(user, rolled_oats).
has(user, soy_milk).
has(user, maple_syrup).
has(user, jar).
has(user, spoon).
has(user, fridge).
excludes(user, animal_product).
```

```datalog difference
change(make_overnight_oats_s2, object_of, milk, soy_milk).
drop(make_overnight_oats_s3).
swap(make_overnight_oats_s4, honey, maple_syrup).
extra_need(soy_milk).
extra_need(maple_syrup).
no_need(milk).
no_need(yogurt).
no_need(honey).
```
