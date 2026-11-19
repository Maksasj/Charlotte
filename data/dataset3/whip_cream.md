---
id: whip_cream
stage: procedures
task: whip_cream
schema: v3
---
# Whip cream

## requests
- I want to make whipped cream for a dessert tonight. What do I need and how do I do it?
- How do I whip cream by hand with a whisk?
- Can you walk me through making whipping cream with sugar in a bowl?

## situation
- I have heavy cream, sugar, a bowl, a whisk and a fridge.
- I have about half an hour before the dessert is served.

## ground truth
```datalog
task(whip_cream).
domain(whip_cream, cooking).
goal(whip_cream, whipped_cream).
needs(whip_cream, heavy_cream).
needs(whip_cream, sugar).
needs(whip_cream, bowl).
needs(whip_cream, whisk).
needs(whip_cream, fridge).
takes_time(whip_cream, 20, 25, minute).
step(whip_cream, whip_cream_s1).
action(whip_cream_s1, cool).
object_of(whip_cream_s1, bowl).
uses(whip_cream_s1, fridge).
role(whip_cream_s1, fridge, cool).
takes_time(whip_cream_s1, 15, 15, minute).
purpose(whip_cream_s1, whips_faster).
step(whip_cream, whip_cream_s2).
action(whip_cream_s2, pour).
object_of(whip_cream_s2, heavy_cream).
into(whip_cream_s2, bowl).
amount(whip_cream_s2, heavy_cream, 250, millilitre).
step(whip_cream, whip_cream_s3).
action(whip_cream_s3, add).
object_of(whip_cream_s3, sugar).
into(whip_cream_s3, bowl).
amount(whip_cream_s3, sugar, 25, gram).
step(whip_cream, whip_cream_s4).
action(whip_cream_s4, beat).
object_of(whip_cream_s4, heavy_cream).
uses(whip_cream_s4, whisk).
role(whip_cream_s4, whisk, beat).
until(whip_cream_s4, soft_peaks).
takes_time(whip_cream_s4, 3, 5, minute).
warning(whip_cream_s4, turns_grainy).
before(whip_cream_s1, whip_cream_s2).
before(whip_cream_s2, whip_cream_s3).
before(whip_cream_s3, whip_cream_s4).
```

## aliases
```datalog
task_alias(whip_cream, "whipped cream").
task_alias(whip_cream, "whipping cream").
alias(heavy_cream, "cream").
alias(fridge, "refrigerator").
alias(bowl, "mixing bowl").
alias(cool, "chill").
alias(beat, "whisk").
alias(whips_faster, "whips faster").
alias(soft_peaks, "soft peaks").
alias(turns_grainy, "turns grainy").
```

## teach
- To whip cream you need heavy cream, sugar, a bowl, a whisk and a fridge.
- Whipping cream takes 20 to 25 minutes.
- Step 1 of whipping cream: chill the bowl in the fridge for 15 minutes so the cream whips faster.
- Step 2 of whipping cream: pour 250 millilitres of heavy cream into the bowl.
- Step 3 of whipping cream: add 25 grams of sugar to the bowl.
- Step 4 of whipping cream: whisk the cream until it forms soft peaks, for 3 to 5 minutes, and stop before it turns grainy.

## checklist
- names what is needed, including heavy cream, sugar, a bowl, a whisk and a fridge
- gives the steps in order: chill the bowl, pour the cream, add the sugar, whisk to soft peaks
- says to stop whisking before the cream turns grainy
- gives a total time of about 20 to 25 minutes

## variant
- change: no whisk, so an electric mixer does the whisking in step 4
- request: I don't have a whisk. Can I still make whipped cream?
- situation: I have heavy cream, sugar, a bowl, an electric mixer and a fridge.
- expect: The reply uses the electric mixer in place of the whisk for the whipping step and keeps the other steps the same.

```datalog situation
has(user, heavy_cream).
has(user, sugar).
has(user, bowl).
has(user, mixer).
has(user, fridge).
lacks(user, whisk).
```

```datalog difference
swap(whip_cream_s4, whisk, mixer).
extra_need(mixer).
no_need(whisk).
```

- change: only 10 minutes, so skip the chilling step and whisk straight away
- request: I only have 10 minutes. Is there a quick way to whip some cream?
- situation: I have heavy cream, sugar, a bowl, a whisk and a fridge, and 10 minutes.
- expect: The reply skips the chilling step, pours in the cream and sugar, whisks to soft peaks, and says the whole thing takes about 5 to 7 minutes.

```datalog situation
has(user, heavy_cream).
has(user, sugar).
has(user, bowl).
has(user, whisk).
has(user, fridge).
time_limit(user, 10, minute).
```

```datalog difference
drop(whip_cream_s1).
change(whip_cream, takes_time, 20_25, 5_7).
no_need(fridge).
```
