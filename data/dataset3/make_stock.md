---
id: make_stock
stage: procedures
task: make_stock
schema: v3
---
# Make stock

## requests
- I want to make a pot of chicken stock from scratch. What's the method?
- How do you make homemade broth from chicken bones?
- Could you tell me how to simmer a stock for soup? I've never made one.

## situation
- I have chicken bones, water, an onion, a pot, a stove and a strainer.
- I have a free afternoon and no time pressure.

## ground truth
```datalog
task(make_stock).
domain(make_stock, cooking).
goal(make_stock, stock).
goal(make_stock, soup_base).
needs(make_stock, chicken_bones).
needs(make_stock, water).
needs(make_stock, onion).
needs(make_stock, pot).
needs(make_stock, stove).
needs(make_stock, strainer).
takes_time(make_stock, 90, 120, minute).
step(make_stock, make_stock_s1).
action(make_stock_s1, heat).
object_of(make_stock_s1, water).
uses(make_stock_s1, pot).
role(make_stock_s1, pot, contain).
uses(make_stock_s1, stove).
role(make_stock_s1, stove, heat).
setting(make_stock_s1, heat, high, level).
takes_time(make_stock_s1, 10, 15, minute).
step(make_stock, make_stock_s2).
action(make_stock_s2, add).
object_of(make_stock_s2, chicken_bones).
into(make_stock_s2, pot).
step(make_stock, make_stock_s3).
action(make_stock_s3, add).
object_of(make_stock_s3, onion).
into(make_stock_s3, pot).
step(make_stock, make_stock_s4).
action(make_stock_s4, simmer).
object_of(make_stock_s4, stock).
uses(make_stock_s4, stove).
role(make_stock_s4, stove, heat).
setting(make_stock_s4, heat, low, level).
takes_time(make_stock_s4, 60, 90, minute).
until(make_stock_s4, reduced).
purpose(make_stock_s4, flavour_develops).
step(make_stock, make_stock_s5).
action(make_stock_s5, strain).
object_of(make_stock_s5, stock).
from(make_stock_s5, pot).
uses(make_stock_s5, strainer).
role(make_stock_s5, strainer, hold).
before(make_stock_s1, make_stock_s2).
before(make_stock_s2, make_stock_s3).
before(make_stock_s3, make_stock_s4).
before(make_stock_s4, make_stock_s5).
```

## aliases
```datalog
task_alias(make_stock, "stock").
task_alias(make_stock, "homemade stock").
task_alias(make_stock, "chicken broth").
alias(chicken_bones, "chicken bones").
alias(chicken_bones, "carcass").
alias(strainer, "sieve").
alias(stove, "hob").
alias(simmer, "gently simmer").
alias(stock, "broth").
alias(reduced, "reduced").
alias(flavour_develops, "flavour develops").
```

## teach
- To make stock you need chicken bones, water, an onion, a pot, a stove and a strainer.
- Making stock takes 90 to 120 minutes.
- Step 1 of making stock: heat the water in the pot on the stove on high for 10 to 15 minutes.
- Step 2 of making stock: add the chicken bones to the pot.
- Step 3 of making stock: add the onion to the pot.
- Step 4 of making stock: simmer the stock on low heat for 60 to 90 minutes until reduced, so the flavour develops.
- Step 5 of making stock: strain the stock from the pot through the strainer.

## checklist
- names what is needed, including chicken bones, water, an onion, a pot, a stove and a strainer
- gives the steps in order: heat water, add bones, add onion, simmer until reduced, strain
- says the simmer is on low heat and runs until the stock is reduced
- gives a total time of about 90 to 120 minutes

## variant
- change: only 45 minutes: shorten the simmer to 20 to 25 minutes instead of 60 to 90
- request: I need stock for tonight but I only have about 45 minutes. Can I still make it?
- situation: I have chicken bones, water, an onion, a pot, a stove and a strainer, and about 45 minutes.
- expect: The reply keeps the same steps but shortens the simmer to about 20 to 25 minutes, giving a total of about 40 to 50 minutes.

```datalog situation
has(user, chicken_bones).
has(user, water).
has(user, onion).
has(user, pot).
has(user, stove).
has(user, strainer).
time_limit(user, 45, minute).
```

```datalog difference
change(make_stock_s4, takes_time, 60_90, 20_25).
change(make_stock, takes_time, 90_120, 40_50).
```

## variant
- change: no onion: leave out the onion step
- request: I don't eat onion. Can you show me how to make stock without it?
- situation: I have chicken bones, water, a pot, a stove and a strainer, and I want to avoid onion.
- expect: The reply gives the same method without adding onion, and says the onion is left out of the stock.

```datalog situation
has(user, chicken_bones).
has(user, water).
has(user, pot).
has(user, stove).
has(user, strainer).
excludes(user, onion).
```

```datalog difference
drop(make_stock_s3).
no_need(onion).
```
