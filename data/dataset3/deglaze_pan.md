---
id: deglaze_pan
stage: procedures
task: deglaze_pan
schema: v3
---
# Deglaze a pan

## requests
- I just seared some steaks. How do I turn the browned bits in the pan into a sauce with wine?
- What's the method for making a quick pan sauce after searing meat?
- Can you explain how to deglaze a pan? I want something to pour over the steak.

## situation
- I've just seared steaks in a pan on the stove and I have wine, salt and a wooden spoon.
- I'd like the sauce ready in about ten minutes.

## ground truth
```datalog
task(deglaze_pan).
domain(deglaze_pan, cooking).
goal(deglaze_pan, pan_sauce).
needs(deglaze_pan, pan).
needs(deglaze_pan, wine).
needs(deglaze_pan, wooden_spoon).
needs(deglaze_pan, stove).
needs(deglaze_pan, salt).
takes_time(deglaze_pan, 5, 8, minute).
step(deglaze_pan, deglaze_pan_s1).
action(deglaze_pan_s1, pour).
object_of(deglaze_pan_s1, wine).
into(deglaze_pan_s1, pan).
uses(deglaze_pan_s1, pan).
role(deglaze_pan_s1, pan, contain).
requires_state(deglaze_pan_s1, hot).
step(deglaze_pan, deglaze_pan_s2).
action(deglaze_pan_s2, scrape).
object_of(deglaze_pan_s2, pan).
uses(deglaze_pan_s2, wooden_spoon).
role(deglaze_pan_s2, wooden_spoon, loosen).
purpose(deglaze_pan_s2, browned_bits_lift).
step(deglaze_pan, deglaze_pan_s3).
action(deglaze_pan_s3, simmer).
object_of(deglaze_pan_s3, wine).
uses(deglaze_pan_s3, stove).
role(deglaze_pan_s3, stove, heat).
uses(deglaze_pan_s3, pan).
role(deglaze_pan_s3, pan, contain).
setting(deglaze_pan_s3, heat, low, level).
takes_time(deglaze_pan_s3, 3, 5, minute).
until(deglaze_pan_s3, reduced_by_half).
check(deglaze_pan_s3, syrupy).
warning(deglaze_pan_s3, boils_dry).
step(deglaze_pan, deglaze_pan_s4).
action(deglaze_pan_s4, season).
object_of(deglaze_pan_s4, sauce).
uses(deglaze_pan_s4, salt).
role(deglaze_pan_s4, salt, season).
before(deglaze_pan_s1, deglaze_pan_s2).
before(deglaze_pan_s2, deglaze_pan_s3).
before(deglaze_pan_s3, deglaze_pan_s4).
```

## aliases
```datalog
task_alias(deglaze_pan, "deglazing a pan").
task_alias(deglaze_pan, "pan sauce").
alias(pan, "frying pan").
alias(wine, "red wine").
alias(wooden_spoon, "wooden spoon").
alias(stove, "hob").
alias(browned_bits_lift, "lift the browned bits").
alias(reduced_by_half, "reduced by half").
alias(boils_dry, "boils dry").
alias(syrupy, "syrupy").
alias(hot, "hot").
```

## teach
- To deglaze a pan you need a pan, wine, a wooden spoon, a stove and salt.
- Deglazing a pan takes 5 to 8 minutes.
- Step 1 of deglazing a pan: pour the wine into the hot pan.
- Step 2 of deglazing a pan: scrape the browned bits off the pan with the wooden spoon to lift the flavour.
- Step 3 of deglazing a pan: simmer the wine on the stove on low heat for 3 to 5 minutes until it is reduced by half and syrupy, and do not let it boil dry.
- Step 4 of deglazing a pan: season the sauce with salt.

## checklist
- names what is needed, including a pan, wine, a wooden spoon, a stove and salt
- gives the steps in order: pour the wine, scrape the browned bits, simmer until reduced by half, season with salt
- says to simmer on low heat and not let the wine boil dry
- gives a total time of about 5 to 8 minutes

## variant
- change: no wine: use stock in its place, with the same steps
- request: I don't have any wine. Can I still make the pan sauce with stock?
- situation: I have stock, salt, a wooden spoon, a pan and a stove, but no wine.
- expect: The reply uses stock where the wine was in the pouring and simmering steps, keeps the scraping and seasoning steps, and gives the same 5 to 8 minute total.

```datalog situation
lacks(user, wine).
has(user, stock).
has(user, pan).
has(user, wooden_spoon).
has(user, stove).
has(user, salt).
```

```datalog difference
swap(deglaze_pan_s1, wine, stock).
swap(deglaze_pan_s3, wine, stock).
no_need(wine).
extra_need(stock).
```

## variant
- change: a glossier sauce: whisk butter into the simmered sauce before seasoning
- request: I'd like a glossier pan sauce. Can I finish it with butter? I have a whisk.
- situation: I have wine, salt, a wooden spoon, a pan, a stove, butter and a whisk.
- expect: The reply adds a step after the simmering where the sauce is whisked with about 15 grams of butter, then seasons it with salt.

```datalog situation
has(user, wine).
has(user, pan).
has(user, wooden_spoon).
has(user, stove).
has(user, salt).
has(user, butter).
has(user, whisk).
wants(user, glossy_sauce).
```

```datalog difference
add_step(deglaze_pan_s9, deglaze_pan_s3).
step(deglaze_pan, deglaze_pan_s9).
action(deglaze_pan_s9, beat).
object_of(deglaze_pan_s9, sauce).
uses(deglaze_pan_s9, whisk).
role(deglaze_pan_s9, whisk, beat).
amount(deglaze_pan_s9, butter, 15, gram).
before(deglaze_pan_s3, deglaze_pan_s9).
before(deglaze_pan_s9, deglaze_pan_s4).
extra_need(butter).
extra_need(whisk).
```
