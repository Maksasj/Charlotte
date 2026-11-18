---
id: make_tomato_soup
stage: procedures
task: make_tomato_soup
schema: v3
tier: T
---
# Make tomato soup

## requests
- Can you tell me how to make a simple tomato soup from tins?
- It's freezing out and I fancy a bowl of homemade tomato soup. How do I make it?
- I've got a few tins of tomatoes in the cupboard. What warm lunch could I cook with them, step by step?

## situation
- I have an onion, garlic, olive oil, two tins of chopped tomatoes, vegetable stock, a pot, a knife, a stick blender and salt.

## ground truth
```datalog
task(make_tomato_soup).
domain(make_tomato_soup, cooking).
goal(make_tomato_soup, tomato_soup).
goal(make_tomato_soup, warm_meal).
needs(make_tomato_soup, onion).
needs(make_tomato_soup, garlic).
needs(make_tomato_soup, olive_oil).
needs(make_tomato_soup, canned_tomato).
needs(make_tomato_soup, vegetable_stock).
needs(make_tomato_soup, pot).
needs(make_tomato_soup, knife).
needs(make_tomato_soup, blender).
needs(make_tomato_soup, salt).
serves(make_tomato_soup, 4).
takes_time(make_tomato_soup, 30, 40, minute).
step(make_tomato_soup, make_tomato_soup_s1).
action(make_tomato_soup_s1, chop).
object_of(make_tomato_soup_s1, onion).
uses(make_tomato_soup_s1, knife).
role(make_tomato_soup_s1, knife, cut).
step(make_tomato_soup, make_tomato_soup_s2).
action(make_tomato_soup_s2, fry).
object_of(make_tomato_soup_s2, onion).
uses(make_tomato_soup_s2, olive_oil).
role(make_tomato_soup_s2, olive_oil, heat).
uses(make_tomato_soup_s2, pot).
role(make_tomato_soup_s2, pot, heat).
uses(make_tomato_soup_s2, garlic).
role(make_tomato_soup_s2, garlic, flavour).
until(make_tomato_soup_s2, soft).
takes_time(make_tomato_soup_s2, 5, 8, minute).
step(make_tomato_soup, make_tomato_soup_s3).
action(make_tomato_soup_s3, add).
object_of(make_tomato_soup_s3, canned_tomato).
into(make_tomato_soup_s3, pot).
step(make_tomato_soup, make_tomato_soup_s4).
action(make_tomato_soup_s4, pour).
object_of(make_tomato_soup_s4, vegetable_stock).
into(make_tomato_soup_s4, pot).
step(make_tomato_soup, make_tomato_soup_s5).
action(make_tomato_soup_s5, simmer).
object_of(make_tomato_soup_s5, soup).
uses(make_tomato_soup_s5, pot).
role(make_tomato_soup_s5, pot, heat).
takes_time(make_tomato_soup_s5, 15, 20, minute).
step(make_tomato_soup, make_tomato_soup_s6).
action(make_tomato_soup_s6, blend).
object_of(make_tomato_soup_s6, soup).
uses(make_tomato_soup_s6, blender).
role(make_tomato_soup_s6, blender, blend).
warning(make_tomato_soup_s6, hot_splash).
check(make_tomato_soup_s6, smooth).
step(make_tomato_soup, make_tomato_soup_s7).
action(make_tomato_soup_s7, season).
object_of(make_tomato_soup_s7, soup).
uses(make_tomato_soup_s7, salt).
role(make_tomato_soup_s7, salt, season).
before(make_tomato_soup_s1, make_tomato_soup_s2).
before(make_tomato_soup_s2, make_tomato_soup_s3).
before(make_tomato_soup_s3, make_tomato_soup_s4).
before(make_tomato_soup_s4, make_tomato_soup_s5).
before(make_tomato_soup_s5, make_tomato_soup_s6).
before(make_tomato_soup_s6, make_tomato_soup_s7).
```

## aliases
```datalog
task_alias(make_tomato_soup, "tomato soup").
task_alias(make_tomato_soup, "homemade tomato soup").
task_alias(make_tomato_soup, "making soup from tins").
alias(canned_tomato, "tinned tomatoes").
alias(canned_tomato, "chopped tomatoes").
alias(blender, "stick blender").
alias(vegetable_stock, "stock").
alias(hot_splash, "hot soup can splash").
```

## teach
- To make tomato soup you need an onion, garlic, olive oil, tinned tomatoes, vegetable stock, a pot, a knife, a stick blender and salt.
- Making tomato soup takes 30 to 40 minutes and serves 4.
- Step 1 of making tomato soup: chop the onion with the knife.
- Step 2 of making tomato soup: fry the onion with the garlic in olive oil in the pot until soft, for 5 to 8 minutes.
- Step 3 of making tomato soup: add the tinned tomatoes to the pot.
- Step 4 of making tomato soup: pour the vegetable stock into the pot.
- Step 5 of making tomato soup: simmer the soup for 15 to 20 minutes.
- Step 6 of making tomato soup: blend the soup smooth with the stick blender; take care, hot soup can splash.
- Step 7 of making tomato soup: season the soup with salt.

## checklist
- names what is needed, including onion, tinned tomatoes, stock and a blender
- gives the steps in order: chop, fry, add tomatoes, add stock, simmer, blend, season
- warns that hot soup can splash when blending
- gives a total time of about 30 to 40 minutes

## variant
- change: no blender: leave the soup chunky
- request: My blender broke last week. Can I still make tomato soup?
- situation: I have an onion, garlic, olive oil, tinned tomatoes, vegetable stock, a pot, a knife and salt, but no blender.
- expect: The reply makes the soup the same way but skips blending, so it is served as a chunky soup.

```datalog situation
has(user, onion).
has(user, garlic).
has(user, olive_oil).
has(user, canned_tomato).
has(user, vegetable_stock).
has(user, pot).
has(user, knife).
has(user, salt).
lacks(user, blender).
```

```datalog difference
drop(make_tomato_soup_s6).
no_need(blender).
```

## variant
- change: wants it creamy: stir in cream at the end
- request: I like my tomato soup creamy. How do I do that?
- situation: I have everything for the soup plus a small pot of cream.
- expect: The reply stirs a little cream into the blended soup before seasoning.

```datalog situation
has(user, onion).
has(user, garlic).
has(user, olive_oil).
has(user, canned_tomato).
has(user, vegetable_stock).
has(user, pot).
has(user, knife).
has(user, blender).
has(user, salt).
has(user, cream).
wants(user, creamy).
```

```datalog difference
extra_need(cream).
add_step(make_tomato_soup_s9, make_tomato_soup_s6).
step(make_tomato_soup, make_tomato_soup_s9).
action(make_tomato_soup_s9, stir).
object_of(make_tomato_soup_s9, cream).
into(make_tomato_soup_s9, soup).
before(make_tomato_soup_s6, make_tomato_soup_s9).
before(make_tomato_soup_s9, make_tomato_soup_s7).
```
