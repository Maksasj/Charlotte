---
id: dye_hair_at_home
stage: procedures
task: dye_hair_at_home
schema: v3
---
# Dye hair at home

## requests
- I want to colour my own hair at home. What do I need and how do I do it?
- Can you explain how to dye your hair yourself with a box kit?
- What's the right way to apply hair dye at home so the colour comes out even?

## situation
- I have hair dye, developer, a mixing bowl, a tint brush and gloves.
- I have about an hour free this evening.

## ground truth
```datalog
task(dye_hair_at_home).
domain(dye_hair_at_home, personal_care).
goal(dye_hair_at_home, hair_coloured).
needs(dye_hair_at_home, hair_dye).
needs(dye_hair_at_home, developer).
needs(dye_hair_at_home, bowl).
needs(dye_hair_at_home, brush).
needs(dye_hair_at_home, gloves).
takes_time(dye_hair_at_home, 45, 60, minute).
step(dye_hair_at_home, dye_hair_at_home_s1).
action(dye_hair_at_home_s1, mix).
object_of(dye_hair_at_home_s1, hair_dye).
uses(dye_hair_at_home_s1, developer).
role(dye_hair_at_home_s1, developer, mix).
uses(dye_hair_at_home_s1, bowl).
role(dye_hair_at_home_s1, bowl, contain).
takes_time(dye_hair_at_home_s1, 2, 3, minute).
step(dye_hair_at_home, dye_hair_at_home_s2).
action(dye_hair_at_home_s2, spread).
object_of(dye_hair_at_home_s2, hair_dye).
into(dye_hair_at_home_s2, hair).
uses(dye_hair_at_home_s2, brush).
role(dye_hair_at_home_s2, brush, spread).
step(dye_hair_at_home, dye_hair_at_home_s3).
action(dye_hair_at_home_s3, leave).
object_of(dye_hair_at_home_s3, hair).
takes_time(dye_hair_at_home_s3, 30, 35, minute).
step(dye_hair_at_home, dye_hair_at_home_s4).
action(dye_hair_at_home_s4, rinse).
object_of(dye_hair_at_home_s4, hair).
until(dye_hair_at_home_s4, clear).
before(dye_hair_at_home_s1, dye_hair_at_home_s2).
before(dye_hair_at_home_s2, dye_hair_at_home_s3).
before(dye_hair_at_home_s3, dye_hair_at_home_s4).
```

## aliases
```datalog
task_alias(dye_hair_at_home, "dyeing hair at home").
task_alias(dye_hair_at_home, "colouring hair at home").
alias(hair_dye, "hair dye").
alias(developer, "developer").
alias(bowl, "mixing bowl").
alias(brush, "tint brush").
alias(gloves, "gloves").
alias(clear, "runs clear").
alias(sponge, "clean sponge").
alias(conditioner, "conditioner").
```

## teach
- To dye your hair at home you need hair dye, developer, a bowl, a brush and gloves.
- Dyeing your hair at home takes 45 to 60 minutes.
- Step 1 of dyeing your hair at home: mix the hair dye with the developer in a bowl, for 2 to 3 minutes.
- Step 2 of dyeing your hair at home: spread the dye through the hair with the brush.
- Step 3 of dyeing your hair at home: leave the dye on the hair for 30 to 35 minutes.
- Step 4 of dyeing your hair at home: rinse the hair until it runs clear.

## checklist
- names what is needed, including hair dye, developer, a bowl, a brush and gloves
- gives the steps in order: mix the dye, spread it through the hair, leave it on, rinse until clear
- gives a total time of about 45 to 60 minutes
- says the dye must be mixed in a bowl before it is brushed on

## variant
- change: no brush, so use a clean sponge to spread the dye
- request: I don't have a tint brush. Can I use something else to put the dye on?
- situation: I have hair dye, developer, a mixing bowl, gloves and a clean sponge, but no brush.
- expect: The reply uses the clean sponge in place of the brush to spread the dye and keeps the mixing, waiting and rinsing steps unchanged.

```datalog situation
has(user, hair_dye).
has(user, developer).
has(user, bowl).
has(user, gloves).
has(user, sponge).
lacks(user, brush).
```

```datalog difference
swap(dye_hair_at_home_s2, brush, sponge).
no_need(brush).
extra_need(sponge).
```

## variant
- change: add a conditioner step after rinsing
- request: I want to finish with conditioner after I rinse the dye out. How do I fit that in?
- situation: I have hair dye, developer, a mixing bowl, a tint brush, gloves and conditioner, and I want conditioner at the end.
- expect: The reply keeps the four base steps and adds a conditioner step after rinsing, taking about 2 to 3 minutes.

```datalog situation
has(user, hair_dye).
has(user, developer).
has(user, bowl).
has(user, brush).
has(user, gloves).
has(user, conditioner).
wants(user, conditioner).
```

```datalog difference
add_step(dye_hair_at_home_s9, dye_hair_at_home_s4).
step(dye_hair_at_home, dye_hair_at_home_s9).
action(dye_hair_at_home_s9, apply).
object_of(dye_hair_at_home_s9, hair).
uses(dye_hair_at_home_s9, conditioner).
role(dye_hair_at_home_s9, conditioner, coat).
takes_time(dye_hair_at_home_s9, 2, 3, minute).
before(dye_hair_at_home_s4, dye_hair_at_home_s9).
extra_need(conditioner).
```
