---
id: make_chili_con_carne
stage: procedures
task: make_chili_con_carne
---
# Make chili con carne

## requests
- How do I make a pot of chili con carne?
- Can you walk me through cooking chili with mince and kidney beans?
- I want to make a big batch of chili for the week. What are the steps?

## situation
- I have minced beef, an onion, a tin of kidney beans, a tin of chopped tomatoes, chili powder, oil, a knife and a large pot.
- Dinner is in about an hour and a quarter.

## ground truth
```datalog
task(make_chili_con_carne).
needs(make_chili_con_carne, minced_beef).
needs(make_chili_con_carne, onion).
needs(make_chili_con_carne, kidney_bean).
needs(make_chili_con_carne, chopped_tomato).
needs(make_chili_con_carne, chili_powder).
needs(make_chili_con_carne, oil).
needs(make_chili_con_carne, knife).
needs(make_chili_con_carne, pot).
takes_time(make_chili_con_carne, 45, 60, minute).
step(make_chili_con_carne, make_chili_con_carne_s1).
action(make_chili_con_carne_s1, chop).
object_of(make_chili_con_carne_s1, onion).
uses(make_chili_con_carne_s1, knife).
takes_time(make_chili_con_carne_s1, 3, 4, minute).
step(make_chili_con_carne, make_chili_con_carne_s2).
action(make_chili_con_carne_s2, fry).
object_of(make_chili_con_carne_s2, onion).
uses(make_chili_con_carne_s2, pot).
uses(make_chili_con_carne_s2, oil).
until(make_chili_con_carne_s2, soft).
takes_time(make_chili_con_carne_s2, 5, 6, minute).
step(make_chili_con_carne, make_chili_con_carne_s3).
action(make_chili_con_carne_s3, brown).
object_of(make_chili_con_carne_s3, minced_beef).
uses(make_chili_con_carne_s3, pot).
takes_time(make_chili_con_carne_s3, 5, 6, minute).
step(make_chili_con_carne, make_chili_con_carne_s4).
action(make_chili_con_carne_s4, add).
object_of(make_chili_con_carne_s4, chopped_tomato).
uses(make_chili_con_carne_s4, kidney_bean).
uses(make_chili_con_carne_s4, chili_powder).
takes_time(make_chili_con_carne_s4, 1, 2, minute).
step(make_chili_con_carne, make_chili_con_carne_s5).
action(make_chili_con_carne_s5, simmer).
object_of(make_chili_con_carne_s5, chili).
uses(make_chili_con_carne_s5, pot).
takes_time(make_chili_con_carne_s5, 30, 40, minute).
before(make_chili_con_carne_s1, make_chili_con_carne_s2).
before(make_chili_con_carne_s2, make_chili_con_carne_s3).
before(make_chili_con_carne_s3, make_chili_con_carne_s4).
before(make_chili_con_carne_s4, make_chili_con_carne_s5).
```

## teach
- To make chili con carne you need minced beef, an onion, a tin of kidney beans, a tin of chopped tomatoes, chili powder, oil, a knife and a large pot.
- Making chili con carne takes 45 to 60 minutes.
- Step 1 of making chili con carne: chop the onion with the knife for 3 to 4 minutes.
- Step 2 of making chili con carne: fry the onion in the oil in the pot until soft, for 5 to 6 minutes.
- Step 3 of making chili con carne: brown the minced beef with the onion in the pot for 5 to 6 minutes.
- Step 4 of making chili con carne: add the chopped tomatoes, the kidney beans and the chili powder, for 1 to 2 minutes.
- Step 5 of making chili con carne: simmer the chili in the pot with the lid half on, stirring now and then, for 30 to 40 minutes.

## checklist
- names what is needed: minced beef, an onion, a tin of kidney beans, a tin of chopped tomatoes, chili powder, oil, a knife and a large pot
- gives the steps in order: chop onion, fry onion, brown minced beef, add chopped tomato, simmer chili
- gives a total time of about 45 to 60 minutes
- closes politely

## variant
- change: vegetarian: use a tin of lentils in place of the beef and skip the browning
- request: My sister's coming and she doesn't eat meat. Can I make the chili without beef?
- situation: I have a tin of lentils, an onion, a tin of kidney beans, a tin of chopped tomatoes, chili powder, oil, a knife and a large pot.
- expect: The reply leaves out the beef and the browning step, adds the drained lentils with the tomatoes and beans, and shortens the simmer to about 20 minutes.
