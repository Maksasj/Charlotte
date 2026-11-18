---
id: bake_muffin
stage: procedures
task: bake_muffin
schema: v3
tier: T
---
# Bake muffins

## requests
- How do I bake a batch of plain muffins?
- I'd like to make a dozen little cakes in a muffin tray for the school bake sale. What's the method?
- The kids want to bake something this afternoon. Could you take me through muffins?

## situation
- I have flour, sugar, baking powder, eggs, milk, melted butter, a bowl, a spoon, paper cases, a 12-hole muffin tin and an oven.

## ground truth
```datalog
task(bake_muffin).
domain(bake_muffin, cooking).
goal(bake_muffin, muffin).
goal(bake_muffin, snack).
needs(bake_muffin, flour).
needs(bake_muffin, sugar).
needs(bake_muffin, baking_powder).
needs(bake_muffin, egg).
needs(bake_muffin, milk).
needs(bake_muffin, butter).
needs(bake_muffin, bowl).
needs(bake_muffin, spoon).
needs(bake_muffin, paper_case).
needs(bake_muffin, muffin_tin).
needs(bake_muffin, oven).
serves(bake_muffin, 12).
takes_time(bake_muffin, 35, 45, minute).
step(bake_muffin, bake_muffin_s1).
action(bake_muffin_s1, heat).
object_of(bake_muffin_s1, oven).
setting(bake_muffin_s1, temperature, 190, celsius).
takes_time(bake_muffin_s1, 10, 10, minute).
step(bake_muffin, bake_muffin_s2).
action(bake_muffin_s2, mix).
object_of(bake_muffin_s2, flour).
into(bake_muffin_s2, bowl).
uses(bake_muffin_s2, bowl).
role(bake_muffin_s2, bowl, contain).
uses(bake_muffin_s2, sugar).
role(bake_muffin_s2, sugar, sweeten).
uses(bake_muffin_s2, baking_powder).
role(bake_muffin_s2, baking_powder, raise).
amount(bake_muffin_s2, flour, 250, gram).
overlaps(bake_muffin_s2, bake_muffin_s1).
step(bake_muffin, bake_muffin_s3).
action(bake_muffin_s3, stir).
object_of(bake_muffin_s3, batter).
uses(bake_muffin_s3, egg).
role(bake_muffin_s3, egg, bind).
uses(bake_muffin_s3, milk).
role(bake_muffin_s3, milk, moisten).
uses(bake_muffin_s3, butter).
role(bake_muffin_s3, butter, flavour).
uses(bake_muffin_s3, spoon).
role(bake_muffin_s3, spoon, stir).
avoid(bake_muffin_s3, overmix, batter).
overlaps(bake_muffin_s3, bake_muffin_s1).
step(bake_muffin, bake_muffin_s4).
action(bake_muffin_s4, line).
object_of(bake_muffin_s4, muffin_tin).
uses(bake_muffin_s4, paper_case).
role(bake_muffin_s4, paper_case, contain).
overlaps(bake_muffin_s4, bake_muffin_s1).
step(bake_muffin, bake_muffin_s5).
action(bake_muffin_s5, fill).
object_of(bake_muffin_s5, paper_case).
uses(bake_muffin_s5, spoon).
role(bake_muffin_s5, spoon, hold).
amount(bake_muffin_s5, batter, 2, tablespoon).
step(bake_muffin, bake_muffin_s6).
action(bake_muffin_s6, bake).
object_of(bake_muffin_s6, muffin).
uses(bake_muffin_s6, muffin_tin).
role(bake_muffin_s6, muffin_tin, hold).
uses(bake_muffin_s6, oven).
role(bake_muffin_s6, oven, heat).
takes_time(bake_muffin_s6, 20, 25, minute).
check(bake_muffin_s6, golden).
before(bake_muffin_s2, bake_muffin_s3).
before(bake_muffin_s3, bake_muffin_s5).
before(bake_muffin_s4, bake_muffin_s5).
before(bake_muffin_s1, bake_muffin_s6).
before(bake_muffin_s5, bake_muffin_s6).
```

## aliases
```datalog
task_alias(bake_muffin, "baking muffins").
task_alias(bake_muffin, "muffins").
task_alias(bake_muffin, "a batch of muffins").
alias(muffin, "muffins").
alias(paper_case, "muffin cases").
alias(muffin_tin, "muffin tray").
alias(heat, "preheat").
alias(golden, "golden and springy").
alias(overmix, "stir too much").
```

## teach
- To bake muffins you need flour, sugar, baking powder, eggs, milk, melted butter, a bowl, a spoon, paper cases, a muffin tin and an oven.
- Baking muffins takes 35 to 45 minutes and makes 12.
- Step 1 of baking muffins: heat the oven to 190 degrees for 10 minutes.
- While step 1 runs, step 2 of baking muffins: mix 250 grams of flour with the sugar and baking powder in the bowl.
- While step 1 runs, step 3 of baking muffins: stir in the eggs, milk and melted butter with the spoon; do not overmix the batter.
- While step 1 runs, step 4 of baking muffins: line the muffin tin with paper cases.
- Step 5 of baking muffins: fill each paper case with 2 tablespoons of batter using the spoon.
- Step 6 of baking muffins: bake the muffins in the tin in the oven for 20 to 25 minutes until golden.

## checklist
- names what is needed, including flour, sugar, baking powder, eggs, milk, butter and a muffin tin
- gives the steps in order: heat oven, mix dry, stir in wet, line tin, fill cases, bake
- says mixing and lining can be done while the oven heats and warns not to overmix
- gives a bake time of about 20 to 25 minutes

## variant
- change: no paper cases: line the tin with squares of baking paper
- request: I've run out of muffin cases. Can I still bake muffins in the tin?
- situation: I have flour, sugar, baking powder, eggs, milk, butter, a bowl, a spoon, a roll of baking paper, a muffin tin and an oven.
- expect: The reply lines each hole of the tin with a square of baking paper instead of a paper case and fills those.

```datalog situation
has(user, flour).
has(user, sugar).
has(user, baking_powder).
has(user, egg).
has(user, milk).
has(user, butter).
has(user, bowl).
has(user, spoon).
has(user, baking_paper).
has(user, muffin_tin).
has(user, oven).
lacks(user, paper_case).
```

```datalog difference
swap(bake_muffin_s4, paper_case, baking_paper).
change(bake_muffin_s5, object_of, paper_case, baking_paper).
extra_need(baking_paper).
no_need(paper_case).
```

## variant
- change: 24 muffins: double the mix and bake two trays
- request: I need two dozen muffins for the bake sale and I only have one tin. How do I manage?
- situation: I have plenty of flour, sugar, baking powder, eggs, milk and butter, a bowl, a spoon, paper cases, one 12-hole muffin tin and an oven.
- expect: The reply doubles the mix to 500 grams of flour and bakes the muffins in two batches in the one tin.

```datalog situation
has(user, flour).
has(user, sugar).
has(user, baking_powder).
has(user, egg).
has(user, milk).
has(user, butter).
has(user, bowl).
has(user, spoon).
has(user, paper_case).
has(user, muffin_tin).
has(user, oven).
count(user, muffin, 24).
```

```datalog difference
change(bake_muffin_s2, amount, flour_250_gram, flour_500_gram).
change(bake_muffin_s6, repeat, 1, 2).
change(bake_muffin, serves, 12, 24).
change(bake_muffin, takes_time, 35_45, 60_70).
```
