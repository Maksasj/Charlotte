---
id: clean_cast_iron_pan
stage: procedures
task: clean_cast_iron_pan
schema: v3
---
# Clean a cast iron pan

## requests
- My cast iron pan has stuck-on food. What's the right way to clean it without ruining the seasoning?
- How do you clean a cast iron skillet properly?
- Could you tell me how to get my cast iron pan clean and stop it rusting afterwards?

## situation
- I have a cast iron pan with stuck-on food, coarse salt, a sponge, paper towel, cooking oil and a stove.
- I have about 20 minutes after dinner.

## ground truth
```datalog
task(clean_cast_iron_pan).
domain(clean_cast_iron_pan, cleaning).
goal(clean_cast_iron_pan, clean_pan).
needs(clean_cast_iron_pan, coarse_salt).
needs(clean_cast_iron_pan, sponge).
needs(clean_cast_iron_pan, paper_towel).
needs(clean_cast_iron_pan, cooking_oil).
needs(clean_cast_iron_pan, stove).
takes_time(clean_cast_iron_pan, 15, 20, minute).
step(clean_cast_iron_pan, clean_cast_iron_pan_s1).
action(clean_cast_iron_pan_s1, scrub).
object_of(clean_cast_iron_pan_s1, pan).
uses(clean_cast_iron_pan_s1, coarse_salt).
role(clean_cast_iron_pan_s1, coarse_salt, scrub).
uses(clean_cast_iron_pan_s1, sponge).
role(clean_cast_iron_pan_s1, sponge, scrub).
warning(clean_cast_iron_pan_s1, soap_strips_seasoning).
step(clean_cast_iron_pan, clean_cast_iron_pan_s2).
action(clean_cast_iron_pan_s2, rinse).
object_of(clean_cast_iron_pan_s2, pan).
step(clean_cast_iron_pan, clean_cast_iron_pan_s3).
action(clean_cast_iron_pan_s3, dry).
object_of(clean_cast_iron_pan_s3, pan).
uses(clean_cast_iron_pan_s3, paper_towel).
role(clean_cast_iron_pan_s3, paper_towel, wipe).
step(clean_cast_iron_pan, clean_cast_iron_pan_s4).
action(clean_cast_iron_pan_s4, heat).
object_of(clean_cast_iron_pan_s4, pan).
uses(clean_cast_iron_pan_s4, stove).
role(clean_cast_iron_pan_s4, stove, heat).
setting(clean_cast_iron_pan_s4, heat, medium, level).
takes_time(clean_cast_iron_pan_s4, 3, 5, minute).
purpose(clean_cast_iron_pan_s4, moisture_gone).
step(clean_cast_iron_pan, clean_cast_iron_pan_s5).
action(clean_cast_iron_pan_s5, coat).
object_of(clean_cast_iron_pan_s5, pan).
uses(clean_cast_iron_pan_s5, cooking_oil).
role(clean_cast_iron_pan_s5, cooking_oil, coat).
purpose(clean_cast_iron_pan_s5, prevent_rust).
before(clean_cast_iron_pan_s1, clean_cast_iron_pan_s2).
before(clean_cast_iron_pan_s2, clean_cast_iron_pan_s3).
before(clean_cast_iron_pan_s3, clean_cast_iron_pan_s4).
before(clean_cast_iron_pan_s4, clean_cast_iron_pan_s5).
```

## aliases
```datalog
task_alias(clean_cast_iron_pan, "clean cast iron").
task_alias(clean_cast_iron_pan, "cleaning a cast iron pan").
task_alias(clean_cast_iron_pan, "cast iron skillet").
alias(coarse_salt, "coarse salt").
alias(sponge, "scrubbing sponge").
alias(paper_towel, "paper towel").
alias(cooking_oil, "oil").
alias(stove, "hob").
alias(soap_strips_seasoning, "soap strips the seasoning").
alias(moisture_gone, "moisture is gone").
alias(prevent_rust, "stop it rusting").
alias(medium, "medium heat").
```

## teach
- To clean a cast iron pan you need coarse salt, a sponge, paper towel, cooking oil and a stove.
- Cleaning a cast iron pan takes 15 to 20 minutes.
- Step 1 of cleaning a cast iron pan: scrub the pan with coarse salt and a sponge, without soap, so the seasoning is not stripped.
- Step 2 of cleaning a cast iron pan: rinse the pan under hot water.
- Step 3 of cleaning a cast iron pan: dry the pan with paper towel.
- Step 4 of cleaning a cast iron pan: heat the pan on the stove on medium for 3 to 5 minutes so all the moisture is gone.
- Step 5 of cleaning a cast iron pan: coat the warm pan with a thin layer of cooking oil to stop it rusting.

## checklist
- names what is needed, including coarse salt, a sponge, paper towel, cooking oil and a stove
- gives the steps in order: scrub with salt and sponge without soap, rinse, dry, heat until dry, coat with oil
- says to avoid soap so the seasoning is not stripped
- gives a total time of about 15 to 20 minutes

## variant
- change: no coarse salt: scrub with baking soda instead.
- request: I don't have any coarse salt. Can I use baking soda to scrub my cast iron pan?
- situation: I have a cast iron pan with stuck-on food, baking soda, a sponge, paper towel, cooking oil and a stove.
- expect: The reply scrubs with baking soda and the sponge instead of coarse salt, then rinses, dries, heats and oils the pan as usual.

```datalog situation
lacks(user, coarse_salt).
has(user, baking_soda).
has(user, sponge).
has(user, paper_towel).
has(user, cooking_oil).
has(user, stove).
```

```datalog difference
swap(clean_cast_iron_pan_s1, coarse_salt, baking_soda).
extra_need(baking_soda).
no_need(coarse_salt).
```

## variant
- change: no cooking oil: coat the pan with vegetable shortening instead.
- request: I'm out of cooking oil. What can I rub on my cast iron pan to protect it?
- situation: I have a cast iron pan, coarse salt, a sponge, paper towel, vegetable shortening and a stove.
- expect: The reply cleans the pan as usual and then coats the warm pan with vegetable shortening instead of cooking oil to stop it rusting.

```datalog situation
lacks(user, cooking_oil).
has(user, vegetable_shortening).
has(user, coarse_salt).
has(user, sponge).
has(user, paper_towel).
has(user, stove).
```

```datalog difference
swap(clean_cast_iron_pan_s5, cooking_oil, vegetable_shortening).
extra_need(vegetable_shortening).
no_need(cooking_oil).
```
