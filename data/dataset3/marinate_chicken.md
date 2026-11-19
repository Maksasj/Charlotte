---
id: marinate_chicken
stage: procedures
task: marinate_chicken
schema: v3
---
# Marinate chicken

## requests
- How do I marinate chicken with olive oil and lemon before cooking it?
- What's the way to get chicken flavoured ahead of time? I've got oil, lemon and salt.
- Could you talk me through marinating chicken so it's ready for tomorrow's dinner?

## situation
- I have chicken, olive oil, a lemon, salt, a bowl and a fridge.
- I want it ready for dinner tomorrow.

## ground truth
```datalog
task(marinate_chicken).
domain(marinate_chicken, cooking).
goal(marinate_chicken, marinated_chicken).
needs(marinate_chicken, chicken).
needs(marinate_chicken, olive_oil).
needs(marinate_chicken, lemon).
needs(marinate_chicken, salt).
needs(marinate_chicken, bowl).
needs(marinate_chicken, fridge).
takes_time(marinate_chicken, 120, 240, minute).
step(marinate_chicken, marinate_chicken_s1).
action(marinate_chicken_s1, pour).
object_of(marinate_chicken_s1, olive_oil).
into(marinate_chicken_s1, bowl).
uses(marinate_chicken_s1, bowl).
role(marinate_chicken_s1, bowl, contain).
step(marinate_chicken, marinate_chicken_s2).
action(marinate_chicken_s2, squeeze).
object_of(marinate_chicken_s2, lemon).
into(marinate_chicken_s2, bowl).
step(marinate_chicken, marinate_chicken_s3).
action(marinate_chicken_s3, sprinkle).
object_of(marinate_chicken_s3, salt).
onto(marinate_chicken_s3, chicken).
uses(marinate_chicken_s3, salt).
role(marinate_chicken_s3, salt, season).
step(marinate_chicken, marinate_chicken_s4).
action(marinate_chicken_s4, turn).
object_of(marinate_chicken_s4, chicken).
uses(marinate_chicken_s4, bowl).
role(marinate_chicken_s4, bowl, hold).
step(marinate_chicken, marinate_chicken_s5).
action(marinate_chicken_s5, chill).
object_of(marinate_chicken_s5, chicken).
uses(marinate_chicken_s5, fridge).
role(marinate_chicken_s5, fridge, cool).
setting(marinate_chicken_s5, temperature, 4, celsius).
takes_time(marinate_chicken_s5, 120, 240, minute).
purpose(marinate_chicken_s5, flavour_soaks_in).
before(marinate_chicken_s1, marinate_chicken_s2).
before(marinate_chicken_s2, marinate_chicken_s3).
before(marinate_chicken_s3, marinate_chicken_s4).
before(marinate_chicken_s4, marinate_chicken_s5).
```

## aliases
```datalog
task_alias(marinate_chicken, "marinate chicken").
task_alias(marinate_chicken, "marinading chicken").
alias(chicken, "chicken pieces").
alias(olive_oil, "oil").
alias(bowl, "mixing bowl").
alias(fridge, "refrigerator").
alias(chill, "refrigerate").
alias(flavour_soaks_in, "flavour soaks in").
```

## teach
- To marinate chicken you need chicken, olive oil, a lemon, salt, a bowl and a fridge.
- Marinating chicken takes 120 to 240 minutes.
- Step 1 of marinate chicken: pour the olive oil into the bowl.
- Step 2 of marinate chicken: squeeze the lemon into the bowl.
- Step 3 of marinate chicken: sprinkle the salt onto the chicken.
- Step 4 of marinate chicken: turn the chicken over in the bowl.
- Step 5 of marinate chicken: chill the chicken in the fridge at 4 degrees for 120 to 240 minutes so the flavour soaks in.

## checklist
- names what is needed, including chicken, olive oil, a lemon, salt, a bowl and a fridge
- gives the steps in order: oil into bowl, squeeze lemon, sprinkle salt, turn chicken, chill
- gives a total time of about 120 to 240 minutes
- says the chicken is chilled so the flavour soaks in

## variant
- change: no bowl: marinate in a zip-top bag instead of a bowl
- request: I don't own a mixing bowl. Can I marinate chicken in a zip-top bag instead?
- situation: I have chicken, olive oil, a lemon, salt, a zip-top bag and a fridge, but no bowl.
- expect: The reply uses the zip-top bag in place of the bowl for pouring the oil, turning the chicken and chilling it in the fridge.

```datalog situation
has(user, chicken).
has(user, olive_oil).
has(user, lemon).
has(user, salt).
has(user, zip_bag).
has(user, fridge).
lacks(user, bowl).
```

```datalog difference
swap(marinate_chicken_s1, bowl, zip_bag).
swap(marinate_chicken_s4, bowl, zip_bag).
no_need(bowl).
extra_need(zip_bag).
```

## variant
- change: only 90 minutes: shorten the chill to 60 to 90 minutes
- request: I want marinated chicken but I only have 90 minutes before I need to cook it. What do I do?
- situation: I have chicken, olive oil, a lemon, salt, a bowl and a fridge, and 90 minutes.
- expect: The reply gives the same steps and shortens the chill to about 60 to 90 minutes, saying the marinade will be lighter than a longer soak.

```datalog situation
has(user, chicken).
has(user, olive_oil).
has(user, lemon).
has(user, salt).
has(user, bowl).
has(user, fridge).
time_limit(user, 90, minute).
```

```datalog difference
change(marinate_chicken, takes_time, 120_240, 60_90).
change(marinate_chicken_s5, takes_time, 120_240, 60_90).
```
